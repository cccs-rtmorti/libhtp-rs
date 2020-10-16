use crate::error::Result;
use crate::hook::{
    DataHook, DataNativeCallbackFn, FileDataHook, LogHook, LogNativeCallbackFn, TxHook,
    TxNativeCallbackFn,
};
use crate::log::htp_log_level_t;
use crate::{content_handlers, transaction, Status};

#[derive(Clone)]
pub struct Config {
    /// The maximum size of the buffer that is used when the current
    /// input chunk does not contain all the necessary data (e.g., a very header
    /// line that spans several packets).
    pub field_limit: usize,
    /// Log level, which will be used when deciding whether to store or
    /// ignore the messages issued by the parser.
    pub log_level: htp_log_level_t,
    /// Whether to delete each transaction after the last hook is invoked. This
    /// feature should be used when parsing traffic streams in real time.
    pub tx_auto_destroy: bool,
    /// Server personality identifier.
    pub server_personality: htp_server_personality_t,
    /// The function to use to transform parameters after parsing.
    pub parameter_processor: Option<fn(_: &mut transaction::Param) -> Result<()>>,
    /// Decoder configuration for url path.
    pub decoder_cfg: DecoderConfig,
    /// Whether to decompress compressed response bodies.
    pub response_decompression_enabled: bool,
    /// Whether to parse request cookies.
    pub parse_request_cookies: bool,
    /// Whether to parse HTTP Authentication headers.
    pub parse_request_auth: bool,
    /// Whether to extract files from requests using Multipart encoding.
    pub extract_request_files: bool,
    /// How many extracted files are allowed in a single Multipart request?
    pub extract_request_files_limit: u32,
    /// The location on disk where temporary files will be created.
    pub tmpdir: String,
    /// Request start hook, invoked when the parser receives the first byte of a new
    /// request. Because in HTTP a transaction always starts with a request, this hook
    /// doubles as a transaction start hook.
    pub hook_request_start: TxHook,
    /// Request line hook, invoked after a request line has been parsed.
    pub hook_request_line: TxHook,
    /// Request URI normalization hook, for overriding default normalization of URI.
    pub hook_request_uri_normalize: TxHook,
    /// Receives raw request header data, starting immediately after the request line,
    /// including all headers as they are seen on the TCP connection, and including the
    /// terminating empty line. Not available on genuine HTTP/0.9 requests (because
    /// they don't use headers).
    pub hook_request_header_data: DataHook,
    /// Request headers hook, invoked after all request headers are seen.
    pub hook_request_headers: TxHook,
    /// Request body data hook, invoked every time body data is available. Each
    /// invocation will provide a Data instance. Chunked data
    /// will be dechunked before the data is passed to this hook. Decompression
    /// is not currently implemented. At the end of the request body
    /// there will be a call with the data pointer set to NULL.
    pub hook_request_body_data: DataHook,
    /// Request file data hook, which is invoked whenever request file data is
    /// available. Currently used only by the Multipart parser.
    pub hook_request_file_data: FileDataHook,
    /// Receives raw request trailer data, which can be available on requests that have
    /// chunked bodies. The data starts immediately after the zero-length chunk
    /// and includes the terminating empty line.
    pub hook_request_trailer_data: DataHook,
    /// Request trailer hook, invoked after all trailer headers are seen,
    /// and if they are seen (not invoked otherwise).
    pub hook_request_trailer: TxHook,
    /// Request hook, invoked after a complete request is seen.
    pub hook_request_complete: TxHook,
    /// Response startup hook, invoked when a response transaction is found and
    /// processing started.
    pub hook_response_start: TxHook,
    /// Response line hook, invoked after a response line has been parsed.
    pub hook_response_line: TxHook,
    /// Receives raw response header data, starting immediately after the status line
    /// and including all headers as they are seen on the TCP connection, and including the
    /// terminating empty line. Not available on genuine HTTP/0.9 responses (because
    /// they don't have response headers).
    pub hook_response_header_data: DataHook,
    /// Response headers book, invoked after all response headers have been seen.
    pub hook_response_headers: TxHook,
    /// Response body data hook, invoked every time body data is available. Each
    /// invocation will provide a Data instance. Chunked data
    /// will be dechunked before the data is passed to this hook. By default,
    /// compressed data will be decompressed, but decompression can be disabled
    /// in configuration. At the end of the response body there will be a call
    /// with the data pointer set to NULL.
    pub hook_response_body_data: DataHook,
    /// Receives raw response trailer data, which can be available on responses that have
    /// chunked bodies. The data starts immediately after the zero-length chunk
    /// and includes the terminating empty line.
    pub hook_response_trailer_data: DataHook,
    /// Response trailer hook, invoked after all trailer headers have been processed,
    /// and only if the trailer exists.
    pub hook_response_trailer: TxHook,
    /// Response hook, invoked after a response has been seen. Because sometimes servers
    /// respond before receiving complete requests, a response_complete callback may be
    /// invoked prior to a request_complete callback.
    pub hook_response_complete: TxHook,
    /// Transaction complete hook, which is invoked once the entire transaction is
    /// considered complete (request and response are both complete). This is always
    /// the last hook to be invoked.
    pub hook_transaction_complete: TxHook,
    /// Log hook, invoked every time the library wants to log.
    pub hook_log: LogHook,
    // Request Line parsing options.

