use crate::{
    bstr::Bstr,
    config::{Config, HtpServerPersonality},
    connection::{Connection, Flags},
    error::Result,
    hook::DataHook,
    log::Logger,
    transaction::Transaction,
    util::{File, FlagOperations},
    HtpStatus,
};
use chrono::{DateTime, Utc};
use std::{any::Any, io::Cursor, net::IpAddr, rc::Rc, time::SystemTime};

/// Enumerates parsing state.
#[derive(Debug, Copy, Clone, PartialEq)]
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
    // Used by in_state only
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
    // Used by out_state only
    /// State to consume response remaining body data when content-length is unknown.
    BODY_IDENTITY_STREAM_CLOSE,
    /// State to consume response body data when content-length is known.
    BODY_IDENTITY_CL_KNOWN,
}

/// Enumerates all stream states. Each connection has two streams, one
/// inbound and one outbound. Their states are tracked separately.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
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
    pub in_status: HtpStreamState,
    /// Parser outbound status. Starts as OK, but may turn into ERROR.
    pub out_status: HtpStreamState,
    /// When true, this field indicates that there is unprocessed inbound data, and
    /// that the response parsing code should stop at the end of the current request
    /// in order to allow more requests to be produced.
    pub out_data_other_at_tx_end: bool,
    /// The time when the last request data chunk was received.
    pub in_timestamp: DateTime<Utc>,
    /// Pointer to the current request data chunk.
    pub in_curr_data: Cursor<Vec<u8>>,
    /// Marks the starting point of raw data within the inbound data chunk. Raw
    /// data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
    /// request_header_data).
    pub in_current_receiver_offset: u64,
    /// How many data chunks does the inbound connection stream consist of?
    pub in_chunk_count: usize,
    /// The index of the first chunk used in the current request.
    pub in_chunk_request_index: usize,
    /// Used to buffer a line of inbound data when buffering cannot be avoided.
    pub in_buf: Bstr,
    /// Stores the current value of a folded request header. Such headers span
    /// multiple lines, and are processed only when all data is available.
    pub in_header: Option<Bstr>,
    /// Ongoing inbound transaction.
    in_tx: Option<usize>,
    /// The request body length declared in a valid request header. The key here
    /// is "valid". This field will not be populated if the request contains both
    /// a Transfer-Encoding header and a Content-Length header.
    pub in_content_length: i64,
    /// Holds the remaining request body length that we expect to read. This
    /// field will be available only when the length of a request body is known
    /// in advance, i.e. when request headers contain a Content-Length header.
    pub in_body_data_left: i64,
    /// Holds the amount of data that needs to be read from the
    /// current data chunk. Only used with chunked request bodies.
    pub in_chunked_length: Option<i32>,
    /// Current request parser state.
    pub in_state: State,
    /// Previous request parser state. Used to detect state changes.
    pub in_state_previous: State,
    /// The hook that should be receiving raw connection data.
    pub in_data_receiver_hook: Option<DataHook>,

    // Response parser fields
    /// Response counter, incremented with every new response. This field is
    /// used to match responses to requests. The expectation is that for every
    /// response there will already be a transaction (request) waiting.
    pub out_next_tx_index: usize,
    /// The time when the last response data chunk was received.
    pub out_timestamp: DateTime<Utc>,
    /// Pointer to the current response data chunk.
    pub out_curr_data: Cursor<Vec<u8>>,
    /// Marks the starting point of raw data within the outbound data chunk. Raw
    /// data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
    /// response_header_data).
    pub out_current_receiver_offset: u64,
    /// Used to buffer a line of outbound data when buffering cannot be avoided.
    pub out_buf: Bstr,
    /// Stores the current value of a folded response header. Such headers span
    /// multiple lines, and are processed only when all data is available.
    pub out_header: Option<Bstr>,
    /// Ongoing outbound transaction
    out_tx: Option<usize>,
    /// The length of the current response body as presented in the
    /// Content-Length response header.
    pub out_content_length: i64,
    /// The remaining length of the current response body, if known. Set to -1 otherwise.
    pub out_body_data_left: i64,
    /// Holds the amount of data that needs to be read from the
    /// current response data chunk. Only used with chunked response bodies.
    pub out_chunked_length: Option<i32>,
    /// Current response parser state.
    pub out_state: State,
    /// Previous response parser state.
    pub out_state_previous: State,
    /// The hook that should be receiving raw connection data.
    pub out_data_receiver_hook: Option<DataHook>,
    /// On a PUT request, this field contains additional file data.
    pub put_file: Option<File>,
}

impl std::fmt::Debug for ConnectionParser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectionParser")
            .field("in_status", &self.in_status)
            .field("out_status", &self.out_status)
            .field("in_tx", &self.in_tx)
            .field("out_tx", &self.out_tx)
            .finish()
    }
}

