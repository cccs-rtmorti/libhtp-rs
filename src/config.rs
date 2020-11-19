use crate::decompressors::Options;
use crate::{
    content_handlers::{
        callback_multipart_request_headers, callback_urlencoded_request_headers,
        callback_urlencoded_request_line,
    },
    error::Result,
    hook::{
        DataHook, DataNativeCallbackFn, FileDataHook, LogHook, LogNativeCallbackFn, TxHook,
        TxNativeCallbackFn,
    },
    log::HtpLogLevel,
    transaction::Param,
    unicode_bestfit_map::UnicodeBestfitMap,
    HtpStatus,
};

#[derive(Clone)]
pub struct Config {
    /// The maximum size of the buffer that is used when the current
    /// input chunk does not contain all the necessary data (e.g., a very header
    /// line that spans several packets).
    pub field_limit: usize,
    /// Log level, which will be used when deciding whether to store or
    /// ignore the messages issued by the parser.
    pub log_level: HtpLogLevel,
    /// Whether to delete each transaction after the last hook is invoked. This
    /// feature should be used when parsing traffic streams in real time.
    pub tx_auto_destroy: bool,
    /// Server personality identifier.
    pub server_personality: HtpServerPersonality,
    /// The function to use to transform parameters after parsing.
    pub parameter_processor: Option<fn(_: &mut Param) -> Result<()>>,
    /// Decoder configuration for url path.
    pub decoder_cfg: DecoderConfig,
    /// Whether to decompress compressed response bodies.
    pub response_decompression_enabled: bool,
    /// Whether to parse request cookies.
    pub parse_request_cookies: bool,
    /// Whether to parse HTTP Authentication headers.
    pub parse_request_auth: bool,
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
    pub requestline_leading_whitespace_unwanted: HtpUnwanted,
    /// How many layers of compression we will decompress (0 => no limit).
    pub response_decompression_layer_limit: i32,
    /// decompression options
    pub compression_options: Options,
    pub multipart_cfg: MultipartConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            field_limit: 18000,
            log_level: HtpLogLevel::NOTICE,
            tx_auto_destroy: false,
            server_personality: HtpServerPersonality::MINIMAL,
            parameter_processor: None,
            decoder_cfg: Default::default(),
            response_decompression_enabled: true,
            parse_request_cookies: true,
            parse_request_auth: true,
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
            requestline_leading_whitespace_unwanted: HtpUnwanted::IGNORE,
            response_decompression_layer_limit: 2,
            compression_options: Options::default(),
            multipart_cfg: Default::default(),
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
    pub path_separators_encoded_unwanted: HtpUnwanted,
    // Special characters options.
    /// Controls how raw NUL bytes are handled.
    pub nul_raw_terminates: bool,
    /// Determines server response to a raw NUL byte in the path.
    pub nul_raw_unwanted: HtpUnwanted,
    /// Reaction to control characters.
    pub control_chars_unwanted: HtpUnwanted,
    // URL encoding options.
    /// Should we decode %u-encoded characters?
    pub u_encoding_decode: bool,
    /// Reaction to %u encoding.
    pub u_encoding_unwanted: HtpUnwanted,
    /// Handling of invalid URL encodings.
    pub url_encoding_invalid_handling: HtpUrlEncodingHandling,
    /// Reaction to invalid URL encoding.
    pub url_encoding_invalid_unwanted: HtpUnwanted,
    /// Controls how encoded NUL bytes are handled.
    pub nul_encoded_terminates: bool,
    /// How are we expected to react to an encoded NUL byte?
    pub nul_encoded_unwanted: HtpUnwanted,
    // UTF-8 options.
    /// Controls how invalid UTF-8 characters are handled.
    pub utf8_invalid_unwanted: HtpUnwanted,
    /// Convert UTF-8 characters into bytes using best-fit mapping.
    pub utf8_convert_bestfit: bool,
    // Best-fit map
    pub bestfit_map: UnicodeBestfitMap,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            backslash_convert_slashes: false,
            convert_lowercase: false,
            path_separators_compress: false,
            path_separators_decode: false,
            plusspace_decode: true,
            path_separators_encoded_unwanted: HtpUnwanted::IGNORE,
            nul_raw_terminates: false,
            nul_raw_unwanted: HtpUnwanted::IGNORE,
            control_chars_unwanted: HtpUnwanted::IGNORE,
            u_encoding_decode: false,
            u_encoding_unwanted: HtpUnwanted::IGNORE,
            url_encoding_invalid_handling: HtpUrlEncodingHandling::PRESERVE_PERCENT,
            url_encoding_invalid_unwanted: HtpUnwanted::IGNORE,
            nul_encoded_terminates: false,
            nul_encoded_unwanted: HtpUnwanted::IGNORE,
            utf8_invalid_unwanted: HtpUnwanted::IGNORE,
            utf8_convert_bestfit: false,
            bestfit_map: UnicodeBestfitMap::default(),
        }
    }
}

