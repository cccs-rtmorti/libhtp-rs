use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn htp_list_array_size(l: *const crate::src::htp_list::htp_list_array_t) -> size_t;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn htp_connp_res_data(
        connp: *mut htp_connp_t,
        timestamp: *const htp_time_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_connp_req_data(
        connp: *mut htp_connp_t,
        timestamp: *const htp_time_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_conn_close(
        conn: *mut crate::src::htp_connection::htp_conn_t,
        timestamp: *const htp_time_t,
    );
    #[no_mangle]
    fn htp_connp_RES_IDLE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_IDLE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_conn_create() -> *mut crate::src::htp_connection::htp_conn_t;
    #[no_mangle]
    fn htp_connp_destroy_decompressors(connp: *mut htp_connp_t);
    #[no_mangle]
    fn htp_conn_destroy(conn: *mut crate::src::htp_connection::htp_conn_t);
    #[no_mangle]
    fn htp_conn_open(
        conn: *mut crate::src::htp_connection::htp_conn_t,
        remote_addr: *const libc::c_char,
        remote_port: libc::c_int,
        local_addr: *const libc::c_char,
        local_port: libc::c_int,
        timestamp: *const htp_time_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_log(
        connp: *mut htp_connp_t,
        file: *const libc::c_char,
        line: libc::c_int,
        level: htp_log_level_t,
        code: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn htp_tx_create(connp: *mut htp_connp_t) -> *mut crate::src::htp_transaction::htp_tx_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type htp_status_t = libc::c_int;

/* *
 * Enumerates the ways in which servers respond to malformed data.
 */
pub type htp_unwanted_t = libc::c_uint;
/* * Responds with HTTP 404 status code. */
pub const HTP_UNWANTED_404: htp_unwanted_t = 404;
/* * Responds with HTTP 400 status code. */
pub const HTP_UNWANTED_400: htp_unwanted_t = 400;
/* * Ignores problem. */
pub const HTP_UNWANTED_IGNORE: htp_unwanted_t = 0;

/* *
 * Enumerates the possible approaches to handling invalid URL-encodings.
 */
pub type htp_url_encoding_handling_t = libc::c_uint;
/* * Decode invalid URL encodings. */
pub const HTP_URL_DECODE_PROCESS_INVALID: htp_url_encoding_handling_t = 2;
/* * Ignore invalid URL encodings, but remove the % from the data. */
pub const HTP_URL_DECODE_REMOVE_PERCENT: htp_url_encoding_handling_t = 1;
/* * Ignore invalid URL encodings and leave the % in the data. */
pub const HTP_URL_DECODE_PRESERVE_PERCENT: htp_url_encoding_handling_t = 0;

// A collection of unique parser IDs.
pub type htp_parser_id_t = libc::c_uint;
/* * multipart/form-data parser. */
pub const HTP_PARSER_MULTIPART: htp_parser_id_t = 1;
/* * application/x-www-form-urlencoded parser. */
pub const HTP_PARSER_URLENCODED: htp_parser_id_t = 0;
// Protocol version constants; an enum cannot be
// used here because we allow any properly-formatted protocol
// version (e.g., 1.3), even those that do not actually exist.
// A collection of possible data sources.
pub type htp_data_source_t = libc::c_uint;
/* * Transported in the request body. */
pub const HTP_SOURCE_BODY: htp_data_source_t = 3;
/* * Cookies. */
pub const HTP_SOURCE_COOKIE: htp_data_source_t = 2;
/* * Transported in the query string. */
pub const HTP_SOURCE_QUERY_STRING: htp_data_source_t = 1;
/* * Embedded in the URL. */
pub const HTP_SOURCE_URL: htp_data_source_t = 0;
pub type bstr = crate::src::bstr::bstr_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_connp_t {
    // General fields
    /** Current parser configuration structure. */
    pub cfg: *mut crate::src::htp_config::htp_cfg_t,
    /** The connection structure associated with this parser. */
    pub conn: *mut crate::src::htp_connection::htp_conn_t,
    /** Opaque user data associated with this parser. */
    pub user_data: *const libc::c_void,
    /**
     * On parser failure, this field will contain the error information. Do note, however,
     * that the value in this field will only be valid immediately after an error condition,
     * but it is not guaranteed to remain valid if the parser is invoked again.
     */
    pub last_error: *mut crate::src::htp_util::htp_log_t,
    // Request parser fields
    /** Parser inbound status. Starts as HTP_OK, but may turn into HTP_ERROR. */
    pub in_status: htp_stream_state_t,
    /** Parser output status. Starts as HTP_OK, but may turn into HTP_ERROR. */
    pub out_status: htp_stream_state_t,
    /**
     * When true, this field indicates that there is unprocessed inbound data, and
     * that the response parsing code should stop at the end of the current request
     * in order to allow more requests to be produced.
     */
    pub out_data_other_at_tx_end: libc::c_uint,
    /**
     * The time when the last request data chunk was received. Can be NULL if
     * the upstream code is not providing the timestamps when calling us.
     */
    pub in_timestamp: htp_time_t,
    /** Pointer to the current request data chunk. */
    pub in_current_data: *mut libc::c_uchar,
    /** The length of the current request data chunk. */
    pub in_current_len: int64_t,
    /** The offset of the next byte in the request data chunk to read. */
    pub in_current_read_offset: int64_t,
    /**
     * The starting point of the data waiting to be consumed. This field is used
     * in the states where reading data is not the same as consumption.
     */
    pub in_current_consume_offset: int64_t,
    /**
     * Marks the starting point of raw data within the inbound data chunk. Raw
     * data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
     * REQUEST_HEADER_DATA).
     */
    pub in_current_receiver_offset: int64_t,
    /** How many data chunks does the inbound connection stream consist of? */
    pub in_chunk_count: size_t,
    /** The index of the first chunk used in the current request. */
    pub in_chunk_request_index: size_t,
    /** The offset, in the entire connection stream, of the next request byte. */
    pub in_stream_offset: int64_t,
    /**
     * The value of the request byte currently being processed. This field is
     * populated when the IN_NEXT_* or IN_PEEK_* macros are invoked.
     */
    pub in_next_byte: libc::c_int,
    /** Used to buffer a line of inbound data when buffering cannot be avoided. */
    pub in_buf: *mut libc::c_uchar,
    /** Stores the size of the buffer. Valid only when htp_tx_t::in_buf is not NULL. */
    pub in_buf_size: size_t,
    /**
     * Stores the current value of a folded request header. Such headers span
     * multiple lines, and are processed only when all data is available.
     */
    pub in_header: *mut bstr,
    /** Ongoing inbound transaction. */
    pub in_tx: *mut crate::src::htp_transaction::htp_tx_t,
    /**
     * The request body length declared in a valid request header. The key here
     * is "valid". This field will not be populated if the request contains both
     * a Transfer-Encoding header and a Content-Length header.
     */
    pub in_content_length: int64_t,
    /**
     * Holds the remaining request body length that we expect to read. This
     * field will be available only when the length of a request body is known
     * in advance, i.e. when request headers contain a Content-Length header.
     */
    pub in_body_data_left: int64_t,
    /**
     * Holds the amount of data that needs to be read from the
     * current data chunk. Only used with chunked request bodies.
     */
    pub in_chunked_length: int64_t,
    /** Current request parser state. */
    pub in_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> libc::c_int>,
    /** Previous request parser state. Used to detect state changes. */
    pub in_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> libc::c_int>,
    /** The hook that should be receiving raw connection data. */
    pub in_data_receiver_hook: *mut crate::src::htp_hooks::htp_hook_t,

    /**
     * Response counter, incremented with every new response. This field is
     * used to match responses to requests. The expectation is that for every
     * response there will already be a transaction (request) waiting.
     */
    pub out_next_tx_index: size_t,
    /** The time when the last response data chunk was received. Can be NULL. */
    pub out_timestamp: htp_time_t,
    /** Pointer to the current response data chunk. */
    pub out_current_data: *mut libc::c_uchar,
    /** The length of the current response data chunk. */
    pub out_current_len: int64_t,
    /** The offset of the next byte in the response data chunk to consume. */
    pub out_current_read_offset: int64_t,
    /**
     * The starting point of the data waiting to be consumed. This field is used
     * in the states where reading data is not the same as consumption.
     */
    pub out_current_consume_offset: int64_t,
    /**
     * Marks the starting point of raw data within the outbound data chunk. Raw
     * data (e.g., complete headers) is sent to appropriate callbacks (e.g.,
     * RESPONSE_HEADER_DATA).
     */
    pub out_current_receiver_offset: int64_t,
    /** The offset, in the entire connection stream, of the next response byte. */
    pub out_stream_offset: int64_t,
    /** The value of the response byte currently being processed. */
    pub out_next_byte: libc::c_int,
    /** Used to buffer a line of outbound data when buffering cannot be avoided. */
    pub out_buf: *mut libc::c_uchar,
    /** Stores the size of the buffer. Valid only when htp_tx_t::out_buf is not NULL. */
    pub out_buf_size: size_t,
    /**
     * Stores the current value of a folded response header. Such headers span
     * multiple lines, and are processed only when all data is available.
     */
    pub out_header: *mut bstr,
    /** Ongoing outbound transaction */
    pub out_tx: *mut crate::src::htp_transaction::htp_tx_t,
    /**
     * The length of the current response body as presented in the
     * Content-Length response header.
     */
    pub out_content_length: int64_t,
    /** The remaining length of the current response body, if known. Set to -1 otherwise. */
    pub out_body_data_left: int64_t,
    /**
     * Holds the amount of data that needs to be read from the
     * current response data chunk. Only used with chunked response bodies.
     */
    pub out_chunked_length: int64_t,
    /** Current response parser state. */
    pub out_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> libc::c_int>,
    /** Previous response parser state. */
    pub out_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t) -> libc::c_int>,
    /** The hook that should be receiving raw connection data. */
    pub out_data_receiver_hook: *mut crate::src::htp_hooks::htp_hook_t,
    /** Response decompressor used to decompress response body data. */
    pub out_decompressor: *mut crate::src::htp_decompressors::htp_decompressor_t,
    /** On a PUT request, this field contains additional file data. */
    pub put_file: *mut crate::src::htp_util::htp_file_t,
}

pub type htp_file_source_t = libc::c_uint;
pub const HTP_FILE_PUT: htp_file_source_t = 2;
pub const HTP_FILE_MULTIPART: htp_file_source_t = 1;

/* *
 * Possible states of a progressing transaction. Internally, progress will change
 * to the next state when the processing activities associated with that state
 * begin. For example, when we start to process request line bytes, the request
 * state will change from HTP_REQUEST_NOT_STARTED to HTP_REQUEST_LINE.*
 */
pub type htp_tx_res_progress_t = libc::c_uint;
pub const HTP_RESPONSE_COMPLETE: htp_tx_res_progress_t = 5;
pub const HTP_RESPONSE_TRAILER: htp_tx_res_progress_t = 4;
pub const HTP_RESPONSE_BODY: htp_tx_res_progress_t = 3;
pub const HTP_RESPONSE_HEADERS: htp_tx_res_progress_t = 2;
pub const HTP_RESPONSE_LINE: htp_tx_res_progress_t = 1;
pub const HTP_RESPONSE_NOT_STARTED: htp_tx_res_progress_t = 0;
pub type htp_tx_req_progress_t = libc::c_uint;
pub const HTP_REQUEST_COMPLETE: htp_tx_req_progress_t = 5;
pub const HTP_REQUEST_TRAILER: htp_tx_req_progress_t = 4;
pub const HTP_REQUEST_BODY: htp_tx_req_progress_t = 3;
pub const HTP_REQUEST_HEADERS: htp_tx_req_progress_t = 2;
pub const HTP_REQUEST_LINE: htp_tx_req_progress_t = 1;
pub const HTP_REQUEST_NOT_STARTED: htp_tx_req_progress_t = 0;
pub type htp_content_encoding_t = libc::c_uint;
pub const HTP_COMPRESSION_LZMA: htp_content_encoding_t = 4;
pub const HTP_COMPRESSION_DEFLATE: htp_content_encoding_t = 3;
pub const HTP_COMPRESSION_GZIP: htp_content_encoding_t = 2;
pub const HTP_COMPRESSION_NONE: htp_content_encoding_t = 1;
pub const HTP_COMPRESSION_UNKNOWN: htp_content_encoding_t = 0;
pub type htp_transfer_coding_t = libc::c_uint;
pub const HTP_CODING_INVALID: htp_transfer_coding_t = 4;
pub const HTP_CODING_CHUNKED: htp_transfer_coding_t = 3;
pub const HTP_CODING_IDENTITY: htp_transfer_coding_t = 2;
pub const HTP_CODING_NO_BODY: htp_transfer_coding_t = 1;
pub const HTP_CODING_UNKNOWN: htp_transfer_coding_t = 0;

pub type htp_table_alloc_t = libc::c_uint;
pub const HTP_TABLE_KEYS_REFERENCED: htp_table_alloc_t = 3;
pub const HTP_TABLE_KEYS_ADOPTED: htp_table_alloc_t = 2;
pub const HTP_TABLE_KEYS_COPIED: htp_table_alloc_t = 1;
pub const HTP_TABLE_KEYS_ALLOC_UKNOWN: htp_table_alloc_t = 0;
pub type htp_auth_type_t = libc::c_uint;
pub const HTP_AUTH_UNRECOGNIZED: htp_auth_type_t = 9;
pub const HTP_AUTH_DIGEST: htp_auth_type_t = 3;
pub const HTP_AUTH_BASIC: htp_auth_type_t = 2;
pub const HTP_AUTH_NONE: htp_auth_type_t = 1;
pub const HTP_AUTH_UNKNOWN: htp_auth_type_t = 0;

pub type htp_part_mode_t = libc::c_uint;
pub const MODE_DATA: htp_part_mode_t = 1;
pub const MODE_LINE: htp_part_mode_t = 0;

pub type htp_multipart_type_t = libc::c_uint;
pub const MULTIPART_PART_EPILOGUE: htp_multipart_type_t = 4;
pub const MULTIPART_PART_PREAMBLE: htp_multipart_type_t = 3;
pub const MULTIPART_PART_FILE: htp_multipart_type_t = 2;
pub const MULTIPART_PART_TEXT: htp_multipart_type_t = 1;
pub const MULTIPART_PART_UNKNOWN: htp_multipart_type_t = 0;
pub type htp_multipart_state_t = libc::c_uint;
pub const STATE_BOUNDARY_EAT_LWS_CR: htp_multipart_state_t = 6;
pub const STATE_BOUNDARY_EAT_LWS: htp_multipart_state_t = 5;
pub const STATE_BOUNDARY_IS_LAST2: htp_multipart_state_t = 4;
pub const STATE_BOUNDARY_IS_LAST1: htp_multipart_state_t = 3;
pub const STATE_BOUNDARY: htp_multipart_state_t = 2;
pub const STATE_DATA: htp_multipart_state_t = 1;
pub const STATE_INIT: htp_multipart_state_t = 0;

pub type htp_method_t = libc::c_uint;
pub const HTP_M_INVALID: htp_method_t = 28;
pub const HTP_M_MERGE: htp_method_t = 27;
pub const HTP_M_BASELINE_CONTROL: htp_method_t = 26;
pub const HTP_M_MKACTIVITY: htp_method_t = 25;
pub const HTP_M_MKWORKSPACE: htp_method_t = 24;
pub const HTP_M_REPORT: htp_method_t = 23;
pub const HTP_M_LABEL: htp_method_t = 22;
pub const HTP_M_UPDATE: htp_method_t = 21;
pub const HTP_M_CHECKIN: htp_method_t = 20;
pub const HTP_M_UNCHECKOUT: htp_method_t = 19;
pub const HTP_M_CHECKOUT: htp_method_t = 18;
pub const HTP_M_VERSION_CONTROL: htp_method_t = 17;
pub const HTP_M_UNLOCK: htp_method_t = 16;
pub const HTP_M_LOCK: htp_method_t = 15;
pub const HTP_M_MOVE: htp_method_t = 14;
pub const HTP_M_COPY: htp_method_t = 13;
pub const HTP_M_MKCOL: htp_method_t = 12;
pub const HTP_M_PROPPATCH: htp_method_t = 11;
pub const HTP_M_PROPFIND: htp_method_t = 10;
pub const HTP_M_PATCH: htp_method_t = 9;
pub const HTP_M_TRACE: htp_method_t = 8;
pub const HTP_M_OPTIONS: htp_method_t = 7;
pub const HTP_M_CONNECT: htp_method_t = 6;
pub const HTP_M_DELETE: htp_method_t = 5;
pub const HTP_M_POST: htp_method_t = 4;
pub const HTP_M_PUT: htp_method_t = 3;
pub const HTP_M_GET: htp_method_t = 2;
pub const HTP_M_HEAD: htp_method_t = 1;
pub const HTP_M_UNKNOWN: htp_method_t = 0;

pub type htp_time_t = libc::timeval;
/* *
 * Enumerates all stream states. Each connection has two streams, one
 * inbound and one outbound. Their states are tracked separately.
 */
pub type htp_stream_state_t = libc::c_uint;
pub const HTP_STREAM_DATA: htp_stream_state_t = 9;
pub const HTP_STREAM_STOP: htp_stream_state_t = 6;
pub const HTP_STREAM_DATA_OTHER: htp_stream_state_t = 5;
pub const HTP_STREAM_TUNNEL: htp_stream_state_t = 4;
pub const HTP_STREAM_ERROR: htp_stream_state_t = 3;
pub const HTP_STREAM_CLOSED: htp_stream_state_t = 2;
pub const HTP_STREAM_OPEN: htp_stream_state_t = 1;
pub const HTP_STREAM_NEW: htp_stream_state_t = 0;

pub type htp_log_level_t = libc::c_uint;
pub const HTP_LOG_DEBUG2: htp_log_level_t = 6;
pub const HTP_LOG_DEBUG: htp_log_level_t = 5;
pub const HTP_LOG_INFO: htp_log_level_t = 4;
pub const HTP_LOG_NOTICE: htp_log_level_t = 3;
pub const HTP_LOG_WARNING: htp_log_level_t = 2;
pub const HTP_LOG_ERROR: htp_log_level_t = 1;
pub const HTP_LOG_NONE: htp_log_level_t = 0;
pub type htp_server_personality_t = libc::c_uint;
pub const HTP_SERVER_APACHE_2: htp_server_personality_t = 9;
pub const HTP_SERVER_IIS_7_5: htp_server_personality_t = 8;
pub const HTP_SERVER_IIS_7_0: htp_server_personality_t = 7;
pub const HTP_SERVER_IIS_6_0: htp_server_personality_t = 6;
pub const HTP_SERVER_IIS_5_1: htp_server_personality_t = 5;
pub const HTP_SERVER_IIS_5_0: htp_server_personality_t = 4;
pub const HTP_SERVER_IIS_4_0: htp_server_personality_t = 3;
pub const HTP_SERVER_IDS: htp_server_personality_t = 2;
pub const HTP_SERVER_GENERIC: htp_server_personality_t = 1;
pub const HTP_SERVER_MINIMAL: htp_server_personality_t = 0;

/**
 * Clears the most recent error, if any.
 *
 * @param[in] connp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_clear_error(mut connp: *mut htp_connp_t) {
    (*connp).last_error = 0 as *mut crate::src::htp_util::htp_log_t;
}

/**
 * Closes the connection associated with the supplied parser.
 *
 * @param[in] connp
 * @param[in] timestamp Optional.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_req_close(
    mut connp: *mut htp_connp_t,
    mut timestamp: *const htp_time_t,
) {
    if connp.is_null() {
        return;
    }
    // Update internal flags
    if (*connp).in_status as libc::c_uint != HTP_STREAM_ERROR as libc::c_int as libc::c_uint {
        (*connp).in_status = HTP_STREAM_CLOSED
    }
    // Call the parsers one last time, which will allow them
    // to process the events that depend on stream closure
    htp_connp_req_data(
        connp,
        timestamp,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}

/**
 * Closes the connection associated with the supplied parser.
 *
 * @param[in] connp
 * @param[in] timestamp Optional.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_close(
    mut connp: *mut htp_connp_t,
    mut timestamp: *const htp_time_t,
) {
    if connp.is_null() {
        return;
    }
    // Close the underlying connection.
    htp_conn_close((*connp).conn, timestamp);
    // Update internal flags
    if (*connp).in_status as libc::c_uint != HTP_STREAM_ERROR as libc::c_int as libc::c_uint {
        (*connp).in_status = HTP_STREAM_CLOSED
    }
    if (*connp).out_status as libc::c_uint != HTP_STREAM_ERROR as libc::c_int as libc::c_uint {
        (*connp).out_status = HTP_STREAM_CLOSED
    }
    // Call the parsers one last time, which will allow them
    // to process the events that depend on stream closure
    htp_connp_req_data(
        connp,
        timestamp,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    htp_connp_res_data(
        connp,
        timestamp,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}

/**
 * Creates a new connection parser using the provided configuration. Because
 * the configuration structure is used directly, in a multithreaded environment
 * you are not allowed to change the structure, ever. If you have a need to
 * change configuration on per-connection basis, make a copy of the configuration
 * structure to go along with every connection parser.
 *
 * @param[in] cfg
 * @return New connection parser instance, or NULL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_create(
    mut cfg: *mut crate::src::htp_config::htp_cfg_t,
) -> *mut htp_connp_t {
    let mut connp: *mut htp_connp_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_connp_t>() as libc::c_ulong,
    ) as *mut htp_connp_t;
    if connp.is_null() {
        return 0 as *mut htp_connp_t;
    }
    // Use the supplied configuration structure
    (*connp).cfg = cfg;
    // Create a new connection.
    (*connp).conn = htp_conn_create();
    if (*connp).conn.is_null() {
        free(connp as *mut libc::c_void);
        return 0 as *mut htp_connp_t;
    }
    // Request parsing
    (*connp).in_state =
        Some(htp_connp_REQ_IDLE as unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t);
    (*connp).in_status = HTP_STREAM_NEW;
    // Response parsing
    (*connp).out_state =
        Some(htp_connp_RES_IDLE as unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t);
    (*connp).out_status = HTP_STREAM_NEW;
    return connp;
}

/**
 * Destroys the connection parser and its data structures, leaving
 * all the data (connection, transactions, etc) intact.
 *
 * @param[in] connp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_destroy(mut connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    if !(*connp).in_buf.is_null() {
        free((*connp).in_buf as *mut libc::c_void);
    }
    if !(*connp).out_buf.is_null() {
        free((*connp).out_buf as *mut libc::c_void);
    }
    htp_connp_destroy_decompressors(connp);
    if !(*connp).put_file.is_null() {
        bstr_free((*(*connp).put_file).filename);
        free((*connp).put_file as *mut libc::c_void);
    }
    if !(*connp).in_header.is_null() {
        bstr_free((*connp).in_header);
        (*connp).in_header = 0 as *mut bstr
    }
    if !(*connp).out_header.is_null() {
        bstr_free((*connp).out_header);
        (*connp).out_header = 0 as *mut bstr
    }
    free(connp as *mut libc::c_void);
}

/**
 * Destroys the connection parser, its data structures, as well
 * as the connection and its transactions.
 *
 * @param[in] connp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_destroy_all(mut connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    // Destroy connection
    htp_conn_destroy((*connp).conn);
    (*connp).conn = 0 as *mut crate::src::htp_connection::htp_conn_t;
    // Destroy everything else
    htp_connp_destroy(connp);
}

/**
 * Returns the connection associated with the connection parser.
 *
 * @param[in] connp
 * @return htp_conn_t instance, or NULL if one is not available.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_connection(
    mut connp: *const htp_connp_t,
) -> *mut crate::src::htp_connection::htp_conn_t {
    if connp.is_null() {
        return 0 as *mut crate::src::htp_connection::htp_conn_t;
    }
    return (*connp).conn;
}

/**
 * Retrieves the pointer to the active inbound transaction. In connection
 * parsing mode there can be many open transactions, and up to 2 active
 * transactions at any one time. This is due to HTTP pipelining. Can be NULL.
 *
 * @param[in] connp
 * @return Active inbound transaction, or NULL if there isn't one.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_in_tx(
    mut connp: *const htp_connp_t,
) -> *mut crate::src::htp_transaction::htp_tx_t {
    if connp.is_null() {
        return 0 as *mut crate::src::htp_transaction::htp_tx_t;
    }
    return (*connp).in_tx;
}

/**
 * Returns the last error that occurred with this connection parser. Do note, however,
 * that the value in this field will only be valid immediately after an error condition,
 * but it is not guaranteed to remain valid if the parser is invoked again.
 *
 * @param[in] connp
 * @return A pointer to an htp_log_t instance if there is an error, or NULL
 *         if there isn't.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_last_error(
    mut connp: *const htp_connp_t,
) -> *mut crate::src::htp_util::htp_log_t {
    if connp.is_null() {
        return 0 as *mut crate::src::htp_util::htp_log_t;
    }
    return (*connp).last_error;
}

/**
 * Retrieves the pointer to the active outbound transaction. In connection
 * parsing mode there can be many open transactions, and up to 2 active
 * transactions at any one time. This is due to HTTP pipelining. Can be NULL.
 *
 * @param[in] connp
 * @return Active outbound transaction, or NULL if there isn't one.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_out_tx(
    mut connp: *const htp_connp_t,
) -> *mut crate::src::htp_transaction::htp_tx_t {
    if connp.is_null() {
        return 0 as *mut crate::src::htp_transaction::htp_tx_t;
    }
    return (*connp).out_tx;
}

/**
 * Retrieve the user data associated with this connection parser.
 *
 * @param[in] connp
 * @return User data, or NULL if there isn't any.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_user_data(
    mut connp: *const htp_connp_t,
) -> *mut libc::c_void {
    if connp.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*connp).user_data as *mut libc::c_void;
}

/* *
 * This function is most likely not used and/or not needed.
 *
 * @param[in] connp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_in_reset(mut connp: *mut htp_connp_t) {
    if connp.is_null() {
        return;
    }
    (*connp).in_content_length = -(1 as libc::c_int) as int64_t;
    (*connp).in_body_data_left = -(1 as libc::c_int) as int64_t;
    (*connp).in_chunk_request_index = (*connp).in_chunk_count;
}

/**
 * Opens connection.
 *
 * @param[in] connp
 * @param[in] client_addr Client address
 * @param[in] client_port Client port
 * @param[in] server_addr Server address
 * @param[in] server_port Server port
 * @param[in] timestamp Optional.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_open(
    mut connp: *mut htp_connp_t,
    mut client_addr: *const libc::c_char,
    mut client_port: libc::c_int,
    mut server_addr: *const libc::c_char,
    mut server_port: libc::c_int,
    mut timestamp: *mut htp_time_t,
) {
    if connp.is_null() {
        return;
    }
    // Check connection parser state first.
    if (*connp).in_status as libc::c_uint != HTP_STREAM_NEW as libc::c_int as libc::c_uint
        || (*connp).out_status as libc::c_uint != HTP_STREAM_NEW as libc::c_int as libc::c_uint
    {
        htp_log(
            connp,
            b"htp_connection_parser.c\x00" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            HTP_LOG_ERROR,
            0 as libc::c_int,
            b"Connection is already open\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if htp_conn_open(
        (*connp).conn,
        client_addr,
        client_port,
        server_addr,
        server_port,
        timestamp,
    ) != 1 as libc::c_int
    {
        return;
    }
    (*connp).in_status = HTP_STREAM_OPEN;
    (*connp).out_status = HTP_STREAM_OPEN;
}

/**
 * Associate user data with the supplied parser.
 *
 * @param[in] connp
 * @param[in] user_data
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_set_user_data(
    mut connp: *mut htp_connp_t,
    mut user_data: *const libc::c_void,
) {
    if connp.is_null() {
        return;
    }
    (*connp).user_data = user_data;
}

/* *
 * Create a new transaction using the connection parser provided.
 *
 * @param[in] connp
 * @return Transaction instance on success, NULL on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_tx_create(
    mut connp: *mut htp_connp_t,
) -> *mut crate::src::htp_transaction::htp_tx_t {
    if connp.is_null() {
        return 0 as *mut crate::src::htp_transaction::htp_tx_t;
    }
    // Detect pipelining.
    if htp_list_array_size((*(*connp).conn).transactions) > (*connp).out_next_tx_index {
        (*(*connp).conn).flags =
            ((*(*connp).conn).flags as libc::c_ulonglong | 0x1 as libc::c_ulonglong) as uint8_t
    }
    let mut tx: *mut crate::src::htp_transaction::htp_tx_t = htp_tx_create(connp);
    if tx.is_null() {
        return 0 as *mut crate::src::htp_transaction::htp_tx_t;
    }
    (*connp).in_tx = tx;
    htp_connp_in_reset(connp);
    return tx;
}

/* *
 * Removes references to the supplied transaction.
 *
 * @param[in] connp
 * @param[in] tx
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_tx_remove(
    mut connp: *mut htp_connp_t,
    mut tx: *mut crate::src::htp_transaction::htp_tx_t,
) {
    if connp.is_null() {
        return;
    }
    if (*connp).in_tx == tx {
        (*connp).in_tx = 0 as *mut crate::src::htp_transaction::htp_tx_t
    }
    if (*connp).out_tx == tx {
        (*connp).out_tx = 0 as *mut crate::src::htp_transaction::htp_tx_t
    };
}
