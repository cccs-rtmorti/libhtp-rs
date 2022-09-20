use crate::{
    bstr::Bstr,
    config::{Config, HtpServerPersonality},
    connection::{Connection, Flags},
    decompressors::HtpContentEncoding,
    error::Result,
    hook::DataHook,
    log::Logger,
    transaction::{HtpRequestProgress, HtpResponseProgress, HtpTransferCoding, Transaction},
    transactions::Transactions,
    util::{File, FlagOperations, HtpFileSource, HtpFlags},
    HtpStatus,
};
use chrono::{DateTime, Utc};
use std::{any::Any, borrow::Cow, io::Cursor, net::IpAddr, rc::Rc, time::SystemTime};

/// Enumerates parsing state.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    /// Default state.
    NONE,
    /// State once a transaction is processed or about to be processed.
    IDLE,
    /// State for request/response line parsing.
    LINE,
    /// State for header parsing.
    HEADERS,
    /// State for finalizing chunked body data parsing.
    BODY_CHUNKED_DATA_END,
    /// State for chunked body data.
    BODY_CHUNKED_DATA,
    /// Parse the chunked length state.
    BODY_CHUNKED_LENGTH,
    /// State to determine encoding of body data.
    BODY_DETERMINE,
    /// State for finalizing transaction side.
    FINALIZE,
    // Used by request_state only
    /// State for determining the request protocol.
    PROTOCOL,
    /// State to determine if there is a CONNECT request.
    CONNECT_CHECK,
    /// State to determine if inbound parsing needs to be suspended.
    CONNECT_PROBE_DATA,
    /// State to determine if inbound parsing can continue if it was suspended.
    CONNECT_WAIT_RESPONSE,
    /// State to process request body data.
    BODY_IDENTITY,
    /// State to consume remaining data in request buffer for the HTTP 0.9 case.
    IGNORE_DATA_AFTER_HTTP_0_9,
    // Used by response_state only
    /// State to consume response remaining body data when content-length is unknown.
    BODY_IDENTITY_STREAM_CLOSE,
    /// State to consume response body data when content-length is known.
    BODY_IDENTITY_CL_KNOWN,
}

/// Enumerates all stream states. Each connection has two streams, one
/// inbound and one outbound. Their states are tracked separately.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HtpStreamState {
    /// Default stream state.
    NEW,
    /// State when connection is open.
    OPEN,
    /// State when connection is closed.
    CLOSED,
    /// State when stream produces a fatal error.
    ERROR,
    /// State for a tunnelled stream.
    TUNNEL,
    /// State when parsing is suspended and not consumed in order. This is to
    /// allow processing on another stream.
    DATA_OTHER,
    /// State when we should stop parsing the associated connection.
    STOP,
    /// State when all current data in the stream has been processed.
    DATA,
}

#[derive(Debug, Default, Clone)]
/// This structure is used to pass data (for example
/// request and response body buffers or gaps) to parsers.
pub struct ParserData<'a> {
    /// Ref to the data buffer.
    data: Option<Cow<'a, [u8]>>,
    // Length of data gap. Only set if is a gap.
    gap_len: Option<usize>,
    // Current position offset of the data to parse
    position: usize,
}

impl<'a> ParserData<'a> {
    /// Returns a pointer to the raw data associated with Data.
    pub fn data_ptr(&self) -> *const u8 {
        self.data()
            .as_ref()
            .map(|data| data.as_ptr())
            .unwrap_or(std::ptr::null())
    }

    /// Returns the data
    pub fn data(&self) -> Option<&[u8]> {
        let data = self.data.as_ref()?;
        if self.position <= data.len() {
            Some(&data[self.position..])
        } else {
            None
        }
    }

    /// Returns the length of the data.
    pub fn len(&self) -> usize {
        self.gap_len.unwrap_or(self.as_slice().len())
    }

    /// Return an immutable slice view of the data.
    pub fn as_slice(&self) -> &[u8] {
        if let Some(data) = self.data.as_ref() {
            if self.position <= data.len() {
                return &data[self.position..];
            }
        }
        b""
    }

    /// Determines if this chunk is a gap or not
    pub fn is_gap(&self) -> bool {
        self.gap_len.is_some()
    }

    /// Determine whether this data is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set the position offset into the data for parsing
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    /// Advances the internal position where we are parsing
    pub fn consume(&mut self, consumed: usize) {
        self.set_position(self.position + consumed);
    }

