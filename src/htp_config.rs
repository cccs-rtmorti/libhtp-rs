use crate::{
    htp_connection_parser, htp_content_handlers, htp_hooks, htp_request_apache_2_2,
    htp_request_generic, htp_response_generic, htp_transaction, htp_util, Status,
};

extern "C" {
    #[no_mangle]
    fn malloc(_: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
    #[no_mangle]
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: libc::size_t,
    ) -> *mut core::ffi::c_void;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_cfg_t {
    /// The maximum size of the buffer that is used when the current
    /// input chunk does not contain all the necessary data (e.g., a very header
    /// line that spans several packets).
    pub field_limit_hard: usize,
    /// Soft field limit length. If this limit is reached the parser will issue
    /// a warning but continue to run. NOT IMPLEMENTED.
    pub field_limit_soft: usize,
    /// Log level, which will be used when deciding whether to store or
    /// ignore the messages issued by the parser.
    pub log_level: htp_util::htp_log_level_t,
    /// Whether to delete each transaction after the last hook is invoked. This
    /// feature should be used when parsing traffic streams in real time.
    pub tx_auto_destroy: i32,
    /// Server personality identifier.
    pub server_personality: htp_server_personality_t,
    /// The function used for request line parsing. Depends on the personality.
    pub parse_request_line:
        Option<unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status>,
    /// The function used for response line parsing. Depends on the personality.
    pub parse_response_line:
        Option<unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status>,
    /// The function used for request header parsing. Depends on the personality.
    pub process_request_header: Option<
        unsafe extern "C" fn(
            _: *mut htp_connection_parser::htp_connp_t,
            _: *mut u8,
            _: usize,
        ) -> Status,
    >,
    /// The function used for response header parsing. Depends on the personality.
    pub process_response_header: Option<
        unsafe extern "C" fn(
            _: *mut htp_connection_parser::htp_connp_t,
            _: *mut u8,
            _: usize,
        ) -> Status,
    >,
    /// The function to use to transform parameters after parsing.
    pub parameter_processor:
        Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_param_t) -> Status>,
    /// Decoder configuration array, one per context.
    pub decoder_cfgs: [htp_decoder_cfg_t; 3],
    /// Whether to generate the request_uri_normalized field.
    pub generate_request_uri_normalized: i32,
    /// Whether to decompress compressed response bodies.
    pub response_decompression_enabled: i32,
    /// Not fully implemented at the moment.
    pub request_encoding: *mut i8,
    /// Not fully implemented at the moment.
    pub internal_encoding: *mut i8,
    /// Whether to parse request cookies.
    pub parse_request_cookies: i32,
    /// Whether to parse HTTP Authentication headers.
    pub parse_request_auth: i32,
    /// Whether to extract files from requests using Multipart encoding.
    pub extract_request_files: i32,
    /// How many extracted files are allowed in a single Multipart request?
    pub extract_request_files_limit: i32,
    /// The location on disk where temporary files will be created.
    pub tmpdir: *mut i8,
    /// Request start hook, invoked when the parser receives the first byte of a new
    /// request. Because in HTTP a transaction always starts with a request, this hook
    /// doubles as a transaction start hook.
    pub hook_request_start: *mut htp_hooks::htp_hook_t,
    /// Request line hook, invoked after a request line has been parsed.
    pub hook_request_line: *mut htp_hooks::htp_hook_t,
    /// Request URI normalization hook, for overriding default normalization of URI.
    pub hook_request_uri_normalize: *mut htp_hooks::htp_hook_t,
    /// Receives raw request header data, starting immediately after the request line,
    /// including all headers as they are seen on the TCP connection, and including the
    /// terminating empty line. Not available on genuine HTTP/0.9 requests (because
    /// they don't use headers).
    pub hook_request_header_data: *mut htp_hooks::htp_hook_t,
    /// Request headers hook, invoked after all request headers are seen.
    pub hook_request_headers: *mut htp_hooks::htp_hook_t,
    /// Request body data hook, invoked every time body data is available. Each
    /// invocation will provide a htp_tx_data_t instance. Chunked data
    /// will be dechunked before the data is passed to this hook. Decompression
    /// is not currently implemented. At the end of the request body
    /// there will be a call with the data pointer set to NULL.
    pub hook_request_body_data: *mut htp_hooks::htp_hook_t,
    /// Request file data hook, which is invoked whenever request file data is
    /// available. Currently used only by the Multipart parser.
    pub hook_request_file_data: *mut htp_hooks::htp_hook_t,
    /// Receives raw request trailer data, which can be available on requests that have
    /// chunked bodies. The data starts immediately after the zero-length chunk
    /// and includes the terminating empty line.
    pub hook_request_trailer_data: *mut htp_hooks::htp_hook_t,
    /// Request trailer hook, invoked after all trailer headers are seen,
    /// and if they are seen (not invoked otherwise).
    pub hook_request_trailer: *mut htp_hooks::htp_hook_t,
    /// Request hook, invoked after a complete request is seen.
    pub hook_request_complete: *mut htp_hooks::htp_hook_t,
    /// Response startup hook, invoked when a response transaction is found and
    /// processing started.
    pub hook_response_start: *mut htp_hooks::htp_hook_t,
    /// Response line hook, invoked after a response line has been parsed.
    pub hook_response_line: *mut htp_hooks::htp_hook_t,
    /// Receives raw response header data, starting immediately after the status line
    /// and including all headers as they are seen on the TCP connection, and including the
    /// terminating empty line. Not available on genuine HTTP/0.9 responses (because
    /// they don't have response headers).
    pub hook_response_header_data: *mut htp_hooks::htp_hook_t,
    /// Response headers book, invoked after all response headers have been seen.
    pub hook_response_headers: *mut htp_hooks::htp_hook_t,
    /// Response body data hook, invoked every time body data is available. Each
    /// invocation will provide a htp_tx_data_t instance. Chunked data
    /// will be dechunked before the data is passed to this hook. By default,
    /// compressed data will be decompressed, but decompression can be disabled
    /// in configuration. At the end of the response body there will be a call
    /// with the data pointer set to NULL.
    pub hook_response_body_data: *mut htp_hooks::htp_hook_t,
    /// Receives raw response trailer data, which can be available on responses that have
    /// chunked bodies. The data starts immediately after the zero-length chunk
    /// and includes the terminating empty line.
    pub hook_response_trailer_data: *mut htp_hooks::htp_hook_t,
    /// Response trailer hook, invoked after all trailer headers have been processed,
    /// and only if the trailer exists.
    pub hook_response_trailer: *mut htp_hooks::htp_hook_t,
    /// Response hook, invoked after a response has been seen. Because sometimes servers
    /// respond before receiving complete requests, a response_complete callback may be
    /// invoked prior to a request_complete callback.
    pub hook_response_complete: *mut htp_hooks::htp_hook_t,
    /// Transaction complete hook, which is invoked once the entire transaction is
    /// considered complete (request and response are both complete). This is always
    /// the last hook to be invoked.
    pub hook_transaction_complete: *mut htp_hooks::htp_hook_t,
    /// Log hook, invoked every time the library wants to log.
    pub hook_log: *mut htp_hooks::htp_hook_t,
    /// Opaque user data associated with this configuration structure.
    pub user_data: *mut core::ffi::c_void,
    // Request Line parsing options.

    // TODO this was added here to maintain a stable ABI, once we can break that
    // we may want to move this into htp_decoder_cfg_t (VJ)
    /// Reaction to leading whitespace on the request line
    pub requestline_leading_whitespace_unwanted: htp_unwanted_t,
    /// How many layers of compression we will decompress (0 => no limit).
    pub response_decompression_layer_limit: i32,
    /// max memory use by a the lzma decompressor.
    pub lzma_memlimit: usize,
    /// max output size for a compression bomb.
    pub compression_bomb_limit: i32,
    /// max time for a decompression bomb.
    pub compression_time_limit: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_decoder_cfg_t {
    // Path-specific decoding options.
    /// Convert backslash characters to slashes.
    pub backslash_convert_slashes: i32,
    /// Convert to lowercase.
    pub convert_lowercase: i32,
    /// Compress slash characters.
    pub path_separators_compress: i32,
    /// Should we URL-decode encoded path segment separators?
    pub path_separators_decode: i32,
    /// Should we decode '+' characters to spaces?
    pub plusspace_decode: i32,
    /// Reaction to encoded path separators.
    pub path_separators_encoded_unwanted: htp_unwanted_t,
    // Special characters options.
    /// Controls how raw NUL bytes are handled.
    pub nul_raw_terminates: i32,
    /// Determines server response to a raw NUL byte in the path.
    pub nul_raw_unwanted: htp_unwanted_t,
    /// Reaction to control characters.
    pub control_chars_unwanted: htp_unwanted_t,
    // URL encoding options.
    /// Should we decode %u-encoded characters?
    pub u_encoding_decode: i32,
    /// Reaction to %u encoding.
    pub u_encoding_unwanted: htp_unwanted_t,
    /// Handling of invalid URL encodings.
    pub url_encoding_invalid_handling: htp_url_encoding_handling_t,
    /// Reaction to invalid URL encoding.
    pub url_encoding_invalid_unwanted: htp_unwanted_t,
    /// Controls how encoded NUL bytes are handled.
    pub nul_encoded_terminates: i32,
    /// How are we expected to react to an encoded NUL byte?
    pub nul_encoded_unwanted: htp_unwanted_t,
    // UTF-8 options.
    /// Controls how invalid UTF-8 characters are handled.
    pub utf8_invalid_unwanted: htp_unwanted_t,
    /// Convert UTF-8 characters into bytes using best-fit mapping.
    pub utf8_convert_bestfit: i32,
    // Best-fit mapping options.
    /// The best-fit map to use to decode %u-encoded characters.
    pub bestfit_map: *mut u8,
    /// The replacement byte used when there is no best-fit mapping.
    pub bestfit_replacement_byte: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_decoder_ctx_t {
    /// Default settings. Settings applied to this context are propagated to all other contexts.
    HTP_DECODER_DEFAULTS,
    /// Urlencoded decoder settings.
    HTP_DECODER_URLENCODED,
    /// URL path decoder settings.
    HTP_DECODER_URL_PATH,
}

/// Enumerates the possible server personalities.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_server_personality_t {
    /// Minimal personality that performs at little work as possible. All optional
    /// features are disabled. This personality is a good starting point for customization.
    HTP_SERVER_MINIMAL,
    /// A generic personality that aims to work reasonably well for all server types.
    HTP_SERVER_GENERIC,
    /// The IDS personality tries to perform as much decoding as possible.
    HTP_SERVER_IDS,
    /// Mimics the behavior of IIS 4.0, as shipped with Windows NT 4.0.
    HTP_SERVER_IIS_4_0,
    /// Mimics the behavior of IIS 5.0, as shipped with Windows 2000.
    HTP_SERVER_IIS_5_0,
    /// Mimics the behavior of IIS 5.1, as shipped with Windows XP Professional.
    HTP_SERVER_IIS_5_1,
    /// Mimics the behavior of IIS 6.0, as shipped with Windows 2003.
    HTP_SERVER_IIS_6_0,
    /// Mimics the behavior of IIS 7.0, as shipped with Windows 2008.
    HTP_SERVER_IIS_7_0,
    /// Mimics the behavior of IIS 7.5, as shipped with Windows 7.
    HTP_SERVER_IIS_7_5,
    /// Mimics the behavior of Apache 2.x.
    HTP_SERVER_APACHE_2,
}

/// Enumerates the ways in which servers respond to malformed data.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_unwanted_t {
    /// Ignores problem.
    HTP_UNWANTED_IGNORE,
    /// Responds with HTTP 400 status code.
    HTP_UNWANTED_400 = 400,
    /// Responds with HTTP 404 status code.
    HTP_UNWANTED_404 = 404,
}

