use crate::connection_parser::State;
use crate::error::Result;
use crate::hook::{DataHook, DataNativeCallbackFn};
use crate::list::List;
use crate::util::Flags;
use crate::{
    bstr, config, connection_parser, decompressors, multipart, parsers, request, table, urlencoded,
    util, HtpStatus,
};
use std::cmp::Ordering;

/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
/// A collection of possible data sources.
pub enum HtpDataSource {
    /// Embedded in the URL.
    URL,
    /// Transported in the query string.
    QUERY_STRING,
    /// Cookies.
    COOKIE,
    /// Transported in the request body.
    BODY,
}

/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
/// A collection of unique parser IDs.
pub enum HtpParserId {
    /// application/x-www-form-urlencoded parser.
    URLENCODED,
    /// multipart/form-data parser.
    MULTIPART,
}

/// Represents a single request parameter.
#[derive(Clone, Debug)]
pub struct Param {
    /// Parameter name.
    pub name: bstr::Bstr,
    /// Parameter value.
    pub value: bstr::Bstr,
    /// Source of the parameter, for example QUERY_STRING.
    pub source: HtpDataSource,
    /// Type of the data structure referenced below.
    pub parser_id: HtpParserId,
    /// Pointer to the parser data structure that contains
    /// complete information about the parameter. Can be NULL.
    pub parser_data: *mut core::ffi::c_void,
}

