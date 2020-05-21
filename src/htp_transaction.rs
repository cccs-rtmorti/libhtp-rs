use crate::htp_util::Flags;
use crate::{
    bstr, htp_config, htp_connection, htp_connection_parser, htp_cookies, htp_decompressors,
    htp_hooks, htp_list, htp_multipart, htp_parsers, htp_request, htp_response, htp_table,
    htp_urlencoded, htp_util, Status,
};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
/// A collection of possible data sources.
pub enum htp_data_source_t {
    /// Embedded in the URL.
    HTP_SOURCE_URL,
    /// Transported in the query string.
    HTP_SOURCE_QUERY_STRING,
    /// Cookies.
    HTP_SOURCE_COOKIE,
    /// Transported in the request body.
    HTP_SOURCE_BODY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
/// A collection of unique parser IDs.
pub enum htp_parser_id_t {
    /// application/x-www-form-urlencoded parser.
    HTP_PARSER_URLENCODED,
    /// multipart/form-data parser.
    HTP_PARSER_MULTIPART,
}

/// Represents a single request parameter.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_param_t {
    /// Parameter name.
    pub name: *mut bstr::bstr_t,
    /// Parameter value.
    pub value: *mut bstr::bstr_t,
    /// Source of the parameter, for example HTP_SOURCE_QUERY_STRING.
    pub source: htp_data_source_t,
    /// Type of the data structure referenced below.
    pub parser_id: htp_parser_id_t,
    /// Pointer to the parser data structure that contains
    /// complete information about the parameter. Can be NULL.
    pub parser_data: *mut core::ffi::c_void,
}

/// This structure is used to pass transaction data (for example
/// request and response body buffers) to callbacks.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_tx_data_t {
    /// Transaction pointer.
    pub tx: *mut htp_tx_t,
    /// Pointer to the data buffer.
    pub data: *const u8,
    /// Buffer length.
    pub len: usize,
    /// Indicator if this chunk of data is the last in the series. Currently
    /// used only by REQUEST_HEADER_DATA, REQUEST_TRAILER_DATA, RESPONSE_HEADER_DATA,
    /// and RESPONSE_TRAILER_DATA callbacks.
    pub is_last: i32,
}

/// Enumerates the possible request and response body codings.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_transfer_coding_t {
    /// Body coding not determined yet.
    HTP_CODING_UNKNOWN,
    /// No body.
    HTP_CODING_NO_BODY,
    /// Identity coding is used, which means that the body was sent as is.
    HTP_CODING_IDENTITY,
    /// Chunked encoding.
    HTP_CODING_CHUNKED,
    /// We could not recognize the encoding.
    HTP_CODING_INVALID,
}

/// Represents a single HTTP transaction, which is a combination of a request and a response.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_tx_t {
    /// The connection parser associated with this transaction.
    pub connp: *mut htp_connection_parser::htp_connp_t,
    /// The connection to which this transaction belongs.
    pub conn: *mut htp_connection::htp_conn_t,
    /// The configuration structure associated with this transaction.
    pub cfg: *mut htp_config::htp_cfg_t,
    /// Is the configuration structure shared with other transactions or connections? If
    /// this field is set to HTP_CONFIG_PRIVATE, the transaction owns the configuration.
    pub is_config_shared: i32,
    /// The user data associated with this transaction.
    pub user_data: *mut core::ffi::c_void,

    // Request fields
    /// Contains a count of how many empty lines were skipped before the request line.
    pub request_ignored_lines: u32,
    /// The first line of this request.
    pub request_line: *mut bstr::bstr_t,
    /// Request method.
    pub request_method: *mut bstr::bstr_t,
    /// Request method, as number. Available only if we were able to recognize the request method.
    pub request_method_number: u32,
    /// Request URI, raw, as given to us on the request line. This field can take different forms,
    /// for example authority for CONNECT methods, absolute URIs for proxy requests, and the query
    /// string when one is provided. Use htp_tx_t::parsed_uri if you need to access to specific
    /// URI elements. Can be NULL if the request line contains only a request method (which is
    /// an extreme case of HTTP/0.9, but passes in practice.
    pub request_uri: *mut bstr::bstr_t,
    /// Request protocol, as text. Can be NULL if no protocol was specified.
    pub request_protocol: *mut bstr::bstr_t,
    /// Protocol version as a number. Multiply the high version number by 100, then add the low
    /// version number. You should prefer to work the pre-defined Protocol constants.
    pub request_protocol_number: i32,
    /// Is this request using HTTP/0.9? We need a separate field for this purpose because
    /// the protocol version alone is not sufficient to determine if HTTP/0.9 is used. For
    /// example, if you submit "GET / HTTP/0.9" to Apache, it will not treat the request
    /// as HTTP/0.9.
    pub is_protocol_0_9: i32,
    /// This structure holds the individual components parsed out of the request URI, with
    /// appropriate normalization and transformation applied, per configuration. No information
    /// is added. In extreme cases when no URI is provided on the request line, all fields
    /// will be NULL. (Well, except for port_number, which will be -1.) To inspect raw data, use
    /// htp_tx_t::request_uri or htp_tx_t::parsed_uri_raw.
    pub parsed_uri: *mut htp_util::htp_uri_t,
    /// This structure holds the individual components parsed out of the request URI, but
    /// without any modification. The purpose of this field is to allow you to look at the data as it
    /// was supplied on the request line. Fields can be NULL, depending on what data was supplied.
    /// The port_number field is always -1.
    pub parsed_uri_raw: *mut htp_util::htp_uri_t,
    /// HTTP 1.1 RFC
    ///
    /// 4.3 Message Body
    ///
    /// The message-body (if any) of an HTTP message is used to carry the
    /// entity-body associated with the request or response. The message-body
    /// differs from the entity-body only when a transfer-coding has been
    /// applied, as indicated by the Transfer-Encoding header field (section
    /// 14.41).
    ///
    /// ```text
    ///     message-body = entity-body
    ///                  | <entity-body encoded as per Transfer-Encoding>
    /// ```
    ///
    /// The length of the request message-body. In most cases, this value
    /// will be the same as request_entity_len. The values will be different
    /// if request compression or chunking were applied. In that case,
    /// request_message_len contains the length of the request body as it
    /// has been seen over TCP; request_entity_len contains length after
    /// de-chunking and decompression.
    pub request_message_len: i64,
    /// The length of the request entity-body. In most cases, this value
    /// will be the same as request_message_len. The values will be different
    /// if request compression or chunking were applied. In that case,
    /// request_message_len contains the length of the request body as it
    /// has been seen over TCP; request_entity_len contains length after
    /// de-chunking and decompression.
    pub request_entity_len: i64,
    /// Parsed request headers.
    pub request_headers: *mut htp_table::htp_table_t,
    /// Request transfer coding. Can be one of HTP_CODING_UNKNOWN (body presence not
    /// determined yet), HTP_CODING_IDENTITY, HTP_CODING_CHUNKED, HTP_CODING_NO_BODY,
    /// and HTP_CODING_UNRECOGNIZED.
    pub request_transfer_coding: htp_transfer_coding_t,
    /// Request body compression.
    pub request_content_encoding: htp_decompressors::htp_content_encoding_t,
    /// This field contain the request content type when that information is
    /// available in request headers. The contents of the field will be converted
    /// to lowercase and any parameters (e.g., character set information) removed.
    pub request_content_type: *mut bstr::bstr_t,
    /// Contains the value specified in the Content-Length header. The value of this
    /// field will be -1 from the beginning of the transaction and until request
    /// headers are processed. It will stay -1 if the C-L header was not provided,
    /// or if the value in it cannot be parsed.
    pub request_content_length: i64,
    /// Transaction-specific REQUEST_BODY_DATA hook. Behaves as
    /// the configuration hook with the same name.
    pub hook_request_body_data: *mut htp_hooks::htp_hook_t,
    /// Transaction-specific RESPONSE_BODY_DATA hook. Behaves as
    /// the configuration hook with the same name.
    pub hook_response_body_data: *mut htp_hooks::htp_hook_t,
    /// Query string URLENCODED parser. Available only
    /// when the query string is not NULL and not empty.
    pub request_urlenp_query: *mut htp_urlencoded::htp_urlenp_t,
    /// Request body URLENCODED parser. Available only when the request body is in the
    /// application/x-www-form-urlencoded format and the parser was configured to run.
    pub request_urlenp_body: *mut htp_urlencoded::htp_urlenp_t,
    /// Request body MULTIPART parser. Available only when the body is in the
    /// multipart/form-data format and the parser was configured to run.
    pub request_mpartp: *mut htp_multipart::htp_mpartp_t,
    /// Request parameters.
    pub request_params: *mut htp_table::htp_table_t,
    /// Request cookies
    pub request_cookies: *mut htp_table::htp_table_t,
    /// Authentication type used in the request.
    pub request_auth_type: htp_auth_type_t,
    /// Authentication username.
    pub request_auth_username: *mut bstr::bstr_t,
    /// Authentication password. Available only when htp_tx_t::request_auth_type is HTP_AUTH_BASIC.
    pub request_auth_password: *mut bstr::bstr_t,
    /// Request hostname. Per the RFC, the hostname will be taken from the Host header
    /// when available. If the host information is also available in the URI, it is used
    /// instead of whatever might be in the Host header. Can be NULL. This field does
    /// not contain port information.
    pub request_hostname: *mut bstr::bstr_t,
    /// Request port number, if presented. The rules for htp_tx_t::request_host apply. Set to
    /// -1 by default.
    pub request_port_number: i32,

    // Response fields
    /// How many empty lines did we ignore before reaching the status line?
    pub response_ignored_lines: u32,
    /// Response line.
    pub response_line: *mut bstr::bstr_t,
    /// Response protocol, as text. Can be NULL.
    pub response_protocol: *mut bstr::bstr_t,
    /// Response protocol as number. Available only if we were able to parse the protocol version,
    /// INVALID otherwise. UNKNOWN until parsing is attempted.
    pub response_protocol_number: i32,
    /// Response status code, as text. Starts as NULL and can remain NULL on
    /// an invalid response that does not specify status code.
    pub response_status: *mut bstr::bstr_t,
    /// Response status code, available only if we were able to parse it, HTP_STATUS_INVALID
    /// otherwise. HTP_STATUS_UNKNOWN until parsing is attempted.
    pub response_status_number: i32,
    /// This field is set by the protocol decoder with it thinks that the
    /// backend server will reject a request with a particular status code.
    pub response_status_expected_number: i32,
    /// The message associated with the response status code. Can be NULL.
    pub response_message: *mut bstr::bstr_t,
    /// Have we seen the server respond with a 100 response?
    pub seen_100continue: i32,
    /// Parsed response headers. Contains instances of htp_header_t.
    pub response_headers: *mut htp_table::htp_table_t,

    /// HTTP 1.1 RFC
    ///
    /// 4.3 Message Body
    ///
    /// The message-body (if any) of an HTTP message is used to carry the
    /// entity-body associated with the request or response. The message-body
    /// differs from the entity-body only when a transfer-coding has been
    /// applied, as indicated by the Transfer-Encoding header field (section
    /// 14.41).
    ///
    /// ```text
    ///     message-body = entity-body
    ///                  | <entity-body encoded as per Transfer-Encoding>
    /// ```
    ///
    /// The length of the response message-body. In most cases, this value
    /// will be the same as response_entity_len. The values will be different
    /// if response compression or chunking were applied. In that case,
    /// response_message_len contains the length of the response body as it
    /// has been seen over TCP; response_entity_len contains the length after
    /// de-chunking and decompression.
    pub response_message_len: i64,
    /// The length of the response entity-body. In most cases, this value
    /// will be the same as response_message_len. The values will be different
    /// if request compression or chunking were applied. In that case,
    /// response_message_len contains the length of the response body as it
    /// has been seen over TCP; response_entity_len contains length after
    /// de-chunking and decompression.
    pub response_entity_len: i64,
    /// Contains the value specified in the Content-Length header. The value of this
    /// field will be -1 from the beginning of the transaction and until response
    /// headers are processed. It will stay -1 if the C-L header was not provided,
    /// or if the value in it cannot be parsed.
    pub response_content_length: i64,
    /// Response transfer coding, which indicates if there is a response body,
    /// and how it is transported (e.g., as-is, or chunked).
    pub response_transfer_coding: htp_transfer_coding_t,
    /// Response body compression, which indicates if compression is used
    /// for the response body. This field is an interpretation of the information
    /// available in response headers.
    pub response_content_encoding: htp_decompressors::htp_content_encoding_t,
    /// Response body compression processing information, which is related to how
    /// the library is going to process (or has processed) a response body. Changing
    /// this field mid-processing can influence library actions. For example, setting
    /// this field to HTP_COMPRESSION_NONE in a RESPONSE_HEADERS callback will prevent
    /// decompression.
    pub response_content_encoding_processing: htp_decompressors::htp_content_encoding_t,
    /// This field will contain the response content type when that information
    /// is available in response headers. The contents of the field will be converted
    /// to lowercase and any parameters (e.g., character set information) removed.
    pub response_content_type: *mut bstr::bstr_t,

    // Common fields
    /// Parsing flags; a combination of: HTP_REQUEST_INVALID_T_E, HTP_INVALID_FOLDING,
    /// HTP_REQUEST_SMUGGLING, HTP_MULTI_PACKET_HEAD, and HTP_FIELD_UNPARSEABLE.
    pub flags: Flags,
    /// Request progress.
    pub request_progress: htp_tx_req_progress_t,
    /// Response progress.
    pub response_progress: htp_tx_res_progress_t,
    /// Transaction index on the connection.
    pub index: usize,
    /// Total repetitions for headers in request.
    pub req_header_repetitions: u16,
    /// Total repetitions for headers in response.
    pub res_header_repetitions: u16,
}

