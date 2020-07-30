use crate::htp_util::Flags;
use crate::list::List;
use crate::{
    bstr, htp_config, htp_connection, htp_connection_parser, htp_cookies, htp_decompressors,
    htp_hooks, htp_multipart, htp_parsers, htp_request, htp_response, htp_table, htp_urlencoded,
    htp_util, Status,
};
use std::cmp::Ordering;

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
#[derive(Clone, Debug)]
pub struct htp_param_t {
    /// Parameter name.
    pub name: bstr::bstr_t,
    /// Parameter value.
    pub value: bstr::bstr_t,
    /// Source of the parameter, for example HTP_SOURCE_QUERY_STRING.
    pub source: htp_data_source_t,
    /// Type of the data structure referenced below.
    pub parser_id: htp_parser_id_t,
    /// Pointer to the parser data structure that contains
    /// complete information about the parameter. Can be NULL.
    pub parser_data: *mut core::ffi::c_void,
}

impl htp_param_t {
    /// Make a new owned htp_param_t
    pub fn new(
        name: bstr::bstr_t,
        value: bstr::bstr_t,
        source: htp_data_source_t,
        parser_id: htp_parser_id_t,
    ) -> Self {
        htp_param_t {
            name,
            value,
            source,
            parser_id,
            parser_data: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug, Clone)]
/// This structure is used to pass transaction data (for example
/// request and response body buffers) to callbacks.
pub struct htp_tx_data_t {
    /// Transaction pointer.
    tx: *mut htp_tx_t,
    /// Pointer to the data buffer.
    data: *const u8,
    /// Buffer length.
    len: usize,
    /// Indicator if this chunk of data is the last in the series. Currently
    /// used only by REQUEST_HEADER_DATA, REQUEST_TRAILER_DATA, RESPONSE_HEADER_DATA,
    /// and RESPONSE_TRAILER_DATA callbacks.
    is_last: bool,
}

impl htp_tx_data_t {
    pub unsafe fn new(tx: *mut htp_tx_t, data: *const u8, len: usize, is_last: bool) -> Self {
        Self {
            tx,
            data,
            len,
            is_last,
        }
    }

    pub fn tx(&self) -> *mut htp_tx_t {
        self.tx
    }

    pub fn data(&self) -> *const u8 {
        self.data
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_last(&self) -> bool {
        self.is_last
    }

    /// Get whether this data is empty.
    ///
    /// Returns true if data is NULL or zero-length.
    pub fn is_empty(&self) -> bool {
        self.data().is_null() || self.len() == 0
    }
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
    /// Error retrieving the transfer coding.
    HTP_CODING_ERROR,
}

/// Represents a single request or response header.
#[derive(Clone)]
pub struct htp_header_t {
    /// Header name.
    pub name: bstr::bstr_t,
    /// Header value.
    pub value: bstr::bstr_t,
    /// Parsing flags; a combination of: HTP_FIELD_INVALID, HTP_FIELD_FOLDED, HTP_FIELD_REPEATED.
    pub flags: Flags,
}

pub type htp_headers_t = htp_table::htp_table_t<htp_header_t>;

impl htp_header_t {
    pub fn new(name: bstr::bstr_t, value: bstr::bstr_t) -> Self {
        Self::new_with_flags(name, value, Flags::empty())
    }

