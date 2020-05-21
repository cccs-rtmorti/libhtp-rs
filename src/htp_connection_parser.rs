use crate::{
    bstr, htp_config, htp_connection, htp_decompressors, htp_hooks, htp_list, htp_request,
    htp_response, htp_transaction, htp_util, Status,
};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_connp_t {
    // General fields
    /// Current parser configuration structure.
    pub cfg: *mut htp_config::htp_cfg_t,
    /// The connection structure associated with this parser.
    pub conn: *mut htp_connection::htp_conn_t,
    /// Opaque user data associated with this parser.
    pub user_data: *const core::ffi::c_void,
    /// On parser failure, this field will contain the error information. Do note, however,
    /// that the value in this field will only be valid immediately after an error condition,
    /// but it is not guaranteed to remain valid if the parser is invoked again.
    pub last_error: *mut htp_util::htp_log_t,
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
    pub in_tx: *mut htp_transaction::htp_tx_t,
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
    pub in_data_receiver_hook: *mut htp_hooks::htp_hook_t,

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
    pub out_tx: *mut htp_transaction::htp_tx_t,
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
    pub out_data_receiver_hook: *mut htp_hooks::htp_hook_t,
    /// Response decompressor used to decompress response body data.
    pub out_decompressor: *mut htp_decompressors::htp_decompressor_t,
    /// On a PUT request, this field contains additional file data.
    pub put_file: *mut htp_util::htp_file_t,
}

pub type htp_time_t = libc::timeval;

/// Clears the most recent error, if any.
pub unsafe fn htp_connp_clear_error(mut connp: *mut htp_connp_t) {
    (*connp).last_error = 0 as *mut htp_util::htp_log_t;
}