    /// Make an owned version of this data.
    pub fn into_owned(self) -> ParserData<'static> {
        ParserData {
            data: self.data.map(|d| Cow::Owned(d.into_owned())),
            gap_len: self.gap_len,
            position: self.position,
        }
    }
}

impl<'a> From<Option<&'a [u8]>> for ParserData<'a> {
    fn from(data: Option<&'a [u8]>) -> Self {
        ParserData {
            data: data.map(Cow::Borrowed),
            gap_len: None,
            position: 0,
        }
    }
}

impl<'a> From<&'a [u8]> for ParserData<'a> {
    fn from(data: &'a [u8]) -> Self {
        ParserData {
            data: Some(Cow::Borrowed(data)),
            gap_len: None,
            position: 0,
        }
    }
}

impl From<Vec<u8>> for ParserData<'static> {
    fn from(data: Vec<u8>) -> Self {
        ParserData {
            data: Some(Cow::Owned(data)),
            gap_len: None,
            position: 0,
        }
    }
}

impl<'a> From<&'a Vec<u8>> for ParserData<'a> {
    fn from(data: &'a Vec<u8>) -> Self {
        ParserData {
            data: Some(Cow::Borrowed(data.as_slice())),
            gap_len: None,
            position: 0,
        }
    }
}

impl<'a> From<usize> for ParserData<'a> {
    fn from(gap_len: usize) -> Self {
        ParserData {
            data: None,
            gap_len: Some(gap_len),
            position: 0,
        }
    }
}

impl<'a> From<(*const u8, usize)> for ParserData<'a> {
    fn from((data, len): (*const u8, usize)) -> Self {
        if data.is_null() {
            if len > 0 {
                ParserData::from(len)
            } else {
                ParserData::from(b"".as_ref())
            }
        } else {
            unsafe { ParserData::from(std::slice::from_raw_parts(data, len)) }
        }
    }
}

/// Stores information about the parsing process and associated transactions.
pub struct ConnectionParser {
    // General fields
    /// The logger structure associated with this parser
    pub logger: Logger,
    /// A reference to the current parser configuration structure.
    pub cfg: Rc<Config>,
    /// The connection structure associated with this parser.
    pub conn: Connection,
    /// Opaque user data associated with this parser.
    pub user_data: Option<Box<dyn Any>>,
    // Request parser fields
    /// Parser inbound status. Starts as OK, but may turn into ERROR.
    pub request_status: HtpStreamState,
    /// Parser outbound status. Starts as OK, but may turn into ERROR.
    pub response_status: HtpStreamState,
    /// When true, this field indicates that there is unprocessed inbound data, and
    /// that the response parsing code should stop at the end of the current request
    /// in order to allow more requests to be produced.
    pub response_data_other_at_tx_end: bool,
    /// The time when the last request data chunk was received.
    pub request_timestamp: DateTime<Utc>,
    /// Pointer to the current request data chunk.
    pub request_curr_data: Cursor<Vec<u8>>,
    /// Marks the starting point of raw data within the inbound data chunk. Raw
    /// data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
    /// request_header_data).
    pub request_current_receiver_offset: u64,
    /// How many data chunks does the inbound connection stream consist of?
    pub request_chunk_count: usize,
    /// The index of the first chunk used in the current request.
    pub request_chunk_request_index: usize,
    /// Used to buffer a line of inbound data when buffering cannot be avoided.
    pub request_buf: Bstr,
    /// Stores the current value of a folded request header. Such headers span
    /// multiple lines, and are processed only when all data is available.
    pub request_header: Option<Bstr>,
    /// The request body length declared in a valid request header. The key here
    /// is "valid". This field will not be populated if the request contains both
    /// a Transfer-Encoding header and a Content-Length header.
    pub request_content_length: i64,
    /// Holds the remaining request body length that we expect to read. This
    /// field will be available only when the length of a request body is known
    /// in advance, i.e. when request headers contain a Content-Length header.
    pub request_body_data_left: i64,
    /// Holds the amount of data that needs to be read from the
    /// current data chunk. Only used with chunked request bodies.
    pub request_chunked_length: Option<i32>,
    /// Current request parser state.
    pub request_state: State,
    /// Previous request parser state. Used to detect state changes.
    pub request_state_previous: State,
    /// The hook that should be receiving raw connection data.
    pub request_data_receiver_hook: Option<DataHook>,

