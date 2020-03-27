use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn htp_list_array_create(size: size_t) -> *mut crate::src::htp_list::htp_list_array_t;
    #[no_mangle]
    fn htp_list_array_destroy(l: *mut crate::src::htp_list::htp_list_array_t);
    #[no_mangle]
    fn htp_list_array_get(
        l: *const crate::src::htp_list::htp_list_array_t,
        idx: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_list_array_replace(
        l: *mut crate::src::htp_list::htp_list_array_t,
        idx: size_t,
        e: *mut libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_list_array_size(l: *const crate::src::htp_list::htp_list_array_t) -> size_t;
    #[no_mangle]
    fn htp_tx_destroy_incomplete(tx: *mut crate::src::htp_transaction::htp_tx_t);
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_conn_t {
    /** Client IP address. */
    pub client_addr: *mut libc::c_char,
    /** Client port. */
    pub client_port: libc::c_int,
    /** Server IP address. */
    pub server_addr: *mut libc::c_char,
    /** Server port. */
    pub server_port: libc::c_int,
    /**
     * Transactions carried out on this connection. The list may contain
     * NULL elements when some of the transactions are deleted (and then
     * removed from a connection by calling htp_conn_remove_tx().
     */
    pub transactions: *mut crate::src::htp_list::htp_list_array_t,
    /** Log messages associated with this connection. */
    pub messages: *mut crate::src::htp_list::htp_list_array_t,
    /** Parsing flags: HTP_CONN_PIPELINED. */
    pub flags: uint8_t,
    /** When was this connection opened? Can be NULL. */
    pub open_timestamp: htp_time_t,
    /** When was this connection closed? Can be NULL. */
    pub close_timestamp: htp_time_t,
    /** Inbound data counter. */
    pub in_data_counter: int64_t,
    /** Outbound data counter. */
    pub out_data_counter: int64_t,
}
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
 * Creates a new connection structure.
 *
 * @return A new connection structure on success, NULL on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_conn_create() -> *mut htp_conn_t {
    let mut conn: *mut htp_conn_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_conn_t>() as libc::c_ulong,
    ) as *mut htp_conn_t;
    if conn.is_null() {
        return 0 as *mut htp_conn_t;
    }
    (*conn).transactions = htp_list_array_create(16 as libc::c_int as size_t);
    if (*conn).transactions.is_null() {
        free(conn as *mut libc::c_void);
        return 0 as *mut htp_conn_t;
    }
    (*conn).messages = htp_list_array_create(8 as libc::c_int as size_t);
    if (*conn).messages.is_null() {
        htp_list_array_destroy((*conn).transactions);
        (*conn).transactions = 0 as *mut crate::src::htp_list::htp_list_array_t;
        free(conn as *mut libc::c_void);
        return 0 as *mut htp_conn_t;
    }
    return conn;
}

/**
 * Closes the connection.
 *
 * @param[in] conn
 * @param[in] timestamp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_conn_close(
    mut conn: *mut htp_conn_t,
    mut timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    // Update timestamp.
    if !timestamp.is_null() {
        memcpy(
            &mut (*conn).close_timestamp as *mut htp_time_t as *mut libc::c_void,
            timestamp as *const libc::c_void,
            ::std::mem::size_of::<htp_time_t>() as libc::c_ulong,
        );
    };
}

/**
 * Destroys a connection, as well as all the transactions it contains. It is
 * not possible to destroy a connection structure yet leave any of its
 * transactions intact. This is because transactions need its connection and
 * connection structures hold little data anyway. The opposite is true, though
 * it is possible to delete a transaction but leave its connection alive.
 *
 * @param[in] conn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_conn_destroy(mut conn: *mut htp_conn_t) {
    if conn.is_null() {
        return;
    }
    if !(*conn).transactions.is_null() {
        // Destroy individual transactions. Do note that iterating
        // using the iterator does not work here because some of the
        // list element may be NULL (and with the iterator it is impossible
        // to distinguish a NULL element from the end of the list).
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_list_array_size((*conn).transactions);
        while i < n {
            let mut tx: *mut crate::src::htp_transaction::htp_tx_t =
                htp_list_array_get((*conn).transactions, i)
                    as *mut crate::src::htp_transaction::htp_tx_t;
            if !tx.is_null() {
                htp_tx_destroy_incomplete(tx);
            }
            i = i.wrapping_add(1)
        }
        htp_list_array_destroy((*conn).transactions);
        (*conn).transactions = 0 as *mut crate::src::htp_list::htp_list_array_t
    }
    if !(*conn).messages.is_null() {
        // Destroy individual messages.
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        let mut n_0: size_t = htp_list_array_size((*conn).messages);
        while i_0 < n_0 {
            let mut l: *mut crate::src::htp_util::htp_log_t =
                htp_list_array_get((*conn).messages, i_0) as *mut crate::src::htp_util::htp_log_t;
            free((*l).msg as *mut libc::c_void);
            free(l as *mut libc::c_void);
            i_0 = i_0.wrapping_add(1)
        }
        htp_list_array_destroy((*conn).messages);
        (*conn).messages = 0 as *mut crate::src::htp_list::htp_list_array_t
    }
    if !(*conn).server_addr.is_null() {
        free((*conn).server_addr as *mut libc::c_void);
    }
    if !(*conn).client_addr.is_null() {
        free((*conn).client_addr as *mut libc::c_void);
    }
    free(conn as *mut libc::c_void);
}

/**
 * Opens a connection. This function will essentially only store the provided data
 * for future reference. The timestamp parameter is optional.
 *
 * @param[in] conn
 * @param[in] remote_addr
 * @param[in] remote_port
 * @param[in] local_addr
 * @param[in] local_port
 * @param[in] timestamp
 * @return
 */
#[no_mangle]
pub unsafe extern "C" fn htp_conn_open(
    mut conn: *mut htp_conn_t,
    mut client_addr: *const libc::c_char,
    mut client_port: libc::c_int,
    mut server_addr: *const libc::c_char,
    mut server_port: libc::c_int,
    mut timestamp: *const htp_time_t,
) -> htp_status_t {
    if conn.is_null() {
        return -(1 as libc::c_int);
    }
    if !client_addr.is_null() {
        (*conn).client_addr = strdup(client_addr);
        if (*conn).client_addr.is_null() {
            return -(1 as libc::c_int);
        }
    }
    (*conn).client_port = client_port;
    if !server_addr.is_null() {
        (*conn).server_addr = strdup(server_addr);
        if (*conn).server_addr.is_null() {
            if !(*conn).client_addr.is_null() {
                free((*conn).client_addr as *mut libc::c_void);
            }
            return -(1 as libc::c_int);
        }
    }
    (*conn).server_port = server_port;
    // Remember when the connection was opened.
    if !timestamp.is_null() {
        memcpy(
            &mut (*conn).open_timestamp as *mut htp_time_t as *mut libc::c_void,
            timestamp as *const libc::c_void,
            ::std::mem::size_of::<htp_time_t>() as libc::c_ulong,
        );
    }
    return 1 as libc::c_int;
}

/**
 * Removes the given transaction structure, which makes it possible to
 * safely destroy it. It is safe to destroy transactions in this way
 * because the index of the transactions (in a connection) is preserved.
 *
 * @param[in] conn
 * @param[in] tx
 * @return HTP_OK if transaction was removed (replaced with NULL) or HTP_ERROR if it wasn't found.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_conn_remove_tx(
    mut conn: *mut htp_conn_t,
    mut tx: *const crate::src::htp_transaction::htp_tx_t,
) -> htp_status_t {
    if tx.is_null() || conn.is_null() {
        return -(1 as libc::c_int);
    }
    if (*conn).transactions.is_null() {
        return -(1 as libc::c_int);
    }
    return htp_list_array_replace((*conn).transactions, (*tx).index, 0 as *mut libc::c_void);
}

/**
 * Keeps track of inbound packets and data.
 *
 * @param[in] conn
 * @param[in] len
 * @param[in] timestamp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_conn_track_inbound_data(
    mut conn: *mut htp_conn_t,
    mut len: size_t,
    mut _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).in_data_counter =
        ((*conn).in_data_counter as libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
}

/**
 * Keeps track of outbound packets and data.
 *
 * @param[in] conn
 * @param[in] len
 * @param[in] timestamp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_conn_track_outbound_data(
    mut conn: *mut htp_conn_t,
    mut len: size_t,
    mut _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).out_data_counter =
        ((*conn).out_data_counter as libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
}
