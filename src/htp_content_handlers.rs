use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn htp_list_array_get(
        l: *const crate::src::htp_list::htp_list_array_t,
        idx: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_list_array_size(l: *const crate::src::htp_list::htp_list_array_t) -> size_t;
    #[no_mangle]
    fn bstr_begins_with_c(bhaystack: *const bstr, cneedle: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn htp_table_destroy_ex(table: *mut crate::src::htp_table::htp_table_t);
    #[no_mangle]
    fn htp_table_get_c(
        table: *const crate::src::htp_table::htp_table_t,
        ckey: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_table_get_index(
        table: *const crate::src::htp_table::htp_table_t,
        idx: size_t,
        key: *mut *mut bstr,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_table_size(table: *const crate::src::htp_table::htp_table_t) -> size_t;
    #[no_mangle]
    fn htp_mpartp_create(
        cfg: *mut crate::src::htp_config::htp_cfg_t,
        boundary: *mut bstr,
        flags: uint64_t,
    ) -> *mut crate::src::htp_multipart::htp_mpartp_t;
    #[no_mangle]
    fn htp_mpartp_find_boundary(
        content_type: *mut bstr,
        boundary: *mut *mut bstr,
        multipart_flags: *mut uint64_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_mpartp_get_multipart(
        parser: *mut crate::src::htp_multipart::htp_mpartp_t,
    ) -> *mut crate::src::htp_multipart::htp_multipart_t;
    #[no_mangle]
    fn htp_mpartp_finalize(parser: *mut crate::src::htp_multipart::htp_mpartp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_mpartp_parse(
        parser: *mut crate::src::htp_multipart::htp_mpartp_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_tx_register_request_body_data(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
        callback_fn: Option<
            unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_data_t) -> libc::c_int,
        >,
    );
    #[no_mangle]
    fn htp_tx_req_add_param(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
        param: *mut crate::src::htp_transaction::htp_param_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_urlenp_create(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
    ) -> *mut crate::src::htp_urlencoded::htp_urlenp_t;
    #[no_mangle]
    fn htp_urlenp_destroy(urlenp: *mut crate::src::htp_urlencoded::htp_urlenp_t);
    #[no_mangle]
    fn htp_urlenp_parse_partial(
        urlenp: *mut crate::src::htp_urlencoded::htp_urlenp_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_urlenp_parse_complete(
        urlenp: *mut crate::src::htp_urlencoded::htp_urlenp_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_urlenp_finalize(urlenp: *mut crate::src::htp_urlencoded::htp_urlenp_t) -> htp_status_t;
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
 * This callback function feeds request body data to a Urlencoded parser
 * and, later, feeds the parsed parameters to the correct structures.
 *
 * @param[in] d
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_ch_urlencoded_callback_request_body_data(
    mut d: *mut crate::src::htp_transaction::htp_tx_data_t,
) -> htp_status_t {
    let mut tx: *mut crate::src::htp_transaction::htp_tx_t = (*d).tx;
    // Check that we were not invoked again after the finalization.
    if (*(*tx).request_urlenp_body).params.is_null() {
        return -(1 as libc::c_int);
    }
    if !(*d).data.is_null() {
        // Process one chunk of data.
        htp_urlenp_parse_partial(
            (*tx).request_urlenp_body,
            (*d).data as *const libc::c_void,
            (*d).len,
        );
    } else {
        // Finalize parsing.
        htp_urlenp_finalize((*tx).request_urlenp_body);
        // Add all parameters to the transaction.
        let mut name: *mut bstr = 0 as *mut bstr;
        let mut value: *mut bstr = 0 as *mut bstr;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_table_size((*(*tx).request_urlenp_body).params);
        while i < n {
            value =
                htp_table_get_index((*(*tx).request_urlenp_body).params, i, &mut name) as *mut bstr;
            let mut param: *mut crate::src::htp_transaction::htp_param_t = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<crate::src::htp_transaction::htp_param_t>() as libc::c_ulong,
            )
                as *mut crate::src::htp_transaction::htp_param_t;
            if param.is_null() {
                return -(1 as libc::c_int);
            }
            (*param).name = name;
            (*param).value = value;
            (*param).source = HTP_SOURCE_BODY;
            (*param).parser_id = HTP_PARSER_URLENCODED;
            (*param).parser_data = 0 as *mut libc::c_void;
            if htp_tx_req_add_param(tx, param) != 1 as libc::c_int {
                free(param as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            i = i.wrapping_add(1)
        }
        // All the parameter data is now owned by the transaction, and
        // the parser table used to store it is no longer needed. The
        // line below will destroy just the table, leaving keys intact.
        htp_table_destroy_ex((*(*tx).request_urlenp_body).params);
        (*(*tx).request_urlenp_body).params = 0 as *mut crate::src::htp_table::htp_table_t
    }
    return 1 as libc::c_int;
}

/* *
 * Determine if the request has a Urlencoded body, and, if it does, create and
 * attach an instance of the Urlencoded parser to the transaction.
 *
 * @param[in] connp
 * @return HTP_OK if a new parser has been setup, HTP_DECLINED if the MIME type
 *         is not appropriate for this parser, and HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_ch_urlencoded_callback_request_headers(
    mut tx: *mut crate::src::htp_transaction::htp_tx_t,
) -> htp_status_t {
    // Check the request content type to see if it matches our MIME type.
    if (*tx).request_content_type.is_null()
        || bstr_begins_with_c(
            (*tx).request_content_type,
            b"application/x-www-form-urlencoded\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return 0 as libc::c_int;
    }
    // Create parser instance.
    (*tx).request_urlenp_body = htp_urlenp_create(tx);
    if (*tx).request_urlenp_body.is_null() {
        return -(1 as libc::c_int);
    }
    // Register a request body data callback.
    htp_tx_register_request_body_data(
        tx,
        Some(
            htp_ch_urlencoded_callback_request_body_data
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> htp_status_t,
        ),
    );
    return 1 as libc::c_int;
}

/* *
 * Parses request query string, if present.
 *
 * @param[in] connp
 * @param[in] raw_data
 * @param[in] raw_len
 * @return HTP_OK if query string was parsed, HTP_DECLINED if there was no query
 *         string, and HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_ch_urlencoded_callback_request_line(
    mut tx: *mut crate::src::htp_transaction::htp_tx_t,
) -> htp_status_t {
    // Proceed only if there's something for us to parse.
    if (*(*tx).parsed_uri).query.is_null()
        || (*(*(*tx).parsed_uri).query).len == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    // We have a non-zero length query string.
    (*tx).request_urlenp_query = htp_urlenp_create(tx);
    if (*tx).request_urlenp_query.is_null() {
        return -(1 as libc::c_int);
    }
    if htp_urlenp_parse_complete(
        (*tx).request_urlenp_query,
        (if (*(*(*tx).parsed_uri).query).realptr.is_null() {
            ((*(*tx).parsed_uri).query as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*(*(*tx).parsed_uri).query).realptr
        }) as *const libc::c_void,
        (*(*(*tx).parsed_uri).query).len,
    ) != 1 as libc::c_int
    {
        htp_urlenp_destroy((*tx).request_urlenp_query);
        return -(1 as libc::c_int);
    }
    // Add all parameters to the transaction.
    let mut name: *mut bstr = 0 as *mut bstr;
    let mut value: *mut bstr = 0 as *mut bstr;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_table_size((*(*tx).request_urlenp_query).params);
    while i < n {
        value =
            htp_table_get_index((*(*tx).request_urlenp_query).params, i, &mut name) as *mut bstr;
        let mut param: *mut crate::src::htp_transaction::htp_param_t = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<crate::src::htp_transaction::htp_param_t>() as libc::c_ulong,
        )
            as *mut crate::src::htp_transaction::htp_param_t;
        if param.is_null() {
            return -(1 as libc::c_int);
        }
        (*param).name = name;
        (*param).value = value;
        (*param).source = HTP_SOURCE_QUERY_STRING;
        (*param).parser_id = HTP_PARSER_URLENCODED;
        (*param).parser_data = 0 as *mut libc::c_void;
        if htp_tx_req_add_param(tx, param) != 1 as libc::c_int {
            free(param as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    // All the parameter data is now owned by the transaction, and
    // the parser table used to store it is no longer needed. The
    // line below will destroy just the table, leaving keys intact.
    htp_table_destroy_ex((*(*tx).request_urlenp_query).params);
    (*(*tx).request_urlenp_query).params = 0 as *mut crate::src::htp_table::htp_table_t;
    htp_urlenp_destroy((*tx).request_urlenp_query);
    (*tx).request_urlenp_query = 0 as *mut crate::src::htp_urlencoded::htp_urlenp_t;
    return 1 as libc::c_int;
}

/* *
 * Finalize Multipart processing.
 *
 * @param[in] d
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_ch_multipart_callback_request_body_data(
    mut d: *mut crate::src::htp_transaction::htp_tx_data_t,
) -> htp_status_t {
    let mut tx: *mut crate::src::htp_transaction::htp_tx_t = (*d).tx;
    // Check that we were not invoked again after the finalization.
    if (*(*tx).request_mpartp).gave_up_data == 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !(*d).data.is_null() {
        // Process one chunk of data.
        htp_mpartp_parse(
            (*tx).request_mpartp,
            (*d).data as *const libc::c_void,
            (*d).len,
        );
    } else {
        // Finalize parsing.
        htp_mpartp_finalize((*tx).request_mpartp);
        let mut body: *mut crate::src::htp_multipart::htp_multipart_t =
            htp_mpartp_get_multipart((*tx).request_mpartp);
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_list_array_size((*body).parts);
        while i < n {
            let mut part: *mut crate::src::htp_multipart::htp_multipart_part_t =
                htp_list_array_get((*body).parts, i)
                    as *mut crate::src::htp_multipart::htp_multipart_part_t;
            // Use text parameters.
            if (*part).type_0 as libc::c_uint == MULTIPART_PART_TEXT as libc::c_int as libc::c_uint
            {
                let mut param: *mut crate::src::htp_transaction::htp_param_t = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<crate::src::htp_transaction::htp_param_t>()
                        as libc::c_ulong,
                )
                    as *mut crate::src::htp_transaction::htp_param_t;
                if param.is_null() {
                    return -(1 as libc::c_int);
                }
                (*param).name = (*part).name;
                (*param).value = (*part).value;
                (*param).source = HTP_SOURCE_BODY;
                (*param).parser_id = HTP_PARSER_MULTIPART;
                (*param).parser_data = part as *mut libc::c_void;
                if htp_tx_req_add_param(tx, param) != 1 as libc::c_int {
                    free(param as *mut libc::c_void);
                    return -(1 as libc::c_int);
                }
            }
            i = i.wrapping_add(1)
        }
        // Tell the parser that it no longer owns names
        // and values of MULTIPART_PART_TEXT parts.
        (*(*tx).request_mpartp).gave_up_data = 1 as libc::c_int
    }
    return 1 as libc::c_int;
}

/* *
 * Inspect request headers and register the Multipart request data hook
 * if it contains a multipart/form-data body.
 *
 * @param[in] connp
 * @return HTP_OK if a new parser has been setup, HTP_DECLINED if the MIME type
 *         is not appropriate for this parser, and HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_ch_multipart_callback_request_headers(
    mut tx: *mut crate::src::htp_transaction::htp_tx_t,
) -> htp_status_t {
    // The field tx->request_content_type does not contain the entire C-T
    // value and so we cannot use it to look for a boundary, but we can
    // use it for a quick check to determine if the C-T header exists.
    if (*tx).request_content_type.is_null() {
        return 0 as libc::c_int;
    }
    // Look for a boundary.
    let mut ct: *mut crate::src::htp_transaction::htp_header_t = htp_table_get_c(
        (*tx).request_headers,
        b"content-type\x00" as *const u8 as *const libc::c_char,
    )
        as *mut crate::src::htp_transaction::htp_header_t;
    if ct.is_null() {
        return -(1 as libc::c_int);
    }
    let mut boundary: *mut bstr = 0 as *mut bstr;
    let mut flags: uint64_t = 0 as libc::c_int as uint64_t;
    let mut rc: htp_status_t = htp_mpartp_find_boundary((*ct).value, &mut boundary, &mut flags);
    if rc != 1 as libc::c_int {
        // No boundary (HTP_DECLINED) or error (HTP_ERROR).
        return rc;
    }
    if boundary.is_null() {
        return -(1 as libc::c_int);
    }
    // Create a Multipart parser instance.
    (*tx).request_mpartp = htp_mpartp_create((*(*tx).connp).cfg, boundary, flags);
    if (*tx).request_mpartp.is_null() {
        bstr_free(boundary);
        return -(1 as libc::c_int);
    }
    // Configure file extraction.
    if (*(*tx).cfg).extract_request_files != 0 {
        (*(*tx).request_mpartp).extract_files = 1 as libc::c_int;
        (*(*tx).request_mpartp).extract_dir = (*(*(*tx).connp).cfg).tmpdir
    }
    // Register a request body data callback.
    htp_tx_register_request_body_data(
        tx,
        Some(
            htp_ch_multipart_callback_request_body_data
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> htp_status_t,
        ),
    );
    return 1 as libc::c_int;
}
