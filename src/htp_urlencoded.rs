use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn bstr_builder_append_mem(
        bb: *mut crate::src::bstr_builder::bstr_builder_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn bstr_builder_clear(bb: *mut crate::src::bstr_builder::bstr_builder_t);
    #[no_mangle]
    fn bstr_builder_create() -> *mut crate::src::bstr_builder::bstr_builder_t;
    #[no_mangle]
    fn bstr_builder_destroy(bb: *mut crate::src::bstr_builder::bstr_builder_t);
    #[no_mangle]
    fn bstr_builder_size(bb: *const crate::src::bstr_builder::bstr_builder_t) -> size_t;
    #[no_mangle]
    fn bstr_builder_to_str(bb: *const crate::src::bstr_builder::bstr_builder_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_dup_c(cstr: *const libc::c_char) -> *mut bstr;
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
    #[no_mangle]
    fn htp_tx_urldecode_params_inplace(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
        input: *mut bstr,
    ) -> htp_status_t;
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

/**
 * This is the main URLENCODED parser structure. It is used to store
 * parser configuration, temporary parsing data, as well as the parameters.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_urlenp_t {
    /** The transaction this parser belongs to. */
    pub tx: *mut crate::src::htp_transaction::htp_tx_t,
    /** The character used to separate parameters. Defaults to & and should
     *  not be changed without good reason.
     */
    pub argument_separator: libc::c_uchar,
    /** Whether to perform URL-decoding on parameters. */
    pub decode_url_encoding: libc::c_int,
    /** This table contains the list of parameters, indexed by name. */
    pub params: *mut crate::src::htp_table::htp_table_t,
    // Private fields; these are used during the parsing process only
    pub _state: libc::c_int,
    pub _complete: libc::c_int,
    pub _name: *mut bstr,
    pub _bb: *mut crate::src::bstr_builder::bstr_builder_t,
}

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
 * This method is invoked whenever a piece of data, belonging to a single field (name or value)
 * becomes available. It will either create a new parameter or store the transient information
 * until a parameter can be created.
 *
 * @param[in] urlenp
 * @param[in] data
 * @param[in] startpos
 * @param[in] endpos
 * @param[in] c Should contain -1 if the reason this function is called is because the end of
 *          the current data chunk is reached.
 */
unsafe extern "C" fn htp_urlenp_add_field_piece(
    mut urlenp: *mut htp_urlenp_t,
    mut data: *const libc::c_uchar,
    mut startpos: size_t,
    mut endpos: size_t,
    mut last_char: libc::c_int,
) {
    // Add field if we know it ended (last_char is something other than -1)
    // or if we know that there won't be any more input data (urlenp->_complete is true).
    if last_char != -(1 as libc::c_int) || (*urlenp)._complete != 0 {
        // Prepare the field value, assembling from multiple pieces as necessary.
        let mut field: *mut bstr = 0 as *mut bstr;
        // Did we use the string builder for this field?
        if bstr_builder_size((*urlenp)._bb) > 0 as libc::c_int as libc::c_ulong {
            // The current field consists of more than once piece, we have to use the string builder.
            // Add current piece to string builder.
            if !data.is_null() && endpos.wrapping_sub(startpos) > 0 as libc::c_int as libc::c_ulong
            {
                bstr_builder_append_mem(
                    (*urlenp)._bb,
                    data.offset(startpos as isize) as *const libc::c_void,
                    endpos.wrapping_sub(startpos),
                );
            }
            // Generate the field and clear the string builder.
            field = bstr_builder_to_str((*urlenp)._bb);
            if field.is_null() {
                return;
            }
            bstr_builder_clear((*urlenp)._bb);
        } else if !data.is_null()
            && endpos.wrapping_sub(startpos) > 0 as libc::c_int as libc::c_ulong
        {
            field = bstr_dup_mem(
                data.offset(startpos as isize) as *const libc::c_void,
                endpos.wrapping_sub(startpos),
            );
            if field.is_null() {
                return;
            }
        }
        // We only have the current piece to work with, so no need to involve the string builder.
        // Process field as key or value, as appropriate.
        if (*urlenp)._state == 1 as libc::c_int {
            // Key.
            // If there is no more work left to do, then we have a single key. Add it.
            if (*urlenp)._complete != 0 || last_char == (*urlenp).argument_separator as libc::c_int
            {
                // Handling empty pairs is tricky. We don't want to create a pair for
                // an entirely empty input, but in some cases it may be appropriate
                // (e.g., /index.php?&q=2).
                if !field.is_null() || last_char == (*urlenp).argument_separator as libc::c_int {
                    // Add one pair, with an empty value and possibly empty key too.
                    let mut name: *mut bstr = field;
                    if name.is_null() {
                        name = bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                        if name.is_null() {
                            return;
                        }
                    }
                    let mut value: *mut bstr =
                        bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                    if value.is_null() {
                        bstr_free(name);
                        return;
                    }
                    if (*urlenp).decode_url_encoding != 0 {
                        htp_tx_urldecode_params_inplace((*urlenp).tx, name);
                    }
                    htp_table_addn((*urlenp).params, name, value as *const libc::c_void);
                    (*urlenp)._name = 0 as *mut bstr
                }
            } else {
                // This key will possibly be followed by a value, so keep it for later.
                (*urlenp)._name = field
            }
        } else {
            // Value (with a key remembered from before).
            let mut name_0: *mut bstr = (*urlenp)._name;
            (*urlenp)._name = 0 as *mut bstr;
            if name_0.is_null() {
                name_0 = bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                if name_0.is_null() {
                    bstr_free(field);
                    return;
                }
            }
            let mut value_0: *mut bstr = field;
            if value_0.is_null() {
                value_0 = bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                if value_0.is_null() {
                    bstr_free(name_0);
                    return;
                }
            }
            if (*urlenp).decode_url_encoding != 0 {
                htp_tx_urldecode_params_inplace((*urlenp).tx, name_0);
                htp_tx_urldecode_params_inplace((*urlenp).tx, value_0);
            }
            htp_table_addn((*urlenp).params, name_0, value_0 as *const libc::c_void);
        }
    } else if !data.is_null() && endpos.wrapping_sub(startpos) > 0 as libc::c_int as libc::c_ulong {
        bstr_builder_append_mem(
            (*urlenp)._bb,
            data.offset(startpos as isize) as *const libc::c_void,
            endpos.wrapping_sub(startpos),
        );
    };
}

/* *
 * Creates a new URLENCODED parser.
 *
 * @return New parser, or NULL on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_create(
    mut tx: *mut crate::src::htp_transaction::htp_tx_t,
) -> *mut htp_urlenp_t {
    let mut urlenp: *mut htp_urlenp_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_urlenp_t>() as libc::c_ulong,
    ) as *mut htp_urlenp_t;
    if urlenp.is_null() {
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp).tx = tx;
    (*urlenp).params = htp_table_create(32 as libc::c_int as size_t);
    if (*urlenp).params.is_null() {
        free(urlenp as *mut libc::c_void);
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp)._bb = bstr_builder_create();
    if (*urlenp)._bb.is_null() {
        htp_table_destroy((*urlenp).params);
        free(urlenp as *mut libc::c_void);
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp).argument_separator = '&' as i32 as libc::c_uchar;
    (*urlenp).decode_url_encoding = 1 as libc::c_int;
    (*urlenp)._state = 1 as libc::c_int;
    return urlenp;
}

/* *
 * Destroys an existing URLENCODED parser.
 *
 * @param[in] urlenp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_destroy(mut urlenp: *mut htp_urlenp_t) {
    if urlenp.is_null() {
        return;
    }
    if !(*urlenp)._name.is_null() {
        bstr_free((*urlenp)._name);
    }
    bstr_builder_destroy((*urlenp)._bb);
    if !(*urlenp).params.is_null() {
        // Destroy parameters.
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_table_size((*urlenp).params);
        while i < n {
            let mut b: *mut bstr =
                htp_table_get_index((*urlenp).params, i, 0 as *mut *mut bstr) as *mut bstr;
            // Parameter name will be freed by the table code.
            bstr_free(b);
            i = i.wrapping_add(1)
        }
        htp_table_destroy((*urlenp).params);
    }
    free(urlenp as *mut libc::c_void);
}

/* *
 * Finalizes parsing, forcing the parser to convert any outstanding
 * data into parameters. This method should be invoked at the end
 * of a parsing operation that used htp_urlenp_parse_partial().
 *
 * @param[in] urlenp
 * @return Success indication
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_finalize(mut urlenp: *mut htp_urlenp_t) -> htp_status_t {
    (*urlenp)._complete = 1 as libc::c_int;
    return htp_urlenp_parse_partial(urlenp, 0 as *const libc::c_void, 0 as libc::c_int as size_t);
}

/* *
 * Parses the provided data chunk under the assumption
 * that it contains all the data that will be parsed. When this
 * method is used for parsing the finalization method should not
 * be invoked.
 *
 * @param[in] urlenp
 * @param[in] data
 * @param[in] len
 * @return
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_parse_complete(
    mut urlenp: *mut htp_urlenp_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    htp_urlenp_parse_partial(urlenp, data, len);
    return htp_urlenp_finalize(urlenp);
}

/* *
 * Parses the provided data chunk, keeping state to allow streaming parsing, i.e., the
 * parsing where only partial information is available at any one time. The method
 * htp_urlenp_finalize() must be invoked at the end to finalize parsing.
 *
 * @param[in] urlenp
 * @param[in] _data
 * @param[in] len
 * @return
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_parse_partial(
    mut urlenp: *mut htp_urlenp_t,
    mut _data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    let mut data: *mut libc::c_uchar = _data as *mut libc::c_uchar;
    let mut startpos: size_t = 0 as libc::c_int as size_t;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut c: libc::c_int = 0;
    if data.is_null() {
        len = 0 as libc::c_int as size_t
    }
    loop {
        // Get the next character, or use -1 to indicate end of input.
        if pos < len {
            c = *data.offset(pos as isize) as libc::c_int
        } else {
            c = -(1 as libc::c_int)
        }
        match (*urlenp)._state {
            1 => {
                // Look for =, argument separator, or end of input.
                if c == '=' as i32
                    || c == (*urlenp).argument_separator as libc::c_int
                    || c == -(1 as libc::c_int)
                {
                    // Data from startpos to pos.
                    htp_urlenp_add_field_piece(urlenp, data, startpos, pos, c);
                    // If it's not the end of input, then it must be the end of this field.
                    if c != -(1 as libc::c_int) {
                        // Next state.
                        startpos = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
                        if c == (*urlenp).argument_separator as libc::c_int {
                            (*urlenp)._state = 1 as libc::c_int
                        } else {
                            (*urlenp)._state = 2 as libc::c_int
                        }
                    }
                }
                pos = pos.wrapping_add(1)
            }
            2 => {
                // Look for argument separator or end of input.
                if c == (*urlenp).argument_separator as libc::c_int || c == -(1 as libc::c_int) {
                    // Data from startpos to pos.
                    htp_urlenp_add_field_piece(urlenp, data, startpos, pos, c);
                    // If it's not the end of input, then it must be the end of this field.
                    if c != -(1 as libc::c_int) {
                        // Next state.
                        startpos = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
                        (*urlenp)._state = 1 as libc::c_int
                    }
                }
                pos = pos.wrapping_add(1)
            }
            _ => {
                // Invalid state.
                return -(1 as libc::c_int);
            }
        }
        if !(c != -(1 as libc::c_int)) {
            break;
        }
    }
    return 1 as libc::c_int;
}