/// Possible states of a progressing transaction. Internally, progress will change
/// to the next state when the processing activities associated with that state
/// begin. For example, when we start to process request line bytes, the request
/// state will change from HTP_REQUEST_NOT_STARTED to HTP_REQUEST_LINE.*
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

/// Enumerates the possible values for authentication type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_auth_type_t {
    /// This is the default value that is used before
    /// the presence of authentication is determined (e.g.,
    /// before request headers are seen).
    HTP_AUTH_UNKNOWN,
    /// No authentication.
    HTP_AUTH_NONE,
    /// HTTP Basic authentication used.
    HTP_AUTH_BASIC,
    /// HTTP Digest authentication used.
    HTP_AUTH_DIGEST,
    /// Unrecognized authentication method.
    HTP_AUTH_UNRECOGNIZED = 9,
}

/// Protocol version constants
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Protocol {
    INVALID = -2,
    UNKNOWN = -1,
    V0_9 = 9,
    V1_0 = 100,
    V1_1 = 101,
}

/// Represents a single request or response header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_header_t {
    /// Header name.
    pub name: *mut bstr::bstr_t,
    /// Header value.
    pub value: *mut bstr::bstr_t,
    /// Parsing flags; a combination of: HTP_FIELD_INVALID, HTP_FIELD_FOLDED, HTP_FIELD_REPEATED.
    pub flags: Flags,
}
pub type htp_callback_fn_t = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> Status>;
pub type htp_alloc_strategy_t = u32;
pub const HTP_ALLOC_REUSE: htp_alloc_strategy_t = 2;
pub const HTP_ALLOC_COPY: htp_alloc_strategy_t = 1;

unsafe fn copy_or_wrap_mem(
    mut data: *const core::ffi::c_void,
    mut len: usize,
    mut alloc: htp_alloc_strategy_t,
) -> *mut bstr::bstr_t {
    if data == 0 as *mut core::ffi::c_void {
        return 0 as *mut bstr::bstr_t;
    }
    if alloc == HTP_ALLOC_REUSE {
        return bstr::bstr_wrap_mem(data, len);
    } else {
        return bstr::bstr_dup_mem(data, len);
    };
}

