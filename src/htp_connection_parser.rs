use crate::{
    bstr, hook::DataHook, htp_config, htp_connection, htp_decompressors, htp_request, htp_response,
    htp_transaction, htp_util, Status,
};
use std::net::IpAddr;

extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
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
    pub in_buf: *mut u8,
    /// Stores the size of the buffer. Valid only when htp_tx_t::in_buf is not NULL.
    pub in_buf_size: usize,
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
    pub in_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> Status>,
    /// Previous request parser state. Used to detect state changes.
    pub in_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> Status>,
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
    pub out_buf: *mut u8,
    /// Stores the size of the buffer. Valid only when htp_tx_t::out_buf is not NULL.
    pub out_buf_size: usize,
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
    pub out_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> Status>,
    /// Previous response parser state.
    pub out_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> Status>,
    /// The hook that should be receiving raw connection data.
    pub out_data_receiver_hook: Option<DataHook>,
    /// Response decompressor used to decompress response body data.
    pub out_decompressor: *mut htp_decompressors::htp_decompressor_t,
    /// On a PUT request, this field contains additional file data.
    pub put_file: *mut htp_util::htp_file_t,
}

impl htp_connp_t {
    fn new(cfg: *mut htp_config::htp_cfg_t) -> Self {
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
            in_buf: std::ptr::null_mut(),
            in_buf_size: 0,
            in_header: std::ptr::null_mut(),
            in_tx: None,
            in_content_length: 0,
            in_body_data_left: 0,
            in_chunked_length: 0,
            in_state: Some(
                htp_request::htp_connp_REQ_IDLE
                    as unsafe extern "C" fn(_: *mut htp_connp_t) -> Status,
            ),
            in_state_previous: None,
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
            out_buf: std::ptr::null_mut(),
            out_buf_size: 0,
            out_header: std::ptr::null_mut(),
            out_tx: None,
            out_content_length: 0,
            out_body_data_left: 0,
            out_chunked_length: 0,
            out_state: Some(
                htp_response::htp_connp_RES_IDLE
                    as unsafe extern "C" fn(_: *mut htp_connp_t) -> Status,
            ),
            out_state_previous: None,
            out_data_receiver_hook: None,
            out_decompressor: std::ptr::null_mut(),
            put_file: std::ptr::null_mut(),
        }
    }

    /// Creates a transaction and attaches it to this connection.
    ///
    /// Also sets the in_tx to the newly created one.
    pub unsafe fn create_tx(&mut self) -> Result<usize, Status> {
        // Detect pipelining.
        if self.conn.tx_size() > self.out_next_tx_index {
            self.conn.flags |= htp_util::ConnectionFlags::HTP_CONN_PIPELINED
        }
        htp_transaction::htp_tx_t::new(self).map(|tx_id| {
            self.in_tx = Some(tx_id);
            htp_connp_in_reset(self);
            tx_id
        })
    }

    /// Removes references to the supplied transaction.
    pub unsafe fn remove_tx(&mut self, tx: usize) {
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
    pub unsafe fn in_tx(&self) -> Option<&htp_transaction::htp_tx_t> {
        self.in_tx.and_then(|in_tx| self.conn.tx(in_tx))
    }

    /// Get the in_tx as a mutable reference or None if not set.
    pub unsafe fn in_tx_mut(&mut self) -> Option<&mut htp_transaction::htp_tx_t> {
        self.in_tx.and_then(move |in_tx| self.conn.tx_mut(in_tx))
    }

    /// Get the in_tx as a pointer or NULL if not set.
    pub unsafe fn in_tx_ptr(&self) -> *const htp_transaction::htp_tx_t {
        self.in_tx()
            .map(|in_tx| in_tx as *const htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null())
    }

    /// Get the in_tx as a mutable pointer or NULL if not set.
    pub unsafe fn in_tx_mut_ptr(&mut self) -> *mut htp_transaction::htp_tx_t {
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
    pub unsafe fn out_tx(&self) -> Option<&htp_transaction::htp_tx_t> {
        self.out_tx.and_then(|out_tx| self.conn.tx(out_tx))
    }

    /// Get the out_tx as a mutable reference or None if not set.
    pub unsafe fn out_tx_mut(&mut self) -> Option<&mut htp_transaction::htp_tx_t> {
        self.out_tx.and_then(move |out_tx| self.conn.tx_mut(out_tx))
    }

    /// Get the out_tx as a pointer or NULL if not set.
    pub unsafe fn out_tx_ptr(&self) -> *const htp_transaction::htp_tx_t {
        self.out_tx()
            .map(|out_tx| out_tx as *const htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null())
    }

    /// Get the out_tx as a mutable pointer or NULL if not set.
    pub unsafe fn out_tx_mut_ptr(&mut self) -> *mut htp_transaction::htp_tx_t {
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
}

impl Drop for htp_connp_t {
    fn drop(&mut self) {
        unsafe {
            if !self.in_buf.is_null() {
                free(self.in_buf as *mut core::ffi::c_void);
            }
            if !self.out_buf.is_null() {
                free(self.out_buf as *mut core::ffi::c_void);
            }
            htp_transaction::htp_connp_destroy_decompressors(&mut *self);
            if !self.put_file.is_null() {
                bstr::bstr_free((*self.put_file).filename);
                free(self.put_file as *mut core::ffi::c_void);
            }
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

/// Closes the connection associated with the supplied parser.
///
/// timestamp is optional
pub unsafe fn htp_connp_req_close(connp: *mut htp_connp_t, timestamp: Option<htp_time_t>) {
    if connp.is_null() {
        return;
    }
    // Update internal flags
    if (*connp).in_status != htp_stream_state_t::HTP_STREAM_ERROR {
        (*connp).in_status = htp_stream_state_t::HTP_STREAM_CLOSED
    }
    // Call the parsers one last time, which will allow them
    // to process the events that depend on stream closure
    htp_request::htp_connp_req_data(connp, timestamp, 0 as *const core::ffi::c_void, 0);
}

/// Closes the connection associated with the supplied parser.
///
/// timestamp is optional
pub unsafe fn htp_connp_close(connp: *mut htp_connp_t, timestamp: Option<htp_time_t>) {
    if connp.is_null() {
        return;
    }
    // Close the underlying connection.
    (*connp).conn.close(timestamp.clone());
    // Update internal flags
    if (*connp).in_status != htp_stream_state_t::HTP_STREAM_ERROR {
        (*connp).in_status = htp_stream_state_t::HTP_STREAM_CLOSED
    }
    if (*connp).out_status != htp_stream_state_t::HTP_STREAM_ERROR {
        (*connp).out_status = htp_stream_state_t::HTP_STREAM_CLOSED
    }
    // Call the parsers one last time, which will allow them
    // to process the events that depend on stream closure
    htp_request::htp_connp_req_data(connp, timestamp.clone(), 0 as *const core::ffi::c_void, 0);
    htp_response::htp_connp_res_data(connp, timestamp, 0 as *const core::ffi::c_void, 0);
}

/// Creates a new connection parser using the provided configuration. Because
/// the configuration structure is used directly, in a multithreaded environment
/// you are not allowed to change the structure, ever. If you have a need to
/// change configuration on per-connection basis, make a copy of the configuration
/// structure to go along with every connection parser.
///
/// Returns a new connection parser instance, or NULL on error.
pub fn htp_connp_create(cfg: *mut htp_config::htp_cfg_t) -> *mut htp_connp_t {
    Box::into_raw(Box::new(htp_connp_t::new(cfg)))
}

/// Destroys the connection parser and its data structures, leaving
/// all the data (connection, transactions, etc) intact.
pub unsafe fn htp_connp_destroy(connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    // Take back ownership of the box that was consumed in htp_connp_create()
    let _ = Box::from_raw(connp);
}

/// Destroys the connection parser, its data structures, as well
/// as the connection and its transactions.
pub unsafe fn htp_connp_destroy_all(connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    // Destroy everything else
    htp_connp_destroy(connp);
}

/// This function is most likely not used and/or not needed.
pub unsafe fn htp_connp_in_reset(connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    (*connp).in_content_length = -1;
    (*connp).in_body_data_left = -1;
    (*connp).in_chunk_request_index = (*connp).in_chunk_count;
}

/// Opens connection.
///
/// timestamp is optional
pub unsafe fn htp_connp_open(
    connp: *mut htp_connp_t,
    client_addr: Option<IpAddr>,
    client_port: i32,
    server_addr: Option<IpAddr>,
    server_port: i32,
    timestamp: Option<htp_time_t>,
) {
    if connp.is_null() {
        return;
    }
    // Check connection parser state first.
    if (*connp).in_status != htp_stream_state_t::HTP_STREAM_NEW
        || (*connp).out_status != htp_stream_state_t::HTP_STREAM_NEW
    {
        htp_error!(
            connp,
            htp_log_code::CONNECTION_ALREADY_OPEN,
            "Connection is already open"
        );
        return;
    }
    if (*connp).conn.open(
        client_addr,
        client_port,
        server_addr,
        server_port,
        timestamp,
    ) != Status::OK
    {
        return;
    }
    (*connp).in_status = htp_stream_state_t::HTP_STREAM_OPEN;
    (*connp).out_status = htp_stream_state_t::HTP_STREAM_OPEN;
}

/// Associate user data with the supplied parser.
pub unsafe fn htp_connp_set_user_data(connp: *mut htp_connp_t, user_data: *mut core::ffi::c_void) {
    if connp.is_null() {
        return;
    }
    (*connp).user_data = user_data;
}