impl Param {
    /// Make a new owned Param
    pub fn new(
        name: bstr::Bstr,
        value: bstr::Bstr,
        source: HtpDataSource,
        parser_id: HtpParserId,
    ) -> Self {
        Param {
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
pub struct Data<'a> {
    /// Transaction pointer.
    tx: *mut Transaction,
    /// Ref to the data buffer.
    data: Option<&'a [u8]>,
    /// Indicator if this chunk of data is the last in the series. Currently
    /// used only by REQUEST_HEADER_DATA, REQUEST_TRAILER_DATA, RESPONSE_HEADER_DATA,
    /// and RESPONSE_TRAILER_DATA callbacks.
    is_last: bool,
}

impl<'a> Data<'a> {
    pub fn new(tx: *mut Transaction, data: Option<&'a [u8]>, is_last: bool) -> Self {
        Self { tx, data, is_last }
    }

    pub fn tx(&self) -> *mut Transaction {
        self.tx
    }

    pub fn data(&self) -> *const u8 {
        self.data
            .as_ref()
            .map(|data| data.as_ptr())
            .unwrap_or(std::ptr::null())
    }

    pub fn len(&self) -> usize {
        self.data.as_ref().map(|data| data.len()).unwrap_or(0)
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

/// cbindgen:rename-all=QualifiedScreamingSnakeCase
/// Enumerates the possible request and response body codings.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HtpTransferCoding {
    /// Body coding not determined yet.
    UNKNOWN,
    /// No body.
    NO_BODY,
    /// Identity coding is used, which means that the body was sent as is.
    IDENTITY,
    /// Chunked encoding.
    CHUNKED,
    /// We could not recognize the encoding.
    INVALID,
    /// Error retrieving the transfer coding.
    ERROR,
}

/// Represents a single request or response header.
#[derive(Clone)]
pub struct Header {
    /// Header name.
    pub name: bstr::Bstr,
    /// Header value.
    pub value: bstr::Bstr,
    /// Parsing flags; a combination of: HTP_FIELD_INVALID, HTP_FIELD_FOLDED, HTP_FIELD_REPEATED.
    pub flags: Flags,
}

pub type htp_headers_t = table::Table<Header>;

impl Header {
    pub fn new(name: bstr::Bstr, value: bstr::Bstr) -> Self {
        Self::new_with_flags(name, value, Flags::empty())
    }

    pub fn new_with_flags(name: bstr::Bstr, value: bstr::Bstr, flags: Flags) -> Self {
        Self { name, value, flags }
    }
}

/// Possible states of a progressing transaction. Internally, progress will change
/// to the next state when the processing activities associated with that state
/// begin. For example, when we start to process request line bytes, the request
/// state will change from NOT_STARTED to LINE.*
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum HtpResponseProgress {
    NOT_STARTED,
    LINE,
    HEADERS,
    BODY,
    TRAILER,
    COMPLETE,
    ERROR,
}

/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum HtpRequestProgress {
    NOT_STARTED,
    LINE,
    HEADERS,
    BODY,
    TRAILER,
    COMPLETE,
    ERROR,
}

/// cbindgen:rename-all=QualifiedScreamingSnakeCase
/// Enumerates the possible values for authentication type.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HtpAuthType {
    /// This is the default value that is used before
    /// the presence of authentication is determined (e.g.,
    /// before request headers are seen).
    UNKNOWN,
    /// No authentication.
    NONE,
    /// HTTP Basic authentication used.
    BASIC,
    /// HTTP Digest authentication used.
    DIGEST,
    /// Unrecognized authentication method.
    UNRECOGNIZED = 9,
    /// Error retrieving the auth type.
    ERROR,
}

/// Protocol version constants
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum HtpProtocol {
    ERROR = -3,
    INVALID = -2,
    UNKNOWN = -1,
    V0_9 = 9,
    V1_0 = 100,
    V1_1 = 101,
}

/// Represents a single HTTP transaction, which is a combination of a request and a response.
pub struct Transaction {
    /// The connection parser associated with this transaction.
    pub connp: *mut connection_parser::ConnectionParser,
    /// The configuration structure associated with this transaction.
    pub cfg: *mut config::Config,
    /// Is the configuration structure shared with other transactions or connections? If
    /// this field is set to HTP_CONFIG_PRIVATE, the transaction owns the configuration.
    pub is_config_shared: bool,
    /// The user data associated with this transaction.
    pub user_data: *mut core::ffi::c_void,

    // Request fields
    /// Contains a count of how many empty lines were skipped before the request line.
    pub request_ignored_lines: u32,
    /// The first line of this request.
    pub request_line: Option<bstr::Bstr>,
    /// Request method.
    pub request_method: Option<bstr::Bstr>,
    /// Request method, as number. Available only if we were able to recognize the request method.
    pub request_method_number: request::HtpMethod,
    /// Request URI, raw, as given to us on the request line. This field can take different forms,
    /// for example authority for CONNECT methods, absolute URIs for proxy requests, and the query
    /// string when one is provided. Use Transaction::parsed_uri if you need to access to specific
    /// URI elements. Can be NULL if the request line contains only a request method (which is
    /// an extreme case of HTTP/0.9, but passes in practice.
    pub request_uri: Option<bstr::Bstr>,
    /// Request protocol, as text. Can be NULL if no protocol was specified.
    pub request_protocol: Option<bstr::Bstr>,
    /// Protocol version as a number. Multiply the high version number by 100, then add the low
    /// version number. You should prefer to work the pre-defined HtpProtocol constants.
    pub request_protocol_number: HtpProtocol,
    /// Is this request using HTTP/0.9? We need a separate field for this purpose because
    /// the protocol version alone is not sufficient to determine if HTTP/0.9 is used. For
    /// example, if you submit "GET / HTTP/0.9" to Apache, it will not treat the request
    /// as HTTP/0.9.
    pub is_protocol_0_9: bool,
    /// This structure holds the individual components parsed out of the request URI, with
    /// appropriate normalization and transformation applied, per configuration. No information
    /// is added. In extreme cases when no URI is provided on the request line, all fields
    /// will be NULL. (Well, except for port_number, which will be -1.) To inspect raw data, use
    /// Transaction::request_uri or Transaction::parsed_uri_raw.
    pub parsed_uri: Option<util::Uri>,
    /// This structure holds the individual components parsed out of the request URI, but
    /// without any modification. The purpose of this field is to allow you to look at the data as it
    /// was supplied on the request line. Fields can be NULL, depending on what data was supplied.
    /// The port_number field is always -1.
    pub parsed_uri_raw: Option<util::Uri>,
    ///  This structure holds the whole normalized uri, including path, query, fragment, scheme, username, password, hostname, and port
    pub complete_normalized_uri: Option<bstr::Bstr>,
    ///  This structure holds the normalized uri, including path, query, and fragment
    pub partial_normalized_uri: Option<bstr::Bstr>,
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
    /// Request transfer coding. Can be one of UNKNOWN (body presence not
    /// determined yet), IDENTITY, CHUNKED, NO_BODY,
    /// and UNRECOGNIZED.
    pub request_transfer_coding: HtpTransferCoding,
    /// Request body compression.
    pub request_content_encoding: decompressors::HtpContentEncoding,
    /// This field contain the request content type when that information is
    /// available in request headers. The contents of the field will be converted
    /// to lowercase and any parameters (e.g., character set information) removed.
    pub request_content_type: Option<bstr::Bstr>,
    /// Contains the value specified in the Content-Length header. The value of this
    /// field will be -1 from the beginning of the transaction and until request
    /// headers are processed. It will stay -1 if the C-L header was not provided,
    /// or if the value in it cannot be parsed.
    pub request_content_length: i64,
    /// Transaction-specific REQUEST_BODY_DATA hook. Behaves as
    /// the configuration hook with the same name.
    pub hook_request_body_data: DataHook,
    /// Transaction-specific RESPONSE_BODY_DATA hook. Behaves as
    /// the configuration hook with the same name.
    pub hook_response_body_data: DataHook,
    /// Request body URLENCODED parser. Available only when the request body is in the
    /// application/x-www-form-urlencoded format and the parser was configured to run.
    pub request_urlenp_body: Option<urlencoded::Parser>,
    /// Request body MULTIPART parser. Available only when the body is in the
    /// multipart/form-data format and the parser was configured to run.
    pub request_mpartp: Option<multipart::Parser>,
    /// Request parameters.
    pub request_params: table::Table<Param>,
    /// Request cookies
    pub request_cookies: table::Table<bstr::Bstr>,
    /// Authentication type used in the request.
    pub request_auth_type: HtpAuthType,
    /// Authentication username.
    pub request_auth_username: Option<bstr::Bstr>,
    /// Authentication password. Available only when Transaction::request_auth_type is HTP_AUTH_BASIC.
    pub request_auth_password: Option<bstr::Bstr>,
    /// Request hostname. Per the RFC, the hostname will be taken from the Host header
    /// when available. If the host information is also available in the URI, it is used
    /// instead of whatever might be in the Host header. Can be NULL. This field does
    /// not contain port information.
    pub request_hostname: Option<bstr::Bstr>,
    /// Request port number, if presented. The rules for Transaction::request_host apply. Set to
    /// None by default.
    pub request_port_number: Option<u16>,

    // Response fields
    /// How many empty lines did we ignore before reaching the status line?
    pub response_ignored_lines: u32,
    /// Response line.
    pub response_line: Option<bstr::Bstr>,
    /// Response protocol, as text. Can be NULL.
    pub response_protocol: Option<bstr::Bstr>,
    /// Response protocol as number. Available only if we were able to parse the protocol version,
    /// INVALID otherwise. UNKNOWN until parsing is attempted.
    pub response_protocol_number: HtpProtocol,
    /// Response status code, as text. Starts as NULL and can remain NULL on
    /// an invalid response that does not specify status code.
    pub response_status: Option<bstr::Bstr>,
    /// Response status code, available only if we were able to parse it, HTP_STATUS_INVALID
    /// otherwise. HTP_STATUS_UNKNOWN until parsing is attempted.
    pub response_status_number: i32,
    /// This field is set by the protocol decoder with it thinks that the
    /// backend server will reject a request with a particular status code.
    pub response_status_expected_number: config::HtpUnwanted,
    /// The message associated with the response status code. Can be NULL.
    pub response_message: Option<bstr::Bstr>,
    /// Have we seen the server respond with a 100 response?
    pub seen_100continue: bool,
    /// Parsed response headers. Contains instances of Header.
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
    pub response_transfer_coding: HtpTransferCoding,
    /// Response body compression, which indicates if compression is used
    /// for the response body. This field is an interpretation of the information
    /// available in response headers.
    pub response_content_encoding: decompressors::HtpContentEncoding,
    /// Response body compression processing information, which is related to how
    /// the library is going to process (or has processed) a response body. Changing
    /// this field mid-processing can influence library actions. For example, setting
    /// this field to NONE in a RESPONSE_HEADERS callback will prevent
    /// decompression.
    pub response_content_encoding_processing: decompressors::HtpContentEncoding,
    /// This field will contain the response content type when that information
    /// is available in response headers. The contents of the field will be converted
    /// to lowercase and any parameters (e.g., character set information) removed.
    pub response_content_type: Option<bstr::Bstr>,
    /// Response decompressor used to decompress response body data.
    pub out_decompressor: *mut decompressors::htp_decompressor_t,

    // Common fields
    /// Parsing flags; a combination of: HTP_REQUEST_INVALID_T_E, HTP_INVALID_FOLDING,
    /// HTP_REQUEST_SMUGGLING, HTP_MULTI_PACKET_HEAD, and HTP_FIELD_UNPARSEABLE.
    pub flags: Flags,
    /// Request progress.
    pub request_progress: HtpRequestProgress,
    /// Response progress.
    pub response_progress: HtpResponseProgress,
    /// Transaction index on the connection.
    pub index: usize,
    /// Total repetitions for headers in request.
    pub req_header_repetitions: u16,
    /// Total repetitions for headers in response.
    pub res_header_repetitions: u16,
}

pub type htp_txs_t = List<Transaction>;

impl Transaction {
    pub fn new(connp: &mut connection_parser::ConnectionParser) -> Result<usize> {
        let tx = Self {
            connp,
            cfg: connp.cfg,
            is_config_shared: true,
            user_data: std::ptr::null_mut(),
            request_ignored_lines: 0,
            request_line: None,
            request_method: None,
            request_method_number: request::HtpMethod::UNKNOWN,
            request_uri: None,
            request_protocol: None,
            request_protocol_number: HtpProtocol::UNKNOWN,
            is_protocol_0_9: false,
            parsed_uri: None,
            parsed_uri_raw: None,
            complete_normalized_uri: None,
            partial_normalized_uri: None,
            request_message_len: 0,
            request_entity_len: 0,
            request_headers: table::Table::with_capacity(32),
            request_transfer_coding: HtpTransferCoding::UNKNOWN,
            request_content_encoding: decompressors::HtpContentEncoding::UNKNOWN,
            request_content_type: None,
            request_content_length: -1,
            hook_request_body_data: DataHook::new(),
            hook_response_body_data: DataHook::new(),
            request_urlenp_body: None,
            request_mpartp: None,
            request_params: table::Table::with_capacity(32),
            request_cookies: table::Table::with_capacity(32),
            request_auth_type: HtpAuthType::UNKNOWN,
            request_auth_username: None,
            request_auth_password: None,
            request_hostname: None,
            request_port_number: None,
            response_ignored_lines: 0,
            response_line: None,
            response_protocol: None,
            response_protocol_number: HtpProtocol::UNKNOWN,
            response_status: None,
            response_status_number: 0,
            response_status_expected_number: config::HtpUnwanted::IGNORE,
            response_message: None,
            seen_100continue: false,
            response_headers: table::Table::with_capacity(32),
            response_message_len: 0,
            response_entity_len: 0,
            response_content_length: -1,
            response_transfer_coding: HtpTransferCoding::UNKNOWN,
            response_content_encoding: decompressors::HtpContentEncoding::UNKNOWN,
            response_content_encoding_processing: decompressors::HtpContentEncoding::UNKNOWN,
            response_content_type: None,
            out_decompressor: std::ptr::null_mut(),
            flags: Flags::empty(),
            request_progress: HtpRequestProgress::NOT_STARTED,
            response_progress: HtpResponseProgress::NOT_STARTED,
            index: connp.conn.tx_size(),
            req_header_repetitions: 0,
            res_header_repetitions: 0,
        };

        let tx_id = tx.index;
        unsafe { (*tx.connp).conn.push_tx(tx) };
        Ok(tx_id)
    }

    /// Register callback for the transaction-specific REQUEST_BODY_DATA hook.
    pub fn register_request_body_data(&mut self, cbk_fn: DataNativeCallbackFn) {
        self.hook_request_body_data.register(cbk_fn)
    }

    /// Destroys the supplied transaction.
    pub unsafe fn destroy(&mut self) -> Result<()> {
        if !self.is_complete() {
            return Err(HtpStatus::ERROR);
        }
        // remove the tx from the connection so it will be dropped
        let _ = (*self.connp).conn.remove_tx(self.index);
        Ok(())
    }

    /// Returns the user data associated with this transaction.
    pub fn user_data(&self) -> *mut core::ffi::c_void {
        self.user_data
    }

    /// Associates user data with this transaction.
    pub fn set_user_data(&mut self, user_data: *mut core::ffi::c_void) {
        self.user_data = user_data;
    }

    /// Adds one parameter to the request. THis function will take over the
    /// responsibility for the provided Param structure.
    ///
    /// Returns OK on success, ERROR on failure.
    pub unsafe fn req_add_param(&mut self, mut param: Param) -> Result<()> {
        if let Some(parameter_processor_fn) = (*self.cfg).parameter_processor {
            parameter_processor_fn(&mut param)?
        }
        self.request_params.add(param.name.clone(), param);
        Ok(())
    }

    /// Determine if the request has a body.
    ///
    /// Returns true if there is a body, false otherwise.
    pub fn req_has_body(&self) -> bool {
        self.request_transfer_coding == HtpTransferCoding::IDENTITY
            || self.request_transfer_coding == HtpTransferCoding::CHUNKED
    }

    unsafe fn process_request_headers(&mut self) -> Result<()> {
        // Determine if we have a request body, and how it is packaged.
        let cl_opt = self.request_headers.get_nocase_nozero("content-length");
        // Check for the Transfer-Encoding header, which would indicate a chunked request body.
        if let Some((_, te)) = self.request_headers.get_nocase_nozero("transfer-encoding") {
            // Make sure it contains "chunked" only.
            // TODO The HTTP/1.1 RFC also allows the T-E header to contain "identity", which
            //      presumably should have the same effect as T-E header absence. However, Apache
            //      (2.2.22 on Ubuntu 12.04 LTS) instead errors out with "Unknown Transfer-Encoding: identity".
            //      And it behaves strangely, too, sending a 501 and proceeding to process the request
            //      (e.g., PHP is run), but without the body. It then closes the connection.
            if te.value.cmp_nocase("chunked") != Ordering::Equal {
                // Invalid T-E header value.
                self.request_transfer_coding = HtpTransferCoding::INVALID;
                self.flags |= Flags::HTP_REQUEST_INVALID_T_E;
                self.flags |= Flags::HTP_REQUEST_INVALID
            } else {
                // Chunked encoding is a HTTP/1.1 feature, so check that an earlier protocol
                // version is not used. The flag will also be set if the protocol could not be parsed.
                //
                // TODO IIS 7.0, for example, would ignore the T-E header when it
                //      it is used with a protocol below HTTP 1.1. This should be a
                //      personality trait.
                if self.request_protocol_number < HtpProtocol::V1_1 {
                    self.flags |= Flags::HTP_REQUEST_INVALID_T_E;
                    self.flags |= Flags::HTP_REQUEST_SMUGGLING;
                }
                // If the T-E header is present we are going to use it.
                self.request_transfer_coding = HtpTransferCoding::CHUNKED;
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
                    self.flags |= Flags::HTP_REQUEST_SMUGGLING
                }
            }
        } else if let Some((_, cl)) = cl_opt {
            // Check for a folded C-L header.
            if cl.flags.contains(Flags::HTP_FIELD_FOLDED) {
                self.flags |= Flags::HTP_REQUEST_SMUGGLING
            }
            // Check for multiple C-L headers.
            if cl.flags.contains(Flags::HTP_FIELD_REPEATED) {
                self.flags |= Flags::HTP_REQUEST_SMUGGLING
                // TODO Personality trait to determine which C-L header to parse.
                //      At the moment we're parsing the combination of all instances,
                //      which is bound to fail (because it will contain commas).
            }
            // Get the body length.
            if let Some(content_length) =
                util::parse_content_length((*(*cl).value).as_slice(), Some(&mut *self.connp))
            {
                // We have a request body of known length.
                self.request_content_length = content_length;
                self.request_transfer_coding = HtpTransferCoding::IDENTITY
            } else {
                self.request_content_length = -1;
                self.request_transfer_coding = HtpTransferCoding::INVALID;
                self.flags |= Flags::HTP_REQUEST_INVALID_C_L;
                self.flags |= Flags::HTP_REQUEST_INVALID
            }
        } else {
            // No body.
            self.request_transfer_coding = HtpTransferCoding::NO_BODY
        }
        // If we could not determine the correct body handling,
        // consider the request invalid.
        if self.request_transfer_coding == HtpTransferCoding::UNKNOWN {
            self.request_transfer_coding = HtpTransferCoding::INVALID;
            self.flags |= Flags::HTP_REQUEST_INVALID
        }
        // Check for PUT requests, which we need to treat as file uploads.
        if self.request_method_number == request::HtpMethod::PUT && self.req_has_body() {
            // Prepare to treat PUT request body as a file.
            (*self.connp).put_file = Some(util::File::new(util::HtpFileSource::PUT, None));
        }
        // Determine hostname.
        // Use the hostname from the URI, when available.
        if let Some(hostname) = self.get_parsed_uri_hostname() {
            self.request_hostname = Some(bstr::Bstr::from(hostname.as_slice()));
        }

        if let Some(port_number) = self.get_parsed_uri_port_number() {
            self.request_port_number = Some(*port_number);
        }
        // Examine the Host header.
        if let Some((_, header)) = self.request_headers.get_nocase_nozero_mut("host") {
            // Host information available in the headers.
            if let Ok((_, (hostname, port_nmb, valid))) = util::parse_hostport(&mut header.value) {
                if !valid {
                    self.flags |= Flags::HTP_HOSTH_INVALID
                }
                // The host information in the headers is valid.
                // Is there host information in the URI?
                if self.request_hostname.is_none() {
                    // There is no host information in the URI. Place the
                    // hostname from the headers into the parsed_uri structure.
                    let mut hostname = bstr::Bstr::from(hostname);
                    hostname.make_ascii_lowercase();
                    self.request_hostname = Some(hostname);
                    if let Some((_, port)) = port_nmb {
                        self.request_port_number = port;
                    }
                } else {
                    // The host information appears in the URI and in the headers. The
                    // HTTP RFC states that we should ignore the header copy.
                    // Check for different hostnames.
                    if let Some(host) = &self.request_hostname {
                        if host.cmp_nocase(hostname) != Ordering::Equal {
                            self.flags |= Flags::HTP_HOST_AMBIGUOUS
                        }
                    }

                    if let Some((_, port)) = port_nmb {
                        // Check for different ports.
                        if self.request_port_number.is_some() && self.request_port_number != port {
                            self.flags |= Flags::HTP_HOST_AMBIGUOUS
                        }
                    }
                }
            } else if self.request_hostname.is_some() {
                // Invalid host information in the headers.
                // Raise the flag, even though the host information in the headers is invalid.
                self.flags |= Flags::HTP_HOST_AMBIGUOUS
            }
        } else {
            // No host information in the headers.
            // HTTP/1.1 requires host information in the headers.
            if self.request_protocol_number >= HtpProtocol::V1_1 {
                self.flags |= Flags::HTP_HOST_MISSING
            }
        }
        // Determine Content-Type.
        if let Some((_, ct)) = self.request_headers.get_nocase_nozero("content-type") {
            self.request_content_type = Some(util::parse_ct_header(ct.value.as_slice())?);
        }
        // Parse cookies.
        if (*(*self.connp).cfg).parse_request_cookies {
            parsers::parse_cookies_v0((*self.connp).in_tx_mut().ok_or(HtpStatus::ERROR)?)?;
        }
        // Parse authentication information.
        if (*(*self.connp).cfg).parse_request_auth {
            parsers::parse_authorization((*self.connp).in_tx_mut().ok_or(HtpStatus::ERROR)?)
                .or_else(|rc| {
                    if rc == HtpStatus::DECLINED {
                        // Don't fail the stream if an authorization header is invalid, just set a flag.
                        self.flags |= Flags::HTP_AUTH_INVALID;
                        Ok(())
                    } else {
                        Err(rc)
                    }
                })?;
        }
        // Finalize sending raw header data.
        (*self.connp).req_receiver_finalize_clear()?;
        // Run hook REQUEST_HEADERS.
        (*(*self.connp).cfg).hook_request_headers.run_all(self)?;
        // We cannot proceed if the request is invalid.
        if self.flags.contains(Flags::HTP_REQUEST_INVALID) {
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Process a chunk of request body data. This function assumes that
    /// handling of chunked encoding is implemented by the container. When
    /// you're done submitting body data, invoke a state change (to REQUEST)
    /// to finalize any processing that might be pending. The supplied data is
    /// fully consumed and there is no expectation that it will be available
    /// afterwards. The protocol parsing code makes no copies of the data,
    /// but some parsers might.
    ///
    /// Returns OK on success, ERROR on failure.
    #[allow(dead_code)]
    pub unsafe fn req_process_body_data<S: AsRef<[u8]>>(&mut self, data: S) -> Result<()> {
        if data.as_ref().len() == 0 {
            return Ok(());
        }
        self.req_process_body_data_ex(Some(data.as_ref()))
    }

    pub fn req_process_body_data_ex(&mut self, data: Option<&[u8]>) -> Result<()> {
        // NULL data is allowed in this private function; it's
        // used to indicate the end of request body.
        // Keep track of the body length.
        if let Some(data) = data {
            self.request_entity_len =
                (self.request_entity_len as u64).wrapping_add(data.len() as u64) as i64;
        }
        // Send data to the callbacks.
        let mut data = Data::new(self, data, false);
        unsafe {
            (*self.connp)
                .req_run_hook_body_data(&mut data)
                .map_err(|e| {
                    htp_error!(
                        self.connp,
                        HtpLogCode::REQUEST_BODY_DATA_CALLBACK_ERROR,
                        format!("Request body data callback returned error ({:?})", e)
                    );
                    e
                })
        }
    }

    /// Change transaction state to HTP_RESPONSE_LINE and invoke registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_response_line(&mut self) -> Result<()> {
        // Is the response line valid?
        let connp = self.connp;
        if self.response_protocol_number == HtpProtocol::INVALID {
            htp_warn!(
                connp,
                HtpLogCode::RESPONSE_LINE_INVALID_PROTOCOL,
                "Invalid response line: invalid protocol"
            );
            self.flags |= Flags::HTP_STATUS_LINE_INVALID
        }
        if self.response_status_number == -1
            || self.response_status_number < 100
            || self.response_status_number > 999
        {
            htp_warn!(
                connp,
                HtpLogCode::RESPONSE_LINE_INVALID_RESPONSE_STATUS,
                format!(
                    "Invalid response line: invalid response status {}.",
                    self.response_status_number
                )
            );
            self.response_status_number = -1;
            self.flags |= Flags::HTP_STATUS_LINE_INVALID
        }
        // Run hook HTP_RESPONSE_LINE
        (*(*self.connp).cfg).hook_response_line.run_all(self)
    }

    /// Set one response header. This function should be invoked once for
    /// each available header, and in the order in which headers were
    /// seen in the response.
    ///
    /// Returns OK on success, ERROR on failure.
    pub unsafe fn res_set_header<S: AsRef<[u8]>>(&mut self, name: S, value: S) {
        self.response_headers.add(
            name.as_ref().into(),
            Header::new(name.as_ref().into(), value.as_ref().into()),
        )
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
    /// Returns OK on success, ERROR on failure.
    #[allow(dead_code)]
    pub unsafe fn res_process_body_data<S: AsRef<[u8]>>(&mut self, data: S) -> Result<()> {
        if data.as_ref().len() == 0 {
            return Ok(());
        }
        self.res_process_body_data_ex(
            data.as_ref().as_ptr() as *const core::ffi::c_void,
            data.as_ref().len(),
        )
    }

    pub unsafe fn res_process_body_data_ex(
        &mut self,
        data: *const core::ffi::c_void,
        len: usize,
    ) -> Result<()> {
        // NULL data is allowed in this private function; it's
        // used to indicate the end of response body.
        let data_slice = std::slice::from_raw_parts(data as *const u8, len);
        let mut d = Data::new(self, Some(data_slice), false);
        // Keep track of body size before decompression.
        self.response_message_len =
            (self.response_message_len as u64).wrapping_add(d.len() as u64) as i64;
        let connp = self.connp;
        match self.response_content_encoding_processing {
            decompressors::HtpContentEncoding::GZIP
            | decompressors::HtpContentEncoding::DEFLATE
            | decompressors::HtpContentEncoding::LZMA => {
                // In severe memory stress these could be NULL
                if self.out_decompressor.is_null() || (*self.out_decompressor).decompress.is_none()
                {
                    return Err(HtpStatus::ERROR);
                }
                let mut after: libc::timeval = libc::timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                };
                libc::gettimeofday(
                    &mut (*self.out_decompressor).time_before,
                    0 as *mut libc::timezone,
                );
                // Send data buffer to the decompressor.
                (*self.out_decompressor)
                    .decompress
                    .expect("non-null function pointer")(
                    self.out_decompressor, &mut d
                );
                libc::gettimeofday(&mut after, 0 as *mut libc::timezone);
                // sanity check for race condition if system time changed
                if htp_timer_track(
                    &mut (*self.out_decompressor).time_spent,
                    &mut after,
                    &mut (*self.out_decompressor).time_before,
                )
                .is_ok()
                    && (*self.out_decompressor).time_spent > (*(*connp).cfg).compression_time_limit
                {
                    htp_error!(
                        connp,
                        HtpLogCode::COMPRESSION_BOMB,
                        format!(
                            "Compression bomb: spent {} us decompressing",
                            (*self.out_decompressor).time_spent
                        )
                    );
                    return Err(HtpStatus::ERROR);
                }
                if data == 0 as *mut core::ffi::c_void {
                    // Shut down the decompressor, if we used one.
                    self.destroy_decompressors();
                }
            }
            decompressors::HtpContentEncoding::NONE => {
                // When there's no decompression, response_entity_len.
                // is identical to response_message_len.
                self.response_entity_len =
                    (self.response_entity_len as u64).wrapping_add(d.len() as u64) as i64;
                (*self.connp).res_run_hook_body_data(&mut d)?;
            }
            _ => {
                // Internal error.
                htp_error!(
                    connp,
                    HtpLogCode::RESPONSE_BODY_INTERNAL_ERROR,
                    format!(
                    "[Internal Error] Invalid tx->response_content_encoding_processing value: {:?}",
                    self.response_content_encoding_processing
                )
                );
                return Err(HtpStatus::ERROR);
            }
        }
        Ok(())
    }

    pub unsafe fn destroy_decompressors(&mut self) {
        let mut comp: *mut decompressors::htp_decompressor_t = self.out_decompressor;
        while !comp.is_null() {
            let next: *mut decompressors::htp_decompressor_t = (*comp).next;
            (*comp).destroy.expect("non-null function pointer")(comp);
            comp = next
        }
        self.out_decompressor = 0 as *mut decompressors::htp_decompressor_t;
    }

    pub fn state_request_complete_partial(&mut self) -> Result<()> {
        // Finalize request body.
        if self.req_has_body() {
            self.req_process_body_data_ex(None)?;
        }
        self.request_progress = HtpRequestProgress::COMPLETE;
        // Run hook REQUEST_COMPLETE.
        unsafe {
            (*(*self.connp).cfg).hook_request_complete.run_all(self)?;
        }
        Ok(())
    }

    /// Change transaction state to REQUEST and invoke registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub fn state_request_complete(&mut self) -> Result<()> {
        if self.request_progress != HtpRequestProgress::COMPLETE {
            self.state_request_complete_partial()?;
        }
        // Make a copy of the connection parser pointer, so that
        // we don't have to reference it via tx, which may be
        // destroyed later.
        let connp: *mut connection_parser::ConnectionParser = self.connp;
        // Determine what happens next, and remove this transaction from the parser.
        if self.is_protocol_0_9 {
            unsafe {
                (*connp).in_state = State::IGNORE_DATA_AFTER_HTTP_0_9;
            }
        } else {
            unsafe {
                (*connp).in_state = State::IDLE;
            }
        }
        // Check if the entire transaction is complete. This call may
        // destroy the transaction, if auto-destroy is enabled.
        let _ = self.finalize();
        // At this point, tx may no longer be valid.
        unsafe {
            (*connp).clear_in_tx();
        }
        Ok(())
    }

    /// Initialize hybrid parsing mode, change state to TRANSACTION_START,
    /// and invoke all registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_request_start(&mut self) -> Result<()> {
        // Run hook REQUEST_START.
        (*(*self.connp).cfg).hook_request_start.run_all(self)?;
        // Change state into request line parsing.
        (*self.connp).in_state = State::LINE;
        self.request_progress = HtpRequestProgress::LINE;
        Ok(())
    }

