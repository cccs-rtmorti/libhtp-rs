use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn bstr_builder_append_mem(
        bb: *mut crate::src::bstr_builder::bstr_builder_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn bstr_builder_create() -> *mut crate::src::bstr_builder::bstr_builder_t;
    #[no_mangle]
    fn bstr_builder_destroy(bb: *mut crate::src::bstr_builder::bstr_builder_t);
    #[no_mangle]
    fn bstr_builder_to_str(bb: *const crate::src::bstr_builder::bstr_builder_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn htp_table_addn(
        table: *mut crate::src::htp_table::htp_table_t,
        key: *const bstr,
        element: *const libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_table_create(size: size_t) -> *mut crate::src::htp_table::htp_table_t;
    #[no_mangle]
    fn htp_table_destroy(table: *mut crate::src::htp_table::htp_table_t);
    #[no_mangle]
    fn htp_table_get_index(
        table: *const crate::src::htp_table::htp_table_t,
        idx: size_t,
        key: *mut *mut bstr,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_table_size(table: *const crate::src::htp_table::htp_table_t) -> size_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
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

pub type htp_time_t = crate::src::htp_connection_parser::timeval;
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
 * Transcode all parameters supplied in the table.
 *
 * @param[in] connp
 * @param[in] params
 * @param[in] destroy_old
 */
#[no_mangle]
pub unsafe extern "C" fn htp_transcode_params(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    mut params: *mut *mut crate::src::htp_table::htp_table_t,
    mut destroy_old: libc::c_int,
) -> libc::c_int {
    let mut input_params: *mut crate::src::htp_table::htp_table_t = *params;
    // No transcoding unless necessary
    if (*(*connp).cfg).internal_encoding.is_null() || (*(*connp).cfg).request_encoding.is_null() {
        return 1 as libc::c_int;
    }
    // Create a new table that will hold transcoded parameters
    let mut output_params: *mut crate::src::htp_table::htp_table_t =
        htp_table_create(htp_table_size(input_params));
    if output_params.is_null() {
        return -(1 as libc::c_int);
    }
    // Initialize iconv
    let mut cd: iconv_t = iconv_open(
        (*(*connp).cfg).internal_encoding,
        (*(*connp).cfg).request_encoding,
    );
    if cd == -(1 as libc::c_int) as iconv_t {
        htp_table_destroy(output_params);
        return -(1 as libc::c_int);
    }
    // Convert the parameters, one by one
    let mut name: *mut bstr = 0 as *mut bstr;
    let mut value: *mut bstr = 0 as *mut bstr;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = htp_table_size(input_params) as libc::c_int;
    while i < n {
        value = htp_table_get_index(input_params, i as size_t, &mut name) as *mut bstr;
        let mut new_name: *mut bstr = 0 as *mut bstr;
        let mut new_value: *mut bstr = 0 as *mut bstr;
        // Convert name
        htp_transcode_bstr(cd, name, &mut new_name);
        if new_name.is_null() {
            iconv_close(cd);
            let mut b: *mut bstr = 0 as *mut bstr;
            let mut j: libc::c_int = 0 as libc::c_int;
            let mut k: libc::c_int = htp_table_size(output_params) as libc::c_int;
            while j < k {
                b = htp_table_get_index(output_params, j as size_t, 0 as *mut *mut bstr)
                    as *mut bstr;
                bstr_free(b);
                j += 1
            }
            htp_table_destroy(output_params);
            return -(1 as libc::c_int);
        }
        // Convert value
        htp_transcode_bstr(cd, value, &mut new_value);
        if new_value.is_null() {
            bstr_free(new_name);
            iconv_close(cd);
            let mut b_0: *mut bstr = 0 as *mut bstr;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            let mut k_0: libc::c_int = htp_table_size(output_params) as libc::c_int;
            while j_0 < k_0 {
                b_0 = htp_table_get_index(output_params, j_0 as size_t, 0 as *mut *mut bstr)
                    as *mut bstr;
                bstr_free(b_0);
                j_0 += 1
            }
            htp_table_destroy(output_params);
            return -(1 as libc::c_int);
        }
        // Add to new table
        htp_table_addn(output_params, new_name, new_value as *const libc::c_void);
        i += 1
    }
    // Replace the old parameter table
    *params = output_params;
    // Destroy the old parameter table if necessary
    if destroy_old != 0 {
        let mut b_1: *mut bstr = 0 as *mut bstr;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        let mut n_0: libc::c_int = htp_table_size(input_params) as libc::c_int;
        while i_0 < n_0 {
            b_1 =
                htp_table_get_index(input_params, i_0 as size_t, 0 as *mut *mut bstr) as *mut bstr;
            bstr_free(b_1);
            i_0 += 1
        }
        htp_table_destroy(input_params);
    }
    iconv_close(cd);
    return 1 as libc::c_int;
}

/* *
 * Transcode one bstr.
 *
 * @param[in] cd
 * @param[in] input
 * @param[in] output
 */
#[no_mangle]
pub unsafe extern "C" fn htp_transcode_bstr(
    mut cd: iconv_t,
    mut input: *mut bstr,
    mut output: *mut *mut bstr,
) -> libc::c_int {
    // Reset conversion state for every new string
    iconv(
        cd,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
    );
    let mut bb: *mut crate::src::bstr_builder::bstr_builder_t =
        0 as *mut crate::src::bstr_builder::bstr_builder_t;
    let buflen: size_t = 10 as libc::c_int as size_t;
    let mut buf: *mut libc::c_uchar = malloc(buflen) as *mut libc::c_uchar;
    if buf.is_null() {
        return -(1 as libc::c_int);
    }
    let mut inbuf: *const libc::c_char = if (*input).realptr.is_null() {
        (input as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*input).realptr
    } as *const libc::c_char;
    let mut inleft: size_t = (*input).len;
    let mut outbuf: *mut libc::c_char = buf as *mut libc::c_char;
    let mut outleft: size_t = buflen;
    let mut loop_0: libc::c_int = 1 as libc::c_int;
    while loop_0 != 0 {
        loop_0 = 0 as libc::c_int;
        if iconv(
            cd,
            &mut inbuf as *mut *const libc::c_char as *mut *mut libc::c_char,
            &mut inleft,
            &mut outbuf as *mut *mut libc::c_char,
            &mut outleft,
        ) == -(1 as libc::c_int) as size_t
        {
            if *__errno_location() == 7 as libc::c_int {
                // Create bstr builder on-demand
                if bb.is_null() {
                    bb = bstr_builder_create();
                    if bb.is_null() {
                        free(buf as *mut libc::c_void);
                        return -(1 as libc::c_int);
                    }
                }
                // The output buffer is full
                bstr_builder_append_mem(
                    bb,
                    buf as *const libc::c_void,
                    buflen.wrapping_sub(outleft),
                );
                outbuf = buf as *mut libc::c_char;
                outleft = buflen;
                // Continue in the loop, as there's more work to do
                loop_0 = 1 as libc::c_int
            } else {
                // Error
                if !bb.is_null() {
                    bstr_builder_destroy(bb);
                }
                free(buf as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
        }
    }
    if !bb.is_null() {
        bstr_builder_append_mem(bb, buf as *const libc::c_void, buflen.wrapping_sub(outleft));
        *output = bstr_builder_to_str(bb);
        bstr_builder_destroy(bb);
        if (*output).is_null() {
            free(buf as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
    } else {
        *output = bstr_dup_mem(buf as *const libc::c_void, buflen.wrapping_sub(outleft));
        if (*output).is_null() {
            free(buf as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
    }
    free(buf as *mut libc::c_void);
    return 1 as libc::c_int;
}