    // TODO this was added here to maintain a stable ABI, once we can break that
    // we may want to move this into DecoderConfig (VJ)
    /// Reaction to leading whitespace on the request line
    pub requestline_leading_whitespace_unwanted: htp_unwanted_t,
    /// How many layers of compression we will decompress (0 => no limit).
    pub response_decompression_layer_limit: i32,
    /// max memory use by a the lzma decompressor.
    pub lzma_memlimit: usize,
    /// How many layers of compression we will decompress (0 => no lzma).
    pub response_lzma_layer_limit: i32,
    /// max output size for a compression bomb.
    pub compression_bomb_limit: i32,
    /// max time for a decompression bomb.
    pub compression_time_limit: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            field_limit: 18000,
            log_level: htp_log_level_t::HTP_LOG_NOTICE,
            tx_auto_destroy: false,
            server_personality: htp_server_personality_t::HTP_SERVER_MINIMAL,
            parameter_processor: None,
            decoder_cfg: Default::default(),
            response_decompression_enabled: true,
            parse_request_cookies: true,
            parse_request_auth: true,
            extract_request_files: false,
            extract_request_files_limit: 16,
            tmpdir: "/tmp".to_string(),
            hook_request_start: TxHook::new(),
            hook_request_line: TxHook::new(),
            hook_request_uri_normalize: TxHook::new(),
            hook_request_header_data: DataHook::new(),
            hook_request_headers: TxHook::new(),
            hook_request_body_data: DataHook::new(),
            hook_request_file_data: FileDataHook::new(),
            hook_request_trailer_data: DataHook::new(),
            hook_request_trailer: TxHook::new(),
            hook_request_complete: TxHook::new(),
            hook_response_start: TxHook::new(),
            hook_response_line: TxHook::new(),
            hook_response_header_data: DataHook::new(),
            hook_response_headers: TxHook::new(),
            hook_response_body_data: DataHook::new(),
            hook_response_trailer_data: DataHook::new(),
            hook_response_trailer: TxHook::new(),
            hook_response_complete: TxHook::new(),
            hook_transaction_complete: TxHook::new(),
            hook_log: LogHook::new(),
            requestline_leading_whitespace_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            response_decompression_layer_limit: 2,
            lzma_memlimit: 1048576,
            response_lzma_layer_limit: 1,
            compression_bomb_limit: 1048576,
            compression_time_limit: 100000,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DecoderConfig {
    // Path-specific decoding options.
    /// Convert backslash characters to slashes.
    pub backslash_convert_slashes: bool,
    /// Convert to lowercase.
    pub convert_lowercase: bool,
    /// Compress slash characters.
    pub path_separators_compress: bool,
    /// Should we URL-decode encoded path segment separators?
    pub path_separators_decode: bool,
    /// Should we decode '+' characters to spaces?
    pub plusspace_decode: bool,
    /// Reaction to encoded path separators.
    pub path_separators_encoded_unwanted: htp_unwanted_t,
    // Special characters options.
    /// Controls how raw NUL bytes are handled.
    pub nul_raw_terminates: bool,
    /// Determines server response to a raw NUL byte in the path.
    pub nul_raw_unwanted: htp_unwanted_t,
    /// Reaction to control characters.
    pub control_chars_unwanted: htp_unwanted_t,
    // URL encoding options.
    /// Should we decode %u-encoded characters?
    pub u_encoding_decode: bool,
    /// Reaction to %u encoding.
    pub u_encoding_unwanted: htp_unwanted_t,
    /// Handling of invalid URL encodings.
    pub url_encoding_invalid_handling: htp_url_encoding_handling_t,
    /// Reaction to invalid URL encoding.
    pub url_encoding_invalid_unwanted: htp_unwanted_t,
    /// Controls how encoded NUL bytes are handled.
    pub nul_encoded_terminates: bool,
    /// How are we expected to react to an encoded NUL byte?
    pub nul_encoded_unwanted: htp_unwanted_t,
    // UTF-8 options.
    /// Controls how invalid UTF-8 characters are handled.
    pub utf8_invalid_unwanted: htp_unwanted_t,
    /// Convert UTF-8 characters into bytes using best-fit mapping.
    pub utf8_convert_bestfit: bool,
    // Best-fit mapping options.
    /// The best-fit map to use to decode %u-encoded characters.
    pub bestfit_map: &'static [u8],
    /// The replacement byte used when there is no best-fit mapping.
    pub bestfit_replacement_byte: u8,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            backslash_convert_slashes: false,
            convert_lowercase: false,
            path_separators_compress: false,
            path_separators_decode: false,
            plusspace_decode: true,
            path_separators_encoded_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            nul_raw_terminates: false,
            nul_raw_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            control_chars_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            u_encoding_decode: false,
            u_encoding_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            url_encoding_invalid_handling:
                htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
            url_encoding_invalid_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            nul_encoded_terminates: false,
            nul_encoded_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            utf8_invalid_unwanted: htp_unwanted_t::HTP_UNWANTED_IGNORE,
            utf8_convert_bestfit: false,
            bestfit_map: &bestfit_1252,
            bestfit_replacement_byte: '?' as u8,
        }
    }
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