    /// Change transaction state to REQUEST_HEADERS and invoke all
    /// registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_request_headers(&mut self) -> Result<()> {
        // If we're in HTP_REQ_HEADERS that means that this is the
        // first time we're processing headers in a request. Otherwise,
        // we're dealing with trailing headers.
        if self.request_progress > HtpRequestProgress::HEADERS {
            // Request trailers.
            // Run hook HTP_REQUEST_TRAILER.
            (*(*self.connp).cfg).hook_request_trailer.run_all(self)?;
            // Finalize sending raw header data.
            (*self.connp).req_receiver_finalize_clear()?;
            // Completed parsing this request; finalize it now.
            (*self.connp).in_state = State::FINALIZE;
        } else if self.request_progress >= HtpRequestProgress::LINE {
            // Request headers.
            // Did this request arrive in multiple data chunks?
            if (*self.connp).in_chunk_count != (*self.connp).in_chunk_request_index {
                self.flags |= Flags::HTP_MULTI_PACKET_HEAD
            }
            self.process_request_headers()?;
            (*self.connp).in_state = State::CONNECT_CHECK;
        } else {
            htp_warn!(
                self.connp,
                HtpLogCode::RESPONSE_BODY_INTERNAL_ERROR,
                format!(
                    "[Internal Error] Invalid tx progress: {:?}",
                    self.request_progress
                )
            );
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Change transaction state to REQUEST_LINE and invoke all
    /// registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_request_line(&mut self) -> Result<()> {
        // Determine how to process the request URI.
        if self.request_method_number == request::HtpMethod::CONNECT {
            // When CONNECT is used, the request URI contains an authority string.
            self.parsed_uri_raw = Some(util::parse_uri_hostport(
                self.request_uri.as_ref().ok_or(HtpStatus::ERROR)?,
                &mut self.flags,
            ));
        } else if let Some(uri) = self.request_uri.as_ref() {
            self.parsed_uri_raw = Some(util::parse_uri(uri.as_slice()));
        } else {
            self.parsed_uri_raw = Some(util::Uri::new());
        }
        // Parse the request URI into Transaction::parsed_uri_raw.
        // Build Transaction::parsed_uri, but only if it was not explicitly set already.
        if self.parsed_uri.is_none() {
            // Keep the original URI components, but create a copy which we can normalize and use internally.
            self.normalize_parsed_uri();
        }
        // Check parsed_uri hostname.
        if let Some(hostname) = self.get_parsed_uri_hostname() {
            if !util::validate_hostname(hostname.as_slice()) {
                self.flags |= Flags::HTP_HOSTU_INVALID
            }
        }
        // Run hook REQUEST_URI_NORMALIZE.
        (*(*self.connp).cfg)
            .hook_request_uri_normalize
            .run_all(self)?;
        // Run hook REQUEST_LINE.
        (*(*self.connp).cfg).hook_request_line.run_all(self)?;
        if let Some(parsed_uri) = &self.parsed_uri {
            let (partial_normalized_uri, complete_normalized_uri) =
                util::generate_normalized_uri(&(*(self.cfg)).decoder_cfg, parsed_uri);
            self.partial_normalized_uri = partial_normalized_uri;
            self.complete_normalized_uri = complete_normalized_uri;
        }
        // Move on to the next phase.
        (*self.connp).in_state = State::PROTOCOL;
        Ok(())
    }

