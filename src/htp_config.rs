use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_process_response_header_generic(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        data: *mut libc::c_uchar,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_response_line_generic(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_process_request_header_generic(
        _: *mut crate::src::htp_connection_parser::htp_connp_t,
        data: *mut libc::c_uchar,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_request_line_generic(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_process_request_header_apache_2_2(
        _: *mut crate::src::htp_connection_parser::htp_connp_t,
        data: *mut libc::c_uchar,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_request_line_apache_2_2(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_hook_destroy(hook: *mut crate::src::htp_hooks::htp_hook_t);
    #[no_mangle]
    fn htp_hook_copy(
        hook: *const crate::src::htp_hooks::htp_hook_t,
    ) -> *mut crate::src::htp_hooks::htp_hook_t;
    #[no_mangle]
    fn htp_hook_register(
        hook: *mut *mut crate::src::htp_hooks::htp_hook_t,
        callback_fn: htp_callback_fn_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_ch_multipart_callback_request_headers(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_ch_urlencoded_callback_request_headers(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_ch_urlencoded_callback_request_line(
        tx: *mut crate::src::htp_transaction::htp_tx_t,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_cfg_t {
    /**
     * The maximum size of the buffer that is used when the current
     * input chunk does not contain all the necessary data (e.g., a very header
     * line that spans several packets).
     */
    pub field_limit_hard: size_t,
    /**
     * Soft field limit length. If this limit is reached the parser will issue
     * a warning but continue to run. NOT IMPLEMENTED.
     */
    pub field_limit_soft: size_t,
    /**
     * Log level, which will be used when deciding whether to store or
     * ignore the messages issued by the parser.
     */
    pub log_level: htp_log_level_t,
    /**
     * Whether to delete each transaction after the last hook is invoked. This
     * feature should be used when parsing traffic streams in real time.
     */
    pub tx_auto_destroy: libc::c_int,
    /**
     * Server personality identifier.
     */
    pub server_personality: htp_server_personality_t,
    /** The function used for request line parsing. Depends on the personality. */
    pub parse_request_line: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_connection_parser::htp_connp_t) -> libc::c_int,
    >,
    /** The function used for response line parsing. Depends on the personality. */
    pub parse_response_line: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_connection_parser::htp_connp_t) -> libc::c_int,
    >,
    /** The function used for request header parsing. Depends on the personality. */
    pub process_request_header: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::htp_connection_parser::htp_connp_t,
            _: *mut libc::c_uchar,
            _: size_t,
        ) -> libc::c_int,
    >,
    /** The function used for response header parsing. Depends on the personality. */
    pub process_response_header: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::htp_connection_parser::htp_connp_t,
            _: *mut libc::c_uchar,
            _: size_t,
        ) -> libc::c_int,
    >,
    /** The function to use to transform parameters after parsing. */
    pub parameter_processor: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_param_t) -> libc::c_int,
    >,
    /** Decoder configuration array, one per context. */
    pub decoder_cfgs: [htp_decoder_cfg_t; 3],
    /** Whether to generate the request_uri_normalized field. */
    pub generate_request_uri_normalized: libc::c_int,
    /** Whether to decompress compressed response bodies. */
    pub response_decompression_enabled: libc::c_int,
    /** Not fully implemented at the moment. */
    pub request_encoding: *mut libc::c_char,
    /** Not fully implemented at the moment. */
    pub internal_encoding: *mut libc::c_char,
    /** Whether to parse request cookies. */
    pub parse_request_cookies: libc::c_int,
    /** Whether to parse HTTP Authentication headers. */
    pub parse_request_auth: libc::c_int,
    /** Whether to extract files from requests using Multipart encoding. */
    pub extract_request_files: libc::c_int,
    /** How many extracted files are allowed in a single Multipart request? */
    pub extract_request_files_limit: libc::c_int,
    /** The location on disk where temporary files will be created. */
    pub tmpdir: *mut libc::c_char,
    /**
     * Request start hook, invoked when the parser receives the first byte of a new
     * request. Because in HTTP a transaction always starts with a request, this hook
     * doubles as a transaction start hook.
     */
    pub hook_request_start: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Request line hook, invoked after a request line has been parsed.
     */
    pub hook_request_line: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Request URI normalization hook, for overriding default normalization of URI.
     */
    pub hook_request_uri_normalize: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Receives raw request header data, starting immediately after the request line,
     * including all headers as they are seen on the TCP connection, and including the
     * terminating empty line. Not available on genuine HTTP/0.9 requests (because
     * they don't use headers).
     */
    pub hook_request_header_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Request headers hook, invoked after all request headers are seen.
     */
    pub hook_request_headers: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Request body data hook, invoked every time body data is available. Each
     * invocation will provide a htp_tx_data_t instance. Chunked data
     * will be dechunked before the data is passed to this hook. Decompression
     * is not currently implemented. At the end of the request body
     * there will be a call with the data pointer set to NULL.
     */
    pub hook_request_body_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Request file data hook, which is invoked whenever request file data is
     * available. Currently used only by the Multipart parser.
     */
    pub hook_request_file_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Receives raw request trailer data, which can be available on requests that have
     * chunked bodies. The data starts immediately after the zero-length chunk
     * and includes the terminating empty line.
     */
    pub hook_request_trailer_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Request trailer hook, invoked after all trailer headers are seen,
     * and if they are seen (not invoked otherwise).
     */
    pub hook_request_trailer: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Request hook, invoked after a complete request is seen.
     */
    pub hook_request_complete: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Response startup hook, invoked when a response transaction is found and
     * processing started.
     */
    pub hook_response_start: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Response line hook, invoked after a response line has been parsed.
     */
    pub hook_response_line: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Receives raw response header data, starting immediately after the status line
     * and including all headers as they are seen on the TCP connection, and including the
     * terminating empty line. Not available on genuine HTTP/0.9 responses (because
     * they don't have response headers).
     */
    pub hook_response_header_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Response headers book, invoked after all response headers have been seen.
     */
    pub hook_response_headers: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Response body data hook, invoked every time body data is available. Each
     * invocation will provide a htp_tx_data_t instance. Chunked data
     * will be dechunked before the data is passed to this hook. By default,
     * compressed data will be decompressed, but decompression can be disabled
     * in configuration. At the end of the response body there will be a call
     * with the data pointer set to NULL.
     */
    pub hook_response_body_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Receives raw response trailer data, which can be available on responses that have
     * chunked bodies. The data starts immediately after the zero-length chunk
     * and includes the terminating empty line.
     */
    pub hook_response_trailer_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Response trailer hook, invoked after all trailer headers have been processed,
     * and only if the trailer exists.
     */
    pub hook_response_trailer: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Response hook, invoked after a response has been seen. Because sometimes servers
     * respond before receiving complete requests, a response_complete callback may be
     * invoked prior to a request_complete callback.
     */
    pub hook_response_complete: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Transaction complete hook, which is invoked once the entire transaction is
     * considered complete (request and response are both complete). This is always
     * the last hook to be invoked.
     */
    pub hook_transaction_complete: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Log hook, invoked every time the library wants to log.
     */
    pub hook_log: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Opaque user data associated with this configuration structure.
     */
    pub user_data: *mut libc::c_void,
    // Request Line parsing options.

    // TODO this was added here to maintain a stable ABI, once we can break that
    // we may want to move this into htp_decoder_cfg_t (VJ)
    /** Reaction to leading whitespace on the request line */
    pub requestline_leading_whitespace_unwanted: htp_unwanted_t,
    /** How many layers of compression we will decompress (0 => no limit). */
    pub response_decompression_layer_limit: libc::c_int,
    /** max memory use by a the lzma decompressor. */
    pub lzma_memlimit: size_t,
    /** max output size for a compression bomb. */
    pub compression_bomb_limit: int32_t,
}

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_decoder_cfg_t {
    // Path-specific decoding options.
    /** Convert backslash characters to slashes. */
    pub backslash_convert_slashes: libc::c_int,
    /** Convert to lowercase. */
    pub convert_lowercase: libc::c_int,
    /** Compress slash characters. */
    pub path_separators_compress: libc::c_int,
    /** Should we URL-decode encoded path segment separators? */
    pub path_separators_decode: libc::c_int,
    /** Should we decode '+' characters to spaces? */
    pub plusspace_decode: libc::c_int,
    /** Reaction to encoded path separators. */
    pub path_separators_encoded_unwanted: htp_unwanted_t,
    // Special characters options.
    /** Controls how raw NUL bytes are handled. */
    pub nul_raw_terminates: libc::c_int,
    /** Determines server response to a raw NUL byte in the path. */
    pub nul_raw_unwanted: htp_unwanted_t,
    /** Reaction to control characters. */
    pub control_chars_unwanted: htp_unwanted_t,
    // URL encoding options.
    /** Should we decode %u-encoded characters? */
    pub u_encoding_decode: libc::c_int,
    /** Reaction to %u encoding. */
    pub u_encoding_unwanted: htp_unwanted_t,
    /** Handling of invalid URL encodings. */
    pub url_encoding_invalid_handling: htp_url_encoding_handling_t,
    /** Reaction to invalid URL encoding. */
    pub url_encoding_invalid_unwanted: htp_unwanted_t,
    /** Controls how encoded NUL bytes are handled. */
    pub nul_encoded_terminates: libc::c_int,
    /** How are we expected to react to an encoded NUL byte? */
    pub nul_encoded_unwanted: htp_unwanted_t,
    // UTF-8 options.
    /** Controls how invalid UTF-8 characters are handled. */
    pub utf8_invalid_unwanted: htp_unwanted_t,
    /** Convert UTF-8 characters into bytes using best-fit mapping. */
    pub utf8_convert_bestfit: libc::c_int,
    // Best-fit mapping options.
    /** The best-fit map to use to decode %u-encoded characters. */
    pub bestfit_map: *mut libc::c_uchar,
    /** The replacement byte used when there is no best-fit mapping. */
    pub bestfit_replacement_byte: libc::c_uchar,
}

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
/* * LZMA compression. */
pub const HTP_COMPRESSION_LZMA: htp_content_encoding_t = 4;
/* * Deflate compression. */
pub const HTP_COMPRESSION_DEFLATE: htp_content_encoding_t = 3;
/* * Gzip compression. */
pub const HTP_COMPRESSION_GZIP: htp_content_encoding_t = 2;
/* * No compression. */
pub const HTP_COMPRESSION_NONE: htp_content_encoding_t = 1;
/* *
 * This is the default value, which is used until the presence
 * of content encoding is determined (e.g., before request headers
 * are seen.
 */
pub const HTP_COMPRESSION_UNKNOWN: htp_content_encoding_t = 0;
/* *
 * Enumerates the possible request and response body codings.
 */
pub type htp_transfer_coding_t = libc::c_uint;
/* * We could not recognize the encoding. */
pub const HTP_CODING_INVALID: htp_transfer_coding_t = 4;
/* * Chunked encoding. */
pub const HTP_CODING_CHUNKED: htp_transfer_coding_t = 3;
/* * Identity coding is used, which means that the body was sent as is. */
pub const HTP_CODING_IDENTITY: htp_transfer_coding_t = 2;
/* * No body. */
pub const HTP_CODING_NO_BODY: htp_transfer_coding_t = 1;
/* * Body coding not determined yet. */
pub const HTP_CODING_UNKNOWN: htp_transfer_coding_t = 0;

pub type htp_table_alloc_t = libc::c_uint;
/* * Keys are only referenced; the caller is still responsible for freeing them after the table is destroyed. */
pub const HTP_TABLE_KEYS_REFERENCED: htp_table_alloc_t = 3;
/* * Keys are adopted and freed when the table is destroyed. */
pub const HTP_TABLE_KEYS_ADOPTED: htp_table_alloc_t = 2;
/* * Keys are copied.*/
pub const HTP_TABLE_KEYS_COPIED: htp_table_alloc_t = 1;
/* * This is the default value, used only until the first element is added. */
pub const HTP_TABLE_KEYS_ALLOC_UKNOWN: htp_table_alloc_t = 0;
/* *
 * Enumerates the possible values for authentication type.
 */
pub type htp_auth_type_t = libc::c_uint;
/* * Unrecognized authentication method. */
pub const HTP_AUTH_UNRECOGNIZED: htp_auth_type_t = 9;
/* * HTTP Digest authentication used. */
pub const HTP_AUTH_DIGEST: htp_auth_type_t = 3;
/* * HTTP Basic authentication used. */
pub const HTP_AUTH_BASIC: htp_auth_type_t = 2;
/* * No authentication. */
pub const HTP_AUTH_NONE: htp_auth_type_t = 1;
/* *
 * This is the default value that is used before
 * the presence of authentication is determined (e.g.,
 * before request headers are seen).
 */
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

pub type htp_decoder_ctx_t = libc::c_uint;
pub const HTP_DECODER_URL_PATH: htp_decoder_ctx_t = 2;
pub const HTP_DECODER_URLENCODED: htp_decoder_ctx_t = 1;
pub const HTP_DECODER_DEFAULTS: htp_decoder_ctx_t = 0;
pub type htp_callback_fn_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
/* *
 * This map is used by default for best-fit mapping from the Unicode
 * values U+0100-FFFF.
 */
static mut bestfit_1252: [libc::c_uchar; 1173] = [
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];

/**
 * Creates a new configuration structure. Configuration structures created at
 * configuration time must not be changed afterwards in order to support lock-less
 * copying.
 *
 * @return New configuration structure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_create() -> *mut htp_cfg_t {
    let mut cfg: *mut htp_cfg_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_cfg_t>() as libc::c_ulong,
    ) as *mut htp_cfg_t; // Use the parser default.
    if cfg.is_null() {
        return 0 as *mut htp_cfg_t;
    } // 2 layers seem fairly common
    (*cfg).field_limit_hard = 18000 as libc::c_int as size_t;
    (*cfg).field_limit_soft = 9000 as libc::c_int as size_t;
    (*cfg).log_level = HTP_LOG_NOTICE;
    (*cfg).response_decompression_enabled = 1 as libc::c_int;
    (*cfg).parse_request_cookies = 1 as libc::c_int;
    (*cfg).parse_request_auth = 1 as libc::c_int;
    (*cfg).extract_request_files = 0 as libc::c_int;
    (*cfg).extract_request_files_limit = -(1 as libc::c_int);
    (*cfg).response_decompression_layer_limit = 2 as libc::c_int;
    (*cfg).lzma_memlimit = 1048576 as libc::c_int as size_t;
    (*cfg).compression_bomb_limit = 1048576 as libc::c_int;
    // Default settings for URL-encoded data.
    htp_config_set_bestfit_map(
        cfg,
        HTP_DECODER_DEFAULTS,
        bestfit_1252.as_mut_ptr() as *mut libc::c_void,
    );
    htp_config_set_bestfit_replacement_byte(cfg, HTP_DECODER_DEFAULTS, '?' as i32);
    htp_config_set_url_encoding_invalid_handling(
        cfg,
        HTP_DECODER_DEFAULTS,
        HTP_URL_DECODE_PRESERVE_PERCENT,
    );
    htp_config_set_nul_raw_terminates(cfg, HTP_DECODER_DEFAULTS, 0 as libc::c_int);
    htp_config_set_nul_encoded_terminates(cfg, HTP_DECODER_DEFAULTS, 0 as libc::c_int);
    htp_config_set_u_encoding_decode(cfg, HTP_DECODER_DEFAULTS, 0 as libc::c_int);
    htp_config_set_plusspace_decode(cfg, HTP_DECODER_URLENCODED, 1 as libc::c_int);
    htp_config_set_server_personality(cfg, HTP_SERVER_MINIMAL);
    return cfg;
}

/**
 * Creates a copy of the supplied configuration structure. The idea is to create
 * one or more configuration objects at configuration-time, but to use this
 * function to create per-connection copies. That way it will be possible to
 * adjust per-connection configuration as necessary, without affecting the
 * global configuration. Make sure no other thread changes the configuration
 * object while this function is operating.
 *
 * @param[in] cfg
 * @return A copy of the configuration structure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_copy(mut cfg: *mut htp_cfg_t) -> *mut htp_cfg_t {
    if cfg.is_null() {
        return 0 as *mut htp_cfg_t;
    }
    // Start by making a copy of the entire structure,
    // which is essentially a shallow copy.
    let mut copy: *mut htp_cfg_t =
        malloc(::std::mem::size_of::<htp_cfg_t>() as libc::c_ulong) as *mut htp_cfg_t;
    if copy.is_null() {
        return 0 as *mut htp_cfg_t;
    }
    memcpy(
        copy as *mut libc::c_void,
        cfg as *const libc::c_void,
        ::std::mem::size_of::<htp_cfg_t>() as libc::c_ulong,
    );
    // Now create copies of the hooks' structures.
    if !(*cfg).hook_request_start.is_null() {
        (*copy).hook_request_start = htp_hook_copy((*cfg).hook_request_start);
        if (*copy).hook_request_start.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_line.is_null() {
        (*copy).hook_request_line = htp_hook_copy((*cfg).hook_request_line);
        if (*copy).hook_request_line.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_uri_normalize.is_null() {
        (*copy).hook_request_uri_normalize = htp_hook_copy((*cfg).hook_request_uri_normalize);
        if (*copy).hook_request_uri_normalize.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_header_data.is_null() {
        (*copy).hook_request_header_data = htp_hook_copy((*cfg).hook_request_header_data);
        if (*copy).hook_request_header_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_headers.is_null() {
        (*copy).hook_request_headers = htp_hook_copy((*cfg).hook_request_headers);
        if (*copy).hook_request_headers.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_body_data.is_null() {
        (*copy).hook_request_body_data = htp_hook_copy((*cfg).hook_request_body_data);
        if (*copy).hook_request_body_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_file_data.is_null() {
        (*copy).hook_request_file_data = htp_hook_copy((*cfg).hook_request_file_data);
        if (*copy).hook_request_file_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_trailer.is_null() {
        (*copy).hook_request_trailer = htp_hook_copy((*cfg).hook_request_trailer);
        if (*copy).hook_request_trailer.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_trailer_data.is_null() {
        (*copy).hook_request_trailer_data = htp_hook_copy((*cfg).hook_request_trailer_data);
        if (*copy).hook_request_trailer_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_request_complete.is_null() {
        (*copy).hook_request_complete = htp_hook_copy((*cfg).hook_request_complete);
        if (*copy).hook_request_complete.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_start.is_null() {
        (*copy).hook_response_start = htp_hook_copy((*cfg).hook_response_start);
        if (*copy).hook_response_start.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_line.is_null() {
        (*copy).hook_response_line = htp_hook_copy((*cfg).hook_response_line);
        if (*copy).hook_response_line.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_header_data.is_null() {
        (*copy).hook_response_header_data = htp_hook_copy((*cfg).hook_response_header_data);
        if (*copy).hook_response_header_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_headers.is_null() {
        (*copy).hook_response_headers = htp_hook_copy((*cfg).hook_response_headers);
        if (*copy).hook_response_headers.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_body_data.is_null() {
        (*copy).hook_response_body_data = htp_hook_copy((*cfg).hook_response_body_data);
        if (*copy).hook_response_body_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_trailer.is_null() {
        (*copy).hook_response_trailer = htp_hook_copy((*cfg).hook_response_trailer);
        if (*copy).hook_response_trailer.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_trailer_data.is_null() {
        (*copy).hook_response_trailer_data = htp_hook_copy((*cfg).hook_response_trailer_data);
        if (*copy).hook_response_trailer_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_response_complete.is_null() {
        (*copy).hook_response_complete = htp_hook_copy((*cfg).hook_response_complete);
        if (*copy).hook_response_complete.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_transaction_complete.is_null() {
        (*copy).hook_transaction_complete = htp_hook_copy((*cfg).hook_transaction_complete);
        if (*copy).hook_transaction_complete.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    if !(*cfg).hook_log.is_null() {
        (*copy).hook_log = htp_hook_copy((*cfg).hook_log);
        if (*copy).hook_log.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t;
        }
    }
    return copy;
}

/**
 * Destroy a configuration structure.
 *
 * @param[in] cfg
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_destroy(mut cfg: *mut htp_cfg_t) {
    if cfg.is_null() {
        return;
    }
    htp_hook_destroy((*cfg).hook_request_start);
    htp_hook_destroy((*cfg).hook_request_line);
    htp_hook_destroy((*cfg).hook_request_uri_normalize);
    htp_hook_destroy((*cfg).hook_request_header_data);
    htp_hook_destroy((*cfg).hook_request_headers);
    htp_hook_destroy((*cfg).hook_request_body_data);
    htp_hook_destroy((*cfg).hook_request_file_data);
    htp_hook_destroy((*cfg).hook_request_trailer);
    htp_hook_destroy((*cfg).hook_request_trailer_data);
    htp_hook_destroy((*cfg).hook_request_complete);
    htp_hook_destroy((*cfg).hook_response_start);
    htp_hook_destroy((*cfg).hook_response_line);
    htp_hook_destroy((*cfg).hook_response_header_data);
    htp_hook_destroy((*cfg).hook_response_headers);
    htp_hook_destroy((*cfg).hook_response_body_data);
    htp_hook_destroy((*cfg).hook_response_trailer);
    htp_hook_destroy((*cfg).hook_response_trailer_data);
    htp_hook_destroy((*cfg).hook_response_complete);
    htp_hook_destroy((*cfg).hook_transaction_complete);
    htp_hook_destroy((*cfg).hook_log);
    free(cfg as *mut libc::c_void);
}

/**
 * Retrieves user data associated with this configuration.
 *
 * @param[in] cfg
 * @return User data pointer, or NULL if not set.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_get_user_data(mut cfg: *mut htp_cfg_t) -> *mut libc::c_void {
    if cfg.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*cfg).user_data;
}

/**
 * Registers a callback that is invoked every time there is a log message with
 * severity equal and higher than the configured log level.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_log(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_util::htp_log_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_log,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut crate::src::htp_util::htp_log_t) -> libc::c_int>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Adds the built-in Multipart parser to the configuration. This parser will extract information
 * stored in request bodies, when they are in multipart/form-data format.
 *
 * @param[in] cfg
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_multipart_parser(mut cfg: *mut htp_cfg_t) {
    if cfg.is_null() {
        return;
    }
    htp_config_register_request_headers(
        cfg,
        Some(
            htp_ch_multipart_callback_request_headers
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_t,
                ) -> htp_status_t,
        ),
    );
}

/**
 * Registers a REQUEST_COMPLETE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_complete(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_complete,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_BODY_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_body_data(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_data_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_body_data,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_FILE_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_file_data(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_util::htp_file_data_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_file_data,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_util::htp_file_data_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_URI_NORMALIZE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_uri_normalize(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_uri_normalize,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_HEADER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_header_data(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_data_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_header_data,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_HEADERS callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_headers(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_headers,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_LINE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_line(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_line,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_START callback, which is invoked every time a new
 * request begins and before any parsing is done.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_start(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_start,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a HTP_REQUEST_TRAILER callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_trailer,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a REQUEST_TRAILER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer_data(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_data_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_request_trailer_data,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_BODY_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_body_data(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_data_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_body_data,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_COMPLETE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_complete(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_complete,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_HEADER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_header_data(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_data_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_header_data,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_HEADERS callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_headers(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_headers,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_LINE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_line(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_line,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_START callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_start(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_start,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_TRAILER callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_trailer,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a RESPONSE_TRAILER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer_data(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_data_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_response_trailer_data,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_data_t,
                ) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Registers a TRANSACTION_COMPLETE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_transaction_complete(
    mut cfg: *mut htp_cfg_t,
    mut callback_fn: Option<
        unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
    >,
) {
    if cfg.is_null() {
        return;
    }
    htp_hook_register(
        &mut (*cfg).hook_transaction_complete,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(_: *mut crate::src::htp_transaction::htp_tx_t) -> libc::c_int,
            >,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/**
 * Adds the built-in Urlencoded parser to the configuration. The parser will
 * parse query strings and request bodies with the appropriate MIME type.
 *
 * @param[in] cfg
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_urlencoded_parser(mut cfg: *mut htp_cfg_t) {
    if cfg.is_null() {
        return;
    }
    htp_config_register_request_line(
        cfg,
        Some(
            htp_ch_urlencoded_callback_request_line
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_t,
                ) -> htp_status_t,
        ),
    );
    htp_config_register_request_headers(
        cfg,
        Some(
            htp_ch_urlencoded_callback_request_headers
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_transaction::htp_tx_t,
                ) -> htp_status_t,
        ),
    );
}

/**
 * Enables or disables Multipart file extraction. This function can be invoked only
 * after a previous htp_config_set_tmpdir() invocation. Otherwise, the configuration
 * change will fail, and extraction will not be enabled. Disabled by default. Please
 * note that the built-in file extraction implementation uses synchronous I/O, which
 * means that it is not suitable for use in an event-driven container. There's an
 * upper limit to how many files can be created on the filesystem during a single
 * request. The limit exists in order to mitigate against a DoS attack with a
 * Multipart payload that contains hundreds and thousands of files (it's cheap for the
 * attacker to do this, but costly for the server to support it). The default limit
 * may be pretty conservative.
 *
 * @param[in] cfg
 * @param[in] extract_files 1 if you wish extraction to be enabled, 0 otherwise
 * @param[in] limit the maximum number of files allowed; use -1 to use the parser default.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_extract_request_files(
    mut cfg: *mut htp_cfg_t,
    mut extract_request_files: libc::c_int,
    mut limit: libc::c_int,
) -> htp_status_t {
    if cfg.is_null() {
        return -(1 as libc::c_int);
    }
    if (*cfg).tmpdir.is_null() {
        return -(1 as libc::c_int);
    }
    (*cfg).extract_request_files = extract_request_files;
    (*cfg).extract_request_files_limit = limit;
    return 1 as libc::c_int;
}

/**
 * Configures the maximum size of the buffer LibHTP will use when all data is not available
 * in the current buffer (e.g., a very long header line that might span several packets). This
 * limit is controlled by the hard_limit parameter. The soft_limit parameter is not implemented.
 *
 * @param[in] cfg
 * @param[in] soft_limit NOT IMPLEMENTED.
 * @param[in] hard_limit
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_field_limits(
    mut cfg: *mut htp_cfg_t,
    mut soft_limit: size_t,
    mut hard_limit: size_t,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).field_limit_soft = soft_limit;
    (*cfg).field_limit_hard = hard_limit;
}

/**
 * Configures the maximum memlimit LibHTP will pass to liblzma.
 *
 * @param[in] cfg
 * @param[in] memlimit
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_lzma_memlimit(
    mut cfg: *mut htp_cfg_t,
    mut memlimit: size_t,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).lzma_memlimit = memlimit;
}

/**
 * Configures the maximum compression bomb size LibHTP will decompress.
 *
 * @param[in] cfg
 * @param[in] bomblimit
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_compression_bomb_limit(
    mut cfg: *mut htp_cfg_t,
    mut bomblimit: size_t,
) {
    if cfg.is_null() {
        return;
    }
    if bomblimit > 2147483647 as libc::c_int as libc::c_ulong {
        (*cfg).compression_bomb_limit = 2147483647 as libc::c_int
    } else {
        (*cfg).compression_bomb_limit = bomblimit as int32_t
    };
}

/**
 * Configures the desired log level.
 *
 * @param[in] cfg
 * @param[in] log_level
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_log_level(
    mut cfg: *mut htp_cfg_t,
    mut log_level: htp_log_level_t,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).log_level = log_level;
}

/**
 * Enable or disable request HTTP Authentication parsing. Enabled by default.
 *
 * @param[in] cfg
 * @param[in] parse_request_auth
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_request_auth(
    mut cfg: *mut htp_cfg_t,
    mut parse_request_auth: libc::c_int,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).parse_request_auth = parse_request_auth;
}

/**
 * Enable or disable request cookie parsing. Enabled by default.
 *
 * @param[in] cfg
 * @param[in] parse_request_cookies
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_request_cookies(
    mut cfg: *mut htp_cfg_t,
    mut parse_request_cookies: libc::c_int,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).parse_request_cookies = parse_request_cookies;
}

/**
 * Controls whether compressed response bodies will be automatically decompressed.
 *
 * @param[in] cfg
 * @param[in] enabled set to 1 to enable decompression, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_response_decompression(
    mut cfg: *mut htp_cfg_t,
    mut enabled: libc::c_int,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).response_decompression_enabled = enabled;
}

/**
 * Configure desired server personality.
 *
 * @param[in] cfg
 * @param[in] personality
 * @return HTP_OK if the personality is supported, HTP_ERROR if it isn't.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_server_personality(
    mut cfg: *mut htp_cfg_t,
    mut personality: htp_server_personality_t,
) -> htp_status_t {
    if cfg.is_null() {
        return -(1 as libc::c_int);
    }
    match personality as libc::c_uint {
        0 => {
            (*cfg).parse_request_line = Some(
                htp_parse_request_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_request_header = Some(
                htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            (*cfg).parse_response_line = Some(
                htp_parse_response_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_response_header = Some(
                htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            )
        }
        1 => {
            (*cfg).parse_request_line = Some(
                htp_parse_request_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_request_header = Some(
                htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            (*cfg).parse_response_line = Some(
                htp_parse_response_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_response_header = Some(
                htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            htp_config_set_backslash_convert_slashes(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
        }
        2 => {
            (*cfg).parse_request_line = Some(
                htp_parse_request_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_request_header = Some(
                htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            (*cfg).parse_response_line = Some(
                htp_parse_response_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_response_header = Some(
                htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            htp_config_set_backslash_convert_slashes(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_convert_lowercase(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_utf8_convert_bestfit(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                HTP_DECODER_DEFAULTS,
                HTP_UNWANTED_IGNORE,
            );
        }
        9 => {
            (*cfg).parse_request_line = Some(
                htp_parse_request_line_apache_2_2
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_request_header = Some(
                htp_process_request_header_apache_2_2
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            (*cfg).parse_response_line = Some(
                htp_parse_response_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_response_header = Some(
                htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            htp_config_set_backslash_convert_slashes(cfg, HTP_DECODER_URL_PATH, 0 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH, 0 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH, 0 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                HTP_DECODER_URL_PATH,
                HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_url_encoding_invalid_unwanted(
                cfg,
                HTP_DECODER_URL_PATH,
                HTP_UNWANTED_400,
            );
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH, HTP_UNWANTED_IGNORE);
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                HTP_DECODER_DEFAULTS,
                HTP_UNWANTED_400,
            );
        }
        5 => {
            (*cfg).parse_request_line = Some(
                htp_parse_request_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_request_header = Some(
                htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            (*cfg).parse_response_line = Some(
                htp_parse_response_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_response_header = Some(
                htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            htp_config_set_backslash_convert_slashes(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH, 0 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                HTP_DECODER_URL_PATH,
                HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH, HTP_UNWANTED_IGNORE);
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                HTP_DECODER_DEFAULTS,
                HTP_UNWANTED_IGNORE,
            );
        }
        6 => {
            (*cfg).parse_request_line = Some(
                htp_parse_request_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_request_header = Some(
                htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            (*cfg).parse_response_line = Some(
                htp_parse_response_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_response_header = Some(
                htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            htp_config_set_backslash_convert_slashes(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                HTP_DECODER_URL_PATH,
                HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_u_encoding_unwanted(cfg, HTP_DECODER_URL_PATH, HTP_UNWANTED_400);
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH, HTP_UNWANTED_400);
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                HTP_DECODER_DEFAULTS,
                HTP_UNWANTED_IGNORE,
            );
        }
        7 | 8 => {
            (*cfg).parse_request_line = Some(
                htp_parse_request_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_request_header = Some(
                htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            (*cfg).parse_response_line = Some(
                htp_parse_response_line_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            );
            (*cfg).process_response_header = Some(
                htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                        _: *mut libc::c_uchar,
                        _: size_t,
                    ) -> htp_status_t,
            );
            htp_config_set_backslash_convert_slashes(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH, 1 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                HTP_DECODER_URL_PATH,
                HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_url_encoding_invalid_unwanted(
                cfg,
                HTP_DECODER_URL_PATH,
                HTP_UNWANTED_400,
            );
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH, HTP_UNWANTED_400);
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                HTP_DECODER_DEFAULTS,
                HTP_UNWANTED_IGNORE,
            );
        }
        _ => return -(1 as libc::c_int),
    }
    // Remember the personality
    (*cfg).server_personality = personality;
    return 1 as libc::c_int;
}

/**
 * Configures the path where temporary files should be stored. Must be set
 * in order to use the Multipart file extraction functionality.
 *
 * @param[in] cfg
 * @param[in] tmpdir
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_tmpdir(
    mut cfg: *mut htp_cfg_t,
    mut tmpdir: *mut libc::c_char,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).tmpdir = tmpdir;
}

/**
 * Configures whether transactions will be automatically destroyed once they
 * are processed and all callbacks invoked. This option is appropriate for
 * programs that process transactions as they are processed.
 *
 * @param[in] cfg
 * @param[in] tx_auto_destroy
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_tx_auto_destroy(
    mut cfg: *mut htp_cfg_t,
    mut tx_auto_destroy: libc::c_int,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).tx_auto_destroy = tx_auto_destroy;
}

/**
 * Associates provided opaque user data with the configuration.
 *
 * @param[in] cfg
 * @param[in] user_data
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_user_data(
    mut cfg: *mut htp_cfg_t,
    mut user_data: *mut libc::c_void,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).user_data = user_data;
}
unsafe extern "C" fn convert_to_0_or_1(mut b: libc::c_int) -> libc::c_int {
    if b != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

/**
 * Configures a best-fit map, which is used whenever characters longer than one byte
 * need to be converted to a single-byte. By default a Windows 1252 best-fit map is used.
 * The map is an list of triplets, the first 2 bytes being an UCS-2 character to map from,
 * and the third byte being the single byte to map to. Make sure that your map contains
 * the mappings to cover the full-width and half-width form characters (U+FF00-FFEF). The
 * last triplet in the map must be all zeros (3 NUL bytes).
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] map
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_bestfit_map(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut map: *mut libc::c_void,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].bestfit_map = map as *mut libc::c_uchar;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].bestfit_map = map as *mut libc::c_uchar;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Sets the replacement character that will be used to in the lossy best-fit
 * mapping from multi-byte to single-byte streams. The question mark character
 * is used as the default replacement byte.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] replacement_byte
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_bestfit_replacement_byte(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut b: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte = b as libc::c_uchar;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].bestfit_replacement_byte = b as libc::c_uchar;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures how the server handles to invalid URL encoding.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] handling
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_url_encoding_invalid_handling(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut handling: htp_url_encoding_handling_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_handling = handling;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].url_encoding_invalid_handling = handling;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures the handling of raw NUL bytes. If enabled, raw NUL terminates strings.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_raw_terminates(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].nul_raw_terminates = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_raw_terminates = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures how the server reacts to encoded NUL bytes. Some servers will stop at
 * at NUL, while some will respond with 400 or 404. When the termination option is not
 * used, the NUL byte will remain in the path.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_encoded_terminates(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].nul_encoded_terminates = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_encoded_terminates = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures whether %u-encoded sequences are decoded. Such sequences
 * will be treated as invalid URL encoding if decoding is not desirable.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_u_encoding_decode(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_decode = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].u_encoding_decode = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures whether backslash characters are treated as path segment separators. They
 * are not on Unix systems, but are on Windows systems. If this setting is enabled, a path
 * such as "/one\two/three" will be converted to "/one/two/three". Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_backslash_convert_slashes(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].backslash_convert_slashes = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].backslash_convert_slashes = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures whether encoded path segment separators will be decoded. Apache does not do
 * this by default, but IIS does. If enabled, a path such as "/one%2ftwo" will be normalized
 * to "/one/two". If the backslash_separators option is also enabled, encoded backslash
 * characters will be converted too (and subsequently normalized to forward slashes). Implemented
 * only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_decode(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_decode = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].path_separators_decode = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures whether consecutive path segment separators will be compressed. When enabled, a path
 * such as "/one//two" will be normalized to "/one/two". Backslash conversion and path segment separator
 * decoding are carried out before compression. For example, the path "/one\\/two\/%5cthree/%2f//four"
 * will be converted to "/one/two/three/four" (assuming all 3 options are enabled). Implemented only for
 * HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_compress(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_compress = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].path_separators_compress = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures whether plus characters are converted to spaces when decoding URL-encoded strings. This
 * is appropriate to do for parameters, but not for URLs. Only applies to contexts where decoding
 * is taking place.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_plusspace_decode(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].plusspace_decode = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].plusspace_decode = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures whether input data will be converted to lowercase. Useful when set on the
 * HTP_DECODER_URL_PATH context, in order to handle servers with case-insensitive filesystems.
 * Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_convert_lowercase(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].convert_lowercase = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].convert_lowercase = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Controls whether the data should be treated as UTF-8 and converted to a single-byte
 * stream using best-fit mapping. Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_utf8_convert_bestfit(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut enabled: libc::c_int,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].utf8_convert_bestfit = convert_to_0_or_1(enabled);
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].utf8_convert_bestfit = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures reaction to %u-encoded sequences in input data.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_u_encoding_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted = unwanted;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].u_encoding_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Controls reaction to raw control characters in the data.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_control_chars_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted = unwanted;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].u_encoding_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures how the server reacts to invalid URL encoding.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_url_encoding_invalid_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_unwanted = unwanted;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].url_encoding_invalid_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures reaction to encoded NUL bytes in input data.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_encoded_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].nul_encoded_unwanted = unwanted;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_encoded_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures how the server reacts to raw NUL bytes. Some servers will terminate
 * path at NUL, while some will respond with 400 or 404. When the termination option
 * is not used, the NUL byte will remain in the data.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_raw_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].nul_raw_unwanted = unwanted;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_raw_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures reaction to encoded path separator characters (e.g., %2f). Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_encoded_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_encoded_unwanted = unwanted;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].path_separators_encoded_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures how the server reacts to invalid UTF-8 characters. This setting does
 * not affect path normalization; it only controls what response status will be expect for
 * a request that contains invalid UTF-8 characters. Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_utf8_invalid_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].utf8_invalid_unwanted = unwanted;
    if ctx as libc::c_uint == HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].utf8_invalid_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/**
 * Configures how the server reacts to leading whitespace on the request line.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_requestline_leading_whitespace_unwanted(
    mut cfg: *mut htp_cfg_t,
    mut ctx: htp_decoder_ctx_t,
    mut unwanted: htp_unwanted_t,
) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint {
        return;
    }
    (*cfg).requestline_leading_whitespace_unwanted = unwanted;
}

/* *
 * Configures many layers of compression we try to decompress.
 *
 * @param[in] cfg
 * @param[in] limit 0 disables limit
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_response_decompression_layer_limit(
    mut cfg: *mut htp_cfg_t,
    mut limit: libc::c_int,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).response_decompression_layer_limit = limit;
}
