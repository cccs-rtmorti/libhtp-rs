use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn htp_list_array_push(
        l: *mut crate::src::htp_list::htp_list_array_t,
        e: *mut libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_list_array_size(l: *const crate::src::htp_list::htp_list_array_t) -> size_t;
    #[no_mangle]
    fn bstr_cmp_c_nocase(b: *const bstr, cstr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_cmp_c_nocasenorzero(b: *const bstr, cstr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_cmp_mem_nocase(b: *const bstr, data: *const libc::c_void, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn bstr_cmp_nocase(b1: *const bstr, b2: *const bstr) -> libc::c_int;
    #[no_mangle]
    fn bstr_dup(b: *const bstr) -> *mut bstr;
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn bstr_util_cmp_mem(
        data1: *const libc::c_void,
        len1: size_t,
        data2: *const libc::c_void,
        len2: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn bstr_util_mem_index_of_c_nocase(
        data: *const libc::c_void,
        len: size_t,
        cstr: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn bstr_wrap_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn htp_config_destroy(cfg: *mut crate::src::htp_config::htp_cfg_t);
    #[no_mangle]
    fn htp_gzip_decompressor_create(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        format: crate::src::htp_decompressors::htp_content_encoding_t,
    ) -> *mut crate::src::htp_decompressors::htp_decompressor_t;
    #[no_mangle]
    fn htp_hook_destroy(hook: *mut crate::src::htp_hooks::htp_hook_t);
    #[no_mangle]
    fn htp_hook_register(
        hook: *mut *mut crate::src::htp_hooks::htp_hook_t,
        callback_fn: htp_callback_fn_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_hook_run_all(
        hook: *mut crate::src::htp_hooks::htp_hook_t,
        user_data: *mut libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_table_add(
        table: *mut crate::src::htp_table::htp_table_t,
        key: *const bstr,
        element: *const libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_table_addk(
        table: *mut crate::src::htp_table::htp_table_t,
        key: *const bstr,
        element: *const libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_table_create(size: size_t) -> *mut crate::src::htp_table::htp_table_t;
    #[no_mangle]
    fn htp_table_destroy(table: *mut crate::src::htp_table::htp_table_t);
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
    fn htp_table_get_mem(
        table: *const crate::src::htp_table::htp_table_t,
        key: *const libc::c_void,
        key_len: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_table_size(table: *const crate::src::htp_table::htp_table_t) -> size_t;
    #[no_mangle]
    fn htp_mpartp_destroy(parser: *mut crate::src::htp_multipart::htp_mpartp_t);
    #[no_mangle]
    fn htp_urlenp_destroy(urlenp: *mut crate::src::htp_urlencoded::htp_urlenp_t);
    #[no_mangle]
    fn htp_uri_free(uri: *mut crate::src::htp_util::htp_uri_t);
    #[no_mangle]
    fn htp_connp_tx_remove(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        tx: *mut htp_tx_t,
    );
    #[no_mangle]
    fn htp_conn_remove_tx(
        conn: *mut crate::src::htp_connection::htp_conn_t,
        tx: *const htp_tx_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_uri_alloc() -> *mut crate::src::htp_util::htp_uri_t;
    #[no_mangle]
    fn htp_req_run_hook_body_data(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        d: *mut htp_tx_data_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_log(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        file: *const libc::c_char,
        line: libc::c_int,
        level: crate::src::htp_util::htp_log_level_t,
        code: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn htp_res_run_hook_body_data(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        d: *mut htp_tx_data_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_IDLE(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_IGNORE_DATA_AFTER_HTTP_0_9(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_CONNECT_CHECK(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_req_receiver_finalize_clear(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_authorization(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_parse_cookies_v0(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_parse_ct_header(header: *mut bstr, ct: *mut *mut bstr) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_header_hostport(
        authority: *mut bstr,
        hostname: *mut *mut bstr,
        port: *mut *mut bstr,
        port_number: *mut libc::c_int,
        flags: *mut uint64_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_content_length(
        b: *mut bstr,
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> int64_t;
    #[no_mangle]
    fn htp_connp_REQ_FINALIZE(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_PROTOCOL(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_validate_hostname(hostname: *mut bstr) -> libc::c_int;
    #[no_mangle]
    fn htp_normalize_parsed_uri(
        tx: *mut htp_tx_t,
        parsed_uri_incomplete: *mut crate::src::htp_util::htp_uri_t,
        parsed_uri: *mut crate::src::htp_util::htp_uri_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_parse_uri(
        input: *mut bstr,
        uri: *mut *mut crate::src::htp_util::htp_uri_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_parse_uri_hostport(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
        input: *mut bstr,
        uri: *mut crate::src::htp_util::htp_uri_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_connp_REQ_LINE(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_IDLE(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_res_receiver_finalize_clear(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_LINE_complete(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_LINE(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE(
        connp: *mut crate::src::htp_connection_parser::htp_connp_t,
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
#[derive(Copy, Clone, PartialEq, Debug)]
// A collection of possible data sources.
pub enum htp_data_source_t {
    /** Embedded in the URL. */
    HTP_SOURCE_URL,
    /** Transported in the query string. */
    HTP_SOURCE_QUERY_STRING,
    /** Cookies. */
    HTP_SOURCE_COOKIE,
    /** Transported in the request body. */
    HTP_SOURCE_BODY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
// A collection of unique parser IDs.
pub enum htp_parser_id_t {
    /** application/x-www-form-urlencoded parser. */
    HTP_PARSER_URLENCODED,
    /** multipart/form-data parser. */
    HTP_PARSER_MULTIPART,
}

/* *
 * Represents a single request parameter.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_param_t {
    /** Parameter name. */
    pub name: *mut bstr,
    /** Parameter value. */
    pub value: *mut bstr,
    /** Source of the parameter, for example HTP_SOURCE_QUERY_STRING. */
    pub source: htp_data_source_t,
    /** Type of the data structure referenced below. */
    pub parser_id: htp_parser_id_t,
    /**
     * Pointer to the parser data structure that contains
     * complete information about the parameter. Can be NULL.
     */
    pub parser_data: *mut libc::c_void,
}
pub type bstr = crate::src::bstr::bstr_t;

/* *
 * This structure is used to pass transaction data (for example
 * request and response body buffers) to callbacks.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_tx_data_t {
    /** Transaction pointer. */
    pub tx: *mut htp_tx_t,
    /** Pointer to the data buffer. */
    pub data: *const libc::c_uchar,
    /** Buffer length. */
    pub len: size_t,
    /**
     * Indicator if this chunk of data is the last in the series. Currently
     * used only by REQUEST_HEADER_DATA, REQUEST_TRAILER_DATA, RESPONSE_HEADER_DATA,
     * and RESPONSE_TRAILER_DATA callbacks.
     */
    pub is_last: libc::c_int,
}

/**
 * Enumerates the possible request and response body codings.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_transfer_coding_t {
    /** Body coding not determined yet. */
    HTP_CODING_UNKNOWN,
    /** No body. */
    HTP_CODING_NO_BODY,
    /** Identity coding is used, which means that the body was sent as is. */
    HTP_CODING_IDENTITY,
    /** Chunked encoding. */
    HTP_CODING_CHUNKED,
    /** We could not recognize the encoding. */
    HTP_CODING_INVALID,
}

/* *
 * Represents a single HTTP transaction, which is a combination of a request and a response.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_tx_t {
    /** The connection parser associated with this transaction. */
    pub connp: *mut crate::src::htp_connection_parser::htp_connp_t,
    /** The connection to which this transaction belongs. */
    pub conn: *mut crate::src::htp_connection::htp_conn_t,
    /** The configuration structure associated with this transaction. */
    pub cfg: *mut crate::src::htp_config::htp_cfg_t,
    /**
     * Is the configuration structure shared with other transactions or connections? If
     * this field is set to HTP_CONFIG_PRIVATE, the transaction owns the configuration.
     */
    pub is_config_shared: libc::c_int,
    /** The user data associated with this transaction. */
    pub user_data: *mut libc::c_void,

    // Request fields
    /** Contains a count of how many empty lines were skipped before the request line. */
    pub request_ignored_lines: libc::c_uint,
    /** The first line of this request. */
    pub request_line: *mut bstr,
    /** Request method. */
    pub request_method: *mut bstr,
    /** Request method, as number. Available only if we were able to recognize the request method. */
    pub request_method_number: libc::c_uint,
    /**
     * Request URI, raw, as given to us on the request line. This field can take different forms,
     * for example authority for CONNECT methods, absolute URIs for proxy requests, and the query
     * string when one is provided. Use htp_tx_t::parsed_uri if you need to access to specific
     * URI elements. Can be NULL if the request line contains only a request method (which is
     * an extreme case of HTTP/0.9, but passes in practice.
     */
    pub request_uri: *mut bstr,
    /** Request protocol, as text. Can be NULL if no protocol was specified. */
    pub request_protocol: *mut bstr,
    /**
     * Protocol version as a number. Multiply the high version number by 100, then add the low
     * version number. You should prefer to work the pre-defined HTP_PROTOCOL_* constants.
     */
    pub request_protocol_number: libc::c_int,
    /**
     * Is this request using HTTP/0.9? We need a separate field for this purpose because
     * the protocol version alone is not sufficient to determine if HTTP/0.9 is used. For
     * example, if you submit "GET / HTTP/0.9" to Apache, it will not treat the request
     * as HTTP/0.9.
     */
    pub is_protocol_0_9: libc::c_int,
    /**
     * This structure holds the individual components parsed out of the request URI, with
     * appropriate normalization and transformation applied, per configuration. No information
     * is added. In extreme cases when no URI is provided on the request line, all fields
     * will be NULL. (Well, except for port_number, which will be -1.) To inspect raw data, use
     * htp_tx_t::request_uri or htp_tx_t::parsed_uri_raw.
     */
    pub parsed_uri: *mut crate::src::htp_util::htp_uri_t,
    /**
     * This structure holds the individual components parsed out of the request URI, but
     * without any modification. The purpose of this field is to allow you to look at the data as it
     * was supplied on the request line. Fields can be NULL, depending on what data was supplied.
     * The port_number field is always -1.
     */
    pub parsed_uri_raw: *mut crate::src::htp_util::htp_uri_t,
    /* HTTP 1.1 RFC
     *
     * 4.3 Message Body
     *
     * The message-body (if any) of an HTTP message is used to carry the
     * entity-body associated with the request or response. The message-body
     * differs from the entity-body only when a transfer-coding has been
     * applied, as indicated by the Transfer-Encoding header field (section
     * 14.41).
     *
     *     message-body = entity-body
     *                  | <entity-body encoded as per Transfer-Encoding>
     */
    /**
     * The length of the request message-body. In most cases, this value
     * will be the same as request_entity_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * request_message_len contains the length of the request body as it
     * has been seen over TCP; request_entity_len contains length after
     * de-chunking and decompression.
     */
    pub request_message_len: int64_t,
    /**
     * The length of the request entity-body. In most cases, this value
     * will be the same as request_message_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * request_message_len contains the length of the request body as it
     * has been seen over TCP; request_entity_len contains length after
     * de-chunking and decompression.
     */
    pub request_entity_len: int64_t,
    /** Parsed request headers. */
    pub request_headers: *mut crate::src::htp_table::htp_table_t,
    /**
     * Request transfer coding. Can be one of HTP_CODING_UNKNOWN (body presence not
     * determined yet), HTP_CODING_IDENTITY, HTP_CODING_CHUNKED, HTP_CODING_NO_BODY,
     * and HTP_CODING_UNRECOGNIZED.
     */
    pub request_transfer_coding: htp_transfer_coding_t,
    /** Request body compression. */
    pub request_content_encoding: crate::src::htp_decompressors::htp_content_encoding_t,
    /**
     * This field contain the request content type when that information is
     * available in request headers. The contents of the field will be converted
     * to lowercase and any parameters (e.g., character set information) removed.
     */
    pub request_content_type: *mut bstr,
    /**
     * Contains the value specified in the Content-Length header. The value of this
     * field will be -1 from the beginning of the transaction and until request
     * headers are processed. It will stay -1 if the C-L header was not provided,
     * or if the value in it cannot be parsed.
     */
    pub request_content_length: int64_t,
    /**
     * Transaction-specific REQUEST_BODY_DATA hook. Behaves as
     * the configuration hook with the same name.
     */
    pub hook_request_body_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Transaction-specific RESPONSE_BODY_DATA hook. Behaves as
     * the configuration hook with the same name.
     */
    pub hook_response_body_data: *mut crate::src::htp_hooks::htp_hook_t,
    /**
     * Query string URLENCODED parser. Available only
     * when the query string is not NULL and not empty.
     */
    pub request_urlenp_query: *mut crate::src::htp_urlencoded::htp_urlenp_t,
    /**
     * Request body URLENCODED parser. Available only when the request body is in the
     * application/x-www-form-urlencoded format and the parser was configured to run.
     */
    pub request_urlenp_body: *mut crate::src::htp_urlencoded::htp_urlenp_t,
    /**
     * Request body MULTIPART parser. Available only when the body is in the
     * multipart/form-data format and the parser was configured to run.
     */
    pub request_mpartp: *mut crate::src::htp_multipart::htp_mpartp_t,
    /** Request parameters. */
    pub request_params: *mut crate::src::htp_table::htp_table_t,
    /** Request cookies */
    pub request_cookies: *mut crate::src::htp_table::htp_table_t,
    /** Authentication type used in the request. */
    pub request_auth_type: htp_auth_type_t,
    /** Authentication username. */
    pub request_auth_username: *mut bstr,
    /** Authentication password. Available only when htp_tx_t::request_auth_type is HTP_AUTH_BASIC. */
    pub request_auth_password: *mut bstr,
    /**
     * Request hostname. Per the RFC, the hostname will be taken from the Host header
     * when available. If the host information is also available in the URI, it is used
     * instead of whatever might be in the Host header. Can be NULL. This field does
     * not contain port information.
     */
    pub request_hostname: *mut bstr,
    /**
     * Request port number, if presented. The rules for htp_tx_t::request_host apply. Set to
     * -1 by default.
     */
    pub request_port_number: libc::c_int,

    // Response fields
    /** How many empty lines did we ignore before reaching the status line? */
    pub response_ignored_lines: libc::c_uint,
    /** Response line. */
    pub response_line: *mut bstr,
    /** Response protocol, as text. Can be NULL. */
    pub response_protocol: *mut bstr,
    /**
     * Response protocol as number. Available only if we were able to parse the protocol version,
     * HTP_PROTOCOL_INVALID otherwise. HTP_PROTOCOL_UNKNOWN until parsing is attempted.
     */
    pub response_protocol_number: libc::c_int,
    /**
     * Response status code, as text. Starts as NULL and can remain NULL on
     * an invalid response that does not specify status code.
     */
    pub response_status: *mut bstr,
    /**
     * Response status code, available only if we were able to parse it, HTP_STATUS_INVALID
     * otherwise. HTP_STATUS_UNKNOWN until parsing is attempted.
     */
    pub response_status_number: libc::c_int,
    /**
     * This field is set by the protocol decoder with it thinks that the
     * backend server will reject a request with a particular status code.
     */
    pub response_status_expected_number: libc::c_int,
    /** The message associated with the response status code. Can be NULL. */
    pub response_message: *mut bstr,
    /** Have we seen the server respond with a 100 response? */
    pub seen_100continue: libc::c_int,
    /** Parsed response headers. Contains instances of htp_header_t. */
    pub response_headers: *mut crate::src::htp_table::htp_table_t,

    /* HTTP 1.1 RFC
     *
     * 4.3 Message Body
     *
     * The message-body (if any) of an HTTP message is used to carry the
     * entity-body associated with the request or response. The message-body
     * differs from the entity-body only when a transfer-coding has been
     * applied, as indicated by the Transfer-Encoding header field (section
     * 14.41).
     *
     *     message-body = entity-body
     *                  | <entity-body encoded as per Transfer-Encoding>
     */
    /**
     * The length of the response message-body. In most cases, this value
     * will be the same as response_entity_len. The values will be different
     * if response compression or chunking were applied. In that case,
     * response_message_len contains the length of the response body as it
     * has been seen over TCP; response_entity_len contains the length after
     * de-chunking and decompression.
     */
    pub response_message_len: int64_t,
    /**
     * The length of the response entity-body. In most cases, this value
     * will be the same as response_message_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * response_message_len contains the length of the response body as it
     * has been seen over TCP; response_entity_len contains length after
     * de-chunking and decompression.
     */
    pub response_entity_len: int64_t,
    /**
     * Contains the value specified in the Content-Length header. The value of this
     * field will be -1 from the beginning of the transaction and until response
     * headers are processed. It will stay -1 if the C-L header was not provided,
     * or if the value in it cannot be parsed.
     */
    pub response_content_length: int64_t,
    /**
     * Response transfer coding, which indicates if there is a response body,
     * and how it is transported (e.g., as-is, or chunked).
     */
    pub response_transfer_coding: htp_transfer_coding_t,
    /**
     * Response body compression, which indicates if compression is used
     * for the response body. This field is an interpretation of the information
     * available in response headers.
     */
    pub response_content_encoding: crate::src::htp_decompressors::htp_content_encoding_t,
    /**
     * Response body compression processing information, which is related to how
     * the library is going to process (or has processed) a response body. Changing
     * this field mid-processing can influence library actions. For example, setting
     * this field to HTP_COMPRESSION_NONE in a RESPONSE_HEADERS callback will prevent
     * decompression.
     */
    pub response_content_encoding_processing: crate::src::htp_decompressors::htp_content_encoding_t,
    /**
     * This field will contain the response content type when that information
     * is available in response headers. The contents of the field will be converted
     * to lowercase and any parameters (e.g., character set information) removed.
     */
    pub response_content_type: *mut bstr,

    // Common fields
    /**
     * Parsing flags; a combination of: HTP_REQUEST_INVALID_T_E, HTP_INVALID_FOLDING,
     * HTP_REQUEST_SMUGGLING, HTP_MULTI_PACKET_HEAD, and HTP_FIELD_UNPARSEABLE.
     */
    pub flags: uint64_t,
    /** Request progress. */
    pub request_progress: htp_tx_req_progress_t,
    /** Response progress. */
    pub response_progress: htp_tx_res_progress_t,
    /** Transaction index on the connection. */
    pub index: size_t,
    /** Total repetitions for headers in request. */
    pub req_header_repetitions: uint16_t,
    /** Total repetitions for headers in response. */
    pub res_header_repetitions: uint16_t,
}

/* *
 * Possible states of a progressing transaction. Internally, progress will change
 * to the next state when the processing activities associated with that state
 * begin. For example, when we start to process request line bytes, the request
 * state will change from HTP_REQUEST_NOT_STARTED to HTP_REQUEST_LINE.*
 */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum htp_tx_res_progress_t {
    HTP_RESPONSE_NOT_STARTED,
    HTP_RESPONSE_LINE,
    HTP_RESPONSE_HEADERS,
    HTP_RESPONSE_BODY,
    HTP_RESPONSE_TRAILER,
    HTP_RESPONSE_COMPLETE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum htp_tx_req_progress_t {
    HTP_REQUEST_NOT_STARTED,
    HTP_REQUEST_LINE,
    HTP_REQUEST_HEADERS,
    HTP_REQUEST_BODY,
    HTP_REQUEST_TRAILER,
    HTP_REQUEST_COMPLETE,
}

/**
 * Enumerates the possible values for authentication type.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_auth_type_t {
    /**
     * This is the default value that is used before
     * the presence of authentication is determined (e.g.,
     * before request headers are seen).
     */
    HTP_AUTH_UNKNOWN,
    /** No authentication. */
    HTP_AUTH_NONE,
    /** HTTP Basic authentication used. */
    HTP_AUTH_BASIC,
    /** HTTP Digest authentication used. */
    HTP_AUTH_DIGEST,
    /** Unrecognized authentication method. */
    HTP_AUTH_UNRECOGNIZED = 9,
}

pub type htp_time_t = libc::timeval;

/* *
 * Represents a single request or response header.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_header_t {
    /** Header name. */
    pub name: *mut bstr,
    /** Header value. */
    pub value: *mut bstr,
    /** Parsing flags; a combination of: HTP_FIELD_INVALID, HTP_FIELD_FOLDED, HTP_FIELD_REPEATED. */
    pub flags: uint64_t,
}
pub type htp_callback_fn_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
pub type htp_alloc_strategy_t = libc::c_uint;
pub const HTP_ALLOC_REUSE: htp_alloc_strategy_t = 2;
pub const HTP_ALLOC_COPY: htp_alloc_strategy_t = 1;

unsafe extern "C" fn copy_or_wrap_mem(
    mut data: *const libc::c_void,
    mut len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> *mut bstr {
    if data == 0 as *mut libc::c_void {
        return 0 as *mut bstr;
    }
    if alloc as libc::c_uint == HTP_ALLOC_REUSE as libc::c_int as libc::c_uint {
        return bstr_wrap_mem(data, len);
    } else {
        return bstr_dup_mem(data, len);
    };
}

/**
 * Creates a new transaction structure.
 *
 * @param[in] connp Connection parser pointer. Must not be NULL.
 * @return The newly created transaction, or NULL on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_create(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) -> *mut htp_tx_t {
    if connp.is_null() {
        return 0 as *mut htp_tx_t;
    }
    let mut tx: *mut htp_tx_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_tx_t>() as libc::c_ulong,
    ) as *mut htp_tx_t;
    if tx.is_null() {
        return 0 as *mut htp_tx_t;
    }
    (*tx).connp = connp;
    (*tx).conn = (*connp).conn;
    (*tx).index = htp_list_array_size((*(*tx).conn).transactions);
    (*tx).cfg = (*connp).cfg;
    (*tx).is_config_shared = 1 as libc::c_int;
    // Request fields.
    (*tx).request_progress = htp_tx_req_progress_t::HTP_REQUEST_NOT_STARTED;
    (*tx).request_protocol_number = -(1 as libc::c_int);
    (*tx).request_content_length = -(1 as libc::c_int) as int64_t;
    (*tx).parsed_uri_raw = htp_uri_alloc();
    if (*tx).parsed_uri_raw.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    (*tx).request_headers = htp_table_create(32 as libc::c_int as size_t);
    if (*tx).request_headers.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    (*tx).request_params = htp_table_create(32 as libc::c_int as size_t);
    if (*tx).request_params.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    // Response fields.
    (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_NOT_STARTED;
    (*tx).response_status = 0 as *mut bstr;
    (*tx).response_status_number = 0 as libc::c_int;
    (*tx).response_protocol_number = -(1 as libc::c_int);
    (*tx).response_content_length = -(1 as libc::c_int) as int64_t;
    (*tx).response_headers = htp_table_create(32 as libc::c_int as size_t);
    if (*tx).response_headers.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    htp_list_array_push((*(*tx).conn).transactions, tx as *mut libc::c_void);
    return tx;
}

/**
 * Destroys the supplied transaction.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_destroy(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    if htp_tx_is_complete(tx) == 0 {
        return -(1 as libc::c_int);
    }
    htp_tx_destroy_incomplete(tx);
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn htp_tx_destroy_incomplete(mut tx: *mut htp_tx_t) {
    if tx.is_null() {
        return;
    }
    // Disconnect transaction from other structures.
    htp_conn_remove_tx((*tx).conn, tx);
    htp_connp_tx_remove((*tx).connp, tx);
    // Request fields.
    bstr_free((*tx).request_line);
    bstr_free((*tx).request_method);
    bstr_free((*tx).request_uri);
    bstr_free((*tx).request_protocol);
    bstr_free((*tx).request_content_type);
    bstr_free((*tx).request_hostname);
    htp_uri_free((*tx).parsed_uri_raw);
    htp_uri_free((*tx).parsed_uri);
    bstr_free((*tx).request_auth_username);
    bstr_free((*tx).request_auth_password);
    // Request_headers.
    if !(*tx).request_headers.is_null() {
        let mut h: *mut htp_header_t = 0 as *mut htp_header_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_table_size((*tx).request_headers);
        while i < n {
            h = htp_table_get_index((*tx).request_headers, i, 0 as *mut *mut bstr)
                as *mut htp_header_t;
            bstr_free((*h).name);
            bstr_free((*h).value);
            free(h as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        htp_table_destroy((*tx).request_headers);
    }
    // Request parsers.
    htp_urlenp_destroy((*tx).request_urlenp_query);
    htp_urlenp_destroy((*tx).request_urlenp_body);
    htp_mpartp_destroy((*tx).request_mpartp);
    // Request parameters.
    let mut param: *mut htp_param_t = 0 as *mut htp_param_t;
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    let mut n_0: size_t = htp_table_size((*tx).request_params);
    while i_0 < n_0 {
        param =
            htp_table_get_index((*tx).request_params, i_0, 0 as *mut *mut bstr) as *mut htp_param_t;
        bstr_free((*param).name);
        bstr_free((*param).value);
        free(param as *mut libc::c_void);
        i_0 = i_0.wrapping_add(1)
    }
    htp_table_destroy((*tx).request_params);
    // Request cookies.
    if !(*tx).request_cookies.is_null() {
        let mut b: *mut bstr = 0 as *mut bstr;
        let mut i_1: size_t = 0 as libc::c_int as size_t;
        let mut n_1: size_t = htp_table_size((*tx).request_cookies);
        while i_1 < n_1 {
            b = htp_table_get_index((*tx).request_cookies, i_1, 0 as *mut *mut bstr) as *mut bstr;
            bstr_free(b);
            i_1 = i_1.wrapping_add(1)
        }
        htp_table_destroy((*tx).request_cookies);
    }
    htp_hook_destroy((*tx).hook_request_body_data);
    // Response fields.
    bstr_free((*tx).response_line);
    bstr_free((*tx).response_protocol);
    bstr_free((*tx).response_status);
    bstr_free((*tx).response_message);
    bstr_free((*tx).response_content_type);
    // Destroy response headers.
    if !(*tx).response_headers.is_null() {
        let mut h_0: *mut htp_header_t = 0 as *mut htp_header_t;
        let mut i_2: size_t = 0 as libc::c_int as size_t;
        let mut n_2: size_t = htp_table_size((*tx).response_headers);
        while i_2 < n_2 {
            h_0 = htp_table_get_index((*tx).response_headers, i_2, 0 as *mut *mut bstr)
                as *mut htp_header_t;
            bstr_free((*h_0).name);
            bstr_free((*h_0).value);
            free(h_0 as *mut libc::c_void);
            i_2 = i_2.wrapping_add(1)
        }
        htp_table_destroy((*tx).response_headers);
    }
    // If we're using a private configuration structure, destroy it.
    if (*tx).is_config_shared == 0 as libc::c_int {
        htp_config_destroy((*tx).cfg);
    }
    free(tx as *mut libc::c_void);
}

/**
 * Determines if the transaction used a shared configuration structure. See the
 * documentation for htp_tx_set_config() for more information why you might want
 * to know that.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_CFG_SHARED or HTP_CFG_PRIVATE.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_get_is_config_shared(mut tx: *const htp_tx_t) -> libc::c_int {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    return (*tx).is_config_shared;
}

/**
 * Returns the user data associated with this transaction.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return A pointer to user data or NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_get_user_data(mut tx: *const htp_tx_t) -> *mut libc::c_void {
    if tx.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*tx).user_data;
}

/**
 * Sets the configuration that is to be used for this transaction. If the
 * second parameter is set to HTP_CFG_PRIVATE, the transaction will adopt
 * the configuration structure and destroy it when appropriate. This function is
 * useful if you need to make changes to configuration on per-transaction basis.
 * Initially, all transactions will share the configuration with that of the
 * connection; if you were to make changes on it, they would affect all
 * current and future connections. To work around that, you make a copy of the
 * configuration object, call this function with the second parameter set to
 * HTP_CFG_PRIVATE, and modify configuration at will.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] cfg Configuration pointer. Must not be NULL.
 * @param[in] is_cfg_shared HTP_CFG_SHARED or HTP_CFG_PRIVATE
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_config(
    mut tx: *mut htp_tx_t,
    mut cfg: *mut crate::src::htp_config::htp_cfg_t,
    mut is_cfg_shared: libc::c_int,
) {
    if tx.is_null() || cfg.is_null() {
        return;
    }
    if is_cfg_shared != 0 as libc::c_int && is_cfg_shared != 1 as libc::c_int {
        return;
    }
    // If we're using a private configuration, destroy it.
    if (*tx).is_config_shared == 0 as libc::c_int {
        htp_config_destroy((*tx).cfg);
    }
    (*tx).cfg = cfg;
    (*tx).is_config_shared = is_cfg_shared;
}

/**
 * Associates user data with this transaction.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] user_data Opaque user data pointer.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_user_data(
    mut tx: *mut htp_tx_t,
    mut user_data: *mut libc::c_void,
) {
    if tx.is_null() {
        return;
    }
    (*tx).user_data = user_data;
}

/**
 * Adds one parameter to the request. THis function will take over the
 * responsibility for the provided htp_param_t structure.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] param Parameter pointer. Must not be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_add_param(
    mut tx: *mut htp_tx_t,
    mut param: *mut htp_param_t,
) -> htp_status_t {
    if tx.is_null() || param.is_null() {
        return -(1 as libc::c_int);
    }
    if (*(*tx).cfg).parameter_processor.is_some() {
        if (*(*tx).cfg)
            .parameter_processor
            .expect("non-null function pointer")(param)
            != 1 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    return htp_table_addk(
        (*tx).request_params,
        (*param).name,
        param as *const libc::c_void,
    );
}

/**
 * Returns the first request parameter that matches the given name, using case-insensitive matching.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @return htp_param_t instance, or NULL if parameter not found.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_get_param(
    mut tx: *mut htp_tx_t,
    mut name: *const libc::c_char,
    mut name_len: size_t,
) -> *mut htp_param_t {
    if tx.is_null() || name.is_null() {
        return 0 as *mut htp_param_t;
    }
    return htp_table_get_mem((*tx).request_params, name as *const libc::c_void, name_len)
        as *mut htp_param_t;
}

/**
 * Returns the first request parameter from the given source that matches the given name,
 * using case-insensitive matching.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] source Parameter source (where in request the parameter was located).
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @return htp_param_t instance, or NULL if parameter not found.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_get_param_ex(
    mut tx: *mut htp_tx_t,
    mut source: htp_data_source_t,
    mut name: *const libc::c_char,
    mut name_len: size_t,
) -> *mut htp_param_t {
    if tx.is_null() || name.is_null() {
        return 0 as *mut htp_param_t;
    }
    let mut p: *mut htp_param_t = 0 as *mut htp_param_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_table_size((*tx).request_params);
    while i < n {
        p = htp_table_get_index((*tx).request_params, i, 0 as *mut *mut bstr) as *mut htp_param_t;
        if !((*p).source as libc::c_uint != source as libc::c_uint) {
            if bstr_cmp_mem_nocase((*p).name, name as *const libc::c_void, name_len)
                == 0 as libc::c_int
            {
                return p;
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut htp_param_t;
}

/**
 * Determine if the request has a body.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return 1 if there is a body, 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_has_body(mut tx: *const htp_tx_t) -> libc::c_int {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    if (*tx).request_transfer_coding == htp_transfer_coding_t::HTP_CODING_IDENTITY
        || (*tx).request_transfer_coding == htp_transfer_coding_t::HTP_CODING_CHUNKED
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

/**
 * Set one request header. This function should be invoked once for
 * each available header, and in the order in which headers were
 * seen in the request.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @param[in] value Value data pointer. Must not be NULL.
 * @param[in] value_len Value data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_header(
    mut tx: *mut htp_tx_t,
    mut name: *const libc::c_char,
    mut name_len: size_t,
    mut value: *const libc::c_char,
    mut value_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || name.is_null() || value.is_null() {
        return -(1 as libc::c_int);
    }
    let mut h: *mut htp_header_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_header_t>() as libc::c_ulong,
    ) as *mut htp_header_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).name = copy_or_wrap_mem(name as *const libc::c_void, name_len, alloc);
    if (*h).name.is_null() {
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*h).value = copy_or_wrap_mem(value as *const libc::c_void, value_len, alloc);
    if (*h).value.is_null() {
        bstr_free((*h).name);
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if htp_table_add((*tx).request_headers, (*h).name, h as *const libc::c_void) != 1 as libc::c_int
    {
        bstr_free((*h).name);
        bstr_free((*h).value);
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Set transaction request method. This function will enable you to keep
 * track of the text representation of the method.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] method Method data pointer. Must not be NULL.
 * @param[in] method_len Method data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_method(
    mut tx: *mut htp_tx_t,
    mut method: *const libc::c_char,
    mut method_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || method.is_null() {
        return -(1 as libc::c_int);
    }
    (*tx).request_method = copy_or_wrap_mem(method as *const libc::c_void, method_len, alloc);
    if (*tx).request_method.is_null() {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Set transaction request method number. This function enables you to
 * keep track how a particular method string is interpreted. This function
 * is useful with web servers that ignore invalid methods; for example, some
 * web servers will treat them as a GET.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] method_number Method number.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_method_number(
    mut tx: *mut htp_tx_t,
    mut method_number: libc::c_uint,
) {
    if tx.is_null() {
        return;
    }
    (*tx).request_method_number = method_number;
}

/**
 * Set transaction request URI. The value provided here will be stored in htp_tx_t::request_uri
 * and subsequently parsed. If htp_tx_req_set_line() was previously used, the uri provided
 * when calling this function will overwrite any previously parsed value.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] uri URI data pointer. Must not be NULL.
 * @param[in] uri_len URI data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_uri(
    mut tx: *mut htp_tx_t,
    mut uri: *const libc::c_char,
    mut uri_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || uri.is_null() {
        return -(1 as libc::c_int);
    }
    (*tx).request_uri = copy_or_wrap_mem(uri as *const libc::c_void, uri_len, alloc);
    if (*tx).request_uri.is_null() {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Sets the request protocol string (e.g., "HTTP/1.0"). The information provided
 * is only stored, not parsed. Use htp_tx_req_set_protocol_number() to set the
 * actual protocol number, as interpreted by the container.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] protocol Protocol data pointer. Must not be NULL.
 * @param[in] protocol_len Protocol data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_protocol(
    mut tx: *mut htp_tx_t,
    mut protocol: *const libc::c_char,
    mut protocol_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || protocol.is_null() {
        return -(1 as libc::c_int);
    }
    (*tx).request_protocol = copy_or_wrap_mem(protocol as *const libc::c_void, protocol_len, alloc);
    if (*tx).request_protocol.is_null() {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Set request protocol version number. Must be invoked after
 * htp_txh_set_req_protocol(), because it will overwrite the previously
 * extracted version number. Convert the protocol version number to an integer
 * by multiplying it with 100. For example, 1.1 becomes 110. Alternatively,
 * use the HTP_PROTOCOL_0_9, HTP_PROTOCOL_1_0, and HTP_PROTOCOL_1_1 constants.
 * Note: setting protocol to HTP_PROTOCOL_0_9 alone will _not_ get the library to
 * treat the transaction as HTTP/0.9. You need to also invoke htp_tx_req_set_protocol_0_9().
 * This is because HTTP 0.9 is used only when protocol information is absent from the
 * request line, and not when it is explicitly stated (as "HTTP/0.9"). This behavior is
 * consistent with that of Apache httpd.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] protocol_number Protocol number.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_protocol_number(
    mut tx: *mut htp_tx_t,
    mut protocol_number: libc::c_int,
) {
    if tx.is_null() {
        return;
    }
    (*tx).request_protocol_number = protocol_number;
}

/**
 * Forces HTTP/0.9 as the transaction protocol. This method exists to ensure
 * that both LibHTP and the container treat the transaction as HTTP/0.9, despite
 * potential differences in how the protocol version is determined.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] is_protocol_0_9 Zero if protocol is not HTTP/0.9, or 1 if it is.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_protocol_0_9(
    mut tx: *mut htp_tx_t,
    mut is_protocol_0_9: libc::c_int,
) {
    if tx.is_null() {
        return;
    }
    if is_protocol_0_9 != 0 {
        (*tx).is_protocol_0_9 = 1 as libc::c_int
    } else {
        (*tx).is_protocol_0_9 = 0 as libc::c_int
    };
}
unsafe extern "C" fn htp_tx_process_request_headers(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // Determine if we have a request body, and how it is packaged.
    let mut rc: htp_status_t = 1 as libc::c_int;
    let mut cl: *mut htp_header_t = htp_table_get_c(
        (*tx).request_headers,
        b"content-length\x00" as *const u8 as *const libc::c_char,
    ) as *mut htp_header_t;
    let mut te: *mut htp_header_t = htp_table_get_c(
        (*tx).request_headers,
        b"transfer-encoding\x00" as *const u8 as *const libc::c_char,
    ) as *mut htp_header_t;
    // Check for the Transfer-Encoding header, which would indicate a chunked request body.
    if !te.is_null() {
        // Make sure it contains "chunked" only.
        // TODO The HTTP/1.1 RFC also allows the T-E header to contain "identity", which
        //      presumably should have the same effect as T-E header absence. However, Apache
        //      (2.2.22 on Ubuntu 12.04 LTS) instead errors out with "Unknown Transfer-Encoding: identity".
        //      And it behaves strangely, too, sending a 501 and proceeding to process the request
        //      (e.g., PHP is run), but without the body. It then closes the connection.
        if bstr_cmp_c_nocase(
            (*te).value,
            b"chunked\x00" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            // Invalid T-E header value.
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_INVALID;
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x400 as libc::c_ulonglong) as uint64_t;
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x100000000 as libc::c_ulonglong) as uint64_t
        } else {
            // Chunked encoding is a HTTP/1.1 feature, so check that an earlier protocol
            // version is not used. The flag will also be set if the protocol could not be parsed.
            //
            // TODO IIS 7.0, for example, would ignore the T-E header when it
            //      it is used with a protocol below HTTP 1.1. This should be a
            //      personality trait.
            if (*tx).request_protocol_number < 101 as libc::c_int {
                (*tx).flags =
                    ((*tx).flags as libc::c_ulonglong | 0x400 as libc::c_ulonglong) as uint64_t;
                (*tx).flags =
                    ((*tx).flags as libc::c_ulonglong | 0x100 as libc::c_ulonglong) as uint64_t
            }
            // If the T-E header is present we are going to use it.
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_CHUNKED;
            // We are still going to check for the presence of C-L.
            if !cl.is_null() {
                // According to the HTTP/1.1 RFC (section 4.4):
                //
                // "The Content-Length header field MUST NOT be sent
                //  if these two lengths are different (i.e., if a Transfer-Encoding
                //  header field is present). If a message is received with both a
                //  Transfer-Encoding header field and a Content-Length header field,
                //  the latter MUST be ignored."
                //
                (*tx).flags =
                    ((*tx).flags as libc::c_ulonglong | 0x100 as libc::c_ulonglong) as uint64_t
            }
        }
    } else if !cl.is_null() {
        // Check for a folded C-L header.
        if (*cl).flags as libc::c_ulonglong & 0x10 as libc::c_ulonglong != 0 {
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x100 as libc::c_ulonglong) as uint64_t
        }
        // Check for multiple C-L headers.
        if (*cl).flags as libc::c_ulonglong & 0x20 as libc::c_ulonglong != 0 {
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x100 as libc::c_ulonglong) as uint64_t
            // TODO Personality trait to determine which C-L header to parse.
            //      At the moment we're parsing the combination of all instances,
            //      which is bound to fail (because it will contain commas).
        }
        // Get the body length.
        (*tx).request_content_length = htp_parse_content_length((*cl).value, (*tx).connp);
        if (*tx).request_content_length < 0 as libc::c_int as libc::c_long {
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_INVALID;
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x200000000 as libc::c_ulonglong) as uint64_t;
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x100000000 as libc::c_ulonglong) as uint64_t
        } else {
            // We have a request body of known length.
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_IDENTITY
        }
    } else {
        // No body.
        (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_NO_BODY
    }
    // If we could not determine the correct body handling,
    // consider the request invalid.
    if (*tx).request_transfer_coding == htp_transfer_coding_t::HTP_CODING_UNKNOWN {
        (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_INVALID;
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x100000000 as libc::c_ulonglong) as uint64_t
    }
    // Check for PUT requests, which we need to treat as file uploads.
    if (*tx).request_method_number
        == crate::src::htp_request::htp_method_t::HTP_M_PUT as libc::c_uint
    {
        if htp_tx_req_has_body(tx) != 0 {
            // Prepare to treat PUT request body as a file.
            (*(*tx).connp).put_file = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<crate::src::htp_util::htp_file_t>() as libc::c_ulong,
            ) as *mut crate::src::htp_util::htp_file_t;
            if (*(*tx).connp).put_file.is_null() {
                return -(1 as libc::c_int);
            }
            (*(*(*tx).connp).put_file).fd = -(1 as libc::c_int);
            (*(*(*tx).connp).put_file).source =
                crate::src::htp_util::htp_file_source_t::HTP_FILE_PUT
        }
    }
    // Determine hostname.
    // Use the hostname from the URI, when available.
    if !(*(*tx).parsed_uri).hostname.is_null() {
        (*tx).request_hostname = bstr_dup((*(*tx).parsed_uri).hostname);
        if (*tx).request_hostname.is_null() {
            return -(1 as libc::c_int);
        }
    }
    (*tx).request_port_number = (*(*tx).parsed_uri).port_number;
    // Examine the Host header.
    let mut h: *mut htp_header_t = htp_table_get_c(
        (*tx).request_headers,
        b"host\x00" as *const u8 as *const libc::c_char,
    ) as *mut htp_header_t;
    if h.is_null() {
        // No host information in the headers.
        // HTTP/1.1 requires host information in the headers.
        if (*tx).request_protocol_number >= 101 as libc::c_int {
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x1000 as libc::c_ulonglong) as uint64_t
        }
    } else {
        // Host information available in the headers.
        let mut hostname: *mut bstr = 0 as *mut bstr;
        let mut port: libc::c_int = 0;
        rc = htp_parse_header_hostport(
            (*h).value,
            &mut hostname,
            0 as *mut *mut bstr,
            &mut port,
            &mut (*tx).flags,
        );
        if rc != 1 as libc::c_int {
            return rc;
        }
        if !hostname.is_null() {
            // The host information in the headers is valid.
            // Is there host information in the URI?
            if (*tx).request_hostname.is_null() {
                // There is no host information in the URI. Place the
                // hostname from the headers into the parsed_uri structure.
                (*tx).request_hostname = hostname;
                (*tx).request_port_number = port
            } else {
                // The host information appears in the URI and in the headers. The
                // HTTP RFC states that we should ignore the header copy.
                // Check for different hostnames.
                if bstr_cmp_nocase(hostname, (*tx).request_hostname) != 0 as libc::c_int {
                    (*tx).flags =
                        ((*tx).flags as libc::c_ulonglong | 0x2000 as libc::c_ulonglong) as uint64_t
                }
                // Check for different ports.
                if (*tx).request_port_number != -(1 as libc::c_int)
                    && port != -(1 as libc::c_int)
                    && (*tx).request_port_number != port
                {
                    (*tx).flags =
                        ((*tx).flags as libc::c_ulonglong | 0x2000 as libc::c_ulonglong) as uint64_t
                }
                bstr_free(hostname);
            }
        } else if !(*tx).request_hostname.is_null() {
            // Invalid host information in the headers.
            // Raise the flag, even though the host information in the headers is invalid.
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x2000 as libc::c_ulonglong) as uint64_t
        }
    }
    // Determine Content-Type.
    let mut ct: *mut htp_header_t = htp_table_get_c(
        (*tx).request_headers,
        b"content-type\x00" as *const u8 as *const libc::c_char,
    ) as *mut htp_header_t;
    if !ct.is_null() {
        rc = htp_parse_ct_header((*ct).value, &mut (*tx).request_content_type);
        if rc != 1 as libc::c_int {
            return rc;
        }
    }
    // Parse cookies.
    if (*(*(*tx).connp).cfg).parse_request_cookies != 0 {
        rc = htp_parse_cookies_v0((*tx).connp);
        if rc != 1 as libc::c_int {
            return rc;
        }
    }
    // Parse authentication information.
    if (*(*(*tx).connp).cfg).parse_request_auth != 0 {
        rc = htp_parse_authorization((*tx).connp);
        if rc == 0 as libc::c_int {
            // Don't fail the stream if an authorization header is invalid, just set a flag.
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x400000000 as libc::c_ulonglong) as uint64_t
        } else if rc != 1 as libc::c_int {
            return rc;
        }
    }
    // Finalize sending raw header data.
    rc = htp_connp_req_receiver_finalize_clear((*tx).connp);
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Run hook REQUEST_HEADERS.
    rc = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_headers,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // We cannot proceed if the request is invalid.
    if (*tx).flags as libc::c_ulonglong & 0x100000000 as libc::c_ulonglong != 0 {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Process a chunk of request body data. This function assumes that
 * handling of chunked encoding is implemented by the container. When
 * you're done submitting body data, invoke a state change (to REQUEST)
 * to finalize any processing that might be pending. The supplied data is
 * fully consumed and there is no expectation that it will be available
 * afterwards. The protocol parsing code makes no copies of the data,
 * but some parsers might.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] data Data pointer. Must not be NULL.
 * @param[in] len Data length.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_process_body_data(
    mut tx: *mut htp_tx_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    if tx.is_null() || data == 0 as *mut libc::c_void {
        return -(1 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    return htp_tx_req_process_body_data_ex(tx, data, len);
}

#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_process_body_data_ex(
    mut tx: *mut htp_tx_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // NULL data is allowed in this private function; it's
    // used to indicate the end of request body.
    // Keep track of the body length.
    (*tx).request_entity_len =
        ((*tx).request_entity_len as libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
    // Send data to the callbacks.
    let mut d: htp_tx_data_t = htp_tx_data_t {
        tx: 0 as *mut htp_tx_t,
        data: 0 as *const libc::c_uchar,
        len: 0,
        is_last: 0,
    };
    d.tx = tx;
    d.data = data as *mut libc::c_uchar;
    d.len = len;
    let mut rc: htp_status_t = htp_req_run_hook_body_data((*tx).connp, &mut d);
    if rc != 1 as libc::c_int {
        htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
            589 as libc::c_int,
            crate::src::htp_util::htp_log_level_t::HTP_LOG_ERROR,
            0 as libc::c_int,
            b"Request body data callback returned error (%d)\x00" as *const u8
                as *const libc::c_char,
            rc,
        );
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Removes all request headers associated with this transaction. This
 * function is needed because in some cases the container does not
 * differentiate between standard and trailing headers. In that case,
 * you set request headers once at the beginning of the transaction,
 * read the body (at this point the request headers should contain the
 * mix of regular and trailing headers), clear all headers, and then set
 * them all again.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_headers_clear(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() || (*tx).request_headers.is_null() {
        return -(1 as libc::c_int);
    }
    let mut h: *mut htp_header_t = 0 as *mut htp_header_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_table_size((*tx).request_headers);
    while i < n {
        h = htp_table_get_index((*tx).request_headers, i, 0 as *mut *mut bstr) as *mut htp_header_t;
        bstr_free((*h).name);
        bstr_free((*h).value);
        free(h as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    htp_table_destroy((*tx).request_headers);
    (*tx).request_headers = htp_table_create(32 as libc::c_int as size_t);
    if (*tx).request_headers.is_null() {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Set request line. When used, this function should always be called first,
 * with more specific functions following. Must not contain line terminators.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] line Line data pointer. Must not be NULL.
 * @param[in] line_len Line data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_line(
    mut tx: *mut htp_tx_t,
    mut line: *const libc::c_char,
    mut line_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || line.is_null() || line_len == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*tx).request_line = copy_or_wrap_mem(line as *const libc::c_void, line_len, alloc);
    if (*tx).request_line.is_null() {
        return -(1 as libc::c_int);
    }
    if (*(*(*tx).connp).cfg)
        .parse_request_line
        .expect("non-null function pointer")((*tx).connp)
        != 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Set parsed request URI. You don't need to use this function if you are already providing
 * the request line or request URI. But if your container already has this data available,
 * feeding it to LibHTP will minimize any potential data differences. This function assumes
 * management of the data provided in parsed_uri. This function will not change htp_tx_t::parsed_uri_raw
 * (which may have data in it from the parsing of the request URI).
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] parsed_uri URI pointer. Must not be NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_req_set_parsed_uri(
    mut tx: *mut htp_tx_t,
    mut parsed_uri: *mut crate::src::htp_util::htp_uri_t,
) {
    if tx.is_null() || parsed_uri.is_null() {
        return;
    }
    if !(*tx).parsed_uri.is_null() {
        htp_uri_free((*tx).parsed_uri);
    }
    (*tx).parsed_uri = parsed_uri;
}

/**
 * Set response line. Use this function is you have a single buffer containing
 * the entire line. If you have individual request line pieces, use the other
 * available functions.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] line Line data pointer. Must not be NULL.
 * @param[in] line_len Line data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_set_status_line(
    mut tx: *mut htp_tx_t,
    mut line: *const libc::c_char,
    mut line_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || line.is_null() || line_len == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*tx).response_line = copy_or_wrap_mem(line as *const libc::c_void, line_len, alloc);
    if (*tx).response_line.is_null() {
        return -(1 as libc::c_int);
    }
    if (*(*(*tx).connp).cfg)
        .parse_response_line
        .expect("non-null function pointer")((*tx).connp)
        != 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Set response protocol number. See htp_tx_res_set_protocol_number() for more information
 * about the correct format of the protocol_parameter parameter.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] protocol_number Protocol number.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_set_protocol_number(
    mut tx: *mut htp_tx_t,
    mut protocol_number: libc::c_int,
) {
    if tx.is_null() {
        return;
    }
    (*tx).response_protocol_number = protocol_number;
}

/**
 * Set response status code.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] status_code Response status code.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_set_status_code(
    mut tx: *mut htp_tx_t,
    mut status_code: libc::c_int,
) {
    if tx.is_null() {
        return;
    }
    (*tx).response_status_number = status_code;
}

/**
 * Set response status message, which is the part of the response
 * line that comes after the status code.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] msg Message data pointer. Must not be NULL.
 * @param[in] msg_len Message data length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_set_status_message(
    mut tx: *mut htp_tx_t,
    mut msg: *const libc::c_char,
    mut msg_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || msg.is_null() {
        return -(1 as libc::c_int);
    }
    if !(*tx).response_message.is_null() {
        bstr_free((*tx).response_message);
    }
    (*tx).response_message = copy_or_wrap_mem(msg as *const libc::c_void, msg_len, alloc);
    if (*tx).response_message.is_null() {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Change transaction state to HTP_RESPONSE_LINE and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_response_line(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // Is the response line valid?
    if (*tx).response_protocol_number == -(2 as libc::c_int) {
        htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
            688 as libc::c_int,
            crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"Invalid response line: invalid protocol\x00" as *const u8 as *const libc::c_char,
        );
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x1000000 as libc::c_ulonglong) as uint64_t
    }
    if (*tx).response_status_number == -(1 as libc::c_int)
        || (*tx).response_status_number < 100 as libc::c_int
        || (*tx).response_status_number > 999 as libc::c_int
    {
        htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
            695 as libc::c_int,
            crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"Invalid response line: invalid response status %d.\x00" as *const u8
                as *const libc::c_char,
            (*tx).response_status_number,
        );
        (*tx).response_status_number = -(1 as libc::c_int);
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x1000000 as libc::c_ulonglong) as uint64_t
    }
    // Run hook HTP_RESPONSE_LINE
    let mut rc: htp_status_t = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_response_line,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    return 1 as libc::c_int;
}

/**
 * Set one response header. This function should be invoked once for
 * each available header, and in the order in which headers were
 * seen in the response.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] name Name data pointer. Must not be NULL.
 * @param[in] name_len Name data length.
 * @param[in] value Value data pointer. Must not be NULL.
 * @param[in] value_len Value length.
 * @param[in] alloc Desired allocation strategy.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_set_header(
    mut tx: *mut htp_tx_t,
    mut name: *const libc::c_char,
    mut name_len: size_t,
    mut value: *const libc::c_char,
    mut value_len: size_t,
    mut alloc: htp_alloc_strategy_t,
) -> htp_status_t {
    if tx.is_null() || name.is_null() || value.is_null() {
        return -(1 as libc::c_int);
    }
    let mut h: *mut htp_header_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_header_t>() as libc::c_ulong,
    ) as *mut htp_header_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).name = copy_or_wrap_mem(name as *const libc::c_void, name_len, alloc);
    if (*h).name.is_null() {
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*h).value = copy_or_wrap_mem(value as *const libc::c_void, value_len, alloc);
    if (*h).value.is_null() {
        bstr_free((*h).name);
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if htp_table_add((*tx).response_headers, (*h).name, h as *const libc::c_void)
        != 1 as libc::c_int
    {
        bstr_free((*h).name);
        bstr_free((*h).value);
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Removes all response headers associated with this transaction. This
 * function is needed because in some cases the container does not
 * differentiate between standard and trailing headers. In that case,
 * you set response headers once at the beginning of the transaction,
 * read the body, clear all headers, and then set them all again. After
 * the headers are set for the second time, they will potentially contain
 * a mixture of standard and trailing headers.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_set_headers_clear(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() || (*tx).response_headers.is_null() {
        return -(1 as libc::c_int);
    }
    let mut h: *mut htp_header_t = 0 as *mut htp_header_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_table_size((*tx).response_headers);
    while i < n {
        h = htp_table_get_index((*tx).response_headers, i, 0 as *mut *mut bstr)
            as *mut htp_header_t;
        bstr_free((*h).name);
        bstr_free((*h).value);
        free(h as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    htp_table_destroy((*tx).response_headers);
    (*tx).response_headers = htp_table_create(32 as libc::c_int as size_t);
    if (*tx).response_headers.is_null() {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn htp_connp_destroy_decompressors(
    mut connp: *mut crate::src::htp_connection_parser::htp_connp_t,
) {
    let mut comp: *mut crate::src::htp_decompressors::htp_decompressor_t =
        (*connp).out_decompressor;
    while !comp.is_null() {
        let mut next: *mut crate::src::htp_decompressors::htp_decompressor_t = (*comp).next;
        (*comp).destroy.expect("non-null function pointer")(comp);
        comp = next
    }
    (*connp).out_decompressor = 0 as *mut crate::src::htp_decompressors::htp_decompressor_t;
}

/* *
 * Clean up decompressor(s).
 *
 * @param[in] tx
 */
unsafe extern "C" fn htp_tx_res_destroy_decompressors(mut tx: *mut htp_tx_t) {
    htp_connp_destroy_decompressors((*tx).connp);
}

unsafe extern "C" fn htp_tx_res_process_body_data_decompressor_callback(
    mut d: *mut htp_tx_data_t,
) -> htp_status_t {
    if d.is_null() {
        return -(1 as libc::c_int);
    }
    // Keep track of actual response body length.
    (*(*d).tx).response_entity_len = ((*(*d).tx).response_entity_len as libc::c_ulong)
        .wrapping_add((*d).len) as int64_t as int64_t;
    // Invoke all callbacks.
    let mut rc: htp_status_t = htp_res_run_hook_body_data((*(*d).tx).connp, d);
    if rc != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*(*d).tx).response_entity_len
        > (*(*(*(*d).tx).connp).cfg).compression_bomb_limit as libc::c_long
        && (*(*d).tx).response_entity_len
            > 2048 as libc::c_int as libc::c_long * (*(*d).tx).response_message_len
    {
        htp_log(
            (*(*d).tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
            794 as libc::c_int,
            crate::src::htp_util::htp_log_level_t::HTP_LOG_ERROR,
            0 as libc::c_int,
            b"Compression bomb: decompressed %ld bytes out of %ld\x00" as *const u8
                as *const libc::c_char,
            (*(*d).tx).response_entity_len,
            (*(*d).tx).response_message_len,
        );
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Process a chunk of response body data. This function assumes that
 * handling of chunked encoding is implemented by the container. When
 * you're done submitting body data, invoking a state change (to RESPONSE)
 * will finalize any processing that might be pending.
 *
 * The response body data will be decompressed if two conditions are met: one,
 * decompression is enabled in configuration and two, if the response headers
 * indicate compression. Alternatively, you can control decompression from
 * a RESPONSE_HEADERS callback, by setting tx->response_content_encoding either
 * to COMPRESSION_NONE (to disable compression), or to one of the supported
 * decompression algorithms.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @param[in] data Data pointer. Must not be NULL.
 * @param[in] len Data length.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_process_body_data(
    mut tx: *mut htp_tx_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    if tx.is_null() || data == 0 as *mut libc::c_void {
        return -(1 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    return htp_tx_res_process_body_data_ex(tx, data, len);
}

#[no_mangle]
pub unsafe extern "C" fn htp_tx_res_process_body_data_ex(
    mut tx: *mut htp_tx_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // NULL data is allowed in this private function; it's
    // used to indicate the end of response body.
    let mut d: htp_tx_data_t = htp_tx_data_t {
        tx: 0 as *mut htp_tx_t,
        data: 0 as *const libc::c_uchar,
        len: 0,
        is_last: 0,
    };
    d.tx = tx;
    d.data = data as *mut libc::c_uchar;
    d.len = len;
    d.is_last = 0 as libc::c_int;
    // Keep track of body size before decompression.
    (*tx).response_message_len =
        ((*tx).response_message_len as libc::c_ulong).wrapping_add(d.len) as int64_t as int64_t;
    let mut rc: htp_status_t = 0;
    match (*tx).response_content_encoding_processing as libc::c_uint {
        2 | 3 | 4 => {
            // In severe memory stress these could be NULL
            if (*(*tx).connp).out_decompressor.is_null()
                || (*(*(*tx).connp).out_decompressor).decompress.is_none()
            {
                return -(1 as libc::c_int);
            }
            // Send data buffer to the decompressor.
            (*(*(*tx).connp).out_decompressor)
                .decompress
                .expect("non-null function pointer")(
                (*(*tx).connp).out_decompressor, &mut d
            );
            if data == 0 as *mut libc::c_void {
                // Shut down the decompressor, if we used one.
                htp_tx_res_destroy_decompressors(tx);
            }
        }
        1 => {
            // When there's no decompression, response_entity_len.
            // is identical to response_message_len.
            (*tx).response_entity_len = ((*tx).response_entity_len as libc::c_ulong)
                .wrapping_add(d.len) as int64_t as int64_t;
            rc = htp_res_run_hook_body_data((*tx).connp, &mut d);
            if rc != 1 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        _ => {
            // Internal error.
            htp_log(
                (*tx).connp,
                b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
                857 as libc::c_int,
                crate::src::htp_util::htp_log_level_t::HTP_LOG_ERROR,
                0 as libc::c_int,
                b"[Internal Error] Invalid tx->response_content_encoding_processing value: %d\x00"
                    as *const u8 as *const libc::c_char,
                (*tx).response_content_encoding_processing as libc::c_uint,
            );
            return -(1 as libc::c_int);
        }
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_request_complete_partial(
    mut tx: *mut htp_tx_t,
) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // Finalize request body.
    if htp_tx_req_has_body(tx) != 0 {
        let mut rc: htp_status_t = htp_tx_req_process_body_data_ex(
            tx,
            0 as *const libc::c_void,
            0 as libc::c_int as size_t,
        );
        if rc != 1 as libc::c_int {
            return rc;
        }
    }
    (*tx).request_progress = htp_tx_req_progress_t::HTP_REQUEST_COMPLETE;
    // Run hook REQUEST_COMPLETE.
    let mut rc_0: htp_status_t = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_complete,
        tx as *mut libc::c_void,
    );
    if rc_0 != 1 as libc::c_int {
        return rc_0;
    }
    // Clean-up.
    if !(*(*tx).connp).put_file.is_null() {
        bstr_free((*(*(*tx).connp).put_file).filename);
        free((*(*tx).connp).put_file as *mut libc::c_void);
        (*(*tx).connp).put_file = 0 as *mut crate::src::htp_util::htp_file_t
    }
    return 1 as libc::c_int;
}

/**
 * Change transaction state to REQUEST and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_request_complete(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    if (*tx).request_progress != htp_tx_req_progress_t::HTP_REQUEST_COMPLETE {
        let mut rc: htp_status_t = htp_tx_state_request_complete_partial(tx);
        if rc != 1 as libc::c_int {
            return rc;
        }
    }
    // Make a copy of the connection parser pointer, so that
    // we don't have to reference it via tx, which may be
    // destroyed later.
    let mut connp: *mut crate::src::htp_connection_parser::htp_connp_t = (*tx).connp;
    // Determine what happens next, and remove this transaction from the parser.
    if (*tx).is_protocol_0_9 != 0 {
        (*connp).in_state = Some(
            htp_connp_REQ_IGNORE_DATA_AFTER_HTTP_0_9
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    } else {
        (*connp).in_state = Some(
            htp_connp_REQ_IDLE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    }
    // Check if the entire transaction is complete. This call may
    // destroy the transaction, if auto-destroy is enabled.
    htp_tx_finalize(tx);
    // At this point, tx may no longer be valid.
    (*connp).in_tx = 0 as *mut htp_tx_t;
    return 1 as libc::c_int;
}

/**
 * Initialize hybrid parsing mode, change state to TRANSACTION_START,
 * and invoke all registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_request_start(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // Run hook REQUEST_START.
    let mut rc: htp_status_t = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_start,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Change state into request line parsing.
    (*(*tx).connp).in_state = Some(
        htp_connp_REQ_LINE
            as unsafe extern "C" fn(
                _: *mut crate::src::htp_connection_parser::htp_connp_t,
            ) -> htp_status_t,
    );
    (*(*(*tx).connp).in_tx).request_progress = htp_tx_req_progress_t::HTP_REQUEST_LINE;
    return 1 as libc::c_int;
}

/**
 * Change transaction state to REQUEST_HEADERS and invoke all
 * registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_request_headers(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // If we're in HTP_REQ_HEADERS that means that this is the
    // first time we're processing headers in a request. Otherwise,
    // we're dealing with trailing headers.
    if (*tx).request_progress > htp_tx_req_progress_t::HTP_REQUEST_HEADERS {
        // Request trailers.
        // Run hook HTP_REQUEST_TRAILER.
        let mut rc: htp_status_t = htp_hook_run_all(
            (*(*(*tx).connp).cfg).hook_request_trailer,
            tx as *mut libc::c_void,
        );
        if rc != 1 as libc::c_int {
            return rc;
        }
        // Finalize sending raw header data.
        rc = htp_connp_req_receiver_finalize_clear((*tx).connp);
        if rc != 1 as libc::c_int {
            return rc;
        }
        // Completed parsing this request; finalize it now.
        (*(*tx).connp).in_state = Some(
            htp_connp_REQ_FINALIZE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    } else if (*tx).request_progress >= htp_tx_req_progress_t::HTP_REQUEST_LINE {
        // Request headers.
        // Did this request arrive in multiple data chunks?
        if (*(*tx).connp).in_chunk_count != (*(*tx).connp).in_chunk_request_index {
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x800 as libc::c_ulonglong) as uint64_t
        }
        let mut rc_0: htp_status_t = htp_tx_process_request_headers(tx);
        if rc_0 != 1 as libc::c_int {
            return rc_0;
        }
        (*(*tx).connp).in_state = Some(
            htp_connp_REQ_CONNECT_CHECK
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        )
    } else {
        htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
            969 as libc::c_int,
            crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"[Internal Error] Invalid tx progress: %d\x00" as *const u8 as *const libc::c_char,
            (*tx).request_progress as libc::c_uint,
        );
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Change transaction state to REQUEST_LINE and invoke all
 * registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_request_line(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // Determine how to process the request URI.
    if (*tx).request_method_number
        == crate::src::htp_request::htp_method_t::HTP_M_CONNECT as libc::c_uint
    {
        // When CONNECT is used, the request URI contains an authority string.
        if htp_parse_uri_hostport((*tx).connp, (*tx).request_uri, (*tx).parsed_uri_raw)
            != 1 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    } else if htp_parse_uri((*tx).request_uri, &mut (*tx).parsed_uri_raw) != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    // Parse the request URI into htp_tx_t::parsed_uri_raw.
    // Build htp_tx_t::parsed_uri, but only if it was not explicitly set already.
    if (*tx).parsed_uri.is_null() {
        (*tx).parsed_uri = htp_uri_alloc();
        if (*tx).parsed_uri.is_null() {
            return -(1 as libc::c_int);
        }
        // Keep the original URI components, but create a copy which we can normalize and use internally.
        if htp_normalize_parsed_uri(tx, (*tx).parsed_uri_raw, (*tx).parsed_uri) != 1 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    // Check parsed_uri hostname.
    if !(*(*tx).parsed_uri).hostname.is_null() {
        if htp_validate_hostname((*(*tx).parsed_uri).hostname) == 0 as libc::c_int {
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong | 0x2000000 as libc::c_ulonglong) as uint64_t
        }
    }
    // Run hook REQUEST_URI_NORMALIZE.
    let mut rc: htp_status_t = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_uri_normalize,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Run hook REQUEST_LINE.
    rc = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_line,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Move on to the next phase.
    (*(*tx).connp).in_state = Some(
        htp_connp_REQ_PROTOCOL
            as unsafe extern "C" fn(
                _: *mut crate::src::htp_connection_parser::htp_connp_t,
            ) -> htp_status_t,
    );
    return 1 as libc::c_int;
}

/**
 * Change transaction state to RESPONSE and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_response_complete(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    return htp_tx_state_response_complete_ex(tx, 1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn htp_tx_finalize(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    if htp_tx_is_complete(tx) == 0 {
        return 1 as libc::c_int;
    }
    // Run hook TRANSACTION_COMPLETE.
    let mut rc: htp_status_t = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_transaction_complete,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // In streaming processing, we destroy the transaction because it will not be needed any more.
    if (*(*(*tx).connp).cfg).tx_auto_destroy != 0 {
        htp_tx_destroy(tx);
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_response_complete_ex(
    mut tx: *mut htp_tx_t,
    mut hybrid_mode: libc::c_int,
) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    if (*tx).response_progress != htp_tx_res_progress_t::HTP_RESPONSE_COMPLETE {
        (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_COMPLETE;
        // Run the last RESPONSE_BODY_DATA HOOK, but only if there was a response body present.
        if (*tx).response_transfer_coding != htp_transfer_coding_t::HTP_CODING_NO_BODY {
            htp_tx_res_process_body_data_ex(
                tx,
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        // Run hook RESPONSE_COMPLETE.
        let mut rc: htp_status_t = htp_hook_run_all(
            (*(*(*tx).connp).cfg).hook_response_complete,
            tx as *mut libc::c_void,
        );
        if rc != 1 as libc::c_int {
            return rc;
        }
    }
    if hybrid_mode == 0 {
        // Check if the inbound parser is waiting on us. If it is, that means that
        // there might be request data that the inbound parser hasn't consumed yet.
        // If we don't stop parsing we might encounter a response without a request,
        // which is why we want to return straight away before processing any data.
        //
        // This situation will occur any time the parser needs to see the server
        // respond to a particular situation before it can decide how to proceed. For
        // example, when a CONNECT is sent, different paths are used when it is accepted
        // and when it is not accepted.
        //
        // It is not enough to check only in_status here. Because of pipelining, it's possible
        // that many inbound transactions have been processed, and that the parser is
        // waiting on a response that we have not seen yet.
        if (*(*tx).connp).in_status
            == crate::src::htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER
            && (*(*tx).connp).in_tx == (*(*tx).connp).out_tx
        {
            return 3 as libc::c_int;
        }
        // Do we have a signal to yield to inbound processing at
        // the end of the next transaction?
        if (*(*tx).connp).out_data_other_at_tx_end != 0 {
            // We do. Let's yield then.
            (*(*tx).connp).out_data_other_at_tx_end = 0 as libc::c_int as libc::c_uint;
            return 3 as libc::c_int;
        }
    }
    // Make a copy of the connection parser pointer, so that
    // we don't have to reference it via tx, which may be destroyed later.
    let mut connp: *mut crate::src::htp_connection_parser::htp_connp_t = (*tx).connp;
    // Finalize the transaction. This may call may destroy the transaction, if auto-destroy is enabled.
    let mut rc_0: htp_status_t = htp_tx_finalize(tx);
    if rc_0 != 1 as libc::c_int {
        return rc_0;
    }
    // Disconnect transaction from the parser.
    (*connp).out_tx = 0 as *mut htp_tx_t;
    (*connp).out_state = Some(
        htp_connp_RES_IDLE
            as unsafe extern "C" fn(
                _: *mut crate::src::htp_connection_parser::htp_connp_t,
            ) -> htp_status_t,
    );
    return 1 as libc::c_int;
}
/* *
 *  @internal
 *  @brief split input into tokens separated by "seps"
 *  @param seps nul-terminated string: each character is a separator
 */
unsafe extern "C" fn get_token(
    mut in_0: *const libc::c_uchar,
    mut in_len: size_t,
    mut seps: *const libc::c_char,
    mut ret_tok_ptr: *mut *mut libc::c_uchar,
    mut ret_tok_len: *mut size_t,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    /* skip leading 'separators' */
    while i < in_len {
        let mut match_0: libc::c_int = 0 as libc::c_int;
        let mut s: *const libc::c_char = seps;
        while *s as libc::c_int != '\u{0}' as i32 {
            if *in_0.offset(i as isize) as libc::c_int == *s as libc::c_int {
                match_0 += 1;
                break;
            } else {
                s = s.offset(1)
            }
        }
        if match_0 == 0 {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i >= in_len {
        return 0 as libc::c_int;
    }
    in_0 = in_0.offset(i as isize);
    in_len = (in_len as libc::c_ulong).wrapping_sub(i) as size_t as size_t;
    i = 0 as libc::c_int as size_t;
    while i < in_len {
        let mut s_0: *const libc::c_char = seps;
        while *s_0 as libc::c_int != '\u{0}' as i32 {
            if *in_0.offset(i as isize) as libc::c_int == *s_0 as libc::c_int {
                *ret_tok_ptr = in_0 as *mut libc::c_uchar;
                *ret_tok_len = i;
                return 1 as libc::c_int;
            }
            s_0 = s_0.offset(1)
        }
        i = i.wrapping_add(1)
    }
    *ret_tok_ptr = in_0 as *mut libc::c_uchar;
    *ret_tok_len = in_len;
    return 1 as libc::c_int;
}

/**
 * Change transaction state to RESPONSE_HEADERS and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_response_headers(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // Check for compression.
    // Determine content encoding.
    let mut ce_multi_comp: libc::c_int = 0 as libc::c_int;
    (*tx).response_content_encoding =
        crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
    let mut ce: *mut htp_header_t = htp_table_get_c(
        (*tx).response_headers,
        b"content-encoding\x00" as *const u8 as *const libc::c_char,
    ) as *mut htp_header_t;
    if !ce.is_null() {
        /* fast paths: regular gzip and friends */
        if bstr_cmp_c_nocasenorzero((*ce).value, b"gzip\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || bstr_cmp_c_nocasenorzero(
                (*ce).value,
                b"x-gzip\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            (*tx).response_content_encoding =
                crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
        } else if bstr_cmp_c_nocasenorzero(
            (*ce).value,
            b"deflate\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || bstr_cmp_c_nocasenorzero(
                (*ce).value,
                b"x-deflate\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            (*tx).response_content_encoding =
                crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
        } else if bstr_cmp_c_nocasenorzero(
            (*ce).value,
            b"lzma\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*tx).response_content_encoding =
                crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
        } else if !(bstr_cmp_c_nocasenorzero(
            (*ce).value,
            b"inflate\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int)
        {
            /* exceptional cases: enter slow path */
            ce_multi_comp = 1 as libc::c_int
        }
    }
    // Configure decompression, if enabled in the configuration.
    if (*(*(*tx).connp).cfg).response_decompression_enabled != 0 {
        (*tx).response_content_encoding_processing = (*tx).response_content_encoding
    } else {
        (*tx).response_content_encoding_processing =
            crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
        ce_multi_comp = 0 as libc::c_int
    }
    // Finalize sending raw header data.
    let mut rc: htp_status_t = htp_connp_res_receiver_finalize_clear((*tx).connp);
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Run hook RESPONSE_HEADERS.
    rc = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_response_headers,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Initialize the decompression engine as necessary. We can deal with three
    // scenarios:
    //
    // 1. Decompression is enabled, compression indicated in headers, and we decompress.
    //
    // 2. As above, but the user disables decompression by setting response_content_encoding
    //    to COMPRESSION_NONE.
    //
    // 3. Decompression is disabled and we do not attempt to enable it, but the user
    //    forces decompression by setting response_content_encoding to one of the
    //    supported algorithms.
    if (*tx).response_content_encoding_processing
        == crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
        || (*tx).response_content_encoding_processing
            == crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
        || (*tx).response_content_encoding_processing
            == crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
        || ce_multi_comp != 0
    {
        if !(*(*tx).connp).out_decompressor.is_null() {
            htp_tx_res_destroy_decompressors(tx);
        }
        /* normal case */
        if ce_multi_comp == 0 {
            (*(*tx).connp).out_decompressor = htp_gzip_decompressor_create(
                (*tx).connp,
                (*tx).response_content_encoding_processing,
            );
            if (*(*tx).connp).out_decompressor.is_null() {
                return -(1 as libc::c_int);
            }
            (*(*(*tx).connp).out_decompressor).callback = Some(
                htp_tx_res_process_body_data_decompressor_callback
                    as unsafe extern "C" fn(_: *mut htp_tx_data_t) -> htp_status_t,
            )
        /* multiple ce value case */
        } else {
            let mut layers: libc::c_int = 0 as libc::c_int;
            let mut comp: *mut crate::src::htp_decompressors::htp_decompressor_t =
                0 as *mut crate::src::htp_decompressors::htp_decompressor_t;
            let mut tok: *mut uint8_t = 0 as *mut uint8_t;
            let mut tok_len: size_t = 0 as libc::c_int as size_t;
            let mut input: *mut uint8_t = if (*(*ce).value).realptr.is_null() {
                ((*ce).value as *mut libc::c_uchar)
                    .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
            } else {
                (*(*ce).value).realptr
            };
            let mut input_len: size_t = (*(*ce).value).len;
            while input_len > 0 as libc::c_int as libc::c_ulong
                && get_token(
                    input,
                    input_len,
                    b", \x00" as *const u8 as *const libc::c_char,
                    &mut tok,
                    &mut tok_len,
                ) != 0
            {
                let mut cetype: crate::src::htp_decompressors::htp_content_encoding_t =
                    crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
                /* check depth limit (0 means no limit) */
                if (*(*(*tx).connp).cfg).response_decompression_layer_limit != 0 as libc::c_int && {
                    layers += 1;
                    (layers) > (*(*(*tx).connp).cfg).response_decompression_layer_limit
                } {
                    htp_log(
                        (*tx).connp,
                        b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
                        1265 as libc::c_int,
                        crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
                        0 as libc::c_int,
                        b"Too many response content encoding layers\x00" as *const u8
                            as *const libc::c_char,
                    );
                    break;
                } else {
                    if bstr_util_mem_index_of_c_nocase(
                        tok as *const libc::c_void,
                        tok_len,
                        b"gzip\x00" as *const u8 as *const libc::c_char,
                    ) != -(1 as libc::c_int)
                    {
                        if !(bstr_util_cmp_mem(
                            tok as *const libc::c_void,
                            tok_len,
                            b"gzip\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            4 as libc::c_int as size_t,
                        ) == 0 as libc::c_int
                            || bstr_util_cmp_mem(
                                tok as *const libc::c_void,
                                tok_len,
                                b"x-gzip\x00" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                6 as libc::c_int as size_t,
                            ) == 0 as libc::c_int)
                        {
                            htp_log(
                                (*tx).connp,
                                b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
                                1273 as libc::c_int,
                                crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
                                0 as libc::c_int,
                                b"C-E gzip has abnormal value\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        cetype = crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
                    } else if bstr_util_mem_index_of_c_nocase(
                        tok as *const libc::c_void,
                        tok_len,
                        b"deflate\x00" as *const u8 as *const libc::c_char,
                    ) != -(1 as libc::c_int)
                    {
                        if !(bstr_util_cmp_mem(
                            tok as *const libc::c_void,
                            tok_len,
                            b"deflate\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            7 as libc::c_int as size_t,
                        ) == 0 as libc::c_int
                            || bstr_util_cmp_mem(
                                tok as *const libc::c_void,
                                tok_len,
                                b"x-deflate\x00" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                9 as libc::c_int as size_t,
                            ) == 0 as libc::c_int)
                        {
                            htp_log(
                                (*tx).connp,
                                b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
                                1280 as libc::c_int,
                                crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
                                0 as libc::c_int,
                                b"C-E deflate has abnormal value\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        cetype = crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
                    } else if bstr_util_cmp_mem(
                        tok as *const libc::c_void,
                        tok_len,
                        b"lzma\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        4 as libc::c_int as size_t,
                    ) == 0 as libc::c_int
                    {
                        cetype = crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
                    } else if bstr_util_cmp_mem(
                        tok as *const libc::c_void,
                        tok_len,
                        b"inflate\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        7 as libc::c_int as size_t,
                    ) == 0 as libc::c_int
                    {
                        cetype = crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE
                    } else {
                        // continue
                        htp_log(
                            (*tx).connp,
                            b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
                            1290 as libc::c_int,
                            crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
                            0 as libc::c_int,
                            b"C-E unknown setting\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if cetype != crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE
                    {
                        if comp.is_null() {
                            (*tx).response_content_encoding_processing = cetype;
                            (*(*tx).connp).out_decompressor = htp_gzip_decompressor_create(
                                (*tx).connp,
                                (*tx).response_content_encoding_processing,
                            );
                            if (*(*tx).connp).out_decompressor.is_null() {
                                return -(1 as libc::c_int);
                            }
                            (*(*(*tx).connp).out_decompressor).callback = Some(
                                htp_tx_res_process_body_data_decompressor_callback
                                    as unsafe extern "C" fn(_: *mut htp_tx_data_t) -> htp_status_t,
                            );
                            comp = (*(*tx).connp).out_decompressor
                        } else {
                            (*comp).next = htp_gzip_decompressor_create((*tx).connp, cetype);
                            if (*comp).next.is_null() {
                                return -(1 as libc::c_int);
                            }
                            (*(*comp).next).callback = Some(
                                htp_tx_res_process_body_data_decompressor_callback
                                    as unsafe extern "C" fn(_: *mut htp_tx_data_t) -> htp_status_t,
                            );
                            comp = (*comp).next
                        }
                    }
                    if tok_len.wrapping_add(1 as libc::c_int as libc::c_ulong) >= input_len {
                        break;
                    }
                    input = input
                        .offset(tok_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                    input_len = (input_len as libc::c_ulong)
                        .wrapping_sub(tok_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                        as size_t as size_t
                }
            }
        }
    } else if (*tx).response_content_encoding_processing
        != crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE
    {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/* *
 * Change transaction state to RESPONSE_START and invoke registered callbacks.
 *
 * @param[in] tx Transaction pointer. Must not be NULL.
 * @return HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
 *         callbacks does not want to follow the transaction any more.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_response_start(mut tx: *mut htp_tx_t) -> htp_status_t {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    (*(*tx).connp).out_tx = tx;
    // Run hook RESPONSE_START.
    let mut rc: htp_status_t = htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_response_start,
        tx as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    // Change state into response line parsing, except if we're following
    // a HTTP/0.9 request (no status line or response headers).
    if (*tx).is_protocol_0_9 != 0 {
        (*tx).response_transfer_coding = htp_transfer_coding_t::HTP_CODING_IDENTITY;
        (*tx).response_content_encoding_processing =
            crate::src::htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
        (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_BODY;
        (*(*tx).connp).out_state = Some(
            htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        );
        (*(*tx).connp).out_body_data_left = -(1 as libc::c_int) as int64_t
    } else {
        (*(*tx).connp).out_state = Some(
            htp_connp_RES_LINE
                as unsafe extern "C" fn(
                    _: *mut crate::src::htp_connection_parser::htp_connp_t,
                ) -> htp_status_t,
        );
        (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_LINE
    }
    /* If at this point we have no method and no uri and our status
     * is still htp_connp_REQ_LINE, we likely have timed out request
     * or a overly long request */
    if (*tx).request_method.is_null()
        && (*tx).request_uri.is_null()
        && (*(*tx).connp).in_state
            == Some(
                htp_connp_REQ_LINE
                    as unsafe extern "C" fn(
                        _: *mut crate::src::htp_connection_parser::htp_connp_t,
                    ) -> htp_status_t,
            )
    {
        htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const libc::c_char,
            1352 as libc::c_int,
            crate::src::htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"Request line incomplete\x00" as *const u8 as *const libc::c_char,
        );
        if htp_connp_REQ_LINE_complete((*tx).connp) != 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 1 as libc::c_int;
}

/* *
 * Register callback for the transaction-specific REQUEST_BODY_DATA hook.
 *
 * @param[in] tx
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_register_request_body_data(
    mut tx: *mut htp_tx_t,
    mut callback_fn: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> libc::c_int>,
) {
    if tx.is_null() || callback_fn.is_none() {
        return;
    }
    htp_hook_register(
        &mut (*tx).hook_request_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> libc::c_int>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/* *
 * Register callback for the transaction-specific RESPONSE_BODY_DATA hook.
 *
 * @param[in] tx
 * @param[in] callback_fn
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_register_response_body_data(
    mut tx: *mut htp_tx_t,
    mut callback_fn: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> libc::c_int>,
) {
    if tx.is_null() || callback_fn.is_none() {
        return;
    }
    htp_hook_register(
        &mut (*tx).hook_response_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> libc::c_int>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

#[no_mangle]
pub unsafe extern "C" fn htp_tx_is_complete(mut tx: *mut htp_tx_t) -> libc::c_int {
    if tx.is_null() {
        return -(1 as libc::c_int);
    }
    // A transaction is considered complete only when both the request and
    // response are complete. (Sometimes a complete response can be seen
    // even while the request is ongoing.)
    if (*tx).request_progress != htp_tx_req_progress_t::HTP_REQUEST_COMPLETE
        || (*tx).response_progress != htp_tx_res_progress_t::HTP_RESPONSE_COMPLETE
    {
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int;
    };
}