    /// Change transaction state to RESPONSE and invoke registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_response_complete(&mut self) -> Result<()> {
        self.state_response_complete_ex(1)
    }

    pub fn finalize(&mut self) -> Result<()> {
        if !self.is_complete() {
            return Ok(());
        }
        unsafe {
            // Run hook TRANSACTION_COMPLETE.
            (*(*self.connp).cfg)
                .hook_transaction_complete
                .run_all(self)?;
            // In streaming processing, we destroy the transaction because it will not be needed any more.
            if (*(*self.connp).cfg).tx_auto_destroy {
                self.destroy()?;
            }
        }
        Ok(())
    }

    pub unsafe fn state_response_complete_ex(&mut self, hybrid_mode: i32) -> Result<()> {
        if self.response_progress != HtpResponseProgress::COMPLETE {
            self.response_progress = HtpResponseProgress::COMPLETE;
            // Run the last RESPONSE_BODY_DATA HOOK, but only if there was a response body present.
            if self.response_transfer_coding != HtpTransferCoding::NO_BODY {
                let _ = self.res_process_body_data_ex(0 as *const core::ffi::c_void, 0);
            }
            // Run hook RESPONSE_COMPLETE.
            (*(*self.connp).cfg).hook_response_complete.run_all(self)?;
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
            if (*self.connp).in_status == connection_parser::HtpStreamState::DATA_OTHER
                && (*self.connp).in_tx() == (*self.connp).out_tx()
            {
                return Err(HtpStatus::DATA_OTHER);
            }
            // Do we have a signal to yield to inbound processing at
            // the end of the next transaction?
            if (*self.connp).out_data_other_at_tx_end {
                // We do. Let's yield then.
                (*self.connp).out_data_other_at_tx_end = false;
                return Err(HtpStatus::DATA_OTHER);
            }
        }
        // Make a copy of the connection parser pointer, so that
        // we don't have to reference it via tx, which may be destroyed later.
        let connp: *mut connection_parser::ConnectionParser = self.connp;
        // Finalize the transaction. This may call may destroy the transaction, if auto-destroy is enabled.
        self.finalize()?;
        // Disconnect transaction from the parser.
        (*connp).clear_out_tx();
        (*connp).out_state = State::IDLE;
        Ok(())
    }

    /// Change transaction state to RESPONSE_HEADERS and invoke registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_response_headers(&mut self) -> Result<()> {
        // Check for compression.
        // Determine content encoding.
        let mut ce_multi_comp = false;
        self.response_content_encoding = decompressors::HtpContentEncoding::NONE;
        if let Some((_, ce)) = self.response_headers.get_nocase_nozero("content-encoding") {
            // fast paths: regular gzip and friends
            if ce.value.cmp_nocase_nozero("gzip") == Ordering::Equal
                || ce.value.cmp_nocase_nozero("x-gzip") == Ordering::Equal
            {
                self.response_content_encoding = decompressors::HtpContentEncoding::GZIP
            } else if ce.value.cmp_nocase_nozero("deflate") == Ordering::Equal
                || ce.value.cmp_nocase_nozero("x-deflate") == Ordering::Equal
            {
                self.response_content_encoding = decompressors::HtpContentEncoding::DEFLATE
            } else if ce.value.cmp_nocase_nozero("lzma") == Ordering::Equal {
                self.response_content_encoding = decompressors::HtpContentEncoding::LZMA
            } else if !(ce.value.cmp_nocase_nozero("inflate") == Ordering::Equal) {
                // exceptional cases: enter slow path
                ce_multi_comp = true
            }
        }
        // Configure decompression, if enabled in the configuration.
        if (*(*self.connp).cfg).response_decompression_enabled {
            self.response_content_encoding_processing = self.response_content_encoding
        } else {
            self.response_content_encoding_processing = decompressors::HtpContentEncoding::NONE;
            ce_multi_comp = false
        }
        // Finalize sending raw header data.
        (&mut *self.connp).res_receiver_finalize_clear()?;
        // Run hook RESPONSE_HEADERS.
        (*(*self.connp).cfg).hook_response_headers.run_all(self)?;
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
        if self.response_content_encoding_processing == decompressors::HtpContentEncoding::GZIP
            || self.response_content_encoding_processing
                == decompressors::HtpContentEncoding::DEFLATE
            || self.response_content_encoding_processing == decompressors::HtpContentEncoding::LZMA
            || ce_multi_comp
        {
            if !self.out_decompressor.is_null() {
                self.destroy_decompressors();
            }
            // normal case
            if !ce_multi_comp {
                self.out_decompressor = decompressors::htp_gzip_decompressor_create(
                    self.connp,
                    self.response_content_encoding_processing,
                );
                if self.out_decompressor.is_null() {
                    return Err(HtpStatus::ERROR);
                }
                (*self.out_decompressor).callback = Some(
                    htp_tx_res_process_body_data_decompressor_callback
                        as unsafe extern "C" fn(_: *mut Data) -> HtpStatus,
                )
            // multiple ce value case
            } else if let Some((_, ce)) =
                self.response_headers.get_nocase_nozero("content-encoding")
            {
                let mut layers: i32 = 0;
                let mut comp: *mut decompressors::htp_decompressor_t =
                    0 as *mut decompressors::htp_decompressor_t;
                let mut nblzma: i32 = 0;

                let tokens = ce.value.split_str_collect(", ");
                let connp = self.connp;
                for tok in tokens {
                    let token = bstr::Bstr::from(tok);
                    let mut cetype: decompressors::HtpContentEncoding =
                        decompressors::HtpContentEncoding::NONE;
                    // check depth limit (0 means no limit)
                    if (*(*connp).cfg).response_decompression_layer_limit != 0 && {
                        layers += 1;
                        (layers) > (*(*connp).cfg).response_decompression_layer_limit
                    } {
                        htp_warn!(
                            connp,
                            HtpLogCode::TOO_MANY_ENCODING_LAYERS,
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
                                    HtpLogCode::ABNORMAL_CE_HEADER,
                                    "C-E gzip has abnormal value"
                                );
                            }
                            cetype = decompressors::HtpContentEncoding::GZIP
                        } else if token.index_of_nocase("deflate").is_some() {
                            if !(token.cmp("deflate") == Ordering::Equal
                                || token.cmp("x-deflate") == Ordering::Equal)
                            {
                                htp_warn!(
                                    connp,
                                    HtpLogCode::ABNORMAL_CE_HEADER,
                                    "C-E deflate has abnormal value"
                                );
                            }
                            cetype = decompressors::HtpContentEncoding::DEFLATE
                        } else if token.index_of_nocase("lzma").is_some() {
                            cetype = decompressors::HtpContentEncoding::LZMA;
                            nblzma = nblzma.wrapping_add(1);
                            if nblzma > (*(*connp).cfg).response_lzma_layer_limit {
                                htp_error!(
                                    connp,
                                    HtpLogCode::COMPRESSION_BOMB_DOUBLE_LZMA,
                                    "Compression bomb: double lzma encoding"
                                );
                                break;
                            }
                        } else if token.index_of_nocase("inflate").is_some() {
                            cetype = decompressors::HtpContentEncoding::NONE
                        } else {
                            // continue
                            htp_warn!(connp, HtpLogCode::ABNORMAL_CE_HEADER, "C-E unknown setting");
                        }
                        if cetype != decompressors::HtpContentEncoding::NONE {
                            if comp.is_null() {
                                self.response_content_encoding_processing = cetype;
                                self.out_decompressor = decompressors::htp_gzip_decompressor_create(
                                    self.connp,
                                    self.response_content_encoding_processing,
                                );
                                if self.out_decompressor.is_null() {
                                    return Err(HtpStatus::ERROR);
                                }
                                (*self.out_decompressor).callback = Some(
                                    htp_tx_res_process_body_data_decompressor_callback
                                        as unsafe extern "C" fn(_: *mut Data) -> HtpStatus,
                                );
                                comp = self.out_decompressor
                            } else {
                                (*comp).next =
                                    decompressors::htp_gzip_decompressor_create(self.connp, cetype);
                                if (*comp).next.is_null() {
                                    return Err(HtpStatus::ERROR);
                                }
                                (*(*comp).next).callback = Some(
                                    htp_tx_res_process_body_data_decompressor_callback
                                        as unsafe extern "C" fn(_: *mut Data) -> HtpStatus,
                                );
                                comp = (*comp).next
                            }
                        }
                    }
                }
            }
        } else if self.response_content_encoding_processing
            != decompressors::HtpContentEncoding::NONE
        {
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Change transaction state to RESPONSE_START and invoke registered callbacks.
    ///
    /// Returns OK on success; ERROR on error, HTP_STOP if one of the
    ///         callbacks does not want to follow the transaction any more.
    pub unsafe fn state_response_start(&mut self) -> Result<()> {
        (*self.connp).set_out_tx(self);
        // Run hook RESPONSE_START.
        (*(*self.connp).cfg).hook_response_start.run_all(self)?;
        // Change state into response line parsing, except if we're following
        // a HTTP/0.9 request (no status line or response headers).
        if self.is_protocol_0_9 {
            self.response_transfer_coding = HtpTransferCoding::IDENTITY;
            self.response_content_encoding_processing = decompressors::HtpContentEncoding::NONE;
            self.response_progress = HtpResponseProgress::BODY;
            (*self.connp).out_state = State::BODY_IDENTITY_STREAM_CLOSE;
            (*self.connp).out_body_data_left = -1
        } else {
            (*self.connp).out_state = State::LINE;
            self.response_progress = HtpResponseProgress::LINE
        }
        // If at this point we have no method and no uri and our status
        // is still request::htp_connp_REQ_LINE, we likely have timed out request
        // or a overly long request
        if self.request_method.is_none()
            && self.request_uri.is_none()
            && (*self.connp).in_state == State::LINE
        {
            htp_warn!(
                self.connp,
                HtpLogCode::REQUEST_LINE_INCOMPLETE,
                "Request line incomplete"
            );
        }
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        // A transaction is considered complete only when both the request and
        // response are complete. (Sometimes a complete response can be seen
        // even while the request is ongoing.)
        self.request_progress == HtpRequestProgress::COMPLETE
            && self.response_progress == HtpResponseProgress::COMPLETE
    }

    pub fn get_parsed_uri_query(&self) -> Option<&bstr::Bstr> {
        self.parsed_uri
            .as_ref()
            .and_then(|parsed_uri| parsed_uri.query.as_ref())
    }

    pub fn get_parsed_uri_hostname(&self) -> Option<&bstr::Bstr> {
        self.parsed_uri
            .as_ref()
            .and_then(|parsed_uri| parsed_uri.hostname.as_ref())
    }

    pub fn get_parsed_uri_port_number(&self) -> Option<&u16> {
        self.parsed_uri
            .as_ref()
            .and_then(|parsed_uri| parsed_uri.port_number.as_ref())
    }

    /// Normalize a previously-parsed request URI.
    pub unsafe fn normalize_parsed_uri(&mut self) {
        let mut uri = util::Uri::new();
        if let Some(incomplete) = &self.parsed_uri_raw {
            uri.scheme = incomplete.normalized_scheme();
            uri.username =
                incomplete.normalized_username(&(*(self.cfg)).decoder_cfg, &mut self.flags);
            uri.password =
                incomplete.normalized_password(&(*(self.cfg)).decoder_cfg, &mut self.flags);
            uri.hostname =
                incomplete.normalized_hostname(&(*(self.cfg)).decoder_cfg, &mut self.flags);
            uri.port_number = incomplete.normalized_port(&mut self.flags);
            uri.query = incomplete.query.clone();
            uri.fragment =
                incomplete.normalized_fragment(&(*(self.cfg)).decoder_cfg, &mut self.flags);
            uri.path = incomplete.normalized_path(
                &(*(self.cfg)).decoder_cfg,
                &mut self.flags,
                &mut self.response_status_expected_number,
            );
        }
        self.parsed_uri = Some(uri);
    }
}