#[derive(Clone)]
pub struct MultipartConfig {
    /// Whether to extract files from requests using Multipart encoding.
    pub extract_request_files: bool,
    /// How many extracted files are allowed in a single Multipart request?
    pub extract_request_files_limit: u32,
    /// The location on disk where temporary files will be created.
    pub tmpdir: String,
}

impl Default for MultipartConfig {
    fn default() -> Self {
        Self {
            extract_request_files: false,
            extract_request_files_limit: 16,
            tmpdir: "/tmp".to_string(),
        }
    }
}

/// Enumerates the possible server personalities.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HtpServerPersonality {
    /// Minimal personality that performs at little work as possible. All optional
    /// features are disabled. This personality is a good starting point for customization.
    MINIMAL,
    /// A generic personality that aims to work reasonably well for all server types.
    GENERIC,
    /// The IDS personality tries to perform as much decoding as possible.
    IDS,
    /// Mimics the behavior of IIS 4.0, as shipped with Windows NT 4.0.
    IIS_4_0,
    /// Mimics the behavior of IIS 5.0, as shipped with Windows 2000.
    IIS_5_0,
    /// Mimics the behavior of IIS 5.1, as shipped with Windows XP Professional.
    IIS_5_1,
    /// Mimics the behavior of IIS 6.0, as shipped with Windows 2003.
    IIS_6_0,
    /// Mimics the behavior of IIS 7.0, as shipped with Windows 2008.
    IIS_7_0,
    /// Mimics the behavior of IIS 7.5, as shipped with Windows 7.
    IIS_7_5,
    /// Mimics the behavior of Apache 2.x.
    APACHE_2,
}

/// Enumerates the ways in which servers respond to malformed data.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HtpUnwanted {
    /// Ignores problem.
    IGNORE,
    /// Responds with HTTP 400 status code.
    CODE_400 = 400,
    /// Responds with HTTP 404 status code.
    CODE_404 = 404,
}

/// Enumerates the possible approaches to handling invalid URL-encodings.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HtpUrlEncodingHandling {
    /// Ignore invalid URL encodings and leave the % in the data.
    PRESERVE_PERCENT,
    /// Ignore invalid URL encodings, but remove the % from the data.
    REMOVE_PERCENT,
    /// Decode invalid URL encodings.
    PROCESS_INVALID,
}

fn config_alloc() -> *mut Config {
    let cfg: Config = Default::default();
    let b = Box::new(cfg);
    Box::into_raw(b)
}

