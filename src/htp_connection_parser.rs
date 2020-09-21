use crate::error::Result;
use crate::htp_config::htp_server_personality_t;
use crate::{
    bstr, hook::DataHook, htp_config, htp_connection, htp_response, htp_transaction, htp_util,
    Status,
};
use std::net::IpAddr;

extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    NONE,
    IDLE,
    LINE,
    HEADERS,
    BODY_CHUNKED_DATA_END,
    BODY_CHUNKED_DATA,
    BODY_CHUNKED_LENGTH,
    BODY_DETERMINE,
    FINALIZE,
    // Used by in_state only
    PROTOCOL,
    CONNECT_CHECK,
    CONNECT_PROBE_DATA,
    CONNECT_WAIT_RESPONSE,
    BODY_IDENTITY,
    IGNORE_DATA_AFTER_HTTP_0_9,
    // Used by out_state only
    BODY_IDENTITY_STREAM_CLOSE,
    BODY_IDENTITY_CL_KNOWN,
}

/// Enumerates all stream states. Each connection has two streams, one
/// inbound and one outbound. Their states are tracked separately.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_stream_state_t {
    HTP_STREAM_NEW,
    HTP_STREAM_OPEN,
    HTP_STREAM_CLOSED,
    HTP_STREAM_ERROR,
    HTP_STREAM_TUNNEL,
    HTP_STREAM_DATA_OTHER,
    HTP_STREAM_STOP,
    HTP_STREAM_DATA,
}

pub type htp_time_t = libc::timeval;

pub struct htp_connp_t {
    // General fields
    /// Current parser configuration structure.
    pub cfg: *mut htp_config::htp_cfg_t,
    /// The connection structure associated with this parser.
    pub conn: htp_connection::htp_conn_t,
    /// Opaque user data associated with this parser.
    pub user_data: *mut core::ffi::c_void,
    // Request parser fields
    /// Parser inbound status. Starts as HTP_OK, but may turn into HTP_ERROR.
    pub in_status: htp_stream_state_t,
    /// Parser output status. Starts as HTP_OK, but may turn into HTP_ERROR.
    pub out_status: htp_stream_state_t,
    /// When true, this field indicates that there is unprocessed inbound data, and
    /// that the response parsing code should stop at the end of the current request
    /// in order to allow more requests to be produced.
    pub out_data_other_at_tx_end: u32,
    /// The time when the last request data chunk was received. Can be NULL if
    /// the upstream code is not providing the timestamps when calling us.
    pub in_timestamp: htp_time_t,
    /// Pointer to the current request data chunk.
    pub in_current_data: *mut u8,
    /// The length of the current request data chunk.
    pub in_current_len: i64,
    /// The offset of the next byte in the request data chunk to read.
    pub in_current_read_offset: i64,
    /// The starting point of the data waiting to be consumed. This field is used
    /// in the states where reading data is not the same as consumption.
    pub in_current_consume_offset: i64,
    /// Marks the starting point of raw data within the inbound data chunk. Raw
    /// data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
    /// REQUEST_HEADER_DATA).
    pub in_current_receiver_offset: i64,
    /// How many data chunks does the inbound connection stream consist of?
    pub in_chunk_count: usize,
    /// The index of the first chunk used in the current request.
    pub in_chunk_request_index: usize,
    /// The offset, in the entire connection stream, of the next request byte.
    pub in_stream_offset: i64,
    /// The value of the request byte currently being processed. This field is
    /// populated when the IN_NEXT_* or IN_PEEK_* macros are invoked.
    pub in_next_byte: i32,
    /// Used to buffer a line of inbound data when buffering cannot be avoided.
    pub in_buf: bstr::bstr_t,
    /// Stores the current value of a folded request header. Such headers span
    /// multiple lines, and are processed only when all data is available.
    pub in_header: *mut bstr::bstr_t,
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
    pub in_chunked_length: i64,
    /// Current request parser state.
    pub in_state: State,
    /// Previous request parser state. Used to detect state changes.
    pub in_state_previous: State,
    /// The hook that should be receiving raw connection data.
    pub in_data_receiver_hook: Option<DataHook>,