    pub fn new_with_flags(name: bstr::bstr_t, value: bstr::bstr_t, flags: Flags) -> Self {
        Self { name, value, flags }
    }
}

/// Represents a single HTTP transaction, which is a combination of a request and a response.
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
    pub request_method_number: htp_request::htp_method_t,
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
    pub request_protocol_number: Protocol,
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
    pub request_headers: htp_headers_t,
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
    pub request_params: *mut htp_table::htp_table_t<htp_param_t>,
    /// Request cookies
    pub request_cookies: *mut htp_table::htp_table_t<*mut bstr::bstr_t>,
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
    pub response_protocol_number: Protocol,
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
    pub response_headers: htp_headers_t,

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

pub type htp_txs_t = List<htp_tx_t>;

impl htp_tx_t {
    pub unsafe fn new(connp: &mut htp_connection_parser::htp_connp_t) -> Result<usize, Status> {
        let tx = Self {
            connp,
            conn: connp.conn,
            cfg: connp.cfg,
            is_config_shared: 1,
            user_data: std::ptr::null_mut(),
            request_ignored_lines: 0,
            request_line: std::ptr::null_mut(),
            request_method: std::ptr::null_mut(),
            request_method_number: htp_request::htp_method_t::HTP_M_UNKNOWN,
            request_uri: std::ptr::null_mut(),
            request_protocol: std::ptr::null_mut(),
            request_protocol_number: Protocol::UNKNOWN,
            is_protocol_0_9: 0,
            parsed_uri: std::ptr::null_mut(),
            parsed_uri_raw: htp_util::htp_uri_alloc(),
            request_message_len: 0,
            request_entity_len: 0,
            request_headers: htp_table::htp_table_t::with_capacity(32),
            request_transfer_coding: htp_transfer_coding_t::HTP_CODING_UNKNOWN,
            request_content_encoding:
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_UNKNOWN,
            request_content_type: std::ptr::null_mut(),
            request_content_length: -1,
            hook_request_body_data: std::ptr::null_mut(),
            hook_response_body_data: std::ptr::null_mut(),
            request_urlenp_query: std::ptr::null_mut(),
            request_urlenp_body: std::ptr::null_mut(),
            request_mpartp: std::ptr::null_mut(),
            request_params: htp_table::htp_table_alloc(32),
            request_cookies: std::ptr::null_mut(),
            request_auth_type: htp_auth_type_t::HTP_AUTH_UNKNOWN,
            request_auth_username: std::ptr::null_mut(),
            request_auth_password: std::ptr::null_mut(),
            request_hostname: std::ptr::null_mut(),
            request_port_number: 0,
            response_ignored_lines: 0,
            response_line: std::ptr::null_mut(),
            response_protocol: std::ptr::null_mut(),
            response_protocol_number: Protocol::UNKNOWN,
            response_status: std::ptr::null_mut(),
            response_status_number: 0,
            response_status_expected_number: 0,
            response_message: std::ptr::null_mut(),
            seen_100continue: 0,
            response_headers: htp_table::htp_table_t::with_capacity(32),
            response_message_len: 0,
            response_entity_len: 0,
            response_content_length: -1,
            response_transfer_coding: htp_transfer_coding_t::HTP_CODING_UNKNOWN,
            response_content_encoding:
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_UNKNOWN,
            response_content_encoding_processing:
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_UNKNOWN,
            response_content_type: std::ptr::null_mut(),
            flags: Flags::empty(),
            request_progress: htp_tx_req_progress_t::HTP_REQUEST_NOT_STARTED,
            response_progress: htp_tx_res_progress_t::HTP_RESPONSE_NOT_STARTED,
            index: (*connp.conn).tx_size(),
            req_header_repetitions: 0,
            res_header_repetitions: 0,
        };
        if tx.parsed_uri_raw.is_null() {
            return Err(Status::ERROR);
        }
        let tx_id = tx.index;
        (*tx.conn).push_tx(tx);
        Ok(tx_id)
    }

    fn as_void_mut(&mut self) -> *mut std::ffi::c_void {
        (self as *mut htp_tx_t) as *mut std::ffi::c_void
    }
}

impl Drop for htp_tx_t {
    /// Destroys all the fields inside an htp_tx_t.
    fn drop(&mut self) {
        unsafe {
            // Request fields.
            bstr::bstr_free(self.request_line);
            bstr::bstr_free(self.request_method);
            bstr::bstr_free(self.request_uri);
            bstr::bstr_free(self.request_protocol);
            bstr::bstr_free(self.request_content_type);
            bstr::bstr_free(self.request_hostname);
            htp_util::htp_uri_free(self.parsed_uri_raw);
            htp_util::htp_uri_free(self.parsed_uri);
            bstr::bstr_free(self.request_auth_username);
            bstr::bstr_free(self.request_auth_password);

            // Request parsers.
            htp_urlencoded::htp_urlenp_destroy(self.request_urlenp_query);
            htp_urlencoded::htp_urlenp_destroy(self.request_urlenp_body);
            htp_multipart::htp_mpartp_destroy(self.request_mpartp);
            // Request parameters.
            htp_table::htp_table_free(self.request_params);

            // Request cookies.
            if !self.request_cookies.is_null() {
                for (_name, value) in (*self.request_cookies).elements.iter_mut() {
                    bstr::bstr_free(*value);
                }
                htp_table::htp_table_free(self.request_cookies);
            }

            htp_hooks::htp_hook_destroy(self.hook_request_body_data);
            // Response fields.
            bstr::bstr_free(self.response_line);
            bstr::bstr_free(self.response_protocol);
            bstr::bstr_free(self.response_status);
            bstr::bstr_free(self.response_message);
            bstr::bstr_free(self.response_content_type);

            // If we're using a private configuration structure, destroy it.
            if self.is_config_shared == 0 {
                (*self.cfg).destroy();
            }
        }
    }
}

impl PartialEq for htp_tx_t {
    fn eq(&self, other: &Self) -> bool {
        self.conn == other.conn && self.index == other.index
    }
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
    HTP_RESPONSE_ERROR,
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
    HTP_REQUEST_ERROR,
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
    /// Error retrieving the auth type.
    HTP_AUTH_ERROR,
}

/// Protocol version constants
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Protocol {
    ERROR = -3,
    INVALID = -2,
    UNKNOWN = -1,
    V0_9 = 9,
    V1_0 = 100,
    V1_1 = 101,
}

pub type htp_callback_fn_t = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> Status>;

/// Destroys the supplied transaction.
pub unsafe fn htp_tx_destroy(tx: *mut htp_tx_t) -> Status {
    if let Some(tx) = tx.as_mut() {
        if htp_tx_is_complete(tx) == 0 {
            return Status::ERROR;
        }
        // remove the tx from the connection so it will be dropped
        let _ = (*tx.conn).remove_tx(tx.index);
        Status::OK
    } else {
        Status::ERROR
    }
}

/// Returns the user data associated with this transaction.
pub unsafe fn htp_tx_user_data(tx: *const htp_tx_t) -> *mut core::ffi::c_void {
    if tx.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    (*tx).user_data
}

/// Associates user data with this transaction.
pub unsafe fn htp_tx_set_user_data(tx: *mut htp_tx_t, user_data: *mut core::ffi::c_void) {
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
pub unsafe fn htp_tx_req_add_param(tx: *mut htp_tx_t, mut param: htp_param_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if (*(*tx).cfg).parameter_processor.is_some()
        && (*(*tx).cfg)
            .parameter_processor
            .expect("non-null function pointer")(&mut param)
            != Status::OK
    {
        return Status::ERROR;
    }
    (*(*tx).request_params).add(param.name.clone(), param);
    Status::OK
}

/// Returns the first parameter inside the given table that matches the given name, using case-insensitive matching.
///
/// Returns htp_param_t instance, or None if parameter not found.
#[allow(dead_code)]
pub fn htp_tx_req_get_param<'a, S: AsRef<[u8]>>(
    params: &'a htp_table::htp_table_t<htp_param_t>,
    name: S,
) -> Option<&'a htp_param_t> {
    if let Some((_, param)) = params.get_nocase(name) {
        return Some(param);
    }
    None
}

/// Returns the first parameter inside the given table from the given source that matches the given name,
/// using case-insensitive matching.
///
/// Returns htp_param_t instance, or None if parameter not found.
#[allow(dead_code)]
pub fn htp_tx_req_get_param_ex<'a, S: AsRef<[u8]>>(
    params: &'a htp_table::htp_table_t<htp_param_t>,
    source: htp_data_source_t,
    name: S,
) -> Option<&htp_param_t> {
    if let Some((_, param)) = params.elements.iter().find(|x| {
        (*x).1.source as u32 == source as u32 && (*x).0.cmp_nocase(name.as_ref()) == Ordering::Equal
    }) {
        return Some(&param);
    }
    None
}