fn config_free(cfg: *mut Config) {
    if !cfg.is_null() {
        unsafe {
            let _ = Box::from_raw(cfg);
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
            .register(callback_multipart_request_headers)
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
            .register(callback_urlencoded_request_line);
        self.hook_request_headers
            .register(callback_urlencoded_request_headers)
    }

    /// Configures the maximum size of the buffer LibHTP will use when all data is not available
    /// in the current buffer (e.g., a very long header line that might span several packets). This
    /// limit is controlled by the hard_limit parameter. The soft_limit parameter is not implemented.
    /// soft_limit is NOT IMPLEMENTED.
    pub fn set_field_limit(&mut self, field_limit: usize) {
        self.field_limit = field_limit;
    }

    /// Enable or disable request cookie parsing. Enabled by default.
    pub fn set_parse_request_cookies(&mut self, parse_request_cookies: bool) {
        self.parse_request_cookies = parse_request_cookies;
    }

    /// Configure desired server personality.
    /// Returns an error if the personality is not supported.
    pub fn set_server_personality(&mut self, personality: HtpServerPersonality) -> Result<()> {
        match personality {
            HtpServerPersonality::MINIMAL => {}
            HtpServerPersonality::GENERIC => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
            }
            HtpServerPersonality::IDS => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_convert_lowercase(true);
                self.set_utf8_convert_bestfit(true);
                self.set_u_encoding_decode(true);
                self.set_requestline_leading_whitespace_unwanted(HtpUnwanted::IGNORE);
            }
            HtpServerPersonality::APACHE_2 => {
                self.set_backslash_convert_slashes(false);
                self.set_path_separators_decode(false);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(false);
                self.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
                self.set_url_encoding_invalid_unwanted(HtpUnwanted::CODE_400);
                self.set_control_chars_unwanted(HtpUnwanted::IGNORE);
                self.set_requestline_leading_whitespace_unwanted(HtpUnwanted::CODE_400);
            }
            HtpServerPersonality::IIS_5_1 => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(false);
                self.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
                self.set_control_chars_unwanted(HtpUnwanted::IGNORE);
                self.set_requestline_leading_whitespace_unwanted(HtpUnwanted::IGNORE);
            }
            HtpServerPersonality::IIS_6_0 => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(true);
                self.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
                self.set_u_encoding_unwanted(HtpUnwanted::CODE_400);
                self.set_control_chars_unwanted(HtpUnwanted::CODE_400);
                self.set_requestline_leading_whitespace_unwanted(HtpUnwanted::IGNORE);
            }
            HtpServerPersonality::IIS_7_0 | HtpServerPersonality::IIS_7_5 => {
                self.set_backslash_convert_slashes(true);
                self.set_path_separators_decode(true);
                self.set_path_separators_compress(true);
                self.set_u_encoding_decode(true);
                self.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
                self.set_url_encoding_invalid_unwanted(HtpUnwanted::CODE_400);
                self.set_control_chars_unwanted(HtpUnwanted::CODE_400);
                self.set_requestline_leading_whitespace_unwanted(HtpUnwanted::IGNORE);
            }
            _ => return Err(HtpStatus::ERROR),
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
    pub fn set_bestfit_map(&mut self, map: UnicodeBestfitMap) {
        self.decoder_cfg.bestfit_map = map;
    }

    /// Sets the replacement character that will be used to in the lossy best-fit
    /// mapping from multi-byte to single-byte streams. The question mark character
    /// is used as the default replacement byte.
    pub fn set_bestfit_replacement_byte(&mut self, b: u8) {
        self.decoder_cfg.bestfit_map.replacement_byte = b;
    }

    /// Configures how the server handles to invalid URL encoding.
    pub fn set_url_encoding_invalid_handling(&mut self, handling: HtpUrlEncodingHandling) {
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
    pub fn set_u_encoding_unwanted(&mut self, unwanted: HtpUnwanted) {
        self.decoder_cfg.u_encoding_unwanted = unwanted;
    }

    /// Controls reaction to raw control characters in the data.
    pub fn set_control_chars_unwanted(&mut self, unwanted: HtpUnwanted) {
        self.decoder_cfg.control_chars_unwanted = unwanted;
    }

    /// Configures how the server reacts to invalid URL encoding.
    pub fn set_url_encoding_invalid_unwanted(&mut self, unwanted: HtpUnwanted) {
        self.decoder_cfg.url_encoding_invalid_unwanted = unwanted;
    }

    /// Configures how the server reacts to leading whitespace on the request line.
    pub fn set_requestline_leading_whitespace_unwanted(&mut self, unwanted: HtpUnwanted) {
        self.requestline_leading_whitespace_unwanted = unwanted;
    }

    /// Configures many layers of compression we try to decompress.
    /// limit: 0 disables limit
    pub fn set_response_decompression_layer_limit(&mut self, limit: i32) {
        self.response_decompression_layer_limit = limit;
    }
}
