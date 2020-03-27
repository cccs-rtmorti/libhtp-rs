use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn bstr_add_mem(b: *mut bstr, data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn htp_log(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        file: *const libc::c_char,
        line: libc::c_int,
        level: htp_log_level_t,
        code: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn htp_hook_run_all(
        hook: *mut crate::src::htp_hooks::htp_hook_t,
        user_data: *mut libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_is_folding_char(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htp_connp_is_line_folded(data: *mut libc::c_uchar, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn htp_chomp(data: *mut libc::c_uchar, len: *mut size_t) -> libc::c_int;
    #[no_mangle]
    fn htp_tx_state_request_headers(tx: *mut crate::src::htp_transaction::htp_tx_t)
        -> htp_status_t;
    #[no_mangle]
    fn htp_connp_is_line_terminator(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        data: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_conn_track_inbound_data(
        conn: *mut crate::src::htp_connection::htp_conn_t,
        len: size_t,
        timestamp: *const htp_time_t,
    );
    #[no_mangle]
    fn htp_tx_state_request_start(tx: *mut crate::src::htp_transaction::htp_tx_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_tx_create(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> *mut crate::src::htp_transaction::htp_tx_t;
    #[no_mangle]
    fn htp_tx_state_request_complete(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_tx_state_request_line(tx: *mut crate::src::htp_transaction::htp_tx_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_is_line_ignorable(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        data: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_tx_req_process_body_data_ex(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_convert_method_to_number(_: *mut bstr) -> libc::c_int;
    #[no_mangle]
    fn htp_is_space(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htp_is_lws(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htp_parse_chunked_length(data: *mut libc::c_uchar, len: size_t) -> int64_t;
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
/* *
 * Enumerates the possible server personalities.
 */
pub type htp_server_personality_t = libc::c_uint;
/* Mimics the behavior of Apache 2.x. */
pub const HTP_SERVER_APACHE_2: htp_server_personality_t = 9;
/* Mimics the behavior of IIS 7.5, as shipped with Windows 7. */
pub const HTP_SERVER_IIS_7_5: htp_server_personality_t = 8;
/* * Mimics the behavior of IIS 7.0, as shipped with Windows 2008. */
pub const HTP_SERVER_IIS_7_0: htp_server_personality_t = 7;
/* * Mimics the behavior of IIS 6.0, as shipped with Windows 2003. */
pub const HTP_SERVER_IIS_6_0: htp_server_personality_t = 6;
/* * Mimics the behavior of IIS 5.1, as shipped with Windows XP Professional. */
pub const HTP_SERVER_IIS_5_1: htp_server_personality_t = 5;
/* * Mimics the behavior of IIS 5.0, as shipped with Windows 2000. */
pub const HTP_SERVER_IIS_5_0: htp_server_personality_t = 4;
/* * Mimics the behavior of IIS 4.0, as shipped with Windows NT 4.0. */
pub const HTP_SERVER_IIS_4_0: htp_server_personality_t = 3;
/* * The IDS personality tries to perform as much decoding as possible. */
pub const HTP_SERVER_IDS: htp_server_personality_t = 2;
/* * A generic personality that aims to work reasonably well for all server types. */
pub const HTP_SERVER_GENERIC: htp_server_personality_t = 1;
/* *
 * Minimal personality that performs at little work as possible. All optional
 * features are disabled. This personality is a good starting point for customization.
 */
pub const HTP_SERVER_MINIMAL: htp_server_personality_t = 0;

/* *
 * Sends outstanding connection data to the currently active data receiver hook.
 *
 * @param[in] connp
 * @param[in] is_last
 * @return HTP_OK, or a value returned from a callback.
 */
unsafe extern "C" fn htp_connp_req_receiver_send_data(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    mut is_last: libc::c_int,
) -> htp_status_t {
    if (*connp).in_data_receiver_hook.is_null() {
        return 1 as libc::c_int;
    }
    let mut d: crate::src::htp_transaction::htp_tx_data_t =
        crate::src::htp_transaction::htp_tx_data_t {
            tx: 0 as *mut crate::src::htp_transaction::htp_tx_t,
            data: 0 as *const libc::c_uchar,
            len: 0,
            is_last: 0,
        };
    d.tx = (*connp).in_tx;
    d.data = (*connp)
        .in_current_data
        .offset((*connp).in_current_receiver_offset as isize);
    d.len = ((*connp).in_current_read_offset - (*connp).in_current_receiver_offset) as size_t;
    d.is_last = is_last;
    let mut rc: htp_status_t = htp_hook_run_all(
        (*connp).in_data_receiver_hook,
        &mut d as *mut crate::src::htp_transaction::htp_tx_data_t as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    (*connp).in_current_receiver_offset = (*connp).in_current_read_offset;
    return 1 as libc::c_int;
}

/* *
 * Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
 *
 * @param[in] connp
 * @param[in] data_receiver_hook
 * @return HTP_OK, or a value returned from a callback.
 */
unsafe extern "C" fn htp_connp_req_receiver_set(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    mut data_receiver_hook: *mut crate::src::htp_hooks::htp_hook_t,
) -> htp_status_t {
    htp_connp_req_receiver_finalize_clear(connp);
    (*connp).in_data_receiver_hook = data_receiver_hook;
    (*connp).in_current_receiver_offset = (*connp).in_current_read_offset;
    return 1 as libc::c_int;
}

/* *
 * Finalizes an existing data receiver hook by sending any outstanding data to it. The
 * hook is then removed so that it receives no more data.
 *
 * @param[in] connp
 * @return HTP_OK, or a value returned from a callback.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_req_receiver_finalize_clear(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    if (*connp).in_data_receiver_hook.is_null() {
        return 1 as libc::c_int;
    }
    let mut rc: htp_status_t = htp_connp_req_receiver_send_data(connp, 1 as libc::c_int);
    (*connp).in_data_receiver_hook = 0 as *mut crate::src::htp_hooks::htp_hook_t;
    return rc;
}

/* *
 * Handles request parser state changes. At the moment, this function is used only
 * to configure data receivers, which are sent raw connection data.
 *
 * @param[in] connp
 * @return HTP_OK, or a value returned from a callback.
 */
unsafe extern "C" fn htp_req_handle_state_change(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    if (*connp).in_state_previous == (*connp).in_state {
        return 1 as libc::c_int;
    }
    if (*connp).in_state
        == Some(
            htp_connp_REQ_HEADERS
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    {
        let mut rc: htp_status_t = 1 as libc::c_int;
        match (*(*connp).in_tx).request_progress as libc::c_uint {
            2 => {
                rc = htp_connp_req_receiver_set(
                    connp,
                    (*(*(*connp).in_tx).cfg).hook_request_header_data,
                )
            }
            4 => {
                rc = htp_connp_req_receiver_set(
                    connp,
                    (*(*(*connp).in_tx).cfg).hook_request_trailer_data,
                )
            }
            _ => {}
        }
        if rc != 1 as libc::c_int {
            return rc;
        }
    }
    // Initially, I had the finalization of raw data sending here, but that
    // caused the last REQUEST_HEADER_DATA hook to be invoked after the
    // REQUEST_HEADERS hook -- which I thought made no sense. For that reason,
    // the finalization is now initiated from the request header processing code,
    // which is less elegant but provides a better user experience. Having some
    // (or all) hooks to be invoked on state change might work better.
    (*connp).in_state_previous = (*connp).in_state;
    return 1 as libc::c_int;
}

/* *
 * If there is any data left in the inbound data chunk, this function will preserve
 * it for later consumption. The maximum amount accepted for buffering is controlled
 * by htp_config_t::field_limit_hard.
 *
 * @param[in] connp
 * @return HTP_OK, or HTP_ERROR on fatal failure.
 */
unsafe extern "C" fn htp_connp_req_buffer(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    if (*connp).in_current_data.is_null() {
        return 1 as libc::c_int;
    }
    let mut data: *mut libc::c_uchar = (*connp)
        .in_current_data
        .offset((*connp).in_current_consume_offset as isize);
    let mut len: size_t =
        ((*connp).in_current_read_offset - (*connp).in_current_consume_offset) as size_t;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    // Check the hard (buffering) limit.
    let mut newlen: size_t = (*connp).in_buf_size.wrapping_add(len);
    // When calculating the size of the buffer, take into account the
    // space we're using for the request header buffer.
    if !(*connp).in_header.is_null() {
        newlen =
            (newlen as libc::c_ulong).wrapping_add((*(*connp).in_header).len) as size_t as size_t
    }
    if newlen > (*(*(*connp).in_tx).cfg).field_limit_hard {
        htp_log(
            connp,
            b"htp_request.c\x00" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            HTP_LOG_ERROR,
            0 as libc::c_int,
            b"Request buffer over the limit: size %zd limit %zd.\x00" as *const u8
                as *const libc::c_char,
            newlen,
            (*(*(*connp).in_tx).cfg).field_limit_hard,
        );
        return -(1 as libc::c_int);
    }
    // Copy the data remaining in the buffer.
    if (*connp).in_buf.is_null() {
        (*connp).in_buf = malloc(len) as *mut libc::c_uchar;
        if (*connp).in_buf.is_null() {
            return -(1 as libc::c_int);
        }
        memcpy(
            (*connp).in_buf as *mut libc::c_void,
            data as *const libc::c_void,
            len,
        );
        (*connp).in_buf_size = len
    } else {
        let mut newsize: size_t = (*connp).in_buf_size.wrapping_add(len);
        let mut newbuf: *mut libc::c_uchar =
            realloc((*connp).in_buf as *mut libc::c_void, newsize) as *mut libc::c_uchar;
        if newbuf.is_null() {
            return -(1 as libc::c_int);
        }
        (*connp).in_buf = newbuf;
        memcpy(
            (*connp).in_buf.offset((*connp).in_buf_size as isize) as *mut libc::c_void,
            data as *const libc::c_void,
            len,
        );
        (*connp).in_buf_size = newsize
    }
    // Reset the consumer position.
    (*connp).in_current_consume_offset = (*connp).in_current_read_offset;
    return 1 as libc::c_int;
}

/* *
 * Returns to the caller the memory region that should be processed next. This function
 * hides away the buffering process from the rest of the code, allowing it to work with
 * non-buffered data that's in the inbound chunk, or buffered data that's in our structures.
 *
 * @param[in] connp
 * @param[out] data
 * @param[out] len
 * @return HTP_OK
 */
unsafe extern "C" fn htp_connp_req_consolidate_data(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    mut data: *mut *mut libc::c_uchar,
    mut len: *mut size_t,
) -> htp_status_t {
    if (*connp).in_buf.is_null() {
        // We do not have any data buffered; point to the current data chunk.
        *data = (*connp)
            .in_current_data
            .offset((*connp).in_current_consume_offset as isize);
        *len = ((*connp).in_current_read_offset - (*connp).in_current_consume_offset) as size_t
    } else {
        // We already have some data in the buffer. Add the data from the current
        // chunk to it, and point to the consolidated buffer.
        if htp_connp_req_buffer(connp) != 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        *data = (*connp).in_buf;
        *len = (*connp).in_buf_size
    }
    return 1 as libc::c_int;
}

/* *
 * Clears buffered inbound data and resets the consumer position to the reader position.
 *
 * @param[in] connp
 */
unsafe extern "C" fn htp_connp_req_clear_buffer(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) {
    (*connp).in_current_consume_offset = (*connp).in_current_read_offset;
    if !(*connp).in_buf.is_null() {
        free((*connp).in_buf as *mut libc::c_void);
        (*connp).in_buf = 0 as *mut libc::c_uchar;
        (*connp).in_buf_size = 0 as libc::c_int as size_t
    };
}

/* *
 * Performs a check for a CONNECT transaction to decide whether inbound
 * parsing needs to be suspended.
 *
 * @param[in] connp
 * @return HTP_OK if the request does not use CONNECT, HTP_DATA_OTHER if
 *          inbound parsing needs to be suspended until we hear from the
 *          other side
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_CONNECT_CHECK(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // If the request uses the CONNECT method, then there will
    // not be a request body, but first we need to wait to see the
    // response in order to determine if the tunneling request
    // was a success.
    if (*(*connp).in_tx).request_method_number as libc::c_uint
        == HTP_M_CONNECT as libc::c_int as libc::c_uint
    {
        (*connp).in_state = Some(
            htp_connp_REQ_CONNECT_WAIT_RESPONSE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        );
        (*connp).in_status = HTP_STREAM_DATA_OTHER;
        return 3 as libc::c_int;
    }
    // Continue to the next step to determine
    // the presence of request body
    (*connp).in_state = Some(
        htp_connp_REQ_BODY_DETERMINE
            as unsafe extern "C" fn(
                _: *mut crate::src::htp_connection_parser::htp_connp_t,
            ) -> htp_status_t,
    );
    return 1 as libc::c_int;
}

/* *
 * Determines whether inbound parsing needs to continue or stop. In
 * case the data appears to be plain text HTTP, we try to continue.
 *
 * @param[in] connp
 * @return HTP_OK if the parser can resume parsing, HTP_DATA_BUFFER if
 *         we need more data.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_CONNECT_PROBE_DATA(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    loop {
        //;i < max_read; i++) {
        if (*connp).in_current_read_offset >= (*connp).in_current_len {
            (*connp).in_next_byte = -(1 as libc::c_int)
        } else {
            (*connp).in_next_byte = *(*connp)
                .in_current_data
                .offset((*connp).in_current_read_offset as isize)
                as libc::c_int
        }
        // Have we reached the end of the line? For some reason
        // we can't test after IN_COPY_BYTE_OR_RETURN */
        if (*connp).in_next_byte == '\n' as i32 || (*connp).in_next_byte == 0 as libc::c_int {
            break;
        }
        if (*connp).in_current_read_offset < (*connp).in_current_len {
            (*connp).in_next_byte = *(*connp)
                .in_current_data
                .offset((*connp).in_current_read_offset as isize)
                as libc::c_int;
            (*connp).in_current_read_offset += 1;
            (*connp).in_stream_offset += 1
        } else {
            return 5 as libc::c_int;
        }
    }
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    if htp_connp_req_consolidate_data(connp, &mut data, &mut len) != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut mstart: size_t = 0 as libc::c_int as size_t;
    // skip past leading whitespace. IIS allows this
    while pos < len && htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    if pos != 0 {
        mstart = pos
    }
    // The request method starts at the beginning of the
    // line and ends with the first whitespace character.
    while pos < len && htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0 {
        pos = pos.wrapping_add(1)
    }
    let mut methodi: libc::c_int = HTP_M_UNKNOWN as libc::c_int;
    let mut method: *mut bstr = bstr_dup_mem(
        data.offset(mstart as isize) as *const libc::c_void,
        pos.wrapping_sub(mstart),
    );
    if !method.is_null() {
        methodi = htp_convert_method_to_number(method);
        bstr_free(method);
    }
    if methodi != HTP_M_UNKNOWN as libc::c_int {
        return htp_tx_state_request_complete((*connp).in_tx);
    } else {
        (*connp).in_status = HTP_STREAM_TUNNEL;
        (*connp).out_status = HTP_STREAM_TUNNEL
    }
    // not calling htp_connp_req_clear_buffer, we're not consuming the data
    return 1 as libc::c_int;
}

/* *
 * Determines whether inbound parsing, which was suspended after
 * encountering a CONNECT transaction, can proceed (after receiving
 * the response).
 *
 * @param[in] connp
 * @return HTP_OK if the parser can resume parsing, HTP_DATA_OTHER if
 *         it needs to continue waiting.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_CONNECT_WAIT_RESPONSE(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // Check that we saw the response line of the current inbound transaction.
    if (*(*connp).in_tx).response_progress as libc::c_uint
        <= HTP_RESPONSE_LINE as libc::c_int as libc::c_uint
    {
        return 3 as libc::c_int;
    }
    // A 2xx response means a tunnel was established. Anything
    // else means we continue to follow the HTTP stream.
    if (*(*connp).in_tx).response_status_number >= 200 as libc::c_int
        && (*(*connp).in_tx).response_status_number <= 299 as libc::c_int
    {
        // TODO Check that the server did not accept a connection to itself.
        // The requested tunnel was established: we are going
        // to probe the remaining data on this stream to see
        // if we need to ignore it or parse it
        (*connp).in_state = Some(
            htp_connp_REQ_CONNECT_PROBE_DATA
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    } else {
        // No tunnel; continue to the next transaction
        (*connp).in_state = Some(
            htp_connp_REQ_FINALIZE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    }
    return 1 as libc::c_int;
}

/* *
 * Consumes bytes until the end of the current line.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_BODY_CHUNKED_DATA_END(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    loop
    // TODO We shouldn't really see anything apart from CR and LF,
    //      so we should warn about anything else.
    {
        if (*connp).in_current_read_offset < (*connp).in_current_len {
            (*connp).in_next_byte = *(*connp)
                .in_current_data
                .offset((*connp).in_current_read_offset as isize)
                as libc::c_int;
            (*connp).in_current_read_offset += 1;
            (*connp).in_current_consume_offset += 1;
            (*connp).in_stream_offset += 1
        } else {
            return 2 as libc::c_int;
        }
        (*(*connp).in_tx).request_message_len += 1;
        if (*connp).in_next_byte == '\n' as i32 {
            (*connp).in_state = Some(
                htp_connp_REQ_BODY_CHUNKED_LENGTH
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            return 1 as libc::c_int;
        }
    }
}

/* *
 * Processes a chunk of data.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_BODY_CHUNKED_DATA(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // Determine how many bytes we can consume.
    let mut bytes_to_consume: size_t = 0;
    if (*connp).in_current_len - (*connp).in_current_read_offset >= (*connp).in_chunked_length {
        // Entire chunk available in the buffer; read all of it.
        bytes_to_consume = (*connp).in_chunked_length as size_t
    } else {
        // Partial chunk available in the buffer; read as much as we can.
        bytes_to_consume = ((*connp).in_current_len - (*connp).in_current_read_offset) as size_t
    }
    // If the input buffer is empty, ask for more data.
    if bytes_to_consume == 0 as libc::c_int as libc::c_ulong {
        return 2 as libc::c_int;
    }
    // Consume the data.
    let mut rc: htp_status_t = htp_tx_req_process_body_data_ex(
        (*connp).in_tx,
        (*connp)
            .in_current_data
            .offset((*connp).in_current_read_offset as isize) as *const libc::c_void,
        bytes_to_consume,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Adjust counters.
    (*connp).in_current_read_offset = ((*connp).in_current_read_offset as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t as int64_t;
    (*connp).in_current_consume_offset = ((*connp).in_current_consume_offset as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t
        as int64_t;
    (*connp).in_stream_offset = ((*connp).in_stream_offset as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t as int64_t;
    (*(*connp).in_tx).request_message_len = ((*(*connp).in_tx).request_message_len as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t
        as int64_t;
    (*connp).in_chunked_length = ((*connp).in_chunked_length as libc::c_ulong)
        .wrapping_sub(bytes_to_consume) as int64_t as int64_t;
    if (*connp).in_chunked_length == 0 as libc::c_int as libc::c_long {
        // End of the chunk.
        (*connp).in_state = Some(
            htp_connp_REQ_BODY_CHUNKED_DATA_END
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        );
        return 1 as libc::c_int;
    }
    // Ask for more data.
    return 2 as libc::c_int;
}

/* *
 * Extracts chunk length.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_BODY_CHUNKED_LENGTH(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    loop {
        if (*connp).in_current_read_offset < (*connp).in_current_len {
            (*connp).in_next_byte = *(*connp)
                .in_current_data
                .offset((*connp).in_current_read_offset as isize)
                as libc::c_int;
            (*connp).in_current_read_offset += 1;
            (*connp).in_stream_offset += 1
        } else {
            return 5 as libc::c_int;
        }
        // Have we reached the end of the line?
        if (*connp).in_next_byte == '\n' as i32 {
            let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut len: size_t = 0;
            if htp_connp_req_consolidate_data(connp, &mut data, &mut len) != 1 as libc::c_int {
                return -(1 as libc::c_int);
            }
            (*(*connp).in_tx).request_message_len =
                ((*(*connp).in_tx).request_message_len as libc::c_ulong).wrapping_add(len)
                    as int64_t as int64_t;
            htp_chomp(data, &mut len);
            (*connp).in_chunked_length = htp_parse_chunked_length(data, len);
            htp_connp_req_clear_buffer(connp);
            // Handle chunk length.
            if (*connp).in_chunked_length > 0 as libc::c_int as libc::c_long {
                // More data available.
                (*connp).in_state = Some(
                    htp_connp_REQ_BODY_CHUNKED_DATA
                        as unsafe extern "C" fn(
                            _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        ) -> htp_status_t,
                )
            } else if (*connp).in_chunked_length == 0 as libc::c_int as libc::c_long {
                // End of data.
                (*connp).in_state = Some(
                    htp_connp_REQ_HEADERS
                        as unsafe extern "C" fn(
                            _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        ) -> htp_status_t,
                );
                (*(*connp).in_tx).request_progress = HTP_REQUEST_TRAILER
            } else {
                // Invalid chunk length.
                htp_log(
                    connp,
                    b"htp_request.c\x00" as *const u8 as *const libc::c_char,
                    516 as libc::c_int,
                    HTP_LOG_ERROR,
                    0 as libc::c_int,
                    b"Request chunk encoding: Invalid chunk length\x00" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            return 1 as libc::c_int;
        }
    }
}

/* *
 * Processes identity request body.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_BODY_IDENTITY(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // Determine how many bytes we can consume.
    let mut bytes_to_consume: size_t = 0;
    if (*connp).in_current_len - (*connp).in_current_read_offset >= (*connp).in_body_data_left {
        bytes_to_consume = (*connp).in_body_data_left as size_t
    } else {
        bytes_to_consume = ((*connp).in_current_len - (*connp).in_current_read_offset) as size_t
    }
    // If the input buffer is empty, ask for more data.
    if bytes_to_consume == 0 as libc::c_int as libc::c_ulong {
        return 2 as libc::c_int;
    }
    // Consume data.
    let mut rc: libc::c_int = htp_tx_req_process_body_data_ex(
        (*connp).in_tx,
        (*connp)
            .in_current_data
            .offset((*connp).in_current_read_offset as isize) as *const libc::c_void,
        bytes_to_consume,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Adjust counters.
    (*connp).in_current_read_offset = ((*connp).in_current_read_offset as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t as int64_t;
    (*connp).in_current_consume_offset = ((*connp).in_current_consume_offset as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t
        as int64_t;
    (*connp).in_stream_offset = ((*connp).in_stream_offset as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t as int64_t;
    (*(*connp).in_tx).request_message_len = ((*(*connp).in_tx).request_message_len as libc::c_ulong)
        .wrapping_add(bytes_to_consume) as int64_t
        as int64_t;
    (*connp).in_body_data_left = ((*connp).in_body_data_left as libc::c_ulong)
        .wrapping_sub(bytes_to_consume) as int64_t as int64_t;
    if (*connp).in_body_data_left == 0 as libc::c_int as libc::c_long {
        // End of request body.
        (*connp).in_state = Some(
            htp_connp_REQ_FINALIZE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        );
        return 1 as libc::c_int;
    }
    // Ask for more data.
    return 2 as libc::c_int;
}

/* *
 * Determines presence (and encoding) of a request body.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_BODY_DETERMINE(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // Determine the next state based on the presence of the request
    // body, and the coding used.
    match (*(*connp).in_tx).request_transfer_coding as libc::c_uint {
        3 => {
            (*connp).in_state = Some(
                htp_connp_REQ_BODY_CHUNKED_LENGTH
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*(*connp).in_tx).request_progress = HTP_REQUEST_BODY
        }
        2 => {
            (*connp).in_content_length = (*(*connp).in_tx).request_content_length;
            (*connp).in_body_data_left = (*connp).in_content_length;
            if (*connp).in_content_length != 0 as libc::c_int as libc::c_long {
                (*connp).in_state = Some(
                    htp_connp_REQ_BODY_IDENTITY
                        as unsafe extern "C" fn(
                            _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        ) -> htp_status_t,
                );
                (*(*connp).in_tx).request_progress = HTP_REQUEST_BODY
            } else {
                (*(*(*connp).in_tx).connp).in_state = Some(
                    htp_connp_REQ_FINALIZE
                        as unsafe extern "C" fn(
                            _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        ) -> htp_status_t,
                )
            }
        }
        1 => {
            // This request does not have a body, which
            // means that we're done with it
            (*connp).in_state = Some(
                htp_connp_REQ_FINALIZE
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            )
        }
        _ => {
            // Should not be here
            return -(1 as libc::c_int);
        }
    }
    return 1 as libc::c_int;
}

/* *
 * Parses request headers.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_HEADERS(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    loop {
        if (*connp).in_status as libc::c_uint == HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
            // Parse previous header, if any.
            if !(*connp).in_header.is_null() {
                if (*(*connp).cfg)
                    .process_request_header
                    .expect("non-null function pointer")(
                    connp,
                    if (*(*connp).in_header).realptr.is_null() {
                        ((*connp).in_header as *mut libc::c_uchar)
                            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                    } else {
                        (*(*connp).in_header).realptr
                    },
                    (*(*connp).in_header).len,
                ) != 1 as libc::c_int
                {
                    return -(1 as libc::c_int);
                }
                bstr_free((*connp).in_header);
                (*connp).in_header = 0 as *mut bstr
            }
            htp_connp_req_clear_buffer(connp);
            (*(*connp).in_tx).request_progress = HTP_REQUEST_TRAILER;
            // We've seen all the request headers.
            return htp_tx_state_request_headers((*connp).in_tx);
        }
        if (*connp).in_current_read_offset < (*connp).in_current_len {
            (*connp).in_next_byte = *(*connp)
                .in_current_data
                .offset((*connp).in_current_read_offset as isize)
                as libc::c_int;
            (*connp).in_current_read_offset += 1;
            (*connp).in_stream_offset += 1
        } else {
            return 5 as libc::c_int;
        }
        // Have we reached the end of the line?
        if (*connp).in_next_byte == '\n' as i32 {
            let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut len: size_t = 0;
            if htp_connp_req_consolidate_data(connp, &mut data, &mut len) != 1 as libc::c_int {
                return -(1 as libc::c_int);
            }
            // Should we terminate headers?
            if htp_connp_is_line_terminator(connp, data, len) != 0 {
                // Parse previous header, if any.
                if !(*connp).in_header.is_null() {
                    if (*(*connp).cfg)
                        .process_request_header
                        .expect("non-null function pointer")(
                        connp,
                        if (*(*connp).in_header).realptr.is_null() {
                            ((*connp).in_header as *mut libc::c_uchar)
                                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                        } else {
                            (*(*connp).in_header).realptr
                        },
                        (*(*connp).in_header).len,
                    ) != 1 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    bstr_free((*connp).in_header);
                    (*connp).in_header = 0 as *mut bstr
                }
                htp_connp_req_clear_buffer(connp);
                // We've seen all the request headers.
                return htp_tx_state_request_headers((*connp).in_tx);
            }
            htp_chomp(data, &mut len);
            // Check for header folding.
            if htp_connp_is_line_folded(data, len) == 0 as libc::c_int {
                // New header line.
                // Parse previous header, if any.
                if !(*connp).in_header.is_null() {
                    if (*(*connp).cfg)
                        .process_request_header
                        .expect("non-null function pointer")(
                        connp,
                        if (*(*connp).in_header).realptr.is_null() {
                            ((*connp).in_header as *mut libc::c_uchar)
                                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                        } else {
                            (*(*connp).in_header).realptr
                        },
                        (*(*connp).in_header).len,
                    ) != 1 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    bstr_free((*connp).in_header);
                    (*connp).in_header = 0 as *mut bstr
                }
                if (*connp).in_current_read_offset >= (*connp).in_current_len {
                    (*connp).in_next_byte = -(1 as libc::c_int)
                } else {
                    (*connp).in_next_byte = *(*connp)
                        .in_current_data
                        .offset((*connp).in_current_read_offset as isize)
                        as libc::c_int
                }
                if (*connp).in_next_byte != -(1 as libc::c_int)
                    && htp_is_folding_char((*connp).in_next_byte) == 0 as libc::c_int
                {
                    // Because we know this header is not folded, we can process the buffer straight away.
                    if (*(*connp).cfg)
                        .process_request_header
                        .expect("non-null function pointer")(connp, data, len)
                        != 1 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                } else {
                    // Keep the partial header data for parsing later.
                    (*connp).in_header = bstr_dup_mem(data as *const libc::c_void, len);
                    if (*connp).in_header.is_null() {
                        return -(1 as libc::c_int);
                    }
                }
            } else if (*connp).in_header.is_null() {
                // Folding; check that there's a previous header line to add to.
                // Invalid folding.
                // Warn only once per transaction.
                if (*(*connp).in_tx).flags as libc::c_ulonglong & 0x200 as libc::c_ulonglong == 0 {
                    (*(*connp).in_tx).flags = ((*(*connp).in_tx).flags as libc::c_ulonglong
                        | 0x200 as libc::c_ulonglong)
                        as uint64_t;
                    htp_log(
                        connp,
                        b"htp_request.c\x00" as *const u8 as *const libc::c_char,
                        699 as libc::c_int,
                        HTP_LOG_WARNING,
                        0 as libc::c_int,
                        b"Invalid request field folding\x00" as *const u8 as *const libc::c_char,
                    );
                }
                // Keep the header data for parsing later.
                (*connp).in_header = bstr_dup_mem(data as *const libc::c_void, len);
                if (*connp).in_header.is_null() {
                    return -(1 as libc::c_int);
                }
            } else {
                // Add to the existing header.
                let mut new_in_header: *mut bstr =
                    bstr_add_mem((*connp).in_header, data as *const libc::c_void, len);
                if new_in_header.is_null() {
                    return -(1 as libc::c_int);
                }
                (*connp).in_header = new_in_header
            }
            htp_connp_req_clear_buffer(connp);
        }
    }
}

/* *
 * Determines request protocol.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_PROTOCOL(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // Is this a short-style HTTP/0.9 request? If it is,
    // we will not want to parse request headers.
    if (*(*connp).in_tx).is_protocol_0_9 == 0 as libc::c_int {
        // Switch to request header parsing.
        (*connp).in_state = Some(
            htp_connp_REQ_HEADERS
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        );
        (*(*connp).in_tx).request_progress = HTP_REQUEST_HEADERS
    } else {
        // Let's check if the protocol was simply missing
        let mut pos: int64_t = (*connp).in_current_read_offset;
        let mut afterspaces: libc::c_int = 0 as libc::c_int;
        // Probe if data looks like a header line
        while pos < (*connp).in_current_len {
            if *(*connp).in_current_data.offset(pos as isize) as libc::c_int == ':' as i32 {
                htp_log(
                    connp,
                    b"htp_request.c\x00" as *const u8 as *const libc::c_char,
                    740 as libc::c_int,
                    HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"Request line: missing protocol\x00" as *const u8 as *const libc::c_char,
                );
                (*(*connp).in_tx).is_protocol_0_9 = 0 as libc::c_int;
                // Switch to request header parsing.
                (*connp).in_state = Some(
                    htp_connp_REQ_HEADERS
                        as unsafe extern "C" fn(
                            _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        ) -> htp_status_t,
                );
                (*(*connp).in_tx).request_progress = HTP_REQUEST_HEADERS;
                return 1 as libc::c_int;
            } else {
                if htp_is_lws(*(*connp).in_current_data.offset(pos as isize) as libc::c_int) != 0 {
                    // Allows spaces after header name
                    afterspaces = 1 as libc::c_int
                } else if htp_is_space(*(*connp).in_current_data.offset(pos as isize) as libc::c_int)
                    != 0
                    || afterspaces == 1 as libc::c_int
                {
                    break;
                }
                pos += 1
            }
        }
        // We're done with this request.
        (*connp).in_state = Some(
            htp_connp_REQ_FINALIZE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    }
    return 1 as libc::c_int;
}

/* *
 * Parse the request line.
 *
 * @param[in] connp
 * @returns HTP_OK on succesful parse, HTP_ERROR on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_LINE_complete(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    if htp_connp_req_consolidate_data(connp, &mut data, &mut len) != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    // Is this a line that should be ignored?
    if htp_connp_is_line_ignorable(connp, data, len) != 0 {
        // We have an empty/whitespace line, which we'll note, ignore and move on.
        (*(*connp).in_tx).request_ignored_lines =
            (*(*connp).in_tx).request_ignored_lines.wrapping_add(1);
        htp_connp_req_clear_buffer(connp);
        return 1 as libc::c_int;
    }
    // Process request line.
    htp_chomp(data, &mut len);
    (*(*connp).in_tx).request_line = bstr_dup_mem(data as *const libc::c_void, len);
    if (*(*connp).in_tx).request_line.is_null() {
        return -(1 as libc::c_int);
    }
    if (*(*connp).cfg)
        .parse_request_line
        .expect("non-null function pointer")(connp)
        != 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    // Finalize request line parsing.
    if htp_tx_state_request_line((*connp).in_tx) != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    htp_connp_req_clear_buffer(connp);
    return 1 as libc::c_int;
}

/* *
 * Parses request line.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_LINE(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    loop {
        // Get one byte
        if (*connp).in_current_read_offset < (*connp).in_current_len {
            (*connp).in_next_byte = *(*connp)
                .in_current_data
                .offset((*connp).in_current_read_offset as isize)
                as libc::c_int;
            (*connp).in_current_read_offset += 1;
            (*connp).in_stream_offset += 1
        } else {
            return 5 as libc::c_int;
        }
        // Have we reached the end of the line?
        if (*connp).in_next_byte == '\n' as i32 {
            return htp_connp_REQ_LINE_complete(connp);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_FINALIZE(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    if (*connp).in_status as libc::c_uint != HTP_STREAM_CLOSED as libc::c_int as libc::c_uint {
        if (*connp).in_current_read_offset >= (*connp).in_current_len {
            (*connp).in_next_byte = -(1 as libc::c_int)
        } else {
            (*connp).in_next_byte = *(*connp)
                .in_current_data
                .offset((*connp).in_current_read_offset as isize)
                as libc::c_int
        }
        if (*connp).in_next_byte == -(1 as libc::c_int) {
            return htp_tx_state_request_complete((*connp).in_tx);
        }
        if (*connp).in_next_byte != '\n' as i32
            || (*connp).in_current_consume_offset >= (*connp).in_current_read_offset
        {
            loop {
                //;i < max_read; i++) {
                if (*connp).in_current_read_offset < (*connp).in_current_len {
                    (*connp).in_next_byte = *(*connp)
                        .in_current_data
                        .offset((*connp).in_current_read_offset as isize)
                        as libc::c_int;
                    (*connp).in_current_read_offset += 1;
                    (*connp).in_stream_offset += 1
                } else {
                    return 5 as libc::c_int;
                }
                // Have we reached the end of the line? For some reason
                // we can't test after IN_COPY_BYTE_OR_RETURN */
                if (*connp).in_next_byte == '\n' as i32 {
                    break;
                }
            }
        }
    }
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    if htp_connp_req_consolidate_data(connp, &mut data, &mut len) != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        //closing
        return htp_tx_state_request_complete((*connp).in_tx);
    }
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut mstart: size_t = 0 as libc::c_int as size_t;
    // skip past leading whitespace. IIS allows this
    while pos < len && htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    if pos != 0 {
        mstart = pos
    }
    // The request method starts at the beginning of the
    // line and ends with the first whitespace character.
    while pos < len && htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0 {
        pos = pos.wrapping_add(1)
    }
    if pos > mstart {
        let mut methodi: libc::c_int = HTP_M_UNKNOWN as libc::c_int;
        let mut method: *mut bstr = bstr_dup_mem(
            data.offset(mstart as isize) as *const libc::c_void,
            pos.wrapping_sub(mstart),
        );
        if !method.is_null() {
            methodi = htp_convert_method_to_number(method);
            bstr_free(method);
        }
        if methodi == HTP_M_UNKNOWN as libc::c_int {
            // Interpret remaining bytes as body data
            htp_log(
                connp,
                b"htp_request.c\x00" as *const u8 as *const libc::c_char,
                881 as libc::c_int,
                HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Unexpected request body\x00" as *const u8 as *const libc::c_char,
            );
            let mut rc: htp_status_t =
                htp_tx_req_process_body_data_ex((*connp).in_tx, data as *const libc::c_void, len);
            htp_connp_req_clear_buffer(connp);
            return rc;
        }
    }
    //else
    //unread last end of line so that REQ_LINE works
    if (*connp).in_current_read_offset < len as int64_t {
        (*connp).in_current_read_offset = 0 as libc::c_int as int64_t
    } else {
        (*connp).in_current_read_offset = ((*connp).in_current_read_offset as libc::c_ulong)
            .wrapping_sub(len) as int64_t as int64_t
    }
    if (*connp).in_current_read_offset < (*connp).in_current_consume_offset {
        (*connp).in_current_consume_offset = (*connp).in_current_read_offset
    }
    return htp_tx_state_request_complete((*connp).in_tx);
}

#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_IGNORE_DATA_AFTER_HTTP_0_9(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // Consume whatever is left in the buffer.
    let mut bytes_left: size_t =
        ((*connp).in_current_len - (*connp).in_current_read_offset) as size_t;
    if bytes_left > 0 as libc::c_int as libc::c_ulong {
        (*(*connp).conn).flags =
            ((*(*connp).conn).flags as libc::c_ulonglong | 0x2 as libc::c_ulonglong) as uint8_t
    }
    (*connp).in_current_read_offset = ((*connp).in_current_read_offset as libc::c_ulong)
        .wrapping_add(bytes_left) as int64_t as int64_t;
    (*connp).in_current_consume_offset = ((*connp).in_current_consume_offset as libc::c_ulong)
        .wrapping_add(bytes_left) as int64_t as int64_t;
    (*connp).in_stream_offset =
        ((*connp).in_stream_offset as libc::c_ulong).wrapping_add(bytes_left) as int64_t as int64_t;
    return 2 as libc::c_int;
}

/* *
 * The idle state is where the parser will end up after a transaction is processed.
 * If there is more data available, a new request will be started.
 *
 * @param[in] connp
 * @returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_REQ_IDLE(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    // We want to start parsing the next request (and change
    // the state from IDLE) only if there's at least one
    // byte of data available. Otherwise we could be creating
    // new structures even if there's no more data on the
    // connection.
    if (*connp).in_current_read_offset >= (*connp).in_current_len {
        return 2 as libc::c_int;
    }
    (*connp).in_tx = htp_connp_tx_create(connp);
    if (*connp).in_tx.is_null() {
        return -(1 as libc::c_int);
    }
    // Change state to TRANSACTION_START
    htp_tx_state_request_start((*connp).in_tx);
    return 1 as libc::c_int;
}

/* *
 * Returns how many bytes from the current data chunks were consumed so far.
 *
 * @param[in] connp
 * @return The number of bytes consumed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_req_data_consumed(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> size_t {
    return (*connp).in_current_read_offset as size_t;
}

/* *
 *
 * @param[in] connp
 * @param[in] timestamp
 * @param[in] data
 * @param[in] len
 * @return HTP_STREAM_DATA, HTP_STREAM_ERROR or STEAM_STATE_DATA_OTHER (see QUICK_START).
 *         HTP_STREAM_CLOSED and HTP_STREAM_TUNNEL are also possible.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_req_data(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    mut timestamp: *const htp_time_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    // Return if the connection is in stop state.
    if (*connp).in_status as libc::c_uint == HTP_STREAM_STOP as libc::c_int as libc::c_uint {
        htp_log(
            connp,
            b"htp_request.c\x00" as *const u8 as *const libc::c_char,
            959 as libc::c_int,
            HTP_LOG_INFO,
            0 as libc::c_int,
            b"Inbound parser is in HTP_STREAM_STOP\x00" as *const u8 as *const libc::c_char,
        );
        return HTP_STREAM_STOP as libc::c_int;
    }
    // Return if the connection had a fatal error earlier
    if (*connp).in_status as libc::c_uint == HTP_STREAM_ERROR as libc::c_int as libc::c_uint {
        htp_log(
            connp,
            b"htp_request.c\x00" as *const u8 as *const libc::c_char,
            965 as libc::c_int,
            HTP_LOG_ERROR,
            0 as libc::c_int,
            b"Inbound parser is in HTP_STREAM_ERROR\x00" as *const u8 as *const libc::c_char,
        );
        return HTP_STREAM_ERROR as libc::c_int;
    }
    // Sanity check: we must have a transaction pointer if the state is not IDLE (no inbound transaction)
    if (*connp).in_tx.is_null()
        && (*connp).in_state
            != Some(
                htp_connp_REQ_IDLE
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            )
    {
        (*connp).in_status = HTP_STREAM_ERROR;
        htp_log(
            connp,
            b"htp_request.c\x00" as *const u8 as *const libc::c_char,
            978 as libc::c_int,
            HTP_LOG_ERROR,
            0 as libc::c_int,
            b"Missing inbound transaction data\x00" as *const u8 as *const libc::c_char,
        );
        return HTP_STREAM_ERROR as libc::c_int;
    }
    // If the length of the supplied data chunk is zero, proceed
    // only if the stream has been closed. We do not allow zero-sized
    // chunks in the API, but we use them internally to force the parsers
    // to finalize parsing.
    if (data == 0 as *mut libc::c_void || len == 0 as libc::c_int as libc::c_ulong)
        && (*connp).in_status as libc::c_uint != HTP_STREAM_CLOSED as libc::c_int as libc::c_uint
    {
        htp_log(
            connp,
            b"htp_request.c\x00" as *const u8 as *const libc::c_char,
            988 as libc::c_int,
            HTP_LOG_ERROR,
            0 as libc::c_int,
            b"Zero-length data chunks are not allowed\x00" as *const u8 as *const libc::c_char,
        );
        return HTP_STREAM_CLOSED as libc::c_int;
    }
    // Remember the timestamp of the current request data chunk
    if !timestamp.is_null() {
        memcpy(
            &mut (*connp).in_timestamp as *mut htp_time_t as *mut libc::c_void,
            timestamp as *const libc::c_void,
            ::std::mem::size_of::<htp_time_t>() as libc::c_ulong,
        );
    }
    // Store the current chunk information
    (*connp).in_current_data = data as *mut libc::c_uchar;
    (*connp).in_current_len = len as int64_t;
    (*connp).in_current_read_offset = 0 as libc::c_int as int64_t;
    (*connp).in_current_consume_offset = 0 as libc::c_int as int64_t;
    (*connp).in_current_receiver_offset = 0 as libc::c_int as int64_t;
    (*connp).in_chunk_count = (*connp).in_chunk_count.wrapping_add(1);
    htp_conn_track_inbound_data((*connp).conn, len, timestamp);
    // Return without processing any data if the stream is in tunneling
    // mode (which it would be after an initial CONNECT transaction).
    if (*connp).in_status as libc::c_uint == HTP_STREAM_TUNNEL as libc::c_int as libc::c_uint {
        return HTP_STREAM_TUNNEL as libc::c_int;
    }
    if (*connp).out_status as libc::c_uint == HTP_STREAM_DATA_OTHER as libc::c_int as libc::c_uint {
        (*connp).out_status = HTP_STREAM_DATA
    }
    loop
    // Invoke a processor, in a loop, until an error
    // occurs or until we run out of data. Many processors
    // will process a request, each pointing to the next
    // processor that needs to run.
    // Return if there's been an error or if we've run out of data. We are relying
    // on processors to supply error messages, so we'll keep quiet here.
    {
        let mut rc: htp_status_t = (*connp).in_state.expect("non-null function pointer")(connp);
        if rc == 1 as libc::c_int {
            if (*connp).in_status as libc::c_uint
                == HTP_STREAM_TUNNEL as libc::c_int as libc::c_uint
            {
                return HTP_STREAM_TUNNEL as libc::c_int;
            }
            rc = htp_req_handle_state_change(connp)
        }
        if rc != 1 as libc::c_int {
            // Do we need more data?
            if rc == 2 as libc::c_int || rc == 5 as libc::c_int {
                htp_connp_req_receiver_send_data(connp, 0 as libc::c_int);
                if rc == 5 as libc::c_int {
                    if htp_connp_req_buffer(connp) != 1 as libc::c_int {
                        (*connp).in_status = HTP_STREAM_ERROR;
                        return HTP_STREAM_ERROR as libc::c_int;
                    }
                }
                (*connp).in_status = HTP_STREAM_DATA;
                return HTP_STREAM_DATA as libc::c_int;
            }
            // Check for suspended parsing.
            if rc == 3 as libc::c_int {
                // We might have actually consumed the entire data chunk?
                if (*connp).in_current_read_offset >= (*connp).in_current_len {
                    // Do not send STREAM_DATE_DATA_OTHER if we've consumed the entire chunk.
                    (*connp).in_status = HTP_STREAM_DATA;
                    return HTP_STREAM_DATA as libc::c_int;
                } else {
                    // Partial chunk consumption.
                    (*connp).in_status = HTP_STREAM_DATA_OTHER;
                    return HTP_STREAM_DATA_OTHER as libc::c_int;
                }
            }
            // Check for the stop signal.
            if rc == 4 as libc::c_int {
                (*connp).in_status = HTP_STREAM_STOP;
                return HTP_STREAM_STOP as libc::c_int;
            }
            // Permanent stream error.
            (*connp).in_status = HTP_STREAM_ERROR;
            return HTP_STREAM_ERROR as libc::c_int;
        }
    }
}