/// Determine if the request has a body.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns 1 if there is a body, 0 otherwise.
pub unsafe fn htp_tx_req_has_body(tx: *const htp_tx_t) -> i32 {
    if tx.is_null() {
        return -1;
    }
    if (*tx).request_transfer_coding == htp_transfer_coding_t::HTP_CODING_IDENTITY
        || (*tx).request_transfer_coding == htp_transfer_coding_t::HTP_CODING_CHUNKED
    {
        return 1;
    }
    0
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
pub unsafe fn htp_tx_req_set_header<S: AsRef<[u8]>>(
    tx: *mut htp_tx_t,
    name: S,
    value: S,
) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    (*tx).request_headers.add(
        name.as_ref().into(),
        htp_header_t::new(name.as_ref().into(), value.as_ref().into()),
    );
    Status::OK
}

unsafe fn htp_tx_process_request_headers(mut tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Determine if we have a request body, and how it is packaged.
    let mut rc: Status = Status::OK;
    let cl_opt = (*tx).request_headers.get_nocase_nozero("content-length");
    // Check for the Transfer-Encoding header, which would indicate a chunked request body.
    if let Some((_, te)) = (*tx).request_headers.get_nocase_nozero("transfer-encoding") {
        // Make sure it contains "chunked" only.
        // TODO The HTTP/1.1 RFC also allows the T-E header to contain "identity", which
        //      presumably should have the same effect as T-E header absence. However, Apache
        //      (2.2.22 on Ubuntu 12.04 LTS) instead errors out with "Unknown Transfer-Encoding: identity".
        //      And it behaves strangely, too, sending a 501 and proceeding to process the request
        //      (e.g., PHP is run), but without the body. It then closes the connection.
        if te.value.cmp_nocase("chunked") != Ordering::Equal {
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
            if (*tx).request_protocol_number < Protocol::V1_1 {
                (*tx).flags |= Flags::HTP_REQUEST_INVALID_T_E;
                (*tx).flags |= Flags::HTP_REQUEST_SMUGGLING;
            }
            // If the T-E header is present we are going to use it.
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_CHUNKED;
            // We are still going to check for the presence of C-L.
            if cl_opt.is_some() {
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
    } else if let Some((_, cl)) = cl_opt {
        // Check for a folded C-L header.
        if cl.flags.contains(Flags::HTP_FIELD_FOLDED) {
            (*tx).flags |= Flags::HTP_REQUEST_SMUGGLING
        }
        // Check for multiple C-L headers.
        if cl.flags.contains(Flags::HTP_FIELD_REPEATED) {
            (*tx).flags |= Flags::HTP_REQUEST_SMUGGLING
            // TODO Personality trait to determine which C-L header to parse.
            //      At the moment we're parsing the combination of all instances,
            //      which is bound to fail (because it will contain commas).
        }
        // Get the body length.
        if let Some(content_length) =
            htp_util::htp_parse_content_length((*(*cl).value).as_slice(), Some(&mut *(*tx).connp))
        {
            // We have a request body of known length.
            (*tx).request_content_length = content_length;
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_IDENTITY
        } else {
            (*tx).request_content_length = -1;
            (*tx).request_transfer_coding = htp_transfer_coding_t::HTP_CODING_INVALID;
            (*tx).flags |= Flags::HTP_REQUEST_INVALID_C_L;
            (*tx).flags |= Flags::HTP_REQUEST_INVALID
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
    if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_PUT
        && htp_tx_req_has_body(tx) != 0
    {
        // Prepare to treat PUT request body as a file.
        (*(*tx).connp).put_file =
            calloc(1, ::std::mem::size_of::<htp_util::htp_file_t>()) as *mut htp_util::htp_file_t;
        if (*(*tx).connp).put_file.is_null() {
            return Status::ERROR;
        }
        (*(*(*tx).connp).put_file).fd = -1;
        (*(*(*tx).connp).put_file).source = htp_util::htp_file_source_t::HTP_FILE_PUT
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
    if let Some((_, header)) = (*tx).request_headers.get_nocase_nozero_mut("host") {
        // Host information available in the headers.
        if let Ok((_, (hostname, port_nmb, valid))) =
            htp_util::htp_parse_hostport(&mut header.value)
        {
            if !valid {
                (*tx).flags |= Flags::HTP_HOSTH_INVALID
            }
            // The host information in the headers is valid.
            // Is there host information in the URI?
            if (*tx).request_hostname.is_null() {
                // There is no host information in the URI. Place the
                // hostname from the headers into the parsed_uri structure.
                (*tx).request_hostname = bstr::bstr_dup_str(hostname);
                bstr::bstr_to_lowercase((*tx).request_hostname);
                if let Some((_, Some(port))) = port_nmb {
                    (*tx).request_port_number = port as i32;
                }
            } else {
                // The host information appears in the URI and in the headers. The
                // HTTP RFC states that we should ignore the header copy.
                // Check for different hostnames.
                if (*(*tx).request_hostname).cmp_nocase(hostname) != Ordering::Equal {
                    (*tx).flags |= Flags::HTP_HOST_AMBIGUOUS
                }
                if let Some((_, Some(port))) = port_nmb {
                    // Check for different ports.
                    if (*tx).request_port_number != -1 && (*tx).request_port_number != port as i32 {
                        (*tx).flags |= Flags::HTP_HOST_AMBIGUOUS
                    }
                }
            }
        } else if !(*tx).request_hostname.is_null() {
            // Invalid host information in the headers.
            // Raise the flag, even though the host information in the headers is invalid.
            (*tx).flags |= Flags::HTP_HOST_AMBIGUOUS
        }
    } else {
        // No host information in the headers.
        // HTTP/1.1 requires host information in the headers.
        if (*tx).request_protocol_number >= Protocol::V1_1 {
            (*tx).flags |= Flags::HTP_HOST_MISSING
        }
    }
    // Determine Content-Type.
    if let Some((_, ct)) = (*tx).request_headers.get_nocase_nozero("content-type") {
        if (*tx).request_content_type.is_null() {
            (*tx).request_content_type = bstr::bstr_alloc(0);
            if (*tx).request_content_type.is_null() {
                return Status::ERROR;
            }
        }

        rc = htp_util::htp_parse_ct_header(&ct.value, &mut *(*tx).request_content_type);
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
    Status::OK
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
pub unsafe fn htp_tx_req_process_body_data<S: AsRef<[u8]>>(tx: *mut htp_tx_t, data: S) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if data.as_ref().len() == 0 {
        return Status::OK;
    }
    htp_tx_req_process_body_data_ex(
        tx,
        data.as_ref().as_ptr() as *const core::ffi::c_void,
        data.as_ref().len(),
    )
}

pub unsafe fn htp_tx_req_process_body_data_ex(
    tx: *mut htp_tx_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // NULL data is allowed in this private function; it's
    // used to indicate the end of request body.
    // Keep track of the body length.
    (*tx).request_entity_len = ((*tx).request_entity_len as u64).wrapping_add(len as u64) as i64;
    // Send data to the callbacks.
    let mut data = htp_tx_data_t::new(tx, data as *mut u8, len, false);
    let rc: Status = htp_util::htp_req_run_hook_body_data((*tx).connp, &mut data);
    if rc != Status::OK {
        htp_error!(
            (*tx).connp,
            htp_log_code::REQUEST_BODY_DATA_CALLBACK_ERROR,
            format!("Request body data callback returned error ({:?})", rc)
        );
        return Status::ERROR;
    }
    Status::OK
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
pub unsafe fn htp_tx_req_set_line<S: AsRef<[u8]>>(tx: *mut htp_tx_t, line: S) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    (*tx).request_line = bstr::bstr_dup_str(line);
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
    Status::OK
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
pub unsafe fn htp_tx_req_set_parsed_uri(tx: *mut htp_tx_t, parsed_uri: *mut htp_util::htp_uri_t) {
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
pub unsafe fn htp_tx_res_set_status_line<S: AsRef<[u8]>>(tx: *mut htp_tx_t, line: S) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    (*tx).response_line = bstr::bstr_dup_str(line);
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
    Status::OK
}

/// Change transaction state to HTP_RESPONSE_LINE and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_response_line(tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Is the response line valid?
    let connp = (*tx).connp;
    if (*tx).response_protocol_number == Protocol::INVALID {
        htp_warn!(
            connp,
            htp_log_code::RESPONSE_LINE_INVALID_PROTOCOL,
            "Invalid response line: invalid protocol"
        );
        (*tx).flags |= Flags::HTP_STATUS_LINE_INVALID
    }
    if (*tx).response_status_number == -1
        || (*tx).response_status_number < 100
        || (*tx).response_status_number > 999
    {
        htp_warn!(
            connp,
            htp_log_code::RESPONSE_LINE_INVALID_RESPONSE_STATUS,
            format!(
                "Invalid response line: invalid response status {}.",
                (*tx).response_status_number
            )
        );
        (*tx).response_status_number = -1;
        (*tx).flags |= Flags::HTP_STATUS_LINE_INVALID
    }
    // Run hook HTP_RESPONSE_LINE
    let rc: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_response_line,
        tx as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    Status::OK
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
pub unsafe fn htp_tx_res_set_header<S: AsRef<[u8]>>(
    tx: *mut htp_tx_t,
    name: S,
    value: S,
) -> Status {
    if let Some(tx) = tx.as_mut() {
        tx.response_headers.add(
            name.as_ref().into(),
            htp_header_t::new(name.as_ref().into(), value.as_ref().into()),
        );
        Status::OK
    } else {
        Status::ERROR
    }
}

pub unsafe fn htp_connp_destroy_decompressors(connp: *mut htp_connection_parser::htp_connp_t) {
    let mut comp: *mut htp_decompressors::htp_decompressor_t = (*connp).out_decompressor;
    while !comp.is_null() {
        let next: *mut htp_decompressors::htp_decompressor_t = (*comp).next;
        (*comp).destroy.expect("non-null function pointer")(comp);
        comp = next
    }
    (*connp).out_decompressor = 0 as *mut htp_decompressors::htp_decompressor_t;
}

/// Clean up decompressor(s).
unsafe fn htp_tx_res_destroy_decompressors(tx: *mut htp_tx_t) {
    htp_connp_destroy_decompressors((*tx).connp);
}

unsafe fn htp_timer_track(
    time_spent: *mut i32,
    after: *mut libc::timeval,
    before: *mut libc::timeval,
) -> Status {
    if (*after).tv_sec < (*before).tv_sec {
        return Status::ERROR;
    } else if (*after).tv_sec == (*before).tv_sec {
        if (*after).tv_usec < (*before).tv_usec {
            return Status::ERROR;
        }
        *time_spent = *time_spent + ((*after).tv_usec - (*before).tv_usec) as i32
    } else {
        *time_spent = *time_spent
            + (((*after).tv_sec - (*before).tv_sec) * 1000000 + (*after).tv_usec
                - (*before).tv_usec) as i32
    }
    Status::OK
}

unsafe extern "C" fn htp_tx_res_process_body_data_decompressor_callback(
    d: *mut htp_tx_data_t,
) -> Status {
    let d = if let Some(d) = d.as_mut() {
        d
    } else {
        return Status::ERROR;
    };
    let tx = if let Some(tx) = d.tx.as_mut() {
        tx
    } else {
        return Status::ERROR;
    };
    // Keep track of actual response body length.
    tx.response_entity_len = (tx.response_entity_len as u64).wrapping_add(d.len() as u64) as i64;
    // Invoke all callbacks.
    let rc: Status = htp_util::htp_res_run_hook_body_data(tx.connp, d);
    if rc != Status::OK {
        return Status::ERROR;
    }
    (*(*tx.connp).out_decompressor).nb_callbacks =
        (*(*tx.connp).out_decompressor).nb_callbacks.wrapping_add(1);

    if (*(*tx.connp).out_decompressor)
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
            &mut (*(*tx.connp).out_decompressor).time_spent,
            &mut after,
            &mut (*(*tx.connp).out_decompressor).time_before,
        ) == Status::OK
        {
            // updates last tracked time
            (*(*tx.connp).out_decompressor).time_before = after;
            if (*(*tx.connp).out_decompressor).time_spent
                > (*(*tx.connp).cfg).compression_time_limit
            {
                htp_error!(
                    tx.connp,
                    htp_log_code::COMPRESSION_BOMB,
                    format!(
                        "Compression bomb: spent {} us decompressing",
                        (*(*tx.connp).out_decompressor).time_spent
                    )
                );
                return Status::ERROR;
            }
        }
    }
    if tx.response_entity_len > (*(*tx.connp).cfg).compression_bomb_limit as i64
        && tx.response_entity_len > 2048 * tx.response_message_len
    {
        htp_error!(
            tx.connp,
            htp_log_code::COMPRESSION_BOMB,
            format!(
                "Compression bomb: decompressed {} bytes out of {}",
                tx.response_entity_len, tx.response_message_len
            )
        );
        return Status::ERROR;
    }
    Status::OK
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
pub unsafe fn htp_tx_res_process_body_data<S: AsRef<[u8]>>(tx: *mut htp_tx_t, data: S) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if data.as_ref().len() == 0 {
        return Status::OK;
    }
    htp_tx_res_process_body_data_ex(
        tx,
        data.as_ref().as_ptr() as *const core::ffi::c_void,
        data.as_ref().len(),
    )
}

pub unsafe fn htp_tx_res_process_body_data_ex(
    tx: *mut htp_tx_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // NULL data is allowed in this private function; it's
    // used to indicate the end of response body.
    let mut d = htp_tx_data_t::new(tx, data as *const u8, len, false);
    // Keep track of body size before decompression.
    (*tx).response_message_len =
        ((*tx).response_message_len as u64).wrapping_add(d.len as u64) as i64;
    let mut rc: Status = Status::DECLINED;
    let connp = (*tx).connp;
    match (*tx).response_content_encoding_processing as u32 {
        2 | 3 | 4 => {
            // In severe memory stress these could be NULL
            if (*connp).out_decompressor.is_null()
                || (*(*connp).out_decompressor).decompress.is_none()
            {
                return Status::ERROR;
            }
            let mut after: libc::timeval = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            libc::gettimeofday(
                &mut (*(*connp).out_decompressor).time_before,
                0 as *mut libc::timezone,
            );
            // Send data buffer to the decompressor.
            (*(*connp).out_decompressor)
                .decompress
                .expect("non-null function pointer")((*connp).out_decompressor, &mut d);
            libc::gettimeofday(&mut after, 0 as *mut libc::timezone);
            // sanity check for race condition if system time changed
            if htp_timer_track(
                &mut (*(*connp).out_decompressor).time_spent,
                &mut after,
                &mut (*(*connp).out_decompressor).time_before,
            ) == Status::OK
            {
                if (*(*connp).out_decompressor).time_spent > (*(*connp).cfg).compression_time_limit
                {
                    htp_error!(
                        connp,
                        htp_log_code::COMPRESSION_BOMB,
                        format!(
                            "Compression bomb: spent {} us decompressing",
                            (*(*(*tx).connp).out_decompressor).time_spent
                        )
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
            htp_error!(
                connp,
                htp_log_code::RESPONSE_BODY_INTERNAL_ERROR,
                format!(
                    "[Internal Error] Invalid tx->response_content_encoding_processing value: {:?}",
                    (*tx).response_content_encoding_processing
                )
            );
            return Status::ERROR;
        }
    }
    Status::OK
}

pub unsafe fn htp_tx_state_request_complete_partial(tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    // Finalize request body.
    if htp_tx_req_has_body(tx) != 0 {
        let rc: Status = htp_tx_req_process_body_data_ex(tx, 0 as *const core::ffi::c_void, 0);
        if rc != Status::OK {
            return rc;
        }
    }
    (*tx).request_progress = htp_tx_req_progress_t::HTP_REQUEST_COMPLETE;
    // Run hook REQUEST_COMPLETE.
    let rc_0: Status = htp_hooks::htp_hook_run_all(
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
    Status::OK
}

/// Change transaction state to REQUEST and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_complete(tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if (*tx).request_progress != htp_tx_req_progress_t::HTP_REQUEST_COMPLETE {
        let rc: Status = htp_tx_state_request_complete_partial(tx);
        if rc != Status::OK {
            return rc;
        }
    }
    // Make a copy of the connection parser pointer, so that
    // we don't have to reference it via tx, which may be
    // destroyed later.
    let connp: *mut htp_connection_parser::htp_connp_t = (*tx).connp;
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
    (*connp).clear_in_tx();
    Status::OK
}

/// Initialize hybrid parsing mode, change state to TRANSACTION_START,
/// and invoke all registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_start(tx: *mut htp_tx_t) -> Status {
    let tx = if let Some(tx) = tx.as_mut() {
        tx
    } else {
        return Status::ERROR;
    };
    let in_tx = if let Some(in_tx) = (*tx.connp).in_tx_mut() {
        in_tx
    } else {
        return Status::ERROR;
    };
    // Run hook REQUEST_START.
    let rc: Status =
        htp_hooks::htp_hook_run_all((*(*(*tx).connp).cfg).hook_request_start, tx.as_void_mut());
    if rc != Status::OK {
        return rc;
    }
    // Change state into request line parsing.
    (*(*tx).connp).in_state = Some(
        htp_request::htp_connp_REQ_LINE
            as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
    );
    in_tx.request_progress = htp_tx_req_progress_t::HTP_REQUEST_LINE;
    Status::OK
}

/// Change transaction state to REQUEST_HEADERS and invoke all
/// registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_headers(tx: *mut htp_tx_t) -> Status {
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
        let rc_0: Status = htp_tx_process_request_headers(tx);
        if rc_0 != Status::OK {
            return rc_0;
        }
        (*(*tx).connp).in_state = Some(
            htp_request::htp_connp_REQ_CONNECT_CHECK
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    } else {
        htp_warn!(
            (*tx).connp,
            htp_log_code::RESPONSE_BODY_INTERNAL_ERROR,
            format!(
                "[Internal Error] Invalid tx progress: {:?}",
                (*tx).request_progress
            )
        );
        return Status::ERROR;
    }
    Status::OK
}

/// Change transaction state to REQUEST_LINE and invoke all
/// registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_request_line(tx: *mut htp_tx_t) -> Status {
    let tx = if let Some(tx) = tx.as_mut() {
        tx
    } else {
        return Status::ERROR;
    };
    let in_tx = if let Some(in_tx) = (*tx.connp).in_tx_mut() {
        in_tx
    } else {
        return Status::ERROR;
    };
    // Determine how to process the request URI.
    if tx.request_method_number == htp_request::htp_method_t::HTP_M_CONNECT {
        // When CONNECT is used, the request URI contains an authority string.
        if tx.request_uri.is_null() || tx.parsed_uri_raw.is_null() {
            return Status::ERROR;
        }
        if htp_util::htp_parse_uri_hostport(
            &mut *tx.request_uri,
            &mut *tx.parsed_uri_raw,
            &mut in_tx.flags,
        ) != Status::OK
        {
            return Status::ERROR;
        }
    } else if htp_util::htp_parse_uri(tx.request_uri, &mut tx.parsed_uri_raw) != Status::OK {
        return Status::ERROR;
    }
    // Parse the request URI into htp_tx_t::parsed_uri_raw.
    // Build htp_tx_t::parsed_uri, but only if it was not explicitly set already.
    if tx.parsed_uri.is_null() {
        tx.parsed_uri = htp_util::htp_uri_alloc();
        if tx.parsed_uri.is_null() {
            return Status::ERROR;
        }
        // Keep the original URI components, but create a copy which we can normalize and use internally.
        if htp_util::htp_normalize_parsed_uri(tx, tx.parsed_uri_raw, tx.parsed_uri) != 1 {
            return Status::ERROR;
        }
    }
    // Check parsed_uri hostname.
    if !(*(*tx).parsed_uri).hostname.is_null()
        && !htp_util::htp_validate_hostname((*(*(*tx).parsed_uri).hostname).as_slice())
    {
        (*tx).flags |= Flags::HTP_HOSTU_INVALID
    }
    // Run hook REQUEST_URI_NORMALIZE.
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*(*tx).connp).cfg).hook_request_uri_normalize,
        tx.as_void_mut(),
    );
    if rc != Status::OK {
        return rc;
    }
    // Run hook REQUEST_LINE.
    rc = htp_hooks::htp_hook_run_all((*(*(*tx).connp).cfg).hook_request_line, tx.as_void_mut());
    if rc != Status::OK {
        return rc;
    }
    // Move on to the next phase.
    (*(*tx).connp).in_state = Some(
        htp_request::htp_connp_REQ_PROTOCOL
            as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
    );
    Status::OK
}

/// Change transaction state to RESPONSE and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_response_complete(tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    htp_tx_state_response_complete_ex(tx, 1)
}