    /// Response counter, incremented with every new response. This field is
    /// used to match responses to requests. The expectation is that for every
    /// response there will already be a transaction (request) waiting.
    pub out_next_tx_index: usize,
    /// The time when the last response data chunk was received. Can be NULL.
    pub out_timestamp: htp_time_t,
    /// Pointer to the current response data chunk.
    pub out_current_data: *mut u8,
    /// The length of the current response data chunk.
    pub out_current_len: i64,
    /// The offset of the next byte in the response data chunk to consume.
    pub out_current_read_offset: i64,
    /// The starting point of the data waiting to be consumed. This field is used
    /// in the states where reading data is not the same as consumption.
    pub out_current_consume_offset: i64,
    /// Marks the starting point of raw data within the outbound data chunk. Raw
    /// data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
    /// RESPONSE_HEADER_DATA).
    pub out_current_receiver_offset: i64,
    /// The offset, in the entire connection stream, of the next response byte.
    pub out_stream_offset: i64,
    /// The value of the response byte currently being processed.
    pub out_next_byte: i32,
    /// Used to buffer a line of outbound data when buffering cannot be avoided.
    pub out_buf: bstr::bstr_t,
    /// Stores the current value of a folded response header. Such headers span
    /// multiple lines, and are processed only when all data is available.
    pub out_header: *mut bstr::bstr_t,
    /// Ongoing outbound transaction
    out_tx: Option<usize>,
    /// The length of the current response body as presented in the
    /// Content-Length response header.
    pub out_content_length: i64,
    /// The remaining length of the current response body, if known. Set to -1 otherwise.
    pub out_body_data_left: i64,
    /// Holds the amount of data that needs to be read from the
    /// current response data chunk. Only used with chunked response bodies.
    pub out_chunked_length: i64,
    /// Current response parser state.
    pub out_state: State,
    /// Previous response parser state.
    pub out_state_previous: State,
    /// The hook that should be receiving raw connection data.
    pub out_data_receiver_hook: Option<DataHook>,
    /// On a PUT request, this field contains additional file data.
    pub put_file: Option<htp_util::htp_file_t>,
}

impl htp_connp_t {
    pub fn new(cfg: *mut htp_config::htp_cfg_t) -> Self {
        Self {
            cfg,
            conn: htp_connection::htp_conn_t::new(),
            user_data: std::ptr::null_mut(),
            in_status: htp_stream_state_t::HTP_STREAM_NEW,
            out_status: htp_stream_state_t::HTP_STREAM_NEW,
            out_data_other_at_tx_end: 0,
            in_timestamp: htp_time_t {
                tv_sec: 0,
                tv_usec: 0,
            },
            in_current_data: std::ptr::null_mut(),
            in_current_len: 0,
            in_current_read_offset: 0,
            in_current_consume_offset: 0,
            in_current_receiver_offset: 0,
            in_chunk_count: 0,
            in_chunk_request_index: 0,
            in_stream_offset: 0,
            in_next_byte: 0,
            in_buf: bstr::bstr_t::new(),
            in_header: std::ptr::null_mut(),
            in_tx: None,
            in_content_length: 0,
            in_body_data_left: 0,
            in_chunked_length: 0,
            in_state: State::IDLE,
            in_state_previous: State::NONE,
            in_data_receiver_hook: None,
            out_next_tx_index: 0,
            out_timestamp: htp_time_t {
                tv_sec: 0,
                tv_usec: 0,
            },
            out_current_data: std::ptr::null_mut(),
            out_current_len: 0,
            out_current_read_offset: 0,
            out_current_consume_offset: 0,
            out_current_receiver_offset: 0,
            out_stream_offset: 0,
            out_next_byte: 0,
            out_buf: bstr::bstr_t::new(),
            out_header: std::ptr::null_mut(),
            out_tx: None,
            out_content_length: 0,
            out_body_data_left: 0,
            out_chunked_length: 0,
            out_state: State::IDLE,
            out_state_previous: State::NONE,
            out_data_receiver_hook: None,
            put_file: None,
        }
    }

    /// Creates a transaction and attaches it to this connection.
    ///
    /// Also sets the in_tx to the newly created one.
    pub fn create_tx(&mut self) -> Result<usize> {
        // Detect pipelining.
        if self.conn.tx_size() > self.out_next_tx_index {
            self.conn.flags |= htp_util::ConnectionFlags::HTP_CONN_PIPELINED
        }
        htp_transaction::htp_tx_t::new(self).map(|tx_id| {
            self.in_tx = Some(tx_id);
            self.in_reset();
            tx_id
        })
    }

    /// Removes references to the supplied transaction.
    pub fn remove_tx(&mut self, tx: usize) {
        if let Some(in_tx) = self.in_tx() {
            if in_tx.index == tx {
                self.in_tx = None
            }
        }
        if let Some(out_tx) = self.out_tx() {
            if out_tx.index == tx {
                self.out_tx = None
            }
        }
    }

    /// Get the in_tx or None if not set.
    pub fn in_tx(&self) -> Option<&htp_transaction::htp_tx_t> {
        self.in_tx.and_then(|in_tx| self.conn.tx(in_tx))
    }