/// Creates a new transaction structure.
///
/// connp: Connection parser pointer. Must not be NULL.
///
/// Returns The newly created transaction, or NULL on memory allocation failure.
pub unsafe fn htp_tx_create(mut connp: *mut htp_connection_parser::htp_connp_t) -> *mut htp_tx_t {
    if connp.is_null() {
        return 0 as *mut htp_tx_t;
    }
    let mut tx: *mut htp_tx_t = calloc(1, ::std::mem::size_of::<htp_tx_t>()) as *mut htp_tx_t;
    if tx.is_null() {
        return 0 as *mut htp_tx_t;
    }
    (*tx).connp = connp;
    (*tx).conn = (*connp).conn;
    (*tx).index = htp_list::htp_list_array_size((*(*tx).conn).transactions);
    (*tx).cfg = (*connp).cfg;
    (*tx).is_config_shared = 1;
    // Request fields.
    (*tx).request_progress = htp_tx_req_progress_t::HTP_REQUEST_NOT_STARTED;
    (*tx).request_protocol_number = Protocol::UNKNOWN as i32;
    (*tx).request_content_length = -1;
    (*tx).parsed_uri_raw = htp_util::htp_uri_alloc();
    if (*tx).parsed_uri_raw.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    (*tx).request_headers = htp_table::htp_table_create(32);
    if (*tx).request_headers.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    (*tx).request_params = htp_table::htp_table_create(32);
    if (*tx).request_params.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    // Response fields.
    (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_NOT_STARTED;
    (*tx).response_status = 0 as *mut bstr::bstr_t;
    (*tx).response_status_number = 0;
    (*tx).response_protocol_number = Protocol::UNKNOWN as i32;
    (*tx).response_content_length = -1;
    (*tx).response_headers = htp_table::htp_table_create(32);
    if (*tx).response_headers.is_null() {
        htp_tx_destroy_incomplete(tx);
        return 0 as *mut htp_tx_t;
    }
    htp_list::htp_list_array_push((*(*tx).conn).transactions, tx as *mut core::ffi::c_void);
    return tx;
}

/// Destroys the supplied transaction.
pub unsafe fn htp_tx_destroy(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if htp_tx_is_complete(tx) == 0 {
        return Status::ERROR;
    }
    htp_tx_destroy_incomplete(tx);
    Status::OK
}

pub unsafe fn htp_tx_destroy_incomplete(mut tx: *mut htp_tx_t) {
    if tx.is_null() {
        return;
    }
    // Disconnect transaction from other structures.
    htp_connection::htp_conn_remove_tx((*tx).conn, tx);
    htp_connection_parser::htp_connp_tx_remove((*tx).connp, tx);
    // Request fields.
    bstr::bstr_free((*tx).request_line);
    bstr::bstr_free((*tx).request_method);
    bstr::bstr_free((*tx).request_uri);
    bstr::bstr_free((*tx).request_protocol);
    bstr::bstr_free((*tx).request_content_type);
    bstr::bstr_free((*tx).request_hostname);
    htp_util::htp_uri_free((*tx).parsed_uri_raw);
    htp_util::htp_uri_free((*tx).parsed_uri);
    bstr::bstr_free((*tx).request_auth_username);
    bstr::bstr_free((*tx).request_auth_password);
    // Request_headers.
    if !(*tx).request_headers.is_null() {
        let mut h: *mut htp_header_t = 0 as *mut htp_header_t;
        let mut i: usize = 0;
        let mut n: usize = htp_table::htp_table_size((*tx).request_headers);
        while i < n {
            h = htp_table::htp_table_get_index(
                (*tx).request_headers,
                i,
                0 as *mut *mut bstr::bstr_t,
            ) as *mut htp_header_t;
            bstr::bstr_free((*h).name);
            bstr::bstr_free((*h).value);
            free(h as *mut core::ffi::c_void);
            i = i.wrapping_add(1)
        }
        htp_table::htp_table_destroy((*tx).request_headers);
    }
    // Request parsers.
    htp_urlencoded::htp_urlenp_destroy((*tx).request_urlenp_query);
    htp_urlencoded::htp_urlenp_destroy((*tx).request_urlenp_body);
    htp_multipart::htp_mpartp_destroy((*tx).request_mpartp);
    // Request parameters.
    let mut param: *mut htp_param_t = 0 as *mut htp_param_t;
    let mut i_0: usize = 0;
    let mut n_0: usize = htp_table::htp_table_size((*tx).request_params);
    while i_0 < n_0 {
        param =
            htp_table::htp_table_get_index((*tx).request_params, i_0, 0 as *mut *mut bstr::bstr_t)
                as *mut htp_param_t;
        bstr::bstr_free((*param).name);
        bstr::bstr_free((*param).value);
        free(param as *mut core::ffi::c_void);
        i_0 = i_0.wrapping_add(1)
    }
    htp_table::htp_table_destroy((*tx).request_params);
    // Request cookies.
    if !(*tx).request_cookies.is_null() {
        let mut b: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
        let mut i_1: usize = 0;
        let mut n_1: usize = htp_table::htp_table_size((*tx).request_cookies);
        while i_1 < n_1 {
            b = htp_table::htp_table_get_index(
                (*tx).request_cookies,
                i_1,
                0 as *mut *mut bstr::bstr_t,
            ) as *mut bstr::bstr_t;
            bstr::bstr_free(b);
            i_1 = i_1.wrapping_add(1)
        }
        htp_table::htp_table_destroy((*tx).request_cookies);
    }
    htp_hooks::htp_hook_destroy((*tx).hook_request_body_data);
    // Response fields.
    bstr::bstr_free((*tx).response_line);
    bstr::bstr_free((*tx).response_protocol);
    bstr::bstr_free((*tx).response_status);
    bstr::bstr_free((*tx).response_message);
    bstr::bstr_free((*tx).response_content_type);
    // Destroy response headers.
    if !(*tx).response_headers.is_null() {
        let mut h_0: *mut htp_header_t = 0 as *mut htp_header_t;
        let mut i_2: usize = 0;
        let mut n_2: usize = htp_table::htp_table_size((*tx).response_headers);
        while i_2 < n_2 {
            h_0 = htp_table::htp_table_get_index(
                (*tx).response_headers,
                i_2,
                0 as *mut *mut bstr::bstr_t,
            ) as *mut htp_header_t;
            bstr::bstr_free((*h_0).name);
            bstr::bstr_free((*h_0).value);
            free(h_0 as *mut core::ffi::c_void);
            i_2 = i_2.wrapping_add(1)
        }
        htp_table::htp_table_destroy((*tx).response_headers);
    }
    // If we're using a private configuration structure, destroy it.
    if (*tx).is_config_shared == 0 {
        htp_config::htp_config_destroy((*tx).cfg);
    }
    free(tx as *mut core::ffi::c_void);
}

/// Returns the user data associated with this transaction.
pub unsafe fn htp_tx_get_user_data(mut tx: *const htp_tx_t) -> *mut core::ffi::c_void {
    if tx.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    return (*tx).user_data;
}

/// Associates user data with this transaction.
pub unsafe fn htp_tx_set_user_data(mut tx: *mut htp_tx_t, mut user_data: *mut core::ffi::c_void) {
    if tx.is_null() {
        return;
    }
    (*tx).user_data = user_data;
}

/// Adds one parameter to the request. THis function will take over the
/// responsibility for the provided htp_param_t structure.
///
/// tx: Transaction pointer. Must not be NULL.
/// param: Parameter pointer. Must not be NULL.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub unsafe fn htp_tx_req_add_param(mut tx: *mut htp_tx_t, mut param: *mut htp_param_t) -> Status {
    if tx.is_null() || param.is_null() {
        return Status::ERROR;
    }
    if (*(*tx).cfg).parameter_processor.is_some() {
        if (*(*tx).cfg)
            .parameter_processor
            .expect("non-null function pointer")(param)
            != Status::OK
        {
            return Status::ERROR;
        }
    }
    return htp_table::htp_table_addk(
        (*tx).request_params,
        (*param).name,
        param as *const core::ffi::c_void,
    );
}

/// Returns the first request parameter that matches the given name, using case-insensitive matching.
///
/// tx: Transaction pointer. Must not be NULL.
/// name: Name data pointer. Must not be NULL.
/// name_len: Name data length.
///
/// Returns htp_param_t instance, or NULL if parameter not found.
#[allow(dead_code)]
pub unsafe fn htp_tx_req_get_param(
    mut tx: *mut htp_tx_t,
    mut name: *const i8,
    mut name_len: usize,
) -> *mut htp_param_t {
    if tx.is_null() || name.is_null() {
        return 0 as *mut htp_param_t;
    }
    return htp_table::htp_table_get_mem(
        (*tx).request_params,
        name as *const core::ffi::c_void,
        name_len,
    ) as *mut htp_param_t;
}

