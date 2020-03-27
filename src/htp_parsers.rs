use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn bstr_begins_with_c_nocase(
        bhaystack: *const bstr,
        cneedle: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn bstr_dup_ex(b: *const bstr, offset: size_t, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn bstr_index_of_c(bhaystack: *const bstr, cneedle: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn htp_base64_decode_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn htp_table_get_c(
        table: *const crate::src::htp_table::htp_table_t,
        ckey: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_parse_positive_integer_whitespace(
        data: *mut libc::c_uchar,
        len: size_t,
        base: libc::c_int,
    ) -> int64_t;
    #[no_mangle]
    fn htp_extract_quoted_string_as_bstr(
        data: *mut libc::c_uchar,
        len: size_t,
        out: *mut *mut bstr,
        endoffset: *mut size_t,
    ) -> htp_status_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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

/* *
 * Determines protocol number from a textual representation (i.e., "HTTP/1.1"). This
 * function will only understand a properly formatted protocol information. It does
 * not try to be flexible.
 *
 * @param[in] protocol
 * @return Protocol version or PROTOCOL_UNKNOWN.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_protocol(mut protocol: *mut bstr) -> libc::c_int {
    if protocol.is_null() {
        return -(2 as libc::c_int);
    }
    // TODO This function uses a very strict approach to parsing, whereas
    //      browsers will typically be more flexible, allowing whitespace
    //      before and after the forward slash, as well as allowing leading
    //      zeroes in the numbers. We should be able to parse such malformed
    //      content correctly (but emit a warning).
    if (*protocol).len == 8 as libc::c_int as libc::c_ulong {
        let mut ptr: *mut libc::c_uchar = if (*protocol).realptr.is_null() {
            (protocol as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*protocol).realptr
        };
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == 'H' as i32
            && *ptr.offset(1 as libc::c_int as isize) as libc::c_int == 'T' as i32
            && *ptr.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
            && *ptr.offset(3 as libc::c_int as isize) as libc::c_int == 'P' as i32
            && *ptr.offset(4 as libc::c_int as isize) as libc::c_int == '/' as i32
            && *ptr.offset(6 as libc::c_int as isize) as libc::c_int == '.' as i32
        {
            // Check the version numbers
            if *ptr.offset(5 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                if *ptr.offset(7 as libc::c_int as isize) as libc::c_int == '9' as i32 {
                    return 9 as libc::c_int;
                }
            } else if *ptr.offset(5 as libc::c_int as isize) as libc::c_int == '1' as i32 {
                if *ptr.offset(7 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                    return 100 as libc::c_int;
                } else {
                    if *ptr.offset(7 as libc::c_int as isize) as libc::c_int == '1' as i32 {
                        return 101 as libc::c_int;
                    }
                }
            }
        }
    }
    return -(2 as libc::c_int);
}

/* *
 * Determines the numerical value of a response status given as a string.
 *
 * @param[in] status
 * @return Status code on success, or HTP_STATUS_INVALID on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_status(mut status: *mut bstr) -> libc::c_int {
    let mut r: int64_t = htp_parse_positive_integer_whitespace(
        if (*status).realptr.is_null() {
            (status as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*status).realptr
        },
        (*status).len,
        10 as libc::c_int,
    );
    if r >= 100 as libc::c_int as libc::c_long && r <= 999 as libc::c_int as libc::c_long {
        return r as libc::c_int;
    } else {
        return -(1 as libc::c_int);
    };
}

/* *
 * Parses Digest Authorization request header.
 *
 * @param[in] connp
 * @param[in] auth_header
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_authorization_digest(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    mut auth_header: *mut crate::src::htp_transaction::htp_header_t,
) -> libc::c_int {
    // Extract the username
    let mut i: libc::c_int = bstr_index_of_c(
        (*auth_header).value,
        b"username=\x00" as *const u8 as *const libc::c_char,
    );
    if i == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    let mut data: *mut libc::c_uchar = if (*(*auth_header).value).realptr.is_null() {
        ((*auth_header).value as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*(*auth_header).value).realptr
    };
    let mut len: size_t = (*(*auth_header).value).len;
    let mut pos: size_t = (i + 9 as libc::c_int) as size_t;
    // Ignore whitespace
    while pos < len
        && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return 0 as libc::c_int;
    }
    if *data.offset(pos as isize) as libc::c_int != '\"' as i32 {
        return 0 as libc::c_int;
    }
    return htp_extract_quoted_string_as_bstr(
        data.offset(pos as isize),
        len.wrapping_sub(pos),
        &mut (*(*connp).in_tx).request_auth_username,
        0 as *mut size_t,
    );
}

/* *
 * Parses Basic Authorization request header.
 *
 * @param[in] connp
 * @param[in] auth_header
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_authorization_basic(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    mut auth_header: *mut crate::src::htp_transaction::htp_header_t,
) -> libc::c_int {
    let mut data: *mut libc::c_uchar = if (*(*auth_header).value).realptr.is_null() {
        ((*auth_header).value as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*(*auth_header).value).realptr
    };
    let mut len: size_t = (*(*auth_header).value).len;
    let mut pos: size_t = 5 as libc::c_int as size_t;
    // Ignore whitespace
    while pos < len
        && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return 0 as libc::c_int;
    }
    // Decode base64-encoded data
    let mut decoded: *mut bstr = htp_base64_decode_mem(
        data.offset(pos as isize) as *const libc::c_void,
        len.wrapping_sub(pos),
    );
    if decoded.is_null() {
        return -(1 as libc::c_int);
    }
    // Now extract the username and password
    let mut i: libc::c_int = bstr_index_of_c(decoded, b":\x00" as *const u8 as *const libc::c_char);
    if i == -(1 as libc::c_int) {
        bstr_free(decoded);
        return 0 as libc::c_int;
    }
    (*(*connp).in_tx).request_auth_username =
        bstr_dup_ex(decoded, 0 as libc::c_int as size_t, i as size_t);
    if (*(*connp).in_tx).request_auth_username.is_null() {
        bstr_free(decoded);
        return -(1 as libc::c_int);
    }
    (*(*connp).in_tx).request_auth_password = bstr_dup_ex(
        decoded,
        (i + 1 as libc::c_int) as size_t,
        (*decoded)
            .len
            .wrapping_sub(i as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if (*(*connp).in_tx).request_auth_password.is_null() {
        bstr_free(decoded);
        bstr_free((*(*connp).in_tx).request_auth_username);
        return -(1 as libc::c_int);
    }
    bstr_free(decoded);
    return 1 as libc::c_int;
}

/* *
 * Parses Authorization request header.
 *
 * @param[in] connp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_authorization(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> libc::c_int {
    let mut auth_header: *mut crate::src::htp_transaction::htp_header_t = htp_table_get_c(
        (*(*connp).in_tx).request_headers,
        b"authorization\x00" as *const u8 as *const libc::c_char,
    )
        as *mut crate::src::htp_transaction::htp_header_t;
    if auth_header.is_null() {
        (*(*connp).in_tx).request_auth_type = HTP_AUTH_NONE;
        return 1 as libc::c_int;
    }
    // TODO Need a flag to raise when failing to parse authentication headers.
    if bstr_begins_with_c_nocase(
        (*auth_header).value,
        b"basic\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        // Basic authentication
        (*(*connp).in_tx).request_auth_type = HTP_AUTH_BASIC;
        return htp_parse_authorization_basic(connp, auth_header);
    } else {
        if bstr_begins_with_c_nocase(
            (*auth_header).value,
            b"digest\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            // Digest authentication
            (*(*connp).in_tx).request_auth_type = HTP_AUTH_DIGEST;
            return htp_parse_authorization_digest(connp, auth_header);
        } else {
            // Unrecognized authentication method
            (*(*connp).in_tx).request_auth_type = HTP_AUTH_UNRECOGNIZED
        }
    }
    return 1 as libc::c_int;
}