impl ConnectionParser {
    /// Creates a new ConnectionParser with a preconfigured `Config` struct.
    pub fn new(cfg: Config) -> Self {
        let conn = Connection::default();
        Self {
            logger: Logger::new(conn.get_sender(), cfg.log_level),
            cfg: Rc::new(cfg),
            conn,
            user_data: None,
            in_status: HtpStreamState::NEW,
            out_status: HtpStreamState::NEW,
            out_data_other_at_tx_end: false,
            in_timestamp: DateTime::<Utc>::from(SystemTime::now()),
            in_curr_data: Cursor::new(Vec::new()),
            in_current_receiver_offset: 0,
            in_chunk_count: 0,
            in_chunk_request_index: 0,
            in_buf: Bstr::new(),
            in_header: None,
            in_tx: None,
            in_content_length: 0,
            in_body_data_left: 0,
            in_chunked_length: None,
            in_state: State::IDLE,
            in_state_previous: State::NONE,
            in_data_receiver_hook: None,
            out_next_tx_index: 0,
            out_timestamp: DateTime::<Utc>::from(SystemTime::now()),
            out_curr_data: Cursor::new(Vec::new()),
            out_current_receiver_offset: 0,
            out_buf: Bstr::new(),
            out_header: None,
            out_tx: None,
            out_content_length: 0,
            out_body_data_left: 0,
            out_chunked_length: None,
            out_state: State::IDLE,
            out_state_previous: State::NONE,
            out_data_receiver_hook: None,
            put_file: None,
        }
    }

    /// Creates a `Transaction` and attaches it to this connection.
    ///
    /// Returns the index of the new `Transaction`.
    pub fn create_tx(&mut self) -> Result<usize> {
        // Detect pipelining.
        if self.conn.tx_size() > self.out_next_tx_index {
            self.conn.flags.set(Flags::PIPELINED)
        }
        let index = self.conn.tx_size();
        let tx = Transaction::new(self, index);
        self.conn.push_tx(tx);
        Ok(index)
    }

    /// Get the in_tx or None if not set.
    pub fn in_tx(&self) -> Option<&Transaction> {
        self.in_tx.and_then(|in_tx| self.conn.tx(in_tx))
    }

    /// Get the in_tx as a mutable reference or None if not set.
    pub fn in_tx_mut(&mut self) -> Option<&mut Transaction> {
        self.in_tx.and_then(move |in_tx| self.conn.tx_mut(in_tx))
    }

    /// Get the in_tx as a mutable reference or HtpStatus::ERROR if not set.
    pub fn in_tx_mut_ok(&mut self) -> Result<&mut Transaction> {
        self.in_tx
            .and_then(move |in_tx| self.conn.tx_mut(in_tx))
            .ok_or(HtpStatus::ERROR)
    }

    /// Get the in_tx as a pointer or NULL if not set.
    pub fn in_tx_ptr(&self) -> *const Transaction {
        self.in_tx()
            .map(|in_tx| in_tx as *const Transaction)
            .unwrap_or(std::ptr::null())
    }