/// Returns the first request parameter from the given source that matches the given name,
/// using case-insensitive matching.
///
/// tx: Transaction pointer. Must not be NULL.
/// source: Parameter source (where in request the parameter was located).
/// name: Name data pointer. Must not be NULL.
/// name_len: Name data length.
///
/// Returns htp_param_t instance, or NULL if parameter not found.
#[allow(dead_code)]
pub unsafe fn htp_tx_req_get_param_ex(
    mut tx: *mut htp_tx_t,
    mut source: htp_data_source_t,
    mut name: *const i8,
    mut name_len: usize,
) -> *mut htp_param_t {
    if tx.is_null() || name.is_null() {
        return 0 as *mut htp_param_t;
    }
    let mut p: *mut htp_param_t = 0 as *mut htp_param_t;
    let mut i: usize = 0;
    let mut n: usize = htp_table::htp_table_size((*tx).request_params);
    while i < n {
        p = htp_table::htp_table_get_index((*tx).request_params, i, 0 as *mut *mut bstr::bstr_t)
            as *mut htp_param_t;
        if !((*p).source != source) {
            if bstr::bstr_cmp_mem_nocase((*p).name, name as *const core::ffi::c_void, name_len) == 0
            {
                return p;
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut htp_param_t;
}

/// Determine if the request has a body.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns 1 if there is a body, 0 otherwise.
pub unsafe fn htp_tx_req_has_body(mut tx: *const htp_tx_t) -> i32 {
    if tx.is_null() {
        return -1;
    }
    if (*tx).request_transfer_coding == htp_transfer_coding_t::HTP_CODING_IDENTITY
        || (*tx).request_transfer_coding == htp_transfer_coding_t::HTP_CODING_CHUNKED
    {
        return 1;
    }
    return 0;
}

/// Set one request header. This function should be invoked once for
/// each available header, and in the order in which headers were
/// seen in the request.
///
/// tx: Transaction pointer. Must not be NULL.
/// name: Name data pointer. Must not be NULL.
/// name_len: Name data length.
/// value: Value data pointer. Must not be NULL.
/// value_len: Value data length.
/// alloc: Desired allocation strategy.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[allow(dead_code)]
pub unsafe fn htp_tx_req_set_header(
    mut tx: *mut htp_tx_t,
    mut name: *const i8,
    mut name_len: usize,
    mut value: *const i8,
    mut value_len: usize,
    mut alloc: htp_alloc_strategy_t,
) -> Status {
    if tx.is_null() || name.is_null() || value.is_null() {
        return Status::ERROR;
    }
    let mut h: *mut htp_header_t =
        calloc(1, ::std::mem::size_of::<htp_header_t>()) as *mut htp_header_t;
    if h.is_null() {
        return Status::ERROR;
    }
    (*h).name = copy_or_wrap_mem(name as *const core::ffi::c_void, name_len, alloc);
    if (*h).name.is_null() {
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    (*h).value = copy_or_wrap_mem(value as *const core::ffi::c_void, value_len, alloc);
    if (*h).value.is_null() {
        bstr::bstr_free((*h).name);
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    if htp_table::htp_table_add(
        (*tx).request_headers,
        (*h).name,
        h as *const core::ffi::c_void,
    ) != Status::OK
    {
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    Status::OK
}

unsafe fn htp_tx_process_request_headers(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Determine if we have a request body, and how it is packaged.
    let mut rc: Status = Status::OK;
    let mut cl: *mut htp_header_t = htp_table::htp_table_get_c(
        (*tx).request_headers,
        b"content-length\x00" as *const u8 as *const i8,
    ) as *mut htp_header_t;
    let mut te: *mut htp_header_t = htp_table::htp_table_get_c(
        (*tx).request_headers,
        b"transfer-encoding\x00" as *const u8 as *const i8,
    ) as *mut htp_header_t;
    // Check for the Transfer-Encoding header, which would indicate a chunked request body.
    if !te.is_null() {
        // Make sure it contains "chunked" only.
        // TODO The HTTP/1.1 RFC also allows the T-E header to contain "identity", which
        //      presumably should have the same effect as T-E header absence. However, Apache
        //      (2.2.22 on Ubuntu 12.04 LTS) instead errors out with "Unknown Transfer-Encoding: identity".
        //      And it behaves strangely, too, sending a 501 and proceeding to process the request
        //      (e.g., PHP is run), but without the body. It then closes the connection.
        if bstr::bstr_cmp_c_nocase((*te).value, b"chunked\x00" as *const u8 as *const i8) != 0 {
            // Invalid T-E header value.
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_INVALID;
            (*tx).flags |= Flags::HTP_REQUEST_INVALID_T_E;
            (*tx).flags |= Flags::HTP_REQUEST_INVALID
        } else {
            // Chunked encoding is a HTTP/1.1 feature, so check that an earlier protocol
            // version is not used. The flag will also be set if the protocol could not be parsed.
            //
            // TODO IIS 7.0, for example, would ignore the T-E header when it
            //      it is used with a protocol below HTTP 1.1. This should be a
            //      personality trait.
            if (*tx).request_protocol_number < Protocol::V1_1 as i32 {
                (*tx).flags |= Flags::HTP_REQUEST_INVALID_T_E;
                (*tx).flags |= Flags::HTP_REQUEST_SMUGGLING;
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
                (*tx).flags |= Flags::HTP_REQUEST_SMUGGLING
            }
        }
    } else if !cl.is_null() {
        // Check for a folded C-L header.
        if (*cl).flags.contains(Flags::HTP_FIELD_FOLDED) {
            (*tx).flags |= Flags::HTP_REQUEST_SMUGGLING
        }
        // Check for multiple C-L headers.
        if (*cl).flags.contains(Flags::HTP_FIELD_REPEATED) {
            (*tx).flags |= Flags::HTP_REQUEST_SMUGGLING
            // TODO Personality trait to determine which C-L header to parse.
            //      At the moment we're parsing the combination of all instances,
            //      which is bound to fail (because it will contain commas).
        }
        // Get the body length.
        (*tx).request_content_length = htp_util::htp_parse_content_length((*cl).value, (*tx).connp);
        if (*tx).request_content_length < 0 {
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_INVALID;
            (*tx).flags |= Flags::HTP_REQUEST_INVALID_C_L;
            (*tx).flags |= Flags::HTP_REQUEST_INVALID
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
        (*tx).flags |= Flags::HTP_REQUEST_INVALID
    }
    // Check for PUT requests, which we need to treat as file uploads.
    if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_PUT as u32 {
        if htp_tx_req_has_body(tx) != 0 {
            // Prepare to treat PUT request body as a file.
            (*(*tx).connp).put_file = calloc(1, ::std::mem::size_of::<htp_util::htp_file_t>())
                as *mut htp_util::htp_file_t;
            if (*(*tx).connp).put_file.is_null() {
                return Status::ERROR;
            }
            (*(*(*tx).connp).put_file).fd = -1;
            (*(*(*tx).connp).put_file).source = htp_util::htp_file_source_t::HTP_FILE_PUT
        }
    }
    // Determine hostname.
    // Use the hostname from the URI, when available.
    if !(*(*tx).parsed_uri).hostname.is_null() {
        (*tx).request_hostname = bstr::bstr_dup((*(*tx).parsed_uri).hostname);
        if (*tx).request_hostname.is_null() {
            return Status::ERROR;
        }
    }
    (*tx).request_port_number = (*(*tx).parsed_uri).port_number;
    // Examine the Host header.
    let mut h: *mut htp_header_t =
        htp_table::htp_table_get_c((*tx).request_headers, b"host\x00" as *const u8 as *const i8)
            as *mut htp_header_t;
    if h.is_null() {
        // No host information in the headers.
        // HTTP/1.1 requires host information in the headers.
        if (*tx).request_protocol_number >= Protocol::V1_1 as i32 {
            (*tx).flags |= Flags::HTP_HOST_MISSING
        }
    } else {
        // Host information available in the headers.
        let mut hostname: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
        let mut port: i32 = 0;
        rc = htp_util::htp_parse_header_hostport(
            (*h).value,
            &mut hostname,
            0 as *mut *mut bstr::bstr_t,
            &mut port,
            &mut (*tx).flags,
        );
        if rc != Status::OK {
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
                if bstr::bstr_cmp_nocase(hostname, (*tx).request_hostname) != 0 {
                    (*tx).flags |= Flags::HTP_HOST_AMBIGUOUS
                }
                // Check for different ports.
                if (*tx).request_port_number != -1
                    && port != -1
                    && (*tx).request_port_number != port
                {
                    (*tx).flags |= Flags::HTP_HOST_AMBIGUOUS
                }
                bstr::bstr_free(hostname);
            }
        } else if !(*tx).request_hostname.is_null() {
            // Invalid host information in the headers.
            // Raise the flag, even though the host information in the headers is invalid.
            (*tx).flags |= Flags::HTP_HOST_AMBIGUOUS
        }
    }
    // Determine Content-Type.
    let mut ct: *mut htp_header_t = htp_table::htp_table_get_c(
        (*tx).request_headers,
        b"content-type\x00" as *const u8 as *const i8,
    ) as *mut htp_header_t;
    if !ct.is_null() {
        rc = htp_util::htp_parse_ct_header((*ct).value, &mut (*tx).request_content_type);
        if rc != Status::OK {
            return rc;
        }
    }
    // Parse cookies.
    if (*(*(*tx).connp).cfg).parse_request_cookies != 0 {
        rc = htp_cookies::htp_parse_cookies_v0((*tx).connp);
        if rc != Status::OK {
            return rc;
        }
    }
    // Parse authentication information.
    if (*(*(*tx).connp).cfg).parse_request_auth != 0 {
        rc = htp_parsers::htp_parse_authorization((*tx).connp);
        if rc == Status::DECLINED {
            // Don't fail the stream if an authorization header is invalid, just set a flag.
            (*tx).flags |= Flags::HTP_AUTH_INVALID
        } else if rc != Status::OK {
            return rc;
        }
    }
    // Finalize sending raw header data.
    rc = htp_request::htp_connp_req_receiver_finalize_clear((*tx).connp);
    if rc != Status::OK {
        return rc;
    }
    // Run hook REQUEST_HEADERS.
    rc = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_headers,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // We cannot proceed if the request is invalid.
    if (*tx).flags.contains(Flags::HTP_REQUEST_INVALID) {
        return Status::ERROR;
    }
    return Status::OK;
}

/// Process a chunk of request body data. This function assumes that
/// handling of chunked encoding is implemented by the container. When
/// you're done submitting body data, invoke a state change (to REQUEST)
/// to finalize any processing that might be pending. The supplied data is
/// fully consumed and there is no expectation that it will be available
/// afterwards. The protocol parsing code makes no copies of the data,
/// but some parsers might.
///
/// tx: Transaction pointer. Must not be NULL.
/// data: Data pointer. Must not be NULL.
/// len: Data length.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[allow(dead_code)]
pub unsafe fn htp_tx_req_process_body_data(
    mut tx: *mut htp_tx_t,
    mut data: *const core::ffi::c_void,
    mut len: usize,
) -> Status {
    if tx.is_null() || data == 0 as *mut core::ffi::c_void {
        return Status::ERROR;
    }
    if len == 0 {
        return Status::OK;
    }
    return htp_tx_req_process_body_data_ex(tx, data, len);
}

pub unsafe fn htp_tx_req_process_body_data_ex(
    mut tx: *mut htp_tx_t,
    mut data: *const core::ffi::c_void,
    mut len: usize,
) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // NULL data is allowed in this private function; it's
    // used to indicate the end of request body.
    // Keep track of the body length.
    (*tx).request_entity_len = ((*tx).request_entity_len as u64).wrapping_add(len as u64) as i64;
    // Send data to the callbacks.
    let mut d: htp_tx_data_t = htp_tx_data_t {
        tx: 0 as *mut htp_tx_t,
        data: 0 as *const u8,
        len: 0,
        is_last: 0,
    };
    d.tx = tx;
    d.data = data as *mut u8;
    d.len = len;
    let mut rc: Status = htp_util::htp_req_run_hook_body_data((*tx).connp, &mut d);
    if rc != Status::OK {
        htp_util::htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const i8,
            589,
            htp_util::htp_log_level_t::HTP_LOG_ERROR,
            0,
            b"Request body data callback returned error (%d)\x00" as *const u8 as *const i8,
            rc,
        );
        return Status::ERROR;
    }
    return Status::OK;
}

/// Set request line. When used, this function should always be called first,
/// with more specific functions following. Must not contain line terminators.
///
/// tx: Transaction pointer. Must not be NULL.
/// line: Line data pointer. Must not be NULL.
/// line_len: Line data length.
/// alloc: Desired allocation strategy.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[allow(dead_code)]
pub unsafe fn htp_tx_req_set_line(
    mut tx: *mut htp_tx_t,
    mut line: *const i8,
    mut line_len: usize,
    mut alloc: htp_alloc_strategy_t,
) -> Status {
    if tx.is_null() || line.is_null() || line_len == 0 {
        return Status::ERROR;
    }
    (*tx).request_line = copy_or_wrap_mem(line as *const core::ffi::c_void, line_len, alloc);
    if (*tx).request_line.is_null() {
        return Status::ERROR;
    }
    if (*(*(*tx).connp).cfg)
        .parse_request_line
        .expect("non-null function pointer")((*tx).connp)
        != Status::OK
    {
        return Status::ERROR;
    }
    return Status::OK;
}

/// Set parsed request URI. You don't need to use this function if you are already providing
/// the request line or request URI. But if your container already has this data available,
/// feeding it to LibHTP will minimize any potential data differences. This function assumes
/// management of the data provided in parsed_uri. This function will not change htp_tx_t::parsed_uri_raw
/// (which may have data in it from the parsing of the request URI).
///
/// tx: Transaction pointer. Must not be NULL.
/// parsed_uri: URI pointer. Must not be NULL.
#[allow(dead_code)]
pub unsafe fn htp_tx_req_set_parsed_uri(
    mut tx: *mut htp_tx_t,
    mut parsed_uri: *mut htp_util::htp_uri_t,
) {
    if tx.is_null() || parsed_uri.is_null() {
        return;
    }
    if !(*tx).parsed_uri.is_null() {
        htp_util::htp_uri_free((*tx).parsed_uri);
    }
    (*tx).parsed_uri = parsed_uri;
}

/// Set response line. Use this function is you have a single buffer containing
/// the entire line. If you have individual request line pieces, use the other
/// available functions.
///
/// tx: Transaction pointer. Must not be NULL.
/// line: Line data pointer. Must not be NULL.
/// line_len: Line data length.
/// alloc: Desired allocation strategy.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[allow(dead_code)]
pub unsafe fn htp_tx_res_set_status_line(
    mut tx: *mut htp_tx_t,
    mut line: *const i8,
    mut line_len: usize,
    mut alloc: htp_alloc_strategy_t,
) -> Status {
    if tx.is_null() || line.is_null() || line_len == 0 {
        return Status::ERROR;
    }
    (*tx).response_line = copy_or_wrap_mem(line as *const core::ffi::c_void, line_len, alloc);
    if (*tx).response_line.is_null() {
        return Status::ERROR;
    }
    if (*(*(*tx).connp).cfg)
        .parse_response_line
        .expect("non-null function pointer")((*tx).connp)
        != Status::OK
    {
        return Status::ERROR;
    }
    return Status::OK;
}

/// Change transaction state to HTP_RESPONSE_LINE and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_response_line(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Is the response line valid?
    if (*tx).response_protocol_number == Protocol::INVALID as i32 {
        htp_util::htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const i8,
            688,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0,
            b"Invalid response line: invalid protocol\x00" as *const u8 as *const i8,
        );
        (*tx).flags |= Flags::HTP_STATUS_LINE_INVALID
    }
    if (*tx).response_status_number == -1
        || (*tx).response_status_number < 100
        || (*tx).response_status_number > 999
    {
        htp_util::htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const i8,
            695,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0,
            b"Invalid response line: invalid response status %d.\x00" as *const u8 as *const i8,
            (*tx).response_status_number,
        );
        (*tx).response_status_number = -1;
        (*tx).flags |= Flags::HTP_STATUS_LINE_INVALID
    }
    // Run hook HTP_RESPONSE_LINE
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_response_line,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    return Status::OK;
}

/// Set one response header. This function should be invoked once for
/// each available header, and in the order in which headers were
/// seen in the response.
///
/// tx: Transaction pointer. Must not be NULL.
/// name: Name data pointer. Must not be NULL.
/// name_len: Name data length.
/// value: Value data pointer. Must not be NULL.
/// value_len: Value length.
/// alloc: Desired allocation strategy.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[allow(dead_code)]
pub unsafe fn htp_tx_res_set_header(
    mut tx: *mut htp_tx_t,
    mut name: *const i8,
    mut name_len: usize,
    mut value: *const i8,
    mut value_len: usize,
    mut alloc: htp_alloc_strategy_t,
) -> Status {
    if tx.is_null() || name.is_null() || value.is_null() {
        return Status::ERROR;
    }
    let mut h: *mut htp_header_t =
        calloc(1, ::std::mem::size_of::<htp_header_t>()) as *mut htp_header_t;
    if h.is_null() {
        return Status::ERROR;
    }
    (*h).name = copy_or_wrap_mem(name as *const core::ffi::c_void, name_len, alloc);
    if (*h).name.is_null() {
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    (*h).value = copy_or_wrap_mem(value as *const core::ffi::c_void, value_len, alloc);
    if (*h).value.is_null() {
        bstr::bstr_free((*h).name);
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    if htp_table::htp_table_add(
        (*tx).response_headers,
        (*h).name,
        h as *const core::ffi::c_void,
    ) != Status::OK
    {
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    return Status::OK;
}

pub unsafe fn htp_connp_destroy_decompressors(mut connp: *mut htp_connection_parser::htp_connp_t) {
    let mut comp: *mut htp_decompressors::htp_decompressor_t = (*connp).out_decompressor;
    while !comp.is_null() {
        let mut next: *mut htp_decompressors::htp_decompressor_t = (*comp).next;
        (*comp).destroy.expect("non-null function pointer")(comp);
        comp = next
    }
    (*connp).out_decompressor = 0 as *mut htp_decompressors::htp_decompressor_t;
}

/// Clean up decompressor(s).
unsafe fn htp_tx_res_destroy_decompressors(mut tx: *mut htp_tx_t) {
    htp_connp_destroy_decompressors((*tx).connp);
}

unsafe fn htp_timer_track(
    mut time_spent: *mut i32,
    mut after: *mut libc::timeval,
    mut before: *mut libc::timeval,
) -> Status {
    if (*after).tv_sec < (*before).tv_sec {
        return Status::ERROR;
    } else {
        if (*after).tv_sec == (*before).tv_sec {
            if (*after).tv_usec < (*before).tv_usec {
                return Status::ERROR;
            }
            *time_spent = *time_spent + ((*after).tv_usec - (*before).tv_usec) as i32
        } else {
            *time_spent = *time_spent
                + (((*after).tv_sec - (*before).tv_sec) * 1000000 + (*after).tv_usec
                    - (*before).tv_usec) as i32
        }
    }
    return Status::OK;
}

unsafe extern "C" fn htp_tx_res_process_body_data_decompressor_callback(
    mut d: *mut htp_tx_data_t,
) -> Status {
    if d.is_null() {
        return Status::ERROR;
    }
    // Keep track of actual response body length.
    (*(*d).tx).response_entity_len =
        ((*(*d).tx).response_entity_len as u64).wrapping_add((*d).len as u64) as i64;
    // Invoke all callbacks.
    let mut rc: Status = htp_util::htp_res_run_hook_body_data((*(*d).tx).connp, d);
    if rc != Status::OK {
        return Status::ERROR;
    }
    (*(*(*(*d).tx).connp).out_decompressor).nb_callbacks = (*(*(*(*d).tx).connp).out_decompressor)
        .nb_callbacks
        .wrapping_add(1);
    if (*(*(*(*d).tx).connp).out_decompressor)
        .nb_callbacks
        .wrapping_rem(256)
        == 0
    {
        let mut after: libc::timeval = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        libc::gettimeofday(&mut after, 0 as *mut libc::timezone);
        // sanity check for race condition if system time changed
        if htp_timer_track(
            &mut (*(*(*(*d).tx).connp).out_decompressor).time_spent,
            &mut after,
            &mut (*(*(*(*d).tx).connp).out_decompressor).time_before,
        ) == Status::OK
        {
            // updates last tracked time
            (*(*(*(*d).tx).connp).out_decompressor).time_before = after;
            if (*(*(*(*d).tx).connp).out_decompressor).time_spent
                > (*(*(*(*d).tx).connp).cfg).compression_time_limit
            {
                htp_util::htp_log(
                    (*(*d).tx).connp,
                    b"htp_transaction.c\x00" as *const u8 as *const i8,
                    814,
                    htp_util::htp_log_level_t::HTP_LOG_ERROR,
                    0,
                    b"Compression bomb: spent %ld us decompressing\x00" as *const u8 as *const i8,
                    (*(*(*(*d).tx).connp).out_decompressor).time_spent,
                );
                return Status::ERROR;
            }
        }
    }
    if (*(*d).tx).response_entity_len > (*(*(*(*d).tx).connp).cfg).compression_bomb_limit as i64
        && (*(*d).tx).response_entity_len > 2048 * (*(*d).tx).response_message_len
    {
        htp_util::htp_log(
            (*(*d).tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const i8,
            794,
            htp_util::htp_log_level_t::HTP_LOG_ERROR,
            0,
            b"Compression bomb: decompressed %ld bytes out of %ld\x00" as *const u8 as *const i8,
            (*(*d).tx).response_entity_len,
            (*(*d).tx).response_message_len,
        );
        return Status::ERROR;
    }
    return Status::OK;
}

/// Process a chunk of response body data. This function assumes that
/// handling of chunked encoding is implemented by the container. When
/// you're done submitting body data, invoking a state change (to RESPONSE)
/// will finalize any processing that might be pending.
///
/// The response body data will be decompressed if two conditions are met: one,
/// decompression is enabled in configuration and two, if the response headers
/// indicate compression. Alternatively, you can control decompression from
/// a RESPONSE_HEADERS callback, by setting tx->response_content_encoding either
/// to COMPRESSION_NONE (to disable compression), or to one of the supported
/// decompression algorithms.
///
/// tx: Transaction pointer. Must not be NULL.
/// data: Data pointer. Must not be NULL.
/// len: Data length.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[allow(dead_code)]
pub unsafe fn htp_tx_res_process_body_data(
    mut tx: *mut htp_tx_t,
    mut data: *const core::ffi::c_void,
    mut len: usize,
) -> Status {
    if tx.is_null() || data == 0 as *mut core::ffi::c_void {
        return Status::ERROR;
    }
    if len == 0 {
        return Status::OK;
    }
    return htp_tx_res_process_body_data_ex(tx, data, len);
}

pub unsafe fn htp_tx_res_process_body_data_ex(
    mut tx: *mut htp_tx_t,
    mut data: *const core::ffi::c_void,
    mut len: usize,
) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // NULL data is allowed in this private function; it's
    // used to indicate the end of response body.
    let mut d: htp_tx_data_t = htp_tx_data_t {
        tx: 0 as *mut htp_tx_t,
        data: 0 as *const u8,
        len: 0,
        is_last: 0,
    };
    d.tx = tx;
    d.data = data as *mut u8;
    d.len = len;
    d.is_last = 0;
    // Keep track of body size before decompression.
    (*tx).response_message_len =
        ((*tx).response_message_len as u64).wrapping_add(d.len as u64) as i64;
    let mut rc: Status = Status::DECLINED;
    match (*tx).response_content_encoding_processing as u32 {
        2 | 3 | 4 => {
            // In severe memory stress these could be NULL
            if (*(*tx).connp).out_decompressor.is_null()
                || (*(*(*tx).connp).out_decompressor).decompress.is_none()
            {
                return Status::ERROR;
            }
            let mut after: libc::timeval = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            libc::gettimeofday(
                &mut (*(*(*tx).connp).out_decompressor).time_before,
                0 as *mut libc::timezone,
            );
            // Send data buffer to the decompressor.
            (*(*(*tx).connp).out_decompressor)
                .decompress
                .expect("non-null function pointer")(
                (*(*tx).connp).out_decompressor, &mut d
            );
            libc::gettimeofday(&mut after, 0 as *mut libc::timezone);
            // sanity check for race condition if system time changed
            if htp_timer_track(
                &mut (*(*(*tx).connp).out_decompressor).time_spent,
                &mut after,
                &mut (*(*(*tx).connp).out_decompressor).time_before,
            ) == Status::OK
            {
                if (*(*(*tx).connp).out_decompressor).time_spent
                    > (*(*(*tx).connp).cfg).compression_time_limit
                {
                    htp_util::htp_log(
                        (*tx).connp,
                        b"htp_transaction.c\x00" as *const u8 as *const i8,
                        876,
                        htp_util::htp_log_level_t::HTP_LOG_ERROR,
                        0,
                        b"Compression bomb: spent %ld us decompressing\x00" as *const u8
                            as *const i8,
                        (*(*(*tx).connp).out_decompressor).time_spent,
                    );
                    return Status::ERROR;
                }
            }
            if data == 0 as *mut core::ffi::c_void {
                // Shut down the decompressor, if we used one.
                htp_tx_res_destroy_decompressors(tx);
            }
        }
        1 => {
            // When there's no decompression, response_entity_len.
            // is identical to response_message_len.
            (*tx).response_entity_len =
                ((*tx).response_entity_len as u64).wrapping_add(d.len as u64) as i64;
            rc = htp_util::htp_res_run_hook_body_data((*tx).connp, &mut d);
            if rc != Status::OK {
                return Status::ERROR;
            }
        }
        _ => {
            // Internal error.
            htp_util::htp_log(
                (*tx).connp,
                b"htp_transaction.c\x00" as *const u8 as *const i8,
                857,
                htp_util::htp_log_level_t::HTP_LOG_ERROR,
                0,
                b"[Internal Error] Invalid tx->response_content_encoding_processing value: %d\x00"
                    as *const u8 as *const i8,
                (*tx).response_content_encoding_processing as u32,
            );
            return Status::ERROR;
        }
    }
    return Status::OK;
}

pub unsafe fn htp_tx_state_request_complete_partial(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Finalize request body.
    if htp_tx_req_has_body(tx) != 0 {
        let mut rc: Status = htp_tx_req_process_body_data_ex(tx, 0 as *const core::ffi::c_void, 0);
        if rc != Status::OK {
            return rc;
        }
    }
    (*tx).request_progress = htp_tx_req_progress_t::HTP_REQUEST_COMPLETE;
    // Run hook REQUEST_COMPLETE.
    let mut rc_0: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_complete,
        tx as *mut core::ffi::c_void,
    );
    if rc_0 != Status::OK {
        return rc_0;
    }
    // Clean-up.
    if !(*(*tx).connp).put_file.is_null() {
        bstr::bstr_free((*(*(*tx).connp).put_file).filename);
        free((*(*tx).connp).put_file as *mut core::ffi::c_void);
        (*(*tx).connp).put_file = 0 as *mut htp_util::htp_file_t
    }
    return Status::OK;
}

/// Change transaction state to REQUEST and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_complete(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if (*tx).request_progress != htp_tx_req_progress_t::HTP_REQUEST_COMPLETE {
        let mut rc: Status = htp_tx_state_request_complete_partial(tx);
        if rc != Status::OK {
            return rc;
        }
    }
    // Make a copy of the connection parser pointer, so that
    // we don't have to reference it via tx, which may be
    // destroyed later.
    let mut connp: *mut htp_connection_parser::htp_connp_t = (*tx).connp;
    // Determine what happens next, and remove this transaction from the parser.
    if (*tx).is_protocol_0_9 != 0 {
        (*connp).in_state = Some(
            htp_request::htp_connp_REQ_IGNORE_DATA_AFTER_HTTP_0_9
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    } else {
        (*connp).in_state = Some(
            htp_request::htp_connp_REQ_IDLE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    }
    // Check if the entire transaction is complete. This call may
    // destroy the transaction, if auto-destroy is enabled.
    htp_tx_finalize(tx);
    // At this point, tx may no longer be valid.
    (*connp).in_tx = 0 as *mut htp_tx_t;
    return Status::OK;
}

/// Initialize hybrid parsing mode, change state to TRANSACTION_START,
/// and invoke all registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_start(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Run hook REQUEST_START.
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_start,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // Change state into request line parsing.
    (*(*tx).connp).in_state = Some(
        htp_request::htp_connp_REQ_LINE
            as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
    );
    (*(*(*tx).connp).in_tx).request_progress = htp_tx_req_progress_t::HTP_REQUEST_LINE;
    return Status::OK;
}

/// Change transaction state to REQUEST_HEADERS and invoke all
/// registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_headers(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // If we're in HTP_REQ_HEADERS that means that this is the
    // first time we're processing headers in a request. Otherwise,
    // we're dealing with trailing headers.
    if (*tx).request_progress > htp_tx_req_progress_t::HTP_REQUEST_HEADERS {
        // Request trailers.
        // Run hook HTP_REQUEST_TRAILER.
        let mut rc: Status = htp_hooks::htp_hook_run_all(
            (*(*(*tx).connp).cfg).hook_request_trailer,
            tx as *mut core::ffi::c_void,
        );
        if rc != Status::OK {
            return rc;
        }
        // Finalize sending raw header data.
        rc = htp_request::htp_connp_req_receiver_finalize_clear((*tx).connp);
        if rc != Status::OK {
            return rc;
        }
        // Completed parsing this request; finalize it now.
        (*(*tx).connp).in_state = Some(
            htp_request::htp_connp_REQ_FINALIZE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    } else if (*tx).request_progress >= htp_tx_req_progress_t::HTP_REQUEST_LINE {
        // Request headers.
        // Did this request arrive in multiple data chunks?
        if (*(*tx).connp).in_chunk_count != (*(*tx).connp).in_chunk_request_index {
            (*tx).flags |= Flags::HTP_MULTI_PACKET_HEAD
        }
        let mut rc_0: Status = htp_tx_process_request_headers(tx);
        if rc_0 != Status::OK {
            return rc_0;
        }
        (*(*tx).connp).in_state = Some(
            htp_request::htp_connp_REQ_CONNECT_CHECK
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    } else {
        htp_util::htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const i8,
            969,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0,
            b"[Internal Error] Invalid tx progress: %d\x00" as *const u8 as *const i8,
            (*tx).request_progress as u32,
        );
        return Status::ERROR;
    }
    return Status::OK;
}

/// Change transaction state to REQUEST_LINE and invoke all
/// registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_line(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Determine how to process the request URI.
    if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_CONNECT as u32 {
        // When CONNECT is used, the request URI contains an authority string.
        if htp_util::htp_parse_uri_hostport((*tx).connp, (*tx).request_uri, (*tx).parsed_uri_raw)
            != Status::OK
        {
            return Status::ERROR;
        }
    } else if htp_util::htp_parse_uri((*tx).request_uri, &mut (*tx).parsed_uri_raw) != Status::OK {
        return Status::ERROR;
    }
    // Parse the request URI into htp_tx_t::parsed_uri_raw.
    // Build htp_tx_t::parsed_uri, but only if it was not explicitly set already.
    if (*tx).parsed_uri.is_null() {
        (*tx).parsed_uri = htp_util::htp_uri_alloc();
        if (*tx).parsed_uri.is_null() {
            return Status::ERROR;
        }
        // Keep the original URI components, but create a copy which we can normalize and use internally.
        if htp_util::htp_normalize_parsed_uri(tx, (*tx).parsed_uri_raw, (*tx).parsed_uri) != 1 {
            return Status::ERROR;
        }
    }
    // Check parsed_uri hostname.
    if !(*(*tx).parsed_uri).hostname.is_null() {
        if htp_util::htp_validate_hostname((*(*tx).parsed_uri).hostname) == 0 {
            (*tx).flags |= Flags::HTP_HOSTU_INVALID
        }
    }
    // Run hook REQUEST_URI_NORMALIZE.
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_uri_normalize,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // Run hook REQUEST_LINE.
    rc = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_line,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // Move on to the next phase.
    (*(*tx).connp).in_state = Some(
        htp_request::htp_connp_REQ_PROTOCOL
            as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
    );
    return Status::OK;
}

/// Change transaction state to RESPONSE and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_response_complete(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    return htp_tx_state_response_complete_ex(tx, 1);
}