    // Response parser fields
    /// The time when the last response data chunk was received.
    pub response_timestamp: DateTime<Utc>,
    /// Pointer to the current response data chunk.
    pub response_curr_data: Cursor<Vec<u8>>,
    /// Marks the starting point of raw data within the outbound data chunk. Raw
    /// data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
    /// response_header_data).
    pub response_current_receiver_offset: u64,
    /// Used to buffer a line of outbound data when buffering cannot be avoided.
    pub response_buf: Bstr,
    /// Stores the current value of a folded response header. Such headers span
    /// multiple lines, and are processed only when all data is available.
    pub response_header: Option<Bstr>,
    /// The length of the current response body as presented in the
    /// Content-Length response header.
    pub response_content_length: i64,
    /// The remaining length of the current response body, if known. Set to -1 otherwise.
    pub response_body_data_left: i64,
    /// Holds the amount of data that needs to be read from the
    /// current response data chunk. Only used with chunked response bodies.
    pub response_chunked_length: Option<i32>,
    /// Current response parser state.
    pub response_state: State,
    /// Previous response parser state.
    pub response_state_previous: State,
    /// The hook that should be receiving raw connection data.
    pub response_data_receiver_hook: Option<DataHook>,
    /// On request body data, this field contains additional file data.
    pub request_file: Option<File>,

    /// Transactions processed by this parser
    transactions: Transactions,
}

impl std::fmt::Debug for ConnectionParser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectionParser")
            .field("request_status", &self.request_status)
            .field("response_status", &self.response_status)
            .field("request_index", &self.request_index())
            .field("response_index", &self.response_index())
            .finish()
    }
}

impl ConnectionParser {
    /// Creates a new ConnectionParser with a preconfigured `Config` struct.
    pub fn new(cfg: Config) -> Self {
        let cfg = Rc::new(cfg);
        let conn = Connection::default();
        let logger = Logger::new(conn.get_sender(), cfg.log_level);
        Self {
            logger: logger.clone(),
            cfg: Rc::clone(&cfg),
            conn,
            user_data: None,
            request_status: HtpStreamState::NEW,
            response_status: HtpStreamState::NEW,
            response_data_other_at_tx_end: false,
            request_timestamp: DateTime::<Utc>::from(SystemTime::now()),
            request_curr_data: Cursor::new(Vec::new()),
            request_current_receiver_offset: 0,
            request_chunk_count: 0,
            request_chunk_request_index: 0,
            request_buf: Bstr::new(),
            request_header: None,
            request_content_length: 0,
            request_body_data_left: 0,
            request_chunked_length: None,
            request_state: State::IDLE,
            request_state_previous: State::NONE,
            request_data_receiver_hook: None,
            response_timestamp: DateTime::<Utc>::from(SystemTime::now()),
            response_curr_data: Cursor::new(Vec::new()),
            response_current_receiver_offset: 0,
            response_buf: Bstr::new(),
            response_header: None,
            response_content_length: 0,
            response_body_data_left: 0,
            response_chunked_length: None,
            response_state: State::IDLE,
            response_state_previous: State::NONE,
            response_data_receiver_hook: None,
            request_file: None,
            transactions: Transactions::new(&cfg, &logger),
        }
    }

    /// Get the current request transaction
    pub fn request(&mut self) -> &Transaction {
        self.transactions.request()
    }

    /// Get the current request transaction
    pub fn request_mut(&mut self) -> &mut Transaction {
        self.transactions.request_mut()
    }

    /// Get the current response transaction
    pub fn response(&mut self) -> &Transaction {
        self.transactions.response()
    }

    /// Get the current response transaction
    pub fn response_mut(&mut self) -> &mut Transaction {
        self.transactions.response_mut()
    }

    /// Advance to the next request
    /// Returns the next request transaction id
    pub fn request_next(&mut self) -> usize {
        // Detect pipelining.
        if self.transactions.request_index() > self.transactions.response_index() {
            self.conn.flags.set(Flags::PIPELINED)
        }
        self.transactions.request_next()
    }

    /// Advance to the next response
    /// Returns the next response transaction id
    pub fn response_next(&mut self) -> usize {
        self.transactions.response_next()
    }

    /// Get the index of the request transaction
    pub fn request_index(&self) -> usize {
        self.transactions.request_index()
    }