    /// Get the in_tx as a mutable reference or None if not set.
    pub fn in_tx_mut(&mut self) -> Option<&mut htp_transaction::htp_tx_t> {
        self.in_tx.and_then(move |in_tx| self.conn.tx_mut(in_tx))
    }

    /// Get the in_tx as a mutable reference or Status::ERROR if not set.
    pub fn in_tx_mut_ok(&mut self) -> Result<&mut htp_transaction::htp_tx_t> {
        self.in_tx
            .and_then(move |in_tx| self.conn.tx_mut(in_tx))
            .ok_or(Status::ERROR)
    }

    /// Get the in_tx as a pointer or NULL if not set.
    pub fn in_tx_ptr(&self) -> *const htp_transaction::htp_tx_t {
        self.in_tx()
            .map(|in_tx| in_tx as *const htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null())
    }

    /// Get the in_tx as a mutable pointer or NULL if not set.
    pub fn in_tx_mut_ptr(&mut self) -> *mut htp_transaction::htp_tx_t {
        self.in_tx_mut()
            .map(|in_tx| in_tx as *mut htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Set the in_tx to the provided transaction.
    pub fn set_in_tx(&mut self, tx: &htp_transaction::htp_tx_t) {
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
    pub fn out_tx(&self) -> Option<&htp_transaction::htp_tx_t> {
        self.out_tx.and_then(|out_tx| self.conn.tx(out_tx))
    }

    /// Get the out_tx as a mutable reference or None if not set.
    pub fn out_tx_mut(&mut self) -> Option<&mut htp_transaction::htp_tx_t> {
        self.out_tx.and_then(move |out_tx| self.conn.tx_mut(out_tx))
    }

    /// Get the out_tx as a mutable reference or Status::ERROR if not set.
    pub fn out_tx_mut_ok(&mut self) -> Result<&mut htp_transaction::htp_tx_t> {
        self.out_tx
            .and_then(move |out_tx| self.conn.tx_mut(out_tx))
            .ok_or(Status::ERROR)
    }

    /// Get the out_tx as a pointer or NULL if not set.
    pub fn out_tx_ptr(&self) -> *const htp_transaction::htp_tx_t {
        self.out_tx()
            .map(|out_tx| out_tx as *const htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null())
    }

    /// Get the out_tx as a mutable pointer or NULL if not set.
    pub fn out_tx_mut_ptr(&mut self) -> *mut htp_transaction::htp_tx_t {
        self.out_tx_mut()
            .map(|out_tx| out_tx as *mut htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Set the out_tx to the provided transaction.
    pub fn set_out_tx(&mut self, tx: &htp_transaction::htp_tx_t) {
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
    pub fn handle_in_state(&mut self) -> Result<()> {
        unsafe {
            match self.in_state {
                State::NONE => Err(Status::ERROR),
                State::IDLE => self.REQ_IDLE(),
                State::IGNORE_DATA_AFTER_HTTP_0_9 => self.REQ_IGNORE_DATA_AFTER_HTTP_0_9(),
                State::LINE => self.REQ_LINE(),
                State::PROTOCOL => self.REQ_PROTOCOL(),
                State::HEADERS => self.REQ_HEADERS(),
                State::CONNECT_WAIT_RESPONSE => self.REQ_CONNECT_WAIT_RESPONSE(),
                State::CONNECT_CHECK => self.REQ_CONNECT_CHECK(),
                State::CONNECT_PROBE_DATA => self.REQ_CONNECT_PROBE_DATA(),
                State::BODY_DETERMINE => self.REQ_BODY_DETERMINE(),
                State::BODY_CHUNKED_DATA => self.REQ_BODY_CHUNKED_DATA(),
                State::BODY_CHUNKED_LENGTH => self.REQ_BODY_CHUNKED_LENGTH(),
                State::BODY_CHUNKED_DATA_END => self.REQ_BODY_CHUNKED_DATA_END(),
                State::BODY_IDENTITY => self.REQ_BODY_IDENTITY(),
                State::FINALIZE => self.REQ_FINALIZE(),
                // These are only used by out_state
                State::BODY_IDENTITY_STREAM_CLOSE | State::BODY_IDENTITY_CL_KNOWN => {
                    Err(Status::ERROR)
                }
            }
        }
    }

    /// Handle the current state to be processed.
    pub fn handle_out_state(&mut self) -> Result<()> {
        unsafe {
            match self.out_state {
                State::NONE => Err(Status::ERROR),
                State::IDLE => htp_response::htp_connp_RES_IDLE(self),
                State::LINE => htp_response::htp_connp_RES_LINE(self),
                State::HEADERS => htp_response::htp_connp_RES_HEADERS(self),
                State::BODY_DETERMINE => htp_response::htp_connp_RES_BODY_DETERMINE(self),
                State::BODY_CHUNKED_DATA => htp_response::htp_connp_RES_BODY_CHUNKED_DATA(self),
                State::BODY_CHUNKED_LENGTH => htp_response::htp_connp_RES_BODY_CHUNKED_LENGTH(self),
                State::BODY_CHUNKED_DATA_END => {
                    htp_response::htp_connp_RES_BODY_CHUNKED_DATA_END(self)
                }
                State::FINALIZE => htp_response::htp_connp_RES_FINALIZE(self),
                State::BODY_IDENTITY_STREAM_CLOSE => {
                    htp_response::htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE(self)
                }
                State::BODY_IDENTITY_CL_KNOWN => {
                    htp_response::htp_connp_RES_BODY_IDENTITY_CL_KNOWN(self)
                }
                // These are only used by in_state
                State::PROTOCOL
                | State::CONNECT_CHECK
                | State::CONNECT_PROBE_DATA
                | State::CONNECT_WAIT_RESPONSE
                | State::BODY_IDENTITY
                | State::IGNORE_DATA_AFTER_HTTP_0_9 => Err(Status::ERROR),
            }
        }
    }

    /// The function used for request line parsing. Depends on the personality.
    pub fn parse_request_line(&mut self) -> Result<()> {
        unsafe {
            if (*self.cfg).server_personality == htp_server_personality_t::HTP_SERVER_APACHE_2 {
                self.parse_request_line_apache_2_2()
            } else {
                self.parse_request_line_generic()
            }
        }
    }

    /// The function used for response line parsing. Depends on the personality.
    pub fn parse_response_line(&mut self) -> Result<()> {
        unsafe { self.parse_response_line_generic() }
    }

    pub fn process_request_header(&mut self, data: *mut u8, len: usize) -> Result<()> {
        unsafe {
            if (*self.cfg).server_personality == htp_server_personality_t::HTP_SERVER_APACHE_2 {
                self.process_request_header_apache_2_2(data, len)
            } else {
                self.process_request_header_generic(data, len)
            }
        }
    }

    pub fn process_response_header(&mut self, data: *mut u8, len: usize) -> Result<()> {
        unsafe { self.process_response_header_generic(data, len) }
    }

    /// Closes the connection associated with the supplied parser.
    ///
    /// timestamp is optional
    pub unsafe fn req_close(&mut self, timestamp: Option<htp_time_t>) {
        // Update internal flags
        if self.in_status != htp_stream_state_t::HTP_STREAM_ERROR {
            self.in_status = htp_stream_state_t::HTP_STREAM_CLOSED
        }
        // Call the parsers one last time, which will allow them
        // to process the events that depend on stream closure
        self.req_data(timestamp, 0 as *const core::ffi::c_void, 0);
    }

    /// Closes the connection associated with the supplied parser.
    ///
    /// timestamp is optional
    pub unsafe fn close(&mut self, timestamp: Option<htp_time_t>) {
        // Close the underlying connection.
        self.conn.close(timestamp.clone());
        // Update internal flags
        if self.in_status != htp_stream_state_t::HTP_STREAM_ERROR {
            self.in_status = htp_stream_state_t::HTP_STREAM_CLOSED
        }
        if self.out_status != htp_stream_state_t::HTP_STREAM_ERROR {
            self.out_status = htp_stream_state_t::HTP_STREAM_CLOSED
        }
        // Call the parsers one last time, which will allow them
        // to process the events that depend on stream closure
        self.req_data(timestamp.clone(), 0 as *const core::ffi::c_void, 0);
        htp_response::htp_connp_res_data(self, timestamp, 0 as *const core::ffi::c_void, 0);
    }

    /// This function is most likely not used and/or not needed.
    pub fn in_reset(&mut self) {
        self.in_content_length = -1;
        self.in_body_data_left = -1;
        self.in_chunk_request_index = self.in_chunk_count;
    }

    /// Returns the number of bytes consumed from the current data chunks so far or -1 on error.
    pub fn req_data_consumed(&self) -> i64 {
        self.in_current_read_offset
    }

    /// Returns the number of bytes consumed from the most recent outbound data chunk. Normally, an invocation
    /// of htp_connp_res_data() will consume all data from the supplied buffer, but there are circumstances
    /// where only partial consumption is possible. In such cases HTP_STREAM_DATA_OTHER will be returned.
    /// Consumed bytes are no longer necessary, but the remainder of the buffer will be need to be saved
    /// for later.
    /// Returns the number of bytes consumed from the last data chunk sent for outbound processing.
    /// or -1 on error.
    pub fn res_data_consumed(&self) -> i64 {
        self.out_current_read_offset
    }

    /// Opens connection.
    ///
    /// timestamp is optional
    pub unsafe fn open(
        &mut self,
        client_addr: Option<IpAddr>,
        client_port: i32,
        server_addr: Option<IpAddr>,
        server_port: i32,
        timestamp: Option<htp_time_t>,
    ) {
        // Check connection parser state first.
        if self.in_status != htp_stream_state_t::HTP_STREAM_NEW
            || self.out_status != htp_stream_state_t::HTP_STREAM_NEW
        {
            htp_error!(
                self as *mut htp_connp_t,
                htp_log_code::CONNECTION_ALREADY_OPEN,
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
        self.in_status = htp_stream_state_t::HTP_STREAM_OPEN;
        self.out_status = htp_stream_state_t::HTP_STREAM_OPEN;
    }

    /// Associate user data with the supplied parser.
    pub unsafe fn set_user_data(&mut self, user_data: *mut core::ffi::c_void) {
        (*self).user_data = user_data;
    }

    pub unsafe fn req_process_body_data_ex(
        &mut self,
        data: *const core::ffi::c_void,
        len: usize,
    ) -> Result<()> {
        if let Some(tx) = self.in_tx_mut() {
            tx.req_process_body_data_ex(data, len)
        } else {
            Err(Status::ERROR)
        }
    }

    /// Initialize hybrid parsing mode, change state to TRANSACTION_START,
    /// and invoke all registered callbacks.
    ///
    /// tx: Transaction pointer. Must not be NULL.
    ///
    /// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_request_start(&mut self) -> Result<()> {
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_start()
        } else {
            Err(Status::ERROR)
        }
    }

    /// Change transaction state to REQUEST_HEADERS and invoke all
    /// registered callbacks.
    ///
    /// tx: Transaction pointer. Must not be NULL.
    ///
    /// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_request_headers(&mut self) -> Result<()> {
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_headers()
        } else {
            Err(Status::ERROR)
        }
    }

    /// Change transaction state to REQUEST_LINE and invoke all
    /// registered callbacks.
    ///
    /// tx: Transaction pointer. Must not be NULL.
    ///
    /// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_request_line(&mut self) -> Result<()> {
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_line()
        } else {
            Err(Status::ERROR)
        }
    }

    /// Change transaction state to REQUEST and invoke registered callbacks.
    ///
    /// tx: Transaction pointer. Must not be NULL.
    ///
    /// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_request_complete(&mut self) -> Result<()> {
        if let Some(tx) = self.in_tx_mut() {
            tx.state_request_complete()
        } else {
            Err(Status::ERROR)
        }
    }

    pub unsafe fn res_process_body_data_ex(
        &mut self,
        data: *const core::ffi::c_void,
        len: usize,
    ) -> Result<()> {
        if let Some(tx) = self.out_tx_mut() {
            tx.res_process_body_data_ex(data, len)
        } else {
            Err(Status::ERROR)
        }
    }

    pub unsafe fn state_response_start(&mut self) -> Result<()> {
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_start()
        } else {
            Err(Status::ERROR)
        }
    }

    /// Change transaction state to RESPONSE_HEADERS and invoke registered callbacks.
    ///
    /// tx: Transaction pointer. Must not be NULL.
    ///
    /// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_response_headers(&mut self) -> Result<()> {
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_headers()
        } else {
            Err(Status::ERROR)
        }
    }

    /// Change transaction state to HTP_RESPONSE_LINE and invoke registered callbacks.
    ///
    /// tx: Transaction pointer. Must not be NULL.
    ///
    /// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_response_line(&mut self) -> Result<()> {
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_line()
        } else {
            Err(Status::ERROR)
        }
    }

    pub unsafe fn state_response_complete_ex(&mut self, hybrid_mode: i32) -> Result<()> {
        if let Some(tx) = self.out_tx_mut() {
            tx.state_response_complete_ex(hybrid_mode)
        } else {
            Err(Status::ERROR)
        }
    }
}

impl Drop for htp_connp_t {
    fn drop(&mut self) {
        unsafe {
            if !self.in_header.is_null() {
                bstr::bstr_free(self.in_header);
                self.in_header = std::ptr::null_mut()
            }
            if !self.out_header.is_null() {
                bstr::bstr_free(self.out_header);
                self.out_header = std::ptr::null_mut()
            }
        }
    }
}