pub unsafe fn htp_tx_finalize(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if htp_tx_is_complete(tx) == 0 {
        return Status::OK;
    }
    // Run hook TRANSACTION_COMPLETE.
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_transaction_complete,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // In streaming processing, we destroy the transaction because it will not be needed any more.
    if (*(*(*tx).connp).cfg).tx_auto_destroy != 0 {
        htp_tx_destroy(tx);
    }
    return Status::OK;
}

pub unsafe fn htp_tx_state_response_complete_ex(
    mut tx: *mut htp_tx_t,
    mut hybrid_mode: i32,
) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if (*tx).response_progress != htp_tx_res_progress_t::HTP_RESPONSE_COMPLETE {
        (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_COMPLETE;
        // Run the last RESPONSE_BODY_DATA HOOK, but only if there was a response body present.
        if (*tx).response_transfer_coding != htp_transfer_coding_t::HTP_CODING_NO_BODY {
            htp_tx_res_process_body_data_ex(tx, 0 as *const core::ffi::c_void, 0);
        }
        // Run hook RESPONSE_COMPLETE.
        let mut rc: Status = htp_hooks::htp_hook_run_all(
            (*(*(*tx).connp).cfg).hook_response_complete,
            tx as *mut core::ffi::c_void,
        );
        if rc != Status::OK {
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
            == htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER
            && (*(*tx).connp).in_tx == (*(*tx).connp).out_tx
        {
            return Status::DATA_OTHER;
        }
        // Do we have a signal to yield to inbound processing at
        // the end of the next transaction?
        if (*(*tx).connp).out_data_other_at_tx_end != 0 {
            // We do. Let's yield then.
            (*(*tx).connp).out_data_other_at_tx_end = 0;
            return Status::DATA_OTHER;
        }
    }
    // Make a copy of the connection parser pointer, so that
    // we don't have to reference it via tx, which may be destroyed later.
    let mut connp: *mut htp_connection_parser::htp_connp_t = (*tx).connp;
    // Finalize the transaction. This may call may destroy the transaction, if auto-destroy is enabled.
    let mut rc_0: Status = htp_tx_finalize(tx);
    if rc_0 != Status::OK {
        return rc_0;
    }
    // Disconnect transaction from the parser.
    (*connp).out_tx = 0 as *mut htp_tx_t;
    (*connp).out_state = Some(
        htp_response::htp_connp_RES_IDLE
            as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
    );
    return Status::OK;
}

///  split input into tokens separated by "seps"
///
///  seps: nul-terminated string: each character is a separator
unsafe fn get_token(
    mut in_0: *const u8,
    mut in_len: usize,
    mut seps: *const i8,
    mut ret_tok_ptr: *mut *mut u8,
    mut ret_tok_len: *mut usize,
) -> i32 {
    let mut i: usize = 0;
    // skip leading 'separators'
    while i < in_len {
        let mut match_0: i32 = 0;
        let mut s: *const i8 = seps;
        while *s != '\u{0}' as i8 {
            if *in_0.offset(i as isize) as i32 == *s as i32 {
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
        return 0;
    }
    in_0 = in_0.offset(i as isize);
    in_len = (in_len).wrapping_sub(i);
    i = 0;
    while i < in_len {
        let mut s_0: *const i8 = seps;
        while *s_0 != '\u{0}' as i8 {
            if *in_0.offset(i as isize) as i32 == *s_0 as i32 {
                *ret_tok_ptr = in_0 as *mut u8;
                *ret_tok_len = i;
                return 1;
            }
            s_0 = s_0.offset(1)
        }
        i = i.wrapping_add(1)
    }
    *ret_tok_ptr = in_0 as *mut u8;
    *ret_tok_len = in_len;
    return 1;
}

/// Change transaction state to RESPONSE_HEADERS and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_response_headers(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Check for compression.
    // Determine content encoding.
    let mut ce_multi_comp: i32 = 0;
    (*tx).response_content_encoding =
        htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
    let mut ce: *mut htp_header_t = htp_table::htp_table_get_c(
        (*tx).response_headers,
        b"content-encoding\x00" as *const u8 as *const i8,
    ) as *mut htp_header_t;
    if !ce.is_null() {
        // fast paths: regular gzip and friends
        if bstr::bstr_cmp_c_nocasenorzero((*ce).value, b"gzip\x00" as *const u8 as *const i8) == 0
            || bstr::bstr_cmp_c_nocasenorzero((*ce).value, b"x-gzip\x00" as *const u8 as *const i8)
                == 0
        {
            (*tx).response_content_encoding =
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
        } else if bstr::bstr_cmp_c_nocasenorzero(
            (*ce).value,
            b"deflate\x00" as *const u8 as *const i8,
        ) == 0
            || bstr::bstr_cmp_c_nocasenorzero(
                (*ce).value,
                b"x-deflate\x00" as *const u8 as *const i8,
            ) == 0
        {
            (*tx).response_content_encoding =
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
        } else if bstr::bstr_cmp_c_nocasenorzero((*ce).value, b"lzma\x00" as *const u8 as *const i8)
            == 0
        {
            (*tx).response_content_encoding =
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
        } else if !(bstr::bstr_cmp_c_nocasenorzero(
            (*ce).value,
            b"inflate\x00" as *const u8 as *const i8,
        ) == 0)
        {
            // exceptional cases: enter slow path
            ce_multi_comp = 1
        }
    }
    // Configure decompression, if enabled in the configuration.
    if (*(*(*tx).connp).cfg).response_decompression_enabled != 0 {
        (*tx).response_content_encoding_processing = (*tx).response_content_encoding
    } else {
        (*tx).response_content_encoding_processing =
            htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
        ce_multi_comp = 0
    }
    // Finalize sending raw header data.
    let mut rc: Status = htp_response::htp_connp_res_receiver_finalize_clear((*tx).connp);
    if rc != Status::OK {
        return rc;
    }
    // Run hook RESPONSE_HEADERS.
    rc = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_response_headers,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
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
        == htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
        || (*tx).response_content_encoding_processing
            == htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
        || (*tx).response_content_encoding_processing
            == htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
        || ce_multi_comp != 0
    {
        if !(*(*tx).connp).out_decompressor.is_null() {
            htp_tx_res_destroy_decompressors(tx);
        }
        // normal case
        if ce_multi_comp == 0 {
            (*(*tx).connp).out_decompressor = htp_decompressors::htp_gzip_decompressor_create(
                (*tx).connp,
                (*tx).response_content_encoding_processing,
            );
            if (*(*tx).connp).out_decompressor.is_null() {
                return Status::ERROR;
            }
            (*(*(*tx).connp).out_decompressor).callback = Some(
                htp_tx_res_process_body_data_decompressor_callback
                    as unsafe extern "C" fn(_: *mut htp_tx_data_t) -> Status,
            )
        // multiple ce value case
        } else {
            let mut layers: i32 = 0;
            let mut comp: *mut htp_decompressors::htp_decompressor_t =
                0 as *mut htp_decompressors::htp_decompressor_t;
            let mut tok: *mut u8 = 0 as *mut u8;
            let mut tok_len: usize = 0;
            let mut input: *mut u8 = if (*(*ce).value).realptr.is_null() {
                ((*ce).value as *mut u8).offset(::std::mem::size_of::<bstr::bstr_t>() as isize)
            } else {
                (*(*ce).value).realptr
            };
            let mut input_len: usize = (*(*ce).value).len;
            while input_len > 0
                && get_token(
                    input,
                    input_len,
                    b", \x00" as *const u8 as *const i8,
                    &mut tok,
                    &mut tok_len,
                ) != 0
            {
                let mut cetype: htp_decompressors::htp_content_encoding_t =
                    htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
                // check depth limit (0 means no limit)
                if (*(*(*tx).connp).cfg).response_decompression_layer_limit != 0 && {
                    layers += 1;
                    (layers) > (*(*(*tx).connp).cfg).response_decompression_layer_limit
                } {
                    htp_util::htp_log(
                        (*tx).connp,
                        b"htp_transaction.c\x00" as *const u8 as *const i8,
                        1265,
                        htp_util::htp_log_level_t::HTP_LOG_WARNING,
                        0,
                        b"Too many response content encoding layers\x00" as *const u8 as *const i8,
                    );
                    break;
                } else {
                    if bstr::bstr_util_mem_index_of_c_nocase(
                        tok as *const core::ffi::c_void,
                        tok_len,
                        b"gzip\x00" as *const u8 as *const i8,
                    ) != -1
                    {
                        if !(bstr::bstr_util_cmp_mem(
                            tok as *const core::ffi::c_void,
                            tok_len,
                            b"gzip\x00" as *const u8 as *const i8 as *const core::ffi::c_void,
                            4,
                        ) == 0
                            || bstr::bstr_util_cmp_mem(
                                tok as *const core::ffi::c_void,
                                tok_len,
                                b"x-gzip\x00" as *const u8 as *const core::ffi::c_void,
                                6,
                            ) == 0)
                        {
                            htp_util::htp_log(
                                (*tx).connp,
                                b"htp_transaction.c\x00" as *const u8 as *const i8,
                                1273,
                                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                                0,
                                b"C-E gzip has abnormal value\x00" as *const u8 as *const i8,
                            );
                        }
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
                    } else if bstr::bstr_util_mem_index_of_c_nocase(
                        tok as *const core::ffi::c_void,
                        tok_len,
                        b"deflate\x00" as *const u8 as *const i8,
                    ) != -1
                    {
                        if !(bstr::bstr_util_cmp_mem(
                            tok as *const core::ffi::c_void,
                            tok_len,
                            b"deflate\x00" as *const u8 as *const core::ffi::c_void,
                            7,
                        ) == 0
                            || bstr::bstr_util_cmp_mem(
                                tok as *const core::ffi::c_void,
                                tok_len,
                                b"x-deflate\x00" as *const u8 as *const core::ffi::c_void,
                                9,
                            ) == 0)
                        {
                            htp_util::htp_log(
                                (*tx).connp,
                                b"htp_transaction.c\x00" as *const u8 as *const i8,
                                1280,
                                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                                0,
                                b"C-E deflate has abnormal value\x00" as *const u8 as *const i8,
                            );
                        }
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
                    } else if bstr::bstr_util_cmp_mem(
                        tok as *const core::ffi::c_void,
                        tok_len,
                        b"lzma\x00" as *const u8 as *const i8 as *const core::ffi::c_void,
                        4,
                    ) == 0
                    {
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
                    } else if bstr::bstr_util_cmp_mem(
                        tok as *const core::ffi::c_void,
                        tok_len,
                        b"inflate\x00" as *const u8 as *const i8 as *const core::ffi::c_void,
                        7,
                    ) == 0
                    {
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE
                    } else {
                        // continue
                        htp_util::htp_log(
                            (*tx).connp,
                            b"htp_transaction.c\x00" as *const u8 as *const i8,
                            1290,
                            htp_util::htp_log_level_t::HTP_LOG_WARNING,
                            0,
                            b"C-E unknown setting\x00" as *const u8 as *const i8,
                        );
                    }
                    if cetype != htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE {
                        if comp.is_null() {
                            (*tx).response_content_encoding_processing = cetype;
                            (*(*tx).connp).out_decompressor =
                                htp_decompressors::htp_gzip_decompressor_create(
                                    (*tx).connp,
                                    (*tx).response_content_encoding_processing,
                                );
                            if (*(*tx).connp).out_decompressor.is_null() {
                                return Status::ERROR;
                            }
                            (*(*(*tx).connp).out_decompressor).callback = Some(
                                htp_tx_res_process_body_data_decompressor_callback
                                    as unsafe extern "C" fn(_: *mut htp_tx_data_t) -> Status,
                            );
                            comp = (*(*tx).connp).out_decompressor
                        } else {
                            (*comp).next = htp_decompressors::htp_gzip_decompressor_create(
                                (*tx).connp,
                                cetype,
                            );
                            if (*comp).next.is_null() {
                                return Status::ERROR;
                            }
                            (*(*comp).next).callback = Some(
                                htp_tx_res_process_body_data_decompressor_callback
                                    as unsafe extern "C" fn(_: *mut htp_tx_data_t) -> Status,
                            );
                            comp = (*comp).next
                        }
                    }
                    if tok_len.wrapping_add(1) >= input_len {
                        break;
                    }
                    input = input.offset(tok_len.wrapping_add(1) as isize);
                    input_len = (input_len).wrapping_sub(tok_len.wrapping_add(1))
                }
            }
        }
    } else if (*tx).response_content_encoding_processing
        != htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE
    {
        return Status::ERROR;
    }
    return Status::OK;
}

/// Change transaction state to RESPONSE_START and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_response_start(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    (*(*tx).connp).out_tx = tx;
    // Run hook RESPONSE_START.
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_response_start,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // Change state into response line parsing, except if we're following
    // a HTTP/0.9 request (no status line or response headers).
    if (*tx).is_protocol_0_9 != 0 {
        (*tx).response_transfer_coding = htp_transfer_coding_t::HTP_CODING_IDENTITY;
        (*tx).response_content_encoding_processing =
            htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
        (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_BODY;
        (*(*tx).connp).out_state = Some(
            htp_response::htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        );
        (*(*tx).connp).out_body_data_left = -1
    } else {
        (*(*tx).connp).out_state = Some(
            htp_response::htp_connp_RES_LINE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        );
        (*tx).response_progress = htp_tx_res_progress_t::HTP_RESPONSE_LINE
    }
    // If at this point we have no method and no uri and our status
    // is still htp_request::htp_connp_REQ_LINE, we likely have timed out request
    // or a overly long request
    if (*tx).request_method.is_null()
        && (*tx).request_uri.is_null()
        && (*(*tx).connp).in_state
            == Some(
                htp_request::htp_connp_REQ_LINE
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            )
    {
        htp_util::htp_log(
            (*tx).connp,
            b"htp_transaction.c\x00" as *const u8 as *const i8,
            1352,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0,
            b"Request line incomplete\x00" as *const u8 as *const i8,
        );
    }
    return Status::OK;
}

/// Register callback for the transaction-specific REQUEST_BODY_DATA hook.
#[no_mangle]
pub unsafe fn htp_tx_register_request_body_data(
    mut tx: *mut htp_tx_t,
    mut callback_fn: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> i32>,
) {
    if tx.is_null() || callback_fn.is_none() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*tx).hook_request_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> i32>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Register callback for the transaction-specific RESPONSE_BODY_DATA hook.
#[no_mangle]
pub unsafe fn htp_tx_register_response_body_data(
    mut tx: *mut htp_tx_t,
    mut callback_fn: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> i32>,
) {
    if tx.is_null() || callback_fn.is_none() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*tx).hook_response_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> i32>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

pub unsafe fn htp_tx_is_complete(mut tx: *mut htp_tx_t) -> i32 {
    if tx.is_null() {
        return -1;
    }
    // A transaction is considered complete only when both the request and
    // response are complete. (Sometimes a complete response can be seen
    // even while the request is ongoing.)
    if (*tx).request_progress != htp_tx_req_progress_t::HTP_REQUEST_COMPLETE
        || (*tx).response_progress != htp_tx_res_progress_t::HTP_RESPONSE_COMPLETE
    {
        return 0;
    } else {
        return 1;
    };
}