/// Enumerates the possible approaches to handling invalid URL-encodings.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_url_encoding_handling_t {
    /// Ignore invalid URL encodings and leave the % in the data.
    HTP_URL_DECODE_PRESERVE_PERCENT,
    /// Ignore invalid URL encodings, but remove the % from the data.
    HTP_URL_DECODE_REMOVE_PERCENT,
    /// Decode invalid URL encodings.
    HTP_URL_DECODE_PROCESS_INVALID,
}

pub type htp_callback_fn_t = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> Status>;
/// This map is used by default for best-fit mapping from the Unicode
/// values U+0100-FFFF.
static bestfit_1252: [u8; 1173] = [
    0x1, 0, 0x41, 0x1, 0x1, 0x61, 0x1, 0x2, 0x41, 0x1, 0x3, 0x61, 0x1, 0x4, 0x41, 0x1, 0x5, 0x61,
    0x1, 0x6, 0x43, 0x1, 0x7, 0x63, 0x1, 0x8, 0x43, 0x1, 0x9, 0x63, 0x1, 0xa, 0x43, 0x1, 0xb, 0x63,
    0x1, 0xc, 0x43, 0x1, 0xd, 0x63, 0x1, 0xe, 0x44, 0x1, 0xf, 0x64, 0x1, 0x11, 0x64, 0x1, 0x12,
    0x45, 0x1, 0x13, 0x65, 0x1, 0x14, 0x45, 0x1, 0x15, 0x65, 0x1, 0x16, 0x45, 0x1, 0x17, 0x65, 0x1,
    0x18, 0x45, 0x1, 0x19, 0x65, 0x1, 0x1a, 0x45, 0x1, 0x1b, 0x65, 0x1, 0x1c, 0x47, 0x1, 0x1d,
    0x67, 0x1, 0x1e, 0x47, 0x1, 0x1f, 0x67, 0x1, 0x20, 0x47, 0x1, 0x21, 0x67, 0x1, 0x22, 0x47, 0x1,
    0x23, 0x67, 0x1, 0x24, 0x48, 0x1, 0x25, 0x68, 0x1, 0x26, 0x48, 0x1, 0x27, 0x68, 0x1, 0x28,
    0x49, 0x1, 0x29, 0x69, 0x1, 0x2a, 0x49, 0x1, 0x2b, 0x69, 0x1, 0x2c, 0x49, 0x1, 0x2d, 0x69, 0x1,
    0x2e, 0x49, 0x1, 0x2f, 0x69, 0x1, 0x30, 0x49, 0x1, 0x31, 0x69, 0x1, 0x34, 0x4a, 0x1, 0x35,
    0x6a, 0x1, 0x36, 0x4b, 0x1, 0x37, 0x6b, 0x1, 0x39, 0x4c, 0x1, 0x3a, 0x6c, 0x1, 0x3b, 0x4c, 0x1,
    0x3c, 0x6c, 0x1, 0x3d, 0x4c, 0x1, 0x3e, 0x6c, 0x1, 0x41, 0x4c, 0x1, 0x42, 0x6c, 0x1, 0x43,
    0x4e, 0x1, 0x44, 0x6e, 0x1, 0x45, 0x4e, 0x1, 0x46, 0x6e, 0x1, 0x47, 0x4e, 0x1, 0x48, 0x6e, 0x1,
    0x4c, 0x4f, 0x1, 0x4d, 0x6f, 0x1, 0x4e, 0x4f, 0x1, 0x4f, 0x6f, 0x1, 0x50, 0x4f, 0x1, 0x51,
    0x6f, 0x1, 0x54, 0x52, 0x1, 0x55, 0x72, 0x1, 0x56, 0x52, 0x1, 0x57, 0x72, 0x1, 0x58, 0x52, 0x1,
    0x59, 0x72, 0x1, 0x5a, 0x53, 0x1, 0x5b, 0x73, 0x1, 0x5c, 0x53, 0x1, 0x5d, 0x73, 0x1, 0x5e,
    0x53, 0x1, 0x5f, 0x73, 0x1, 0x62, 0x54, 0x1, 0x63, 0x74, 0x1, 0x64, 0x54, 0x1, 0x65, 0x74, 0x1,
    0x66, 0x54, 0x1, 0x67, 0x74, 0x1, 0x68, 0x55, 0x1, 0x69, 0x75, 0x1, 0x6a, 0x55, 0x1, 0x6b,
    0x75, 0x1, 0x6c, 0x55, 0x1, 0x6d, 0x75, 0x1, 0x6e, 0x55, 0x1, 0x6f, 0x75, 0x1, 0x70, 0x55, 0x1,
    0x71, 0x75, 0x1, 0x72, 0x55, 0x1, 0x73, 0x75, 0x1, 0x74, 0x57, 0x1, 0x75, 0x77, 0x1, 0x76,
    0x59, 0x1, 0x77, 0x79, 0x1, 0x79, 0x5a, 0x1, 0x7b, 0x5a, 0x1, 0x7c, 0x7a, 0x1, 0x80, 0x62, 0x1,
    0x97, 0x49, 0x1, 0x9a, 0x6c, 0x1, 0x9f, 0x4f, 0x1, 0xa0, 0x4f, 0x1, 0xa1, 0x6f, 0x1, 0xab,
    0x74, 0x1, 0xae, 0x54, 0x1, 0xaf, 0x55, 0x1, 0xb0, 0x75, 0x1, 0xb6, 0x7a, 0x1, 0xc0, 0x7c, 0x1,
    0xc3, 0x21, 0x1, 0xcd, 0x41, 0x1, 0xce, 0x61, 0x1, 0xcf, 0x49, 0x1, 0xd0, 0x69, 0x1, 0xd1,
    0x4f, 0x1, 0xd2, 0x6f, 0x1, 0xd3, 0x55, 0x1, 0xd4, 0x75, 0x1, 0xd5, 0x55, 0x1, 0xd6, 0x75, 0x1,
    0xd7, 0x55, 0x1, 0xd8, 0x75, 0x1, 0xd9, 0x55, 0x1, 0xda, 0x75, 0x1, 0xdb, 0x55, 0x1, 0xdc,
    0x75, 0x1, 0xde, 0x41, 0x1, 0xdf, 0x61, 0x1, 0xe4, 0x47, 0x1, 0xe5, 0x67, 0x1, 0xe6, 0x47, 0x1,
    0xe7, 0x67, 0x1, 0xe8, 0x4b, 0x1, 0xe9, 0x6b, 0x1, 0xea, 0x4f, 0x1, 0xeb, 0x6f, 0x1, 0xec,
    0x4f, 0x1, 0xed, 0x6f, 0x1, 0xf0, 0x6a, 0x2, 0x61, 0x67, 0x2, 0xb9, 0x27, 0x2, 0xba, 0x22, 0x2,
    0xbc, 0x27, 0x2, 0xc4, 0x5e, 0x2, 0xc8, 0x27, 0x2, 0xcb, 0x60, 0x2, 0xcd, 0x5f, 0x3, 0, 0x60,
    0x3, 0x2, 0x5e, 0x3, 0x3, 0x7e, 0x3, 0xe, 0x22, 0x3, 0x31, 0x5f, 0x3, 0x32, 0x5f, 0x3, 0x7e,
    0x3b, 0x3, 0x93, 0x47, 0x3, 0x98, 0x54, 0x3, 0xa3, 0x53, 0x3, 0xa6, 0x46, 0x3, 0xa9, 0x4f, 0x3,
    0xb1, 0x61, 0x3, 0xb4, 0x64, 0x3, 0xb5, 0x65, 0x3, 0xc0, 0x70, 0x3, 0xc3, 0x73, 0x3, 0xc4,
    0x74, 0x3, 0xc6, 0x66, 0x4, 0xbb, 0x68, 0x5, 0x89, 0x3a, 0x6, 0x6a, 0x25, 0x20, 0, 0x20, 0x20,
    0x1, 0x20, 0x20, 0x2, 0x20, 0x20, 0x3, 0x20, 0x20, 0x4, 0x20, 0x20, 0x5, 0x20, 0x20, 0x6, 0x20,
    0x20, 0x10, 0x2d, 0x20, 0x11, 0x2d, 0x20, 0x17, 0x3d, 0x20, 0x32, 0x27, 0x20, 0x35, 0x60, 0x20,
    0x44, 0x2f, 0x20, 0x74, 0x34, 0x20, 0x75, 0x35, 0x20, 0x76, 0x36, 0x20, 0x77, 0x37, 0x20, 0x78,
    0x38, 0x20, 0x7f, 0x6e, 0x20, 0x80, 0x30, 0x20, 0x81, 0x31, 0x20, 0x82, 0x32, 0x20, 0x83, 0x33,
    0x20, 0x84, 0x34, 0x20, 0x85, 0x35, 0x20, 0x86, 0x36, 0x20, 0x87, 0x37, 0x20, 0x88, 0x38, 0x20,
    0x89, 0x39, 0x20, 0xa7, 0x50, 0x21, 0x2, 0x43, 0x21, 0x7, 0x45, 0x21, 0xa, 0x67, 0x21, 0xb,
    0x48, 0x21, 0xc, 0x48, 0x21, 0xd, 0x48, 0x21, 0xe, 0x68, 0x21, 0x10, 0x49, 0x21, 0x11, 0x49,
    0x21, 0x12, 0x4c, 0x21, 0x13, 0x6c, 0x21, 0x15, 0x4e, 0x21, 0x18, 0x50, 0x21, 0x19, 0x50, 0x21,
    0x1a, 0x51, 0x21, 0x1b, 0x52, 0x21, 0x1c, 0x52, 0x21, 0x1d, 0x52, 0x21, 0x24, 0x5a, 0x21, 0x28,
    0x5a, 0x21, 0x2a, 0x4b, 0x21, 0x2c, 0x42, 0x21, 0x2d, 0x43, 0x21, 0x2e, 0x65, 0x21, 0x2f, 0x65,
    0x21, 0x30, 0x45, 0x21, 0x31, 0x46, 0x21, 0x33, 0x4d, 0x21, 0x34, 0x6f, 0x22, 0x12, 0x2d, 0x22,
    0x15, 0x2f, 0x22, 0x16, 0x5c, 0x22, 0x17, 0x2a, 0x22, 0x1a, 0x76, 0x22, 0x1e, 0x38, 0x22, 0x23,
    0x7c, 0x22, 0x29, 0x6e, 0x22, 0x36, 0x3a, 0x22, 0x3c, 0x7e, 0x22, 0x61, 0x3d, 0x22, 0x64, 0x3d,
    0x22, 0x65, 0x3d, 0x23, 0x3, 0x5e, 0x23, 0x20, 0x28, 0x23, 0x21, 0x29, 0x23, 0x29, 0x3c, 0x23,
    0x2a, 0x3e, 0x25, 0, 0x2d, 0x25, 0xc, 0x2b, 0x25, 0x10, 0x2b, 0x25, 0x14, 0x2b, 0x25, 0x18,
    0x2b, 0x25, 0x1c, 0x2b, 0x25, 0x2c, 0x2d, 0x25, 0x34, 0x2d, 0x25, 0x3c, 0x2b, 0x25, 0x50, 0x2d,
    0x25, 0x52, 0x2b, 0x25, 0x53, 0x2b, 0x25, 0x54, 0x2b, 0x25, 0x55, 0x2b, 0x25, 0x56, 0x2b, 0x25,
    0x57, 0x2b, 0x25, 0x58, 0x2b, 0x25, 0x59, 0x2b, 0x25, 0x5a, 0x2b, 0x25, 0x5b, 0x2b, 0x25, 0x5c,
    0x2b, 0x25, 0x5d, 0x2b, 0x25, 0x64, 0x2d, 0x25, 0x65, 0x2d, 0x25, 0x66, 0x2d, 0x25, 0x67, 0x2d,
    0x25, 0x68, 0x2d, 0x25, 0x69, 0x2d, 0x25, 0x6a, 0x2b, 0x25, 0x6b, 0x2b, 0x25, 0x6c, 0x2b, 0x25,
    0x84, 0x5f, 0x27, 0x58, 0x7c, 0x30, 0, 0x20, 0x30, 0x8, 0x3c, 0x30, 0x9, 0x3e, 0x30, 0x1a,
    0x5b, 0x30, 0x1b, 0x5d, 0xff, 0x1, 0x21, 0xff, 0x2, 0x22, 0xff, 0x3, 0x23, 0xff, 0x4, 0x24,
    0xff, 0x5, 0x25, 0xff, 0x6, 0x26, 0xff, 0x7, 0x27, 0xff, 0x8, 0x28, 0xff, 0x9, 0x29, 0xff, 0xa,
    0x2a, 0xff, 0xb, 0x2b, 0xff, 0xc, 0x2c, 0xff, 0xd, 0x2d, 0xff, 0xe, 0x2e, 0xff, 0xf, 0x2f,
    0xff, 0x10, 0x30, 0xff, 0x11, 0x31, 0xff, 0x12, 0x32, 0xff, 0x13, 0x33, 0xff, 0x14, 0x34, 0xff,
    0x15, 0x35, 0xff, 0x16, 0x36, 0xff, 0x17, 0x37, 0xff, 0x18, 0x38, 0xff, 0x19, 0x39, 0xff, 0x1a,
    0x3a, 0xff, 0x1b, 0x3b, 0xff, 0x1c, 0x3c, 0xff, 0x1d, 0x3d, 0xff, 0x1e, 0x3e, 0xff, 0x20, 0x40,
    0xff, 0x21, 0x41, 0xff, 0x22, 0x42, 0xff, 0x23, 0x43, 0xff, 0x24, 0x44, 0xff, 0x25, 0x45, 0xff,
    0x26, 0x46, 0xff, 0x27, 0x47, 0xff, 0x28, 0x48, 0xff, 0x29, 0x49, 0xff, 0x2a, 0x4a, 0xff, 0x2b,
    0x4b, 0xff, 0x2c, 0x4c, 0xff, 0x2d, 0x4d, 0xff, 0x2e, 0x4e, 0xff, 0x2f, 0x4f, 0xff, 0x30, 0x50,
    0xff, 0x31, 0x51, 0xff, 0x32, 0x52, 0xff, 0x33, 0x53, 0xff, 0x34, 0x54, 0xff, 0x35, 0x55, 0xff,
    0x36, 0x56, 0xff, 0x37, 0x57, 0xff, 0x38, 0x58, 0xff, 0x39, 0x59, 0xff, 0x3a, 0x5a, 0xff, 0x3b,
    0x5b, 0xff, 0x3c, 0x5c, 0xff, 0x3d, 0x5d, 0xff, 0x3e, 0x5e, 0xff, 0x3f, 0x5f, 0xff, 0x40, 0x60,
    0xff, 0x41, 0x61, 0xff, 0x42, 0x62, 0xff, 0x43, 0x63, 0xff, 0x44, 0x64, 0xff, 0x45, 0x65, 0xff,
    0x46, 0x66, 0xff, 0x47, 0x67, 0xff, 0x48, 0x68, 0xff, 0x49, 0x69, 0xff, 0x4a, 0x6a, 0xff, 0x4b,
    0x6b, 0xff, 0x4c, 0x6c, 0xff, 0x4d, 0x6d, 0xff, 0x4e, 0x6e, 0xff, 0x4f, 0x6f, 0xff, 0x50, 0x70,
    0xff, 0x51, 0x71, 0xff, 0x52, 0x72, 0xff, 0x53, 0x73, 0xff, 0x54, 0x74, 0xff, 0x55, 0x75, 0xff,
    0x56, 0x76, 0xff, 0x57, 0x77, 0xff, 0x58, 0x78, 0xff, 0x59, 0x79, 0xff, 0x5a, 0x7a, 0xff, 0x5b,
    0x7b, 0xff, 0x5c, 0x7c, 0xff, 0x5d, 0x7d, 0xff, 0x5e, 0x7e, 0, 0, 0,
];