fn config_alloc() -> *mut Config {
    let cfg: Config = Default::default();
    let b = Box::new(cfg);
    Box::into_raw(b)
}

fn config_free(cfg: *mut Config) {
    if !cfg.is_null() {
        unsafe {
            Box::from_raw(cfg);
        }
    }
}

/// Creates a new configuration structure. Configuration structures created at
/// configuration time must not be changed afterwards in order to support lock-less
/// copying.
pub fn create() -> *mut Config {
    let cfg: *mut Config = config_alloc();
    cfg
}

impl Config {
    /// Destroy a configuration structure.
    pub fn destroy(&mut self) {
        config_free(self);
    }

    /// Registers a callback that is invoked every time there is a log message with
    /// severity equal and higher than the configured log level.
    pub unsafe fn register_log(&mut self, cbk_fn: LogNativeCallbackFn) {
        self.hook_log.register(cbk_fn);
    }

    /// Adds the built-in Multipart parser to the configuration. This parser will extract information
    /// stored in request bodies, when they are in multipart/form-data format.
    pub fn register_multipart_parser(&mut self) {
        self.hook_request_headers
            .register(content_handlers::htp_ch_multipart_callback_request_headers)
    }

    /// Registers a REQUEST_COMPLETE callback.
    pub unsafe fn register_request_complete(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_request_complete.register(cbk_fn);
    }

    /// Registers a REQUEST_BODY_DATA callback.
    pub fn register_request_body_data(&mut self, cbk_fn: DataNativeCallbackFn) {
        self.hook_request_body_data.register(cbk_fn);
    }

    /// Registers a REQUEST_HEADER_DATA callback.
    pub unsafe fn register_request_header_data(&mut self, cbk_fn: DataNativeCallbackFn) {
        self.hook_request_header_data.register(cbk_fn);
    }

    /// Registers a REQUEST_HEADERS callback.
    pub unsafe fn register_request_headers(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_request_headers.register(cbk_fn);
    }

    /// Registers a REQUEST_LINE callback.
    pub unsafe fn register_request_line(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_request_line.register(cbk_fn);
    }

    /// Registers a REQUEST_START callback, which is invoked every time a new
    /// request begins and before any parsing is done.
    pub unsafe fn register_request_start(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_request_start.register(cbk_fn);
    }