    /// Get the in_tx as a mutable pointer or NULL if not set.
    pub fn in_tx_mut_ptr(&mut self) -> *mut Transaction {
        self.in_tx_mut()
            .map(|in_tx| in_tx as *mut Transaction)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Set the in_tx to the provided transaction.
    pub fn set_in_tx(&mut self, tx: &Transaction) {
        self.in_tx = Some(tx.index);
    }

    /// Set the in_tx to the provided transaction id.
    pub fn set_in_tx_id(&mut self, tx_id: Option<usize>) {
        self.in_tx = tx_id;
    }

    /// Unset the in_tx.
    pub fn clear_in_tx(&mut self) {
        self.in_tx = None;
    }

    /// Get the out_tx or None if not set.
    pub fn out_tx(&self) -> Option<&Transaction> {
        self.out_tx.and_then(|out_tx| self.conn.tx(out_tx))
    }

    /// Get the out_tx as a mutable reference or None if not set.
    pub fn out_tx_mut(&mut self) -> Option<&mut Transaction> {
        self.out_tx.and_then(move |out_tx| self.conn.tx_mut(out_tx))
    }

    /// Get the out_tx as a mutable reference or HtpStatus::ERROR if not set.
    pub fn out_tx_mut_ok(&mut self) -> Result<&mut Transaction> {
        self.out_tx
            .and_then(move |out_tx| self.conn.tx_mut(out_tx))
            .ok_or(HtpStatus::ERROR)
    }

    /// Get the out_tx as a pointer or NULL if not set.
    pub fn out_tx_ptr(&self) -> *const Transaction {
        self.out_tx()
            .map(|out_tx| out_tx as *const Transaction)
            .unwrap_or(std::ptr::null())
    }

    /// Get the out_tx as a mutable pointer or NULL if not set.
    pub fn out_tx_mut_ptr(&mut self) -> *mut Transaction {
        self.out_tx_mut()
            .map(|out_tx| out_tx as *mut Transaction)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Set the out_tx to the provided transaction.
    pub fn set_out_tx(&mut self, tx: &Transaction) {
        self.out_tx = Some(tx.index);
    }

    /// Set the out_tx to the provided transaction id.
    pub fn set_out_tx_id(&mut self, tx_id: Option<usize>) {
        self.out_tx = tx_id;
    }

    /// Unset the out_tx.
    pub fn clear_out_tx(&mut self) {
        self.out_tx = None;
    }

    /// Handle the current state to be processed.
    pub fn handle_in_state(&mut self, data: &[u8]) -> Result<()> {
        let data = &data[self.in_curr_data.position() as usize..];
        match self.in_state {
            State::NONE => Err(HtpStatus::ERROR),
            State::IDLE => self.req_idle(),
            State::IGNORE_DATA_AFTER_HTTP_0_9 => self.req_ignore_data_after_http_0_9(),
            State::LINE => self.req_line(&data),
            State::PROTOCOL => self.req_protocol(&data),
            State::HEADERS => self.req_headers(&data),
            State::CONNECT_WAIT_RESPONSE => self.req_connect_wait_response(),
            State::CONNECT_CHECK => self.req_connect_check(),
            State::CONNECT_PROBE_DATA => self.req_connect_probe_data(&data),
            State::BODY_DETERMINE => self.req_body_determine(),
            State::BODY_CHUNKED_DATA => self.req_body_chunked_data(&data),
            State::BODY_CHUNKED_LENGTH => self.req_body_chunked_length(&data),
            State::BODY_CHUNKED_DATA_END => self.req_body_chunked_data_end(&data),
            State::BODY_IDENTITY => self.req_body_identity(&data),
            State::FINALIZE => self.req_finalize(&data),
            // These are only used by out_state
            _ => Err(HtpStatus::ERROR),
        }
    }

    /// Handle the current state to be processed.
    pub fn handle_out_state(&mut self, data: &[u8]) -> Result<()> {
        let data = &data[self.out_curr_data.position() as usize..];
        match self.out_state {
            State::NONE => Err(HtpStatus::ERROR),
            State::IDLE => self.res_idle(),
            State::LINE => self.res_line(data),
            State::HEADERS => self.res_headers(data),
            State::BODY_DETERMINE => self.res_body_determine(),
            State::BODY_CHUNKED_DATA => self.res_body_chunked_data(data),
            State::BODY_CHUNKED_LENGTH => self.res_body_chunked_length(data),
            State::BODY_CHUNKED_DATA_END => self.res_body_chunked_data_end(data),
            State::FINALIZE => self.res_finalize(data),
            State::BODY_IDENTITY_STREAM_CLOSE => self.res_body_identity_stream_close(data),
            State::BODY_IDENTITY_CL_KNOWN => self.res_body_identity_cl_known(data),
            // These are only used by in_state
            _ => Err(HtpStatus::ERROR),
        }
    }

    /// The function used for request line parsing. Depends on the personality.
    pub fn parse_request_line(&mut self, request_line: &[u8]) -> Result<()> {
        self.in_tx_mut_ok()?.request_line = Some(Bstr::from(request_line));
        if self.cfg.server_personality == HtpServerPersonality::APACHE_2 {
            self.parse_request_line_generic_ex(request_line, true)
        } else {
            self.parse_request_line_generic_ex(request_line, false)
        }
    }

    /// The function is used for response line parsing.
    pub fn parse_response_line(&mut self, response_line: &[u8]) -> Result<()> {
        self.out_tx_mut_ok()?.response_line = Some(Bstr::from(response_line));
        self.parse_response_line_generic(response_line)
    }

    /// The function is used for request header parsing.
    pub fn process_request_headers<'a>(&mut self, data: &'a [u8]) -> Result<(&'a [u8], bool)> {
        self.process_request_headers_generic(data)
    }

    /// The function is used for response header parsing.
    pub fn process_response_headers<'a>(&mut self, data: &'a [u8]) -> Result<(&'a [u8], bool)> {
        self.process_response_headers_generic(data)
    }

    /// Closes the connection associated with the supplied parser.
    pub fn req_close(&mut self, timestamp: Option<DateTime<Utc>>) {
        // Update internal flags
        if self.in_status != HtpStreamState::ERROR {
            self.in_status = HtpStreamState::CLOSED
        }
        // Call the parsers one last time, which will allow them
        // to process the events that depend on stream closure
        self.req_data(timestamp, std::ptr::null(), 0);
    }