/// Creates a new configuration structure. Configuration structures created at
/// configuration time must not be changed afterwards in order to support lock-less
/// copying.
pub unsafe fn htp_config_create() -> *mut htp_cfg_t {
    let mut cfg: *mut htp_cfg_t = calloc(1, ::std::mem::size_of::<htp_cfg_t>()) as *mut htp_cfg_t; // Use the parser default.
    if cfg.is_null() {
        return 0 as *mut htp_cfg_t;
    } // 2 layers seem fairly common
    (*cfg).field_limit_hard = 18000;
    (*cfg).field_limit_soft = 9000;
    (*cfg).log_level = htp_util::htp_log_level_t::HTP_LOG_NOTICE;
    (*cfg).response_decompression_enabled = 1;
    (*cfg).parse_request_cookies = 1;
    (*cfg).parse_request_auth = 1;
    (*cfg).extract_request_files = 0;
    (*cfg).extract_request_files_limit = -1;
    (*cfg).response_decompression_layer_limit = 2;
    (*cfg).lzma_memlimit = 1048576;
    (*cfg).compression_bomb_limit = 1048576;
    (*cfg).compression_time_limit = 100000;
    // Default settings for URL-encoded data.
    htp_config_set_bestfit_map(
        cfg,
        htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
        bestfit_1252.as_ptr() as *const core::ffi::c_void,
    );
    htp_config_set_bestfit_replacement_byte(
        cfg,
        htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
        '?' as i32,
    );
    htp_config_set_url_encoding_invalid_handling(
        cfg,
        htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
        htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
    );
    htp_config_set_nul_raw_terminates(cfg, htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 0);
    htp_config_set_nul_encoded_terminates(cfg, htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 0);
    htp_config_set_u_encoding_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 0);
    htp_config_set_plusspace_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URLENCODED, 1);
    htp_config_set_server_personality(cfg, htp_server_personality_t::HTP_SERVER_MINIMAL);
    return cfg;
}