    /// Registers a HTP_REQUEST_TRAILER callback.
    pub unsafe fn register_request_trailer(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_request_trailer.register(cbk_fn);
    }

    /// Registers a REQUEST_TRAILER_DATA callback.
    pub unsafe fn register_request_trailer_data(&mut self, cbk_fn: DataNativeCallbackFn) {
        self.hook_request_trailer_data.register(cbk_fn);
    }

    /// Registers a RESPONSE_BODY_DATA callback.
    pub unsafe fn register_response_body_data(&mut self, cbk_fn: DataNativeCallbackFn) {
        self.hook_response_body_data.register(cbk_fn);
    }

    /// Registers a RESPONSE_COMPLETE callback.
    pub unsafe fn register_response_complete(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_response_complete.register(cbk_fn);
    }

    /// Registers a RESPONSE_HEADER_DATA callback.
    pub unsafe fn register_response_header_data(&mut self, cbk_fn: DataNativeCallbackFn) {
        self.hook_response_header_data.register(cbk_fn);
    }

    /// Registers a RESPONSE_HEADERS callback.
    #[allow(dead_code)]
    pub unsafe fn register_response_headers(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_response_headers.register(cbk_fn);
    }

    /// Registers a RESPONSE_LINE callback.
    #[allow(dead_code)]
    pub unsafe fn register_response_line(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_response_line.register(cbk_fn);
    }

    /// Registers a RESPONSE_START callback.
    pub unsafe fn register_response_start(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_response_start.register(cbk_fn);
    }

    /// Registers a RESPONSE_TRAILER callback.
    pub unsafe fn register_response_trailer(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_response_trailer.register(cbk_fn);
    }

    /// Registers a RESPONSE_TRAILER_DATA callback.
    pub unsafe fn register_response_trailer_data(&mut self, cbk_fn: DataNativeCallbackFn) {
        self.hook_response_trailer_data.register(cbk_fn);
    }

    /// Registers a TRANSACTION_COMPLETE callback.
    pub unsafe fn register_transaction_complete(&mut self, cbk_fn: TxNativeCallbackFn) {
        self.hook_transaction_complete.register(cbk_fn);
    }

    /// Adds the built-in Urlencoded parser to the configuration. The parser will
    /// parse query strings and request bodies with the appropriate MIME type.
    #[allow(dead_code)]
    pub fn register_urlencoded_parser(&mut self) {
        self.hook_request_line
            .register(content_handlers::htp_ch_urlencoded_callback_request_line);
        self.hook_request_headers
            .register(content_handlers::htp_ch_urlencoded_callback_request_headers)
    }

    /// Configures the maximum size of the buffer LibHTP will use when all data is not available
    /// in the current buffer (e.g., a very long header line that might span several packets). This
    /// limit is controlled by the hard_limit parameter. The soft_limit parameter is not implemented.
    /// soft_limit is NOT IMPLEMENTED.
    pub fn set_field_limit(&mut self, field_limit: usize) {
        self.field_limit = field_limit;
    }

    /// Configures the maximum memlimit LibHTP will pass to liblzma.
    pub fn set_lzma_memlimit(&mut self, memlimit: usize) {
        self.lzma_memlimit = memlimit;
    }

    /// Configures the maximum layers LibHTP will pass to liblzma.
    pub fn set_lzma_layers(&mut self, limit: i32) {
        self.response_lzma_layer_limit = limit;
    }

    /// Configures the maximum compression bomb size LibHTP will decompress.
    pub fn set_compression_bomb_limit(&mut self, bomblimit: usize) {
        if bomblimit > std::i32::MAX as usize {
            self.compression_bomb_limit = std::i32::MAX
        } else {
            self.compression_bomb_limit = bomblimit as i32
        };
    }

    /// Enable or disable request cookie parsing. Enabled by default.
    pub fn set_parse_request_cookies(&mut self, parse_request_cookies: bool) {
        self.parse_request_cookies = parse_request_cookies;
    }