pub unsafe fn htp_tx_finalize(tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    if htp_tx_is_complete(tx) == 0 {
        return Status::OK;
    }
    // Run hook TRANSACTION_COMPLETE.
    let rc: Status = htp_hooks::htp_hook_run_all(
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
    Status::OK
}

pub unsafe fn htp_tx_state_response_complete_ex(tx: *mut htp_tx_t, hybrid_mode: i32) -> Status {
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
        let rc: Status = htp_hooks::htp_hook_run_all(
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
            && (*(*tx).connp).in_tx() == (*(*tx).connp).out_tx()
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
    let connp: *mut htp_connection_parser::htp_connp_t = (*tx).connp;
    // Finalize the transaction. This may call may destroy the transaction, if auto-destroy is enabled.
    let rc_0: Status = htp_tx_finalize(tx);
    if rc_0 != Status::OK {
        return rc_0;
    }
    // Disconnect transaction from the parser.
    (*connp).clear_out_tx();
    (*connp).out_state = Some(
        htp_response::htp_connp_RES_IDLE
            as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
    );
    Status::OK
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
    let ce_opt = (*tx).response_headers.get_nocase_nozero("content-encoding");
    if let Some((_, ce)) = ce_opt {
        // fast paths: regular gzip and friends
        if ce.value.cmp_nocase_nozero("gzip") == Ordering::Equal
            || ce.value.cmp_nocase_nozero("x-gzip") == Ordering::Equal
        {
            (*tx).response_content_encoding =
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
        } else if ce.value.cmp_nocase_nozero("deflate") == Ordering::Equal
            || ce.value.cmp_nocase_nozero("x-deflate") == Ordering::Equal
        {
            (*tx).response_content_encoding =
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
        } else if ce.value.cmp_nocase_nozero("lzma") == Ordering::Equal {
            (*tx).response_content_encoding =
                htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
        } else if !(ce.value.cmp_nocase_nozero("inflate") == Ordering::Equal) {
            // exceptional cases: enter slow path
            ce_multi_comp = 1
        }
    }
    // Configure decompression, if enabled in the configuration.
    if (*(*(*tx).connp).cfg).response_decompression_enabled {
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
        } else if let Some((_, ce)) = ce_opt {
            let mut layers: i32 = 0;
            let mut comp: *mut htp_decompressors::htp_decompressor_t =
                0 as *mut htp_decompressors::htp_decompressor_t;
            let tokens = ce.value.split_str_collect(", ");
            let connp = (*tx).connp;
            for tok in tokens {
                let token = bstr::bstr_t::from(tok);
                let mut cetype: htp_decompressors::htp_content_encoding_t =
                    htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
                // check depth limit (0 means no limit)
                if (*(*connp).cfg).response_decompression_layer_limit != 0 && {
                    layers += 1;
                    (layers) > (*(*connp).cfg).response_decompression_layer_limit
                } {
                    htp_warn!(
                        connp,
                        htp_log_code::TOO_MANY_ENCODING_LAYERS,
                        "Too many response content encoding layers"
                    );
                    break;
                } else {
                    if token.index_of_nocase("gzip").is_some() {
                        if !(token.cmp("gzip") == Ordering::Equal
                            || token.cmp("x-gzip") == Ordering::Equal)
                        {
                            htp_warn!(
                                connp,
                                htp_log_code::ABNORMAL_CE_HEADER,
                                "C-E gzip has abnormal value"
                            );
                        }
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_GZIP
                    } else if token.index_of_nocase("deflate").is_some() {
                        if !(token.cmp("deflate") == Ordering::Equal
                            || token.cmp("x-deflate") == Ordering::Equal)
                        {
                            htp_warn!(
                                connp,
                                htp_log_code::ABNORMAL_CE_HEADER,
                                "C-E deflate has abnormal value"
                            );
                        }
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
                    } else if token.index_of_nocase("lzma").is_some() {
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_LZMA
                    } else if token.index_of_nocase("inflate").is_some() {
                        cetype = htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE
                    } else {
                        // continue
                        htp_warn!(
                            connp,
                            htp_log_code::ABNORMAL_CE_HEADER,
                            "C-E unknown setting"
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
                }
            }
        }
    } else if (*tx).response_content_encoding_processing
        != htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE
    {
        return Status::ERROR;
    }
    Status::OK
}

/// Change transaction state to RESPONSE_START and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
pub unsafe fn htp_tx_state_response_start(tx: *mut htp_tx_t) -> Status {
    if tx.is_null() {
        return Status::ERROR;
    }
    (*(*tx).connp).set_out_tx(&*tx);
    // Run hook RESPONSE_START.
    let rc: Status = htp_hooks::htp_hook_run_all(
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
        htp_warn!(
            (*tx).connp,
            htp_log_code::REQUEST_LINE_INCOMPLETE,
            "Request line incomplete"
        );
    }
    Status::OK
}

/// Register callback for the transaction-specific REQUEST_BODY_DATA hook.
#[no_mangle]
pub unsafe fn htp_tx_register_request_body_data(
    tx: *mut htp_tx_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> Status>,
) {
    if tx.is_null() || callback_fn.is_none() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*tx).hook_request_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Register callback for the transaction-specific RESPONSE_BODY_DATA hook.
#[no_mangle]
pub unsafe fn htp_tx_register_response_body_data(
    tx: *mut htp_tx_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> Status>,
) {
    if tx.is_null() || callback_fn.is_none() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*tx).hook_response_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

pub unsafe fn htp_tx_is_complete(tx: *mut htp_tx_t) -> i32 {
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
    }
    1
}