    /// Get the index of the response transaction
    pub fn response_index(&self) -> usize {
        self.transactions.response_index()
    }

    /// Get the number of transactions processed up to now
    pub fn tx_size(&self) -> usize {
        self.transactions.size()
    }

    /// Get a specific transaction
    pub fn tx(&self, index: usize) -> Option<&Transaction> {
        self.transactions.get(index)
    }

    /// Get a specific transaction
    pub fn tx_mut(&mut self, index: usize) -> Option<&mut Transaction> {
        self.transactions.get_mut(index)
    }

    /// Handle the current state to be processed.
    pub fn handle_request_state(&mut self, data: &mut ParserData) -> Result<()> {
        data.set_position(self.request_curr_data.position() as usize);
        match self.request_state {
            State::NONE => Err(HtpStatus::ERROR),
            State::IDLE => self.request_idle(),
            State::IGNORE_DATA_AFTER_HTTP_0_9 => self.request_ignore_data_after_http_0_9(),
            State::LINE => self.request_line(data.as_slice()),
            State::PROTOCOL => self.request_protocol(data.as_slice()),
            State::HEADERS => self.request_headers(data.as_slice()),
            State::CONNECT_WAIT_RESPONSE => self.request_connect_wait_response(),
            State::CONNECT_CHECK => self.request_connect_check(),
            State::CONNECT_PROBE_DATA => self.request_connect_probe_data(data.as_slice()),
            State::BODY_DETERMINE => self.request_body_determine(),
            State::BODY_CHUNKED_DATA => self.request_body_chunked_data(data.as_slice()),
            State::BODY_CHUNKED_LENGTH => self.request_body_chunked_length(data.as_slice()),
            State::BODY_CHUNKED_DATA_END => self.request_body_chunked_data_end(data.as_slice()),
            State::BODY_IDENTITY => self.request_body_identity(data),
            State::FINALIZE => self.request_finalize(data),
            // These are only used by response_state
            _ => Err(HtpStatus::ERROR),
        }
    }

    /// Handle the current state to be processed.
    pub fn handle_response_state(&mut self, data: &mut ParserData) -> Result<()> {
        data.set_position(self.response_curr_data.position() as usize);
        match self.response_state {
            State::NONE => Err(HtpStatus::ERROR),
            State::IDLE => self.response_idle(),
            State::LINE => self.response_line(data.as_slice()),
            State::HEADERS => self.response_headers(data.as_slice()),
            State::BODY_DETERMINE => self.response_body_determine(),
            State::BODY_CHUNKED_DATA => self.response_body_chunked_data(data.as_slice()),
            State::BODY_CHUNKED_LENGTH => self.response_body_chunked_length(data.as_slice()),
            State::BODY_CHUNKED_DATA_END => self.response_body_chunked_data_end(data.as_slice()),
            State::FINALIZE => self.response_finalize(data),
            State::BODY_IDENTITY_STREAM_CLOSE => self.response_body_identity_stream_close(data),
            State::BODY_IDENTITY_CL_KNOWN => self.response_body_identity_cl_known(data),
            // These are only used by request_state
            _ => Err(HtpStatus::ERROR),
        }
    }

    /// The function used for request line parsing. Depends on the personality.
    pub fn parse_request_line(&mut self, request_line: &[u8]) -> Result<()> {
        self.request_mut().request_line = Some(Bstr::from(request_line));
        self.parse_request_line_generic_ex(
            request_line,
            self.cfg.server_personality == HtpServerPersonality::APACHE_2,
        )
    }

    /// The function is used for response line parsing.
    pub fn parse_response_line(&mut self, response_line: &[u8]) -> Result<()> {
        self.response_mut().response_line = Some(Bstr::from(response_line));
        self.parse_response_line_generic(response_line)
    }