/// Closes the connection associated with the supplied parser.
///
/// timestamp is optional
pub unsafe fn htp_connp_req_close(mut connp: *mut htp_connp_t, mut timestamp: *const htp_time_t) {
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
pub unsafe fn htp_connp_close(mut connp: *mut htp_connp_t, mut timestamp: *const htp_time_t) {
    if connp.is_null() {
        return;
    }
    // Close the underlying connection.
    htp_connection::htp_conn_close((*connp).conn, timestamp);
    // Update internal flags
    if (*connp).in_status != htp_stream_state_t::HTP_STREAM_ERROR {
        (*connp).in_status = htp_stream_state_t::HTP_STREAM_CLOSED
    }
    if (*connp).out_status != htp_stream_state_t::HTP_STREAM_ERROR {
        (*connp).out_status = htp_stream_state_t::HTP_STREAM_CLOSED
    }
    // Call the parsers one last time, which will allow them
    // to process the events that depend on stream closure
    htp_request::htp_connp_req_data(connp, timestamp, 0 as *const core::ffi::c_void, 0);
    htp_response::htp_connp_res_data(connp, timestamp, 0 as *const core::ffi::c_void, 0);
}

/// Creates a new connection parser using the provided configuration. Because
/// the configuration structure is used directly, in a multithreaded environment
/// you are not allowed to change the structure, ever. If you have a need to
/// change configuration on per-connection basis, make a copy of the configuration
/// structure to go along with every connection parser.
///
/// Returns a new connection parser instance, or NULL on error.
pub unsafe fn htp_connp_create(mut cfg: *mut htp_config::htp_cfg_t) -> *mut htp_connp_t {
    let mut connp: *mut htp_connp_t =
        calloc(1, ::std::mem::size_of::<htp_connp_t>()) as *mut htp_connp_t;
    if connp.is_null() {
        return 0 as *mut htp_connp_t;
    }
    // Use the supplied configuration structure
    (*connp).cfg = cfg;
    // Create a new connection.
    (*connp).conn = htp_connection::htp_conn_create();
    if (*connp).conn.is_null() {
        free(connp as *mut core::ffi::c_void);
        return 0 as *mut htp_connp_t;
    }
    // Request parsing
    (*connp).in_state = Some(
        htp_request::htp_connp_REQ_IDLE as unsafe extern "C" fn(_: *mut htp_connp_t) -> Status,
    );
    (*connp).in_status = htp_stream_state_t::HTP_STREAM_NEW;
    // Response parsing
    (*connp).out_state = Some(
        htp_response::htp_connp_RES_IDLE as unsafe extern "C" fn(_: *mut htp_connp_t) -> Status,
    );
    (*connp).out_status = htp_stream_state_t::HTP_STREAM_NEW;
    return connp;
}

/// Destroys the connection parser and its data structures, leaving
/// all the data (connection, transactions, etc) intact.
pub unsafe fn htp_connp_destroy(mut connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    if !(*connp).in_buf.is_null() {
        free((*connp).in_buf as *mut core::ffi::c_void);
    }
    if !(*connp).out_buf.is_null() {
        free((*connp).out_buf as *mut core::ffi::c_void);
    }
    htp_transaction::htp_connp_destroy_decompressors(connp);
    if !(*connp).put_file.is_null() {
        bstr::bstr_free((*(*connp).put_file).filename);
        free((*connp).put_file as *mut core::ffi::c_void);
    }
    if !(*connp).in_header.is_null() {
        bstr::bstr_free((*connp).in_header);
        (*connp).in_header = 0 as *mut bstr::bstr_t
    }
    if !(*connp).out_header.is_null() {
        bstr::bstr_free((*connp).out_header);
        (*connp).out_header = 0 as *mut bstr::bstr_t
    }
    free(connp as *mut core::ffi::c_void);
}

/// Destroys the connection parser, its data structures, as well
/// as the connection and its transactions.
pub unsafe fn htp_connp_destroy_all(mut connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    // Destroy connection
    htp_connection::htp_conn_destroy((*connp).conn);
    (*connp).conn = 0 as *mut htp_connection::htp_conn_t;
    // Destroy everything else
    htp_connp_destroy(connp);
}

/// Returns the connection associated with the connection parser.
/// Returns a htp_conn_t instance, or NULL if one is not available.
pub unsafe fn htp_connp_get_connection(
    mut connp: *const htp_connp_t,
) -> *mut htp_connection::htp_conn_t {
    if connp.is_null() {
        return 0 as *mut htp_connection::htp_conn_t;
    }
    return (*connp).conn;
}

/// Retrieves the pointer to the active inbound transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Can be NULL.
///
/// Returns active inbound transaction, or NULL if there isn't one.
pub unsafe fn htp_connp_get_in_tx(mut connp: *const htp_connp_t) -> *mut htp_transaction::htp_tx_t {
    if connp.is_null() {
        return 0 as *mut htp_transaction::htp_tx_t;
    }
    return (*connp).in_tx;
}

/// Returns the last error that occurred with this connection parser. Do note, however,
/// that the value in this field will only be valid immediately after an error condition,
/// but it is not guaranteed to remain valid if the parser is invoked again.
///
/// Returns a pointer to an htp_util::htp_log_t instance if there is an error, or NULL
///         if there isn't.
pub unsafe fn htp_connp_get_last_error(mut connp: *const htp_connp_t) -> *mut htp_util::htp_log_t {
    if connp.is_null() {
        return 0 as *mut htp_util::htp_log_t;
    }
    return (*connp).last_error;
}

/// Retrieves the pointer to the active outbound transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Can be NULL.
///
/// Returns active outbound transaction, or NULL if there isn't one.
pub unsafe fn htp_connp_get_out_tx(
    mut connp: *const htp_connp_t,
) -> *mut htp_transaction::htp_tx_t {
    if connp.is_null() {
        return 0 as *mut htp_transaction::htp_tx_t;
    }
    return (*connp).out_tx;
}

/// Retrieve the user data associated with this connection parser.
///
/// Returns user data, or NULL if there isn't any.
pub unsafe fn htp_connp_get_user_data(mut connp: *const htp_connp_t) -> *mut core::ffi::c_void {
    if connp.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    return (*connp).user_data as *mut core::ffi::c_void;
}

/// This function is most likely not used and/or not needed.
pub unsafe fn htp_connp_in_reset(mut connp: *mut htp_connp_t) {
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
    mut connp: *mut htp_connp_t,
    mut client_addr: *const i8,
    mut client_port: i32,
    mut server_addr: *const i8,
    mut server_port: i32,
    mut timestamp: *mut htp_time_t,
) {
    if connp.is_null() {
        return;
    }
    // Check connection parser state first.
    if (*connp).in_status != htp_stream_state_t::HTP_STREAM_NEW
        || (*connp).out_status != htp_stream_state_t::HTP_STREAM_NEW
    {
        htp_util::htp_log(
            connp,
            b"htp_connection_parser.c\x00" as *const u8 as *const i8,
            181,
            htp_util::htp_log_level_t::HTP_LOG_ERROR,
            0,
            b"Connection is already open\x00" as *const u8 as *const i8,
        );
        return;
    }
    if htp_connection::htp_conn_open(
        (*connp).conn,
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
pub unsafe fn htp_connp_set_user_data(
    mut connp: *mut htp_connp_t,
    mut user_data: *const core::ffi::c_void,
) {
    if connp.is_null() {
        return;
    }
    (*connp).user_data = user_data;
}

/// Create a new transaction using the connection parser provided.
///
/// Returns new transaction instance on success, NULL on failure.
pub unsafe fn htp_connp_tx_create(mut connp: *mut htp_connp_t) -> *mut htp_transaction::htp_tx_t {
    if connp.is_null() {
        return 0 as *mut htp_transaction::htp_tx_t;
    }
    // Detect pipelining.
    if htp_list::htp_list_array_size((*(*connp).conn).transactions) > (*connp).out_next_tx_index {
        (*(*connp).conn).flags |= htp_util::ConnectionFlags::HTP_CONN_PIPELINED
    }
    let mut tx: *mut htp_transaction::htp_tx_t = htp_transaction::htp_tx_create(connp);
    if tx.is_null() {
        return 0 as *mut htp_transaction::htp_tx_t;
    }
    (*connp).in_tx = tx;
    htp_connp_in_reset(connp);
    return tx;
}

/// Removes references to the supplied transaction.
pub unsafe fn htp_connp_tx_remove(
    mut connp: *mut htp_connp_t,
    mut tx: *mut htp_transaction::htp_tx_t,
) {
    if connp.is_null() {
        return;
    }
    if (*connp).in_tx == tx {
        (*connp).in_tx = 0 as *mut htp_transaction::htp_tx_t
    }
    if (*connp).out_tx == tx {
        (*connp).out_tx = 0 as *mut htp_transaction::htp_tx_t
    };
}