    /// Closes the connection associated with the supplied parser.
    pub fn close(&mut self, timestamp: Option<DateTime<Utc>>) {
        // Close the underlying connection.
        self.conn.close(timestamp);
        // Update internal flags
        if self.in_status != HtpStreamState::ERROR {
            self.in_status = HtpStreamState::CLOSED
        }
        if self.out_status != HtpStreamState::ERROR {
            self.out_status = HtpStreamState::CLOSED
        }
        // Call the parsers one last time, which will allow them
        // to process the events that depend on stream closure
        self.req_data(timestamp, std::ptr::null(), 0);
        self.res_data(timestamp, std::ptr::null(), 0);
    }

    /// This function is most likely not used and/or not needed.
    pub fn in_reset(&mut self) {
        self.in_content_length = -1;
        self.in_body_data_left = -1;
        self.in_chunk_request_index = self.in_chunk_count;
    }

    /// Returns the number of bytes consumed from the current data chunks so far.
    pub fn req_data_consumed(&self) -> i64 {
        self.in_curr_data.position() as i64
    }

    /// Returns the number of bytes consumed from the most recent outbound data chunk. Normally, an invocation
    /// of res_data() will consume all data from the supplied buffer, but there are circumstances
    /// where only partial consumption is possible. In such cases DATA_OTHER will be returned.
    /// Consumed bytes are no longer necessary, but the remainder of the buffer will be saved
    /// for later.
    pub fn res_data_consumed(&self) -> i64 {
        self.out_curr_data.position() as i64
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
        if self.in_status != HtpStreamState::NEW || self.out_status != HtpStreamState::NEW {
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
        self.in_status = HtpStreamState::OPEN;
        self.out_status = HtpStreamState::OPEN;
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

    /// Consumes request body data.
    ///
    /// Returns HtpStatus::OK on success or HtpStatus::ERROR if the request transaction
    /// is invalid or request body data hook fails.
    pub fn req_process_body_data_ex(&mut self, data: &[u8]) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.in_tx_mut() {
            tx.req_process_body_data(unsafe { &mut *connp_ptr }, Some(data))
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Initialize hybrid parsing mode, change state to TRANSACTION_START,
    /// and invoke all registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP if one of the
    /// callbacks does not want to follow the transaction any more.
    pub fn state_request_start(&mut self) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_start(unsafe { &mut *connp_ptr })
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Change transaction state to REQUEST_HEADERS and invoke all
    /// registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP if one of the
    /// callbacks does not want to follow the transaction any more.
    pub fn state_request_headers(&mut self) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        // Finalize sending raw header data
        self.req_receiver_finalize_clear()?;
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_headers(unsafe { &mut *connp_ptr })
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Change transaction state to REQUEST_LINE and invoke all
    /// registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP if one of the
    /// callbacks does not want to follow the transaction any more.
    pub fn state_request_line(&mut self) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_line(unsafe { &mut *connp_ptr })
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Advance state after processing request headers.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_request_complete(&mut self) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_complete(unsafe { &mut *connp_ptr })
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Consumes response body data.
    ///
    /// Returns HtpStatus::OK on success or HtpStatus::ERROR if the request transaction
    /// is invalid or response body data hook fails.
    pub fn res_process_body_data_ex(&mut self, data: Option<&[u8]>) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.out_tx_mut() {
            tx.res_process_body_data(unsafe { &mut *connp_ptr }, data)
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Advance state to LINE, or BODY if http version is 0.9.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_start(&mut self) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_start(unsafe { &mut *connp_ptr })
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Advance state after processing response headers.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_headers(&mut self) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        // Finalize sending raw header data.
        self.res_receiver_finalize_clear()?;
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_headers(unsafe { &mut *connp_ptr })
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Change transaction state to RESPONSE_LINE and invoke registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_line(&mut self) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_line(unsafe { &mut *connp_ptr })
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Change transaction state to COMPLETE and invoke registered callbacks.
    ///
    /// Returns HtpStatus::OK on success; HtpStatus::ERROR on error, HtpStatus::STOP
    /// if one of the callbacks does not want to follow the transaction any more.
    pub fn state_response_complete_ex(&mut self, hybrid_mode: i32) -> Result<()> {
        let connp_ptr: *mut Self = self as *mut Self;
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_complete_ex(unsafe { &mut *connp_ptr }, hybrid_mode)
        } else {
            Err(HtpStatus::ERROR)
        }
    }

    /// Remove the given transaction from the parser
    pub fn remove_tx(&mut self, tx_id: usize) -> Result<()> {
        if let Some(tx) = self.conn.tx(tx_id) {
            if !tx.is_complete() {
                return Err(HtpStatus::ERROR);
            }
        }
        self.conn.remove_tx(tx_id)
    }
}