    /// The function is used for request header parsing.
    pub fn parse_request_headers<'a>(&mut self, data: &'a [u8]) -> Result<(&'a [u8], bool)> {
        self.parse_request_headers_generic(data)
    }

    /// The function is used for response header parsing.
    pub fn parse_response_headers<'a>(&mut self, data: &'a [u8]) -> Result<(&'a [u8], bool)> {
        self.parse_response_headers_generic(data)
    }

    /// Closes the connection associated with the supplied parser.
    pub fn request_close(&mut self, timestamp: Option<DateTime<Utc>>) {
        // Update internal flags
        if self.request_status != HtpStreamState::ERROR {
            self.request_status = HtpStreamState::CLOSED
        }
        // Call the parsers one last time, which will allow them
        // to process the events that depend on stream closure
        self.request_data(ParserData::default(), timestamp);
    }

    /// Closes the connection associated with the supplied parser.
    pub fn close(&mut self, timestamp: Option<DateTime<Utc>>) {
        // Close the underlying connection.
        self.conn.close(timestamp);
        // Update internal flags
        if self.request_status != HtpStreamState::ERROR {
            self.request_status = HtpStreamState::CLOSED
        }
        if self.response_status != HtpStreamState::ERROR {
            self.response_status = HtpStreamState::CLOSED
        }
        // Call the parsers one last time, which will allow them
        // to process the events that depend on stream closure
        self.request_data(ParserData::default(), timestamp);
        self.response_data(ParserData::default(), timestamp);
    }

    /// This function is most likely not used and/or not needed.
    pub fn request_reset(&mut self) {
        self.request_content_length = -1;
        self.request_body_data_left = -1;
        self.request_chunk_request_index = self.request_chunk_count;
    }

    /// Returns the number of bytes consumed from the current data chunks so far.
    pub fn request_data_consumed(&self) -> i64 {
        self.request_curr_data.position() as i64
    }

    /// Returns the number of bytes consumed from the most recent outbound data chunk. Normally, an invocation
    /// of response_data() will consume all data from the supplied buffer, but there are circumstances
    /// where only partial consumption is possible. In such cases DATA_OTHER will be returned.
    /// Consumed bytes are no longer necessary, but the remainder of the buffer will be saved
    /// for later.
    pub fn response_data_consumed(&self) -> i64 {
        self.response_curr_data.position() as i64
    }

    /// Opens connection.
    pub fn open(
        &mut self,
        client_addr: Option<IpAddr>,
        client_port: Option<u16>,
        server_addr: Option<IpAddr>,
        server_port: Option<u16>,
        timestamp: Option<DateTime<Utc>>,
    ) {
        // Check connection parser state first.
        if self.request_status != HtpStreamState::NEW || self.response_status != HtpStreamState::NEW
        {
            htp_error!(
                self.logger,
                HtpLogCode::CONNECTION_ALREADY_OPEN,
                "Connection is already open"
            );
            return;
        }
        self.conn.open(
            client_addr,
            client_port,
            server_addr,
            server_port,
            timestamp,
        );
        self.request_status = HtpStreamState::OPEN;
        self.response_status = HtpStreamState::OPEN;
    }

    /// Set the user data.
    pub fn set_user_data(&mut self, data: Box<dyn Any + 'static>) {
        self.user_data = Some(data);
    }

    /// Get a reference to the user data.
    pub fn user_data<T: 'static>(&self) -> Option<&T> {
        self.user_data
            .as_ref()
            .and_then(|ud| ud.downcast_ref::<T>())
    }

    /// Get a mutable reference to the user data.
    pub fn user_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.user_data
            .as_mut()
            .and_then(|ud| ud.downcast_mut::<T>())
    }

    /// Initialize request parsing, change state to LINE,
    /// and invoke all registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP if one of the
    /// callbacks does not want to follow the transaction any more.
    pub fn state_request_start(&mut self) -> Result<()> {
        // Change state into request line parsing.
        self.request_state = State::LINE;
        self.request_mut().request_progress = HtpRequestProgress::LINE;
        // Run hook REQUEST_START.
        self.cfg
            .hook_request_start
            .clone()
            .run_all(self, self.request_index())?;
        Ok(())
    }

    /// Change transaction state to HEADERS and invoke all
    /// registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP if one of the
    /// callbacks does not want to follow the transaction any more.
    pub fn state_request_headers(&mut self) -> Result<()> {
        // Finalize sending raw header data
        self.request_receiver_finalize_clear()?;
        // If we're in HTP_REQ_HEADERS that means that this is the
        // first time we're processing headers in a request. Otherwise,
        // we're dealing with trailing headers.
        let request_progress = self.request().request_progress;
        if request_progress > HtpRequestProgress::HEADERS {
            // Request trailers.
            // Run hook HTP_REQUEST_TRAILER.
            self.cfg
                .hook_request_trailer
                .clone()
                .run_all(self, self.request_index())?;
            // Completed parsing this request; finalize it now.
            self.request_state = State::FINALIZE;
        } else if request_progress >= HtpRequestProgress::LINE {
            // Request headers.
            // Did this request arrive in multiple data chunks?
            if self.request_chunk_count != self.request_chunk_request_index {
                self.request_mut().flags.set(HtpFlags::MULTI_PACKET_HEAD)
            }
            self.request_mut().process_request_headers()?;
            // Check for body data to treat as file uploads.
            if self.request().request_has_body() {
                // Prepare to treat request body as a file.
                self.request_file = Some(File::new(HtpFileSource::REQUEST_BODY, None));
            }
            // Run hook REQUEST_HEADERS.
            self.cfg
                .hook_request_headers
                .clone()
                .run_all(self, self.request_index())?;
            self.request_initialize_decompressors()?;

            // We cannot proceed if the request is invalid.
            if self.request().flags.is_set(HtpFlags::REQUEST_INVALID) {
                return Err(HtpStatus::ERROR);
            }
            self.request_state = State::CONNECT_CHECK;
        } else {
            htp_warn!(
                self.logger,
                HtpLogCode::RESPONSE_BODY_INTERNAL_ERROR,
                format!(
                    "[Internal Error] Invalid tx progress: {:?}",
                    request_progress
                )
            );
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Change transaction state to PROTOCOL and invoke all
    /// registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP if one of the
    /// callbacks does not want to follow the transaction any more.
    pub fn state_request_line(&mut self) -> Result<()> {
        self.request_mut().parse_request_line()?;
        // Run hook REQUEST_URI_NORMALIZE.
        self.cfg
            .hook_request_uri_normalize
            .clone()
            .run_all(self, self.request_index())?;
        // Run hook REQUEST_LINE.
        self.cfg
            .hook_request_line
            .clone()
            .run_all(self, self.request_index())?;
        let logger = self.logger.clone();
        if let Some(parsed_uri) = self.request_mut().parsed_uri.as_mut() {
            let (partial_normalized_uri, complete_normalized_uri) =
                parsed_uri.generate_normalized_uri(Some(logger));
            self.request_mut().partial_normalized_uri = partial_normalized_uri;
            self.request_mut().complete_normalized_uri = complete_normalized_uri;
        }
        // Move on to the next phase.
        self.request_state = State::PROTOCOL;
        Ok(())
    }

    /// Advance state after processing request headers.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_request_complete(&mut self) -> Result<()> {
        if self.request().request_progress != HtpRequestProgress::COMPLETE {
            // Finalize request body.
            if self.request().request_has_body() {
                self.request_body_data(None)?;
            }
            self.request_mut().request_progress = HtpRequestProgress::COMPLETE;
            // Run hook REQUEST_COMPLETE.
            self.cfg
                .hook_request_complete
                .clone()
                .run_all(self, self.request_index())?;
        }
        // Determine what happens next, and remove this transaction from the parser.
        self.request_state = if self.request().is_protocol_0_9 {
            State::IGNORE_DATA_AFTER_HTTP_0_9
        } else {
            State::IDLE
        };
        // Check if the entire transaction is complete.
        self.finalize(self.request_index())?;
        self.request_next();
        Ok(())
    }

    /// Determine if the transaction is complete and run any hooks.
    pub fn finalize(&mut self, tx_index: usize) -> Result<()> {
        if let Some(tx) = self.tx(tx_index) {
            if !tx.is_complete() {
                return Ok(());
            }
            // Disconnect transaction from the parser.
            // Run hook TRANSACTION_COMPLETE.
            self.cfg
                .hook_transaction_complete
                .clone()
                .run_all(self, tx_index)?;
        }
        Ok(())
    }

    /// Advance state to LINE, or BODY if http version is 0.9.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_start(&mut self) -> Result<()> {
        // Change state into response line parsing, except if we're following
        // a HTTP/0.9 request (no status line or response headers).
        if self.response().is_protocol_0_9 {
            let tx = self.response_mut();
            tx.response_transfer_coding = HtpTransferCoding::IDENTITY;
            tx.response_content_encoding_processing = HtpContentEncoding::NONE;
            tx.response_progress = HtpResponseProgress::BODY;
            self.response_state = State::BODY_IDENTITY_STREAM_CLOSE;
            self.response_body_data_left = -1
        } else {
            self.response_state = State::LINE;
            self.response_mut().response_progress = HtpResponseProgress::LINE
        }
        // Run hook RESPONSE_START.
        self.cfg
            .hook_response_start
            .clone()
            .run_all(self, self.response_index())?;
        // If at this point we have no method and no uri and our status
        // is still REQ_LINE, we likely have timed out request
        // or a overly long request
        if self.response().request_method.is_none()
            && self.response().request_uri.is_none()
            && self.request_state == State::LINE
        {
            htp_warn!(
                self.logger,
                HtpLogCode::REQUEST_LINE_INCOMPLETE,
                "Request line incomplete"
            );
        }
        Ok(())
    }

    /// Advance state after processing response headers.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_headers(&mut self) -> Result<()> {
        // Finalize sending raw header data.
        self.response_receiver_finalize_clear()?;
        // Run hook RESPONSE_HEADERS.
        self.cfg
            .hook_response_headers
            .clone()
            .run_all(self, self.response_index())?;
        self.response_initialize_decompressors()
    }

    /// Change transaction state to RESPONSE_LINE and invoke registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_line(&mut self) -> Result<()> {
        // Is the response line valid?
        let tx = self.response_mut();
        tx.validate_response_line();
        let index = tx.index;
        // Run hook HTP_RESPONSE_LINE
        self.cfg.hook_response_line.clone().run_all(self, index)
    }

    /// Change transaction state to COMPLETE and invoke registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_complete(&mut self) -> Result<()> {
        let response_index = self.response_index();
        if self.response().response_progress != HtpResponseProgress::COMPLETE {
            let tx = self.response_mut();
            tx.response_progress = HtpResponseProgress::COMPLETE;
            // Run the last RESPONSE_BODY_DATA HOOK, but only if there was a response body present.
            if tx.response_transfer_coding != HtpTransferCoding::NO_BODY {
                let _ = self.response_body_data(None);
            }
            // Run hook RESPONSE_COMPLETE.
            self.cfg
                .hook_response_complete
                .clone()
                .run_all(self, response_index)?;
        }
        // Check if we want to signal the caller to send request data
        self.request_parser_check_waiting()?;
        // Otherwise finalize the transaction
        self.finalize(response_index)?;
        self.response_next();
        self.response_state = State::IDLE;
        Ok(())
    }

    /// Check if we had previously signalled the caller to give us response
    /// data, and now we are ready to receive it
    fn request_parser_check_waiting(&mut self) -> Result<()> {
        // Check if the inbound parser is waiting on us. If it is, that means that
        // there might be request data that the inbound parser hasn't consumed yet.
        // If we don't stop parsing we might encounter a response without a request,
        // which is why we want to return straight away before processing any data.
        //
        // This situation will occur any time the parser needs to see the server
        // respond to a particular situation before it can decide how to proceed. For
        // example, when a CONNECT is sent, different paths are used when it is accepted
        // and when it is not accepted.
        //
        // It is not enough to check only in_status here. Because of pipelining, it's possible
        // that many inbound transactions have been processed, and that the parser is
        // waiting on a response that we have not seen yet.
        if self.response_status == HtpStreamState::DATA_OTHER
            && self.response_index() == self.request_index()
        {
            return Err(HtpStatus::DATA_OTHER);
        }

        // Do we have a signal to yield to inbound processing at
        // the end of the next transaction?
        if self.response_data_other_at_tx_end {
            // We do. Let's yield then.
            self.response_data_other_at_tx_end = false;
            return Err(HtpStatus::DATA_OTHER);
        }
        Ok(())
    }

    /// Remove the given transaction from the parser
    pub fn remove_tx(&mut self, tx_id: usize) {
        self.transactions.remove(tx_id);
    }

    /// For each transaction that is started but not completed, invoke the
    /// transaction complete callback and remove it from the transactions list.
    ///
    /// This function is meant to be used before dropping the ConnectionParser
    /// so any incomplete transactions can be processed by the caller.
    pub fn flush_incomplete_transactions(&mut self) {
        let mut to_remove = Vec::<usize>::new();
        for tx in &mut self.transactions {
            if tx.is_started() && !tx.is_complete() {
                to_remove.push(tx.index);
            }
        }
        for index in to_remove {
            self.cfg
                .hook_transaction_complete
                .clone()
                .run_all(self, index)
                .ok();
            self.transactions.remove(index);
        }
    }
}