    /// Configure desired server personality.
    /// Returns an error if the personality is not supported.
    pub fn set_server_personality(&mut self, personality: htp_server_personality_t) -> Result<()> {
        match personality {
            htp_server_personality_t::HTP_SERVER_MINIMAL => {}
            htp_server_personality_t::HTP_SERVER_GENERIC => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
            }
            htp_server_personality_t::HTP_SERVER_IDS => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_convert_lowercase(true);
                self.set_utf8_convert_bestfit(true);
                self.set_u_encoding_decode(true);
                self.set_requestline_leading_whitespace_unwanted(
                    htp_unwanted_t::HTP_UNWANTED_IGNORE,
                );
            }
            htp_server_personality_t::HTP_SERVER_APACHE_2 => {
                self.set_backslash_convert_slashes(false);
                self.set_path_separators_decode(false);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(false);
                self.set_url_encoding_invalid_handling(
                    htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
                );
                self.set_url_encoding_invalid_unwanted(htp_unwanted_t::HTP_UNWANTED_400);
                self.set_control_chars_unwanted(htp_unwanted_t::HTP_UNWANTED_IGNORE);
                self.set_requestline_leading_whitespace_unwanted(htp_unwanted_t::HTP_UNWANTED_400);
            }
            htp_server_personality_t::HTP_SERVER_IIS_5_1 => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(false);
                self.set_url_encoding_invalid_handling(
                    htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
                );
                self.set_control_chars_unwanted(htp_unwanted_t::HTP_UNWANTED_IGNORE);
                self.set_requestline_leading_whitespace_unwanted(
                    htp_unwanted_t::HTP_UNWANTED_IGNORE,
                );
            }
            htp_server_personality_t::HTP_SERVER_IIS_6_0 => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(true);
                self.set_url_encoding_invalid_handling(
                    htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
                );
                self.set_u_encoding_unwanted(htp_unwanted_t::HTP_UNWANTED_400);
                self.set_control_chars_unwanted(htp_unwanted_t::HTP_UNWANTED_400);
                self.set_requestline_leading_whitespace_unwanted(
                    htp_unwanted_t::HTP_UNWANTED_IGNORE,
                );
            }
            htp_server_personality_t::HTP_SERVER_IIS_7_0
            | htp_server_personality_t::HTP_SERVER_IIS_7_5 => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(true);
                self.set_url_encoding_invalid_handling(
                    htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
                );
                self.set_url_encoding_invalid_unwanted(htp_unwanted_t::HTP_UNWANTED_400);
                self.set_control_chars_unwanted(htp_unwanted_t::HTP_UNWANTED_400);
                self.set_requestline_leading_whitespace_unwanted(
                    htp_unwanted_t::HTP_UNWANTED_IGNORE,
                );
            }
            _ => return Err(Status::ERROR),
        }
        // Remember the personality
        self.server_personality = personality;
        Ok(())
    }

    /// Configures whether transactions will be automatically destroyed once they
    /// are processed and all callbacks invoked. This option is appropriate for
    /// programs that process transactions as they are processed.
    pub fn set_tx_auto_destroy(&mut self, tx_auto_destroy: bool) {
        self.tx_auto_destroy = tx_auto_destroy;
    }

    /// Configures a best-fit map, which is used whenever characters longer than one byte
    /// need to be converted to a single-byte. By default a Windows 1252 best-fit map is used.
    /// The map is an list of triplets, the first 2 bytes being an UCS-2 character to map from,
    /// and the third byte being the single byte to map to. Make sure that your map contains
    /// the mappings to cover the full-width and half-width form characters (U+FF00-FFEF). The
    /// last triplet in the map must be all zeros (3 NUL bytes).
    pub fn set_bestfit_map(&mut self, map: &'static [u8]) {
        self.decoder_cfg.bestfit_map = map;
    }

    /// Sets the replacement character that will be used to in the lossy best-fit
    /// mapping from multi-byte to single-byte streams. The question mark character
    /// is used as the default replacement byte.
    pub fn set_bestfit_replacement_byte(&mut self, b: u8) {
        self.decoder_cfg.bestfit_replacement_byte = b;
    }

    /// Configures how the server handles to invalid URL encoding.
    pub fn set_url_encoding_invalid_handling(&mut self, handling: htp_url_encoding_handling_t) {
        self.decoder_cfg.url_encoding_invalid_handling = handling;
    }

    /// Configures the handling of raw NUL bytes. If enabled, raw NUL terminates strings.
    pub fn set_nul_raw_terminates(&mut self, enabled: bool) {
        self.decoder_cfg.nul_raw_terminates = enabled;
    }

    /// Configures how the server reacts to encoded NUL bytes. Some servers will stop at
    /// at NUL, while some will respond with 400 or 404. When the termination option is not
    /// used, the NUL byte will remain in the path.
    pub fn set_nul_encoded_terminates(&mut self, enabled: bool) {
        self.decoder_cfg.nul_encoded_terminates = enabled;
    }

    /// Configures whether %u-encoded sequences are decoded. Such sequences
    /// will be treated as invalid URL encoding if decoding is not desirable.
    pub fn set_u_encoding_decode(&mut self, enabled: bool) {
        self.decoder_cfg.u_encoding_decode = enabled;
    }

    /// Configures whether backslash characters are treated as path segment separators. They
    /// are not on Unix systems, but are on Windows systems. If this setting is enabled, a path
    /// such as "/one\two/three" will be converted to "/one/two/three".
    pub fn set_backslash_convert_slashes(&mut self, enabled: bool) {
        self.decoder_cfg.backslash_convert_slashes = enabled;
    }

    /// Configures whether encoded path segment separators will be decoded. Apache does not do
    /// this by default, but IIS does. If enabled, a path such as "/one%2ftwo" will be normalized
    /// to "/one/two". If the backslash_separators option is also enabled, encoded backslash
    /// characters will be converted too (and subsequently normalized to forward slashes).
    pub fn set_path_separators_decode(&mut self, enabled: bool) {
        self.decoder_cfg.path_separators_decode = enabled;
    }

    /// Configures whether consecutive path segment separators will be compressed. When enabled, a path
    /// such as "/one//two" will be normalized to "/one/two". Backslash conversion and path segment separator
    /// decoding are carried out before compression. For example, the path "/one\\/two\/%5cthree/%2f//four"
    /// will be converted to "/one/two/three/four" (assuming all 3 options are enabled).
    pub fn set_path_separators_compress(&mut self, enabled: bool) {
        self.decoder_cfg.path_separators_compress = enabled;
    }

    /// Configures whether plus characters are converted to spaces when decoding URL-encoded strings. This
    /// is appropriate to do for parameters, but not for URLs. Only applies to contexts where decoding
    /// is taking place.
    pub fn set_plusspace_decode(&mut self, enabled: bool) {
        self.decoder_cfg.plusspace_decode = enabled;
    }

    /// Configures whether input data will be converted to lowercase. Useful for handling servers with
    /// case-insensitive filesystems.
    pub fn set_convert_lowercase(&mut self, enabled: bool) {
        self.decoder_cfg.convert_lowercase = enabled;
    }

    /// Controls whether the data should be treated as UTF-8 and converted to a single-byte
    /// stream using best-fit mapping.
    pub fn set_utf8_convert_bestfit(&mut self, enabled: bool) {
        self.decoder_cfg.utf8_convert_bestfit = enabled;
    }

    /// Configures reaction to %u-encoded sequences in input data.
    pub fn set_u_encoding_unwanted(&mut self, unwanted: htp_unwanted_t) {
        self.decoder_cfg.u_encoding_unwanted = unwanted;
    }

    /// Controls reaction to raw control characters in the data.
    pub fn set_control_chars_unwanted(&mut self, unwanted: htp_unwanted_t) {
        self.decoder_cfg.control_chars_unwanted = unwanted;
    }

    /// Configures how the server reacts to invalid URL encoding.
    pub fn set_url_encoding_invalid_unwanted(&mut self, unwanted: htp_unwanted_t) {
        self.decoder_cfg.url_encoding_invalid_unwanted = unwanted;
    }

    /// Configures how the server reacts to leading whitespace on the request line.
    pub fn set_requestline_leading_whitespace_unwanted(&mut self, unwanted: htp_unwanted_t) {
        self.requestline_leading_whitespace_unwanted = unwanted;
    }

    /// Configures many layers of compression we try to decompress.
    /// limit: 0 disables limit
    pub fn set_response_decompression_layer_limit(&mut self, limit: i32) {
        self.response_decompression_layer_limit = limit;
    }
}