impl Drop for Transaction {
    /// Destroys all the fields inside an Transaction.
    fn drop(&mut self) {
        unsafe {
            self.destroy_decompressors();
            // If we're using a private configuration structure, destroy it.
            if !self.is_config_shared {
                (*self.cfg).destroy();
            }
        }
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.connp).conn == (*other.connp).conn && self.index == other.index }
    }
}

unsafe fn htp_timer_track(
    time_spent: *mut i32,
    after: *mut libc::timeval,
    before: *mut libc::timeval,
) -> Result<()> {
    if (*after).tv_sec < (*before).tv_sec {
        return Err(HtpStatus::ERROR);
    } else if (*after).tv_sec == (*before).tv_sec {
        if (*after).tv_usec < (*before).tv_usec {
            return Err(HtpStatus::ERROR);
        }
        *time_spent = *time_spent + ((*after).tv_usec - (*before).tv_usec) as i32
    } else {
        *time_spent = *time_spent
            + (((*after).tv_sec - (*before).tv_sec) * 1000000 + (*after).tv_usec
                - (*before).tv_usec) as i32
    }
    Ok(())
}

unsafe extern "C" fn htp_tx_res_process_body_data_decompressor_callback(d: *mut Data) -> HtpStatus {
    let d = if let Some(d) = d.as_mut() {
        d
    } else {
        return HtpStatus::ERROR;
    };
    let tx = if let Some(tx) = d.tx.as_mut() {
        tx
    } else {
        return HtpStatus::ERROR;
    };
    // Keep track of actual response body length.
    tx.response_entity_len = (tx.response_entity_len as u64).wrapping_add(d.len() as u64) as i64;
    // Invoke all callbacks.
    let rc: HtpStatus = (*tx.connp).res_run_hook_body_data(d).into();
    if rc != HtpStatus::OK {
        return HtpStatus::ERROR;
    }
    (*tx.out_decompressor).nb_callbacks = (*tx.out_decompressor).nb_callbacks.wrapping_add(1);

    if (*tx.out_decompressor).nb_callbacks.wrapping_rem(256) == 0 {
        let mut after: libc::timeval = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        libc::gettimeofday(&mut after, 0 as *mut libc::timezone);
        // sanity check for race condition if system time changed
        if htp_timer_track(
            &mut (*tx.out_decompressor).time_spent,
            &mut after,
            &mut (*tx.out_decompressor).time_before,
        )
        .is_ok()
        {
            // updates last tracked time
            (*tx.out_decompressor).time_before = after;
            if (*tx.out_decompressor).time_spent > (*(*tx.connp).cfg).compression_time_limit {
                htp_error!(
                    tx.connp,
                    HtpLogCode::COMPRESSION_BOMB,
                    format!(
                        "Compression bomb: spent {} us decompressing",
                        (*tx.out_decompressor).time_spent
                    )
                );
                return HtpStatus::ERROR;
            }
        }
    }
    if tx.response_entity_len > (*(*tx.connp).cfg).compression_bomb_limit as i64
        && tx.response_entity_len > 2048 * tx.response_message_len
    {
        htp_error!(
            tx.connp,
            HtpLogCode::COMPRESSION_BOMB,
            format!(
                "Compression bomb: decompressed {} bytes out of {}",
                tx.response_entity_len, tx.response_message_len
            )
        );
        return HtpStatus::ERROR;
    }
    HtpStatus::OK
}