/// Destroy a configuration structure.
pub unsafe fn htp_config_destroy(cfg: *mut htp_cfg_t) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_destroy((*cfg).hook_request_start);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_line);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_uri_normalize);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_header_data);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_headers);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_body_data);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_file_data);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_trailer);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_trailer_data);
    htp_hooks::htp_hook_destroy((*cfg).hook_request_complete);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_start);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_line);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_header_data);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_headers);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_body_data);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_trailer);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_trailer_data);
    htp_hooks::htp_hook_destroy((*cfg).hook_response_complete);
    htp_hooks::htp_hook_destroy((*cfg).hook_transaction_complete);
    htp_hooks::htp_hook_destroy((*cfg).hook_log);
    free(cfg as *mut core::ffi::c_void);
}

/// Registers a callback that is invoked every time there is a log message with
/// severity equal and higher than the configured log level.
pub unsafe fn htp_config_register_log(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_util::htp_log_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_log,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_util::htp_log_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Adds the built-in Multipart parser to the configuration. This parser will extract information
/// stored in request bodies, when they are in multipart/form-data format.
pub unsafe fn htp_config_register_multipart_parser(cfg: *mut htp_cfg_t) {
    if cfg.is_null() {
        return;
    }
    htp_config_register_request_headers(
        cfg,
        Some(
            htp_content_handlers::htp_ch_multipart_callback_request_headers
                as unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status,
        ),
    );
}

/// Registers a REQUEST_COMPLETE callback.
pub unsafe fn htp_config_register_request_complete(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_complete,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a REQUEST_BODY_DATA callback.
pub unsafe fn htp_config_register_request_body_data(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a REQUEST_HEADER_DATA callback.
pub unsafe fn htp_config_register_request_header_data(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_header_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a REQUEST_HEADERS callback.
pub unsafe fn htp_config_register_request_headers(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_headers,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a REQUEST_LINE callback.
pub unsafe fn htp_config_register_request_line(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_line,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a REQUEST_START callback, which is invoked every time a new
/// request begins and before any parsing is done.
pub unsafe fn htp_config_register_request_start(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_start,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a HTP_REQUEST_TRAILER callback.
pub unsafe fn htp_config_register_request_trailer(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_trailer,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a REQUEST_TRAILER_DATA callback.
pub unsafe fn htp_config_register_request_trailer_data(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_request_trailer_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_BODY_DATA callback.
pub unsafe fn htp_config_register_response_body_data(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_body_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_COMPLETE callback.
pub unsafe fn htp_config_register_response_complete(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_complete,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_HEADER_DATA callback.
pub unsafe fn htp_config_register_response_header_data(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_header_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_HEADERS callback.
#[allow(dead_code)]
pub unsafe fn htp_config_register_response_headers(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_headers,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_LINE callback.
#[allow(dead_code)]
pub unsafe fn htp_config_register_response_line(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_line,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_START callback.
pub unsafe fn htp_config_register_response_start(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_start,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_TRAILER callback.
pub unsafe fn htp_config_register_response_trailer(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_trailer,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a RESPONSE_TRAILER_DATA callback.
pub unsafe fn htp_config_register_response_trailer_data(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_response_trailer_data,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Registers a TRANSACTION_COMPLETE callback.
pub unsafe fn htp_config_register_transaction_complete(
    cfg: *mut htp_cfg_t,
    callback_fn: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
) {
    if cfg.is_null() {
        return;
    }
    htp_hooks::htp_hook_register(
        &mut (*cfg).hook_transaction_complete,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status>,
            htp_callback_fn_t,
        >(callback_fn),
    );
}

/// Adds the built-in Urlencoded parser to the configuration. The parser will
/// parse query strings and request bodies with the appropriate MIME type.
#[allow(dead_code)]
pub unsafe fn htp_config_register_urlencoded_parser(cfg: *mut htp_cfg_t) {
    if cfg.is_null() {
        return;
    }
    htp_config_register_request_line(
        cfg,
        Some(
            htp_content_handlers::htp_ch_urlencoded_callback_request_line
                as unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status,
        ),
    );
    htp_config_register_request_headers(
        cfg,
        Some(
            htp_content_handlers::htp_ch_urlencoded_callback_request_headers
                as unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_t) -> Status,
        ),
    );
}

/// Configures the maximum size of the buffer LibHTP will use when all data is not available
/// in the current buffer (e.g., a very long header line that might span several packets). This
/// limit is controlled by the hard_limit parameter. The soft_limit parameter is not implemented.
/// soft_limit is NOT IMPLEMENTED.
pub unsafe fn htp_config_set_field_limits(
    mut cfg: *mut htp_cfg_t,
    soft_limit: usize,
    hard_limit: usize,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).field_limit_soft = soft_limit;
    (*cfg).field_limit_hard = hard_limit;
}

/// Configures the maximum memlimit LibHTP will pass to liblzma.
pub unsafe fn htp_config_set_lzma_memlimit(mut cfg: *mut htp_cfg_t, memlimit: usize) {
    if cfg.is_null() {
        return;
    }
    (*cfg).lzma_memlimit = memlimit;
}

/// Configures the maximum compression bomb size LibHTP will decompress.
pub unsafe fn htp_config_set_compression_bomb_limit(mut cfg: *mut htp_cfg_t, bomblimit: usize) {
    if cfg.is_null() {
        return;
    }
    if bomblimit > 2147483647 {
        (*cfg).compression_bomb_limit = 2147483647
    } else {
        (*cfg).compression_bomb_limit = bomblimit as i32
    };
}

/// Enable or disable request cookie parsing. Enabled by default.
pub unsafe fn htp_config_set_parse_request_cookies(
    mut cfg: *mut htp_cfg_t,
    parse_request_cookies: i32,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).parse_request_cookies = parse_request_cookies;
}

/// Configure desired server personality.
/// Returns Status::OK if the personality is supported, Status::ERROR if it isn't.
pub unsafe fn htp_config_set_server_personality(
    mut cfg: *mut htp_cfg_t,
    personality: htp_server_personality_t,
) -> Status {
    if cfg.is_null() {
        return Status::ERROR;
    }
    match personality as u32 {
        0 => {
            (*cfg).parse_request_line = Some(
                htp_request_generic::htp_parse_request_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_request_header = Some(
                htp_request_generic::htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            (*cfg).parse_response_line = Some(
                htp_response_generic::htp_parse_response_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_response_header = Some(
                htp_response_generic::htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            )
        }
        1 => {
            (*cfg).parse_request_line = Some(
                htp_request_generic::htp_parse_request_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_request_header = Some(
                htp_request_generic::htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            (*cfg).parse_response_line = Some(
                htp_response_generic::htp_parse_response_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_response_header = Some(
                htp_response_generic::htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            htp_config_set_backslash_convert_slashes(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_path_separators_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_path_separators_compress(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
        }
        2 => {
            (*cfg).parse_request_line = Some(
                htp_request_generic::htp_parse_request_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_request_header = Some(
                htp_request_generic::htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            (*cfg).parse_response_line = Some(
                htp_response_generic::htp_parse_response_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_response_header = Some(
                htp_response_generic::htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            htp_config_set_backslash_convert_slashes(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_path_separators_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_path_separators_compress(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_convert_lowercase(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_utf8_convert_bestfit(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_u_encoding_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
                htp_unwanted_t::HTP_UNWANTED_IGNORE,
            );
        }
        9 => {
            (*cfg).parse_request_line = Some(
                htp_request_apache_2_2::htp_parse_request_line_apache_2_2
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_request_header = Some(
                htp_request_apache_2_2::htp_process_request_header_apache_2_2
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            (*cfg).parse_response_line = Some(
                htp_response_generic::htp_parse_response_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_response_header = Some(
                htp_response_generic::htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            htp_config_set_backslash_convert_slashes(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                0,
            );
            htp_config_set_path_separators_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 0);
            htp_config_set_path_separators_compress(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_u_encoding_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 0);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_url_encoding_invalid_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_unwanted_t::HTP_UNWANTED_400,
            );
            htp_config_set_control_chars_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_unwanted_t::HTP_UNWANTED_IGNORE,
            );
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
                htp_unwanted_t::HTP_UNWANTED_400,
            );
        }
        5 => {
            (*cfg).parse_request_line = Some(
                htp_request_generic::htp_parse_request_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_request_header = Some(
                htp_request_generic::htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            (*cfg).parse_response_line = Some(
                htp_response_generic::htp_parse_response_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_response_header = Some(
                htp_response_generic::htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            htp_config_set_backslash_convert_slashes(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_path_separators_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_path_separators_compress(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_u_encoding_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 0);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_control_chars_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_unwanted_t::HTP_UNWANTED_IGNORE,
            );
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
                htp_unwanted_t::HTP_UNWANTED_IGNORE,
            );
        }
        6 => {
            (*cfg).parse_request_line = Some(
                htp_request_generic::htp_parse_request_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_request_header = Some(
                htp_request_generic::htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            (*cfg).parse_response_line = Some(
                htp_response_generic::htp_parse_response_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_response_header = Some(
                htp_response_generic::htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            htp_config_set_backslash_convert_slashes(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_path_separators_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_path_separators_compress(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_u_encoding_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_u_encoding_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_unwanted_t::HTP_UNWANTED_400,
            );
            htp_config_set_control_chars_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_unwanted_t::HTP_UNWANTED_400,
            );
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
                htp_unwanted_t::HTP_UNWANTED_IGNORE,
            );
        }
        7 | 8 => {
            (*cfg).parse_request_line = Some(
                htp_request_generic::htp_parse_request_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_request_header = Some(
                htp_request_generic::htp_process_request_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            (*cfg).parse_response_line = Some(
                htp_response_generic::htp_parse_response_line_generic
                    as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
            );
            (*cfg).process_response_header = Some(
                htp_response_generic::htp_process_response_header_generic
                    as unsafe extern "C" fn(
                        _: *mut htp_connection_parser::htp_connp_t,
                        _: *mut u8,
                        _: usize,
                    ) -> Status,
            );
            htp_config_set_backslash_convert_slashes(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_path_separators_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_path_separators_compress(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                1,
            );
            htp_config_set_u_encoding_decode(cfg, htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
            htp_config_set_url_encoding_invalid_handling(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
            );
            htp_config_set_url_encoding_invalid_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_unwanted_t::HTP_UNWANTED_400,
            );
            htp_config_set_control_chars_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                htp_unwanted_t::HTP_UNWANTED_400,
            );
            htp_config_set_requestline_leading_whitespace_unwanted(
                cfg,
                htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
                htp_unwanted_t::HTP_UNWANTED_IGNORE,
            );
        }
        _ => return Status::ERROR,
    }
    // Remember the personality
    (*cfg).server_personality = personality;
    return Status::OK;
}

/// Configures whether transactions will be automatically destroyed once they
/// are processed and all callbacks invoked. This option is appropriate for
/// programs that process transactions as they are processed.
pub unsafe fn htp_config_set_tx_auto_destroy(mut cfg: *mut htp_cfg_t, tx_auto_destroy: i32) {
    if cfg.is_null() {
        return;
    }
    (*cfg).tx_auto_destroy = tx_auto_destroy;
}

unsafe fn convert_to_0_or_1(b: i32) -> i32 {
    if b != 0 {
        return 1;
    }
    return 0;
}

/// Configures a best-fit map, which is used whenever characters longer than one byte
/// need to be converted to a single-byte. By default a Windows 1252 best-fit map is used.
/// The map is an list of triplets, the first 2 bytes being an UCS-2 character to map from,
/// and the third byte being the single byte to map to. Make sure that your map contains
/// the mappings to cover the full-width and half-width form characters (U+FF00-FFEF). The
/// last triplet in the map must be all zeros (3 NUL bytes).
pub unsafe fn htp_config_set_bestfit_map(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    map: *const core::ffi::c_void,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].bestfit_map = map as *mut u8;
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].bestfit_map = map as *mut u8;
            i = i.wrapping_add(1)
        }
    };
}

/// Sets the replacement character that will be used to in the lossy best-fit
/// mapping from multi-byte to single-byte streams. The question mark character
/// is used as the default replacement byte.
pub unsafe fn htp_config_set_bestfit_replacement_byte(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    b: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte = b as u8;
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].bestfit_replacement_byte = b as u8;
            i = i.wrapping_add(1)
        }
    };
}

/// Configures how the server handles to invalid URL encoding.
pub unsafe fn htp_config_set_url_encoding_invalid_handling(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    handling: htp_url_encoding_handling_t,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_handling = handling;
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].url_encoding_invalid_handling = handling;
            i = i.wrapping_add(1)
        }
    };
}

/// Configures the handling of raw NUL bytes. If enabled, raw NUL terminates strings.
pub unsafe fn htp_config_set_nul_raw_terminates(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].nul_raw_terminates = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].nul_raw_terminates = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures how the server reacts to encoded NUL bytes. Some servers will stop at
/// at NUL, while some will respond with 400 or 404. When the termination option is not
/// used, the NUL byte will remain in the path.
pub unsafe fn htp_config_set_nul_encoded_terminates(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].nul_encoded_terminates = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].nul_encoded_terminates = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures whether %u-encoded sequences are decoded. Such sequences
/// will be treated as invalid URL encoding if decoding is not desirable.
pub unsafe fn htp_config_set_u_encoding_decode(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_decode = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].u_encoding_decode = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures whether backslash characters are treated as path segment separators. They
/// are not on Unix systems, but are on Windows systems. If this setting is enabled, a path
/// such as "/one\two/three" will be converted to "/one/two/three".
/// Implemented only for htp_decoder_ctx_t::HTP_DECODER_URL_PATH.
pub unsafe fn htp_config_set_backslash_convert_slashes(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].backslash_convert_slashes = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].backslash_convert_slashes = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures whether encoded path segment separators will be decoded. Apache does not do
/// this by default, but IIS does. If enabled, a path such as "/one%2ftwo" will be normalized
/// to "/one/two". If the backslash_separators option is also enabled, encoded backslash
/// characters will be converted too (and subsequently normalized to forward slashes). Implemented
/// only for htp_decoder_ctx_t::HTP_DECODER_URL_PATH.
pub unsafe fn htp_config_set_path_separators_decode(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_decode = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].path_separators_decode = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures whether consecutive path segment separators will be compressed. When enabled, a path
/// such as "/one//two" will be normalized to "/one/two". Backslash conversion and path segment separator
/// decoding are carried out before compression. For example, the path "/one\\/two\/%5cthree/%2f//four"
/// will be converted to "/one/two/three/four" (assuming all 3 options are enabled). Implemented only for
/// htp_decoder_ctx_t::HTP_DECODER_URL_PATH.
pub unsafe fn htp_config_set_path_separators_compress(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_compress = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].path_separators_compress = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures whether plus characters are converted to spaces when decoding URL-encoded strings. This
/// is appropriate to do for parameters, but not for URLs. Only applies to contexts where decoding
/// is taking place.
pub unsafe fn htp_config_set_plusspace_decode(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].plusspace_decode = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].plusspace_decode = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures whether input data will be converted to lowercase. Useful when set on the
/// htp_decoder_ctx_t::HTP_DECODER_URL_PATH context, in order to handle servers with
/// case-insensitive filesystems.
/// Implemented only for htp_decoder_ctx_t::HTP_DECODER_URL_PATH.
pub unsafe fn htp_config_set_convert_lowercase(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].convert_lowercase = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].convert_lowercase = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Controls whether the data should be treated as UTF-8 and converted to a single-byte
/// stream using best-fit mapping. Implemented only for htp_decoder_ctx_t::HTP_DECODER_URL_PATH.
pub unsafe fn htp_config_set_utf8_convert_bestfit(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    enabled: i32,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].utf8_convert_bestfit = convert_to_0_or_1(enabled);
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].utf8_convert_bestfit = convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}

/// Configures reaction to %u-encoded sequences in input data.
pub unsafe fn htp_config_set_u_encoding_unwanted(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    unwanted: htp_unwanted_t,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted = unwanted;
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].u_encoding_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/// Controls reaction to raw control characters in the data.
pub unsafe fn htp_config_set_control_chars_unwanted(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    unwanted: htp_unwanted_t,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted = unwanted;
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].u_encoding_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/// Configures how the server reacts to invalid URL encoding.
pub unsafe fn htp_config_set_url_encoding_invalid_unwanted(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    unwanted: htp_unwanted_t,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_unwanted = unwanted;
    if ctx == htp_decoder_ctx_t::HTP_DECODER_DEFAULTS {
        let mut i: usize = 0;
        while i < 3 {
            (*cfg).decoder_cfgs[i as usize].url_encoding_invalid_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}

/// Configures how the server reacts to leading whitespace on the request line.
pub unsafe fn htp_config_set_requestline_leading_whitespace_unwanted(
    mut cfg: *mut htp_cfg_t,
    ctx: htp_decoder_ctx_t,
    unwanted: htp_unwanted_t,
) {
    if ctx as u32 >= 3 {
        return;
    }
    (*cfg).requestline_leading_whitespace_unwanted = unwanted;
}

/// Configures many layers of compression we try to decompress.
/// limit: 0 disables limit
pub unsafe fn htp_config_set_response_decompression_layer_limit(
    mut cfg: *mut htp_cfg_t,
    limit: i32,
) {
    if cfg.is_null() {
        return;
    }
    (*cfg).response_decompression_layer_limit = limit;
}
