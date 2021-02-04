#![deny(missing_docs)]
use crate::{
    bstr::Bstr,
    config::{Config, HtpServerPersonality, HtpUrlEncodingHandling},
    connection::Connection,
    connection_parser::{ConnectionParser, HtpStreamState},
    hook::{DataExternalCallbackFn, LogExternalCallbackFn, TxExternalCallbackFn},
    transaction::{Header, Headers, Transaction},
    util::get_version,
    HtpStatus,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
};

/// Functions for working with Bstr.
pub mod bstr;
/// Functions for working with connection.
pub mod connection;
/// Functions for working with logs.
pub mod log;
/// Functions for working with lzma decompression.
pub mod lzma;
/// Functions for working with transactions.
pub mod transaction;
/// Functions for working with request uri.
pub mod uri;

/// Creates a new configuration structure. Configuration structures created at
/// configuration time must not be changed afterwards in order to support lock-less
/// copying.
#[no_mangle]
pub unsafe extern "C" fn htp_config_create() -> *mut Config {
    let cfg: Config = Default::default();
    let b = Box::new(cfg);
    Box::into_raw(b)
}

/// Destroy a configuration structure.
#[no_mangle]
pub unsafe extern "C" fn htp_config_destroy(cfg: *mut Config) {
    if !cfg.is_null() {
        let _ = Box::from_raw(cfg);
    }
}

/// Registers a REQUEST_BODY_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_body_data(
    cfg: *mut Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_body_data.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_COMPLETE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_complete(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_complete.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_HEADERS callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_headers(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_headers.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_HEADER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_header_data(
    cfg: *mut Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_header_data.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_LINE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_line(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_line.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_START callback, which is invoked every time a new
/// request begins and before any parsing is done.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_start(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_start.register_extern(cbk_fn)
    }
}

/// Registers a HTP_REQUEST_TRAILER callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_trailer.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_TRAILER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer_data(
    cfg: *mut Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_trailer_data.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_BODY_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_body_data(
    cfg: *mut Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_body_data.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_COMPLETE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_complete(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_complete.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_HEADERS callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_headers(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_headers.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_HEADER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_header_data(
    cfg: *mut Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_header_data.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_START callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_start(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_start.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_TRAILER callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_trailer.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_TRAILER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer_data(
    cfg: *mut Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_trailer_data.register_extern(cbk_fn)
    }
}

/// Registers a TRANSACTION_COMPLETE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_transaction_complete(
    cfg: *mut Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_transaction_complete.register_extern(cbk_fn)
    }
}

/// Configures whether backslash characters are treated as path segment separators. They
/// are not on Unix systems, but are on Windows systems. If this setting is enabled, a path
/// such as "/one\two/three" will be converted to "/one/two/three".
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_backslash_convert_slashes(
    cfg: *mut Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_backslash_convert_slashes(enabled == 1)
    }
}

/// Sets the replacement character that will be used to in the lossy best-fit
/// mapping from multi-byte to single-byte streams. The question mark character
/// is used as the default replacement byte.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_bestfit_replacement_byte(cfg: *mut Config, b: libc::c_int) {
    if !cfg.is_null() {
        (*cfg).set_bestfit_replacement_byte(b as u8)
    }
}

/// Configures the maximum compression bomb size LibHTP will decompress.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_compression_bomb_limit(
    cfg: *mut Config,
    bomblimit: libc::size_t,
) {
    if !cfg.is_null() {
        (*cfg).compression_options.set_bomb_limit(bomblimit)
    }
}

/// Configures whether input data will be converted to lowercase. Useful for handling servers with
/// case-insensitive filesystems.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_convert_lowercase(cfg: *mut Config, enabled: libc::c_int) {
    if !cfg.is_null() {
        (*cfg).set_convert_lowercase(enabled == 1)
    }
}

/// Configures the maximum size of the buffer LibHTP will use when all data is not available
/// in the current buffer (e.g., a very long header line that might span several packets). This
/// limit is controlled by the field_limit parameter.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_field_limit(cfg: *mut Config, field_limit: libc::size_t) {
    if !cfg.is_null() {
        (*cfg).set_field_limit(field_limit)
    }
}

/// Configures the maximum memlimit LibHTP will pass to liblzma.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_lzma_memlimit(cfg: *mut Config, memlimit: libc::size_t) {
    if !cfg.is_null() {
        (*cfg).compression_options.set_lzma_memlimit(memlimit)
    }
}

/// Configures the maximum number of lzma layers to pass to the decompressor.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_lzma_layers(cfg: *mut Config, layer: u32) {
    if !cfg.is_null() {
        (*cfg).compression_options.set_lzma_layers(layer)
    }
}

/// Configures how the server reacts to encoded NUL bytes. Some servers will stop at
/// at NUL, while some will respond with 400 or 404. When the termination option is not
/// used, the NUL byte will remain in the path.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_encoded_terminates(
    cfg: *mut Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_nul_encoded_terminates(enabled == 1)
    }
}

/// Configures the handling of raw NUL bytes. If enabled, raw NUL terminates strings.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_raw_terminates(cfg: *mut Config, enabled: libc::c_int) {
    if !cfg.is_null() {
        (*cfg).set_nul_raw_terminates(enabled == 1)
    }
}

/// Enable or disable request cookie parsing. Enabled by default.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_request_cookies(
    cfg: *mut Config,
    parse_request_cookies: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_parse_request_cookies(parse_request_cookies == 1)
    }
}

/// Configures whether consecutive path segment separators will be compressed. When enabled, a path
/// such as "/one//two" will be normalized to "/one/two". Backslash conversion and path segment separator
/// decoding are carried out before compression. For example, the path "/one\\/two\/%5cthree/%2f//four"
/// will be converted to "/one/two/three/four" (assuming all 3 options are enabled).
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_compress(
    cfg: *mut Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_path_separators_compress(enabled == 1)
    }
}

/// Configures whether plus characters are converted to spaces when decoding URL-encoded strings. This
/// is appropriate to do for parameters, but not for URLs. Only applies to contexts where decoding
/// is taking place.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_plusspace_decode(cfg: *mut Config, enabled: libc::c_int) {
    if !cfg.is_null() {
        (*cfg).set_plusspace_decode(enabled == 1)
    }
}

/// Configures whether encoded path segment separators will be decoded. Apache does not do
/// this by default, but IIS does. If enabled, a path such as "/one%2ftwo" will be normalized
/// to "/one/two". If the backslash_separators option is also enabled, encoded backslash
/// characters will be converted too (and subsequently normalized to forward slashes).
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_decode(
    cfg: *mut Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_path_separators_decode(enabled == 1)
    }
}

/// Configures many layers of compression we try to decompress.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_response_decompression_layer_limit(
    cfg: *mut Config,
    limit: libc::c_int,
) {
    if !cfg.is_null() {
        if limit <= 0 {
            (*cfg).set_response_decompression_layer_limit(None)
        } else {
            (*cfg).set_response_decompression_layer_limit(Some(limit as usize))
        }
    }
}

/// Configure desired server personality.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_server_personality(
    cfg: *mut Config,
    personality: HtpServerPersonality,
) -> HtpStatus {
    if !cfg.is_null() {
        (*cfg).set_server_personality(personality).into()
    } else {
        HtpStatus::ERROR
    }
}

/// Configures whether %u-encoded sequences are decoded. Such sequences
/// will be treated as invalid URL encoding if decoding is not desirable.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_u_encoding_decode(cfg: *mut Config, enabled: libc::c_int) {
    if !cfg.is_null() {
        (*cfg).set_u_encoding_decode(enabled == 1)
    }
}

/// Configures how the server handles to invalid URL encoding.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_url_encoding_invalid_handling(
    cfg: *mut Config,
    handling: HtpUrlEncodingHandling,
) {
    if !cfg.is_null() {
        (*cfg).set_url_encoding_invalid_handling(handling)
    }
}

/// Controls whether the data should be treated as UTF-8 and converted to a single-byte
/// stream using best-fit mapping.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_utf8_convert_bestfit(
    cfg: *mut Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_utf8_convert_bestfit(enabled == 1)
    }
}

/// Closes the connection associated with the supplied parser.
///
/// timestamp is optional
#[no_mangle]
pub unsafe extern "C" fn htp_connp_close(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
) {
    if let Some(connp) = connp.as_mut() {
        connp.close(timestamp.as_ref().map(|val| {
            DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(val.tv_sec, val.tv_usec as u32),
                Utc,
            )
        }))
    }
}

/// Creates a new connection parser using the provided configuration or a default configuration if NULL provided.
/// Note the provided config will be copied into the created connection parser. Therefore, subsequent modification
/// to the original config will have no effect.
///
/// Returns a new connection parser instance, or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_create(cfg: *mut Config) -> *mut ConnectionParser {
    if let Some(cfg) = cfg.as_ref() {
        Box::into_raw(Box::new(ConnectionParser::new(cfg.clone())))
    } else {
        Box::into_raw(Box::new(ConnectionParser::new(Config::default())))
    }
}

/// Destroys the connection parser, its data structures, as well
/// as the connection and its transactions.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_destroy_all(connp: *mut ConnectionParser) {
    let _ = Box::from_raw(connp);
}

/// Returns the connection associated with the connection parser.
///
/// Returns Connection instance, or NULL if one is not available.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_connection(connp: *mut ConnectionParser) -> *mut Connection {
    connp
        .as_mut()
        .map(|val| &mut val.conn as *mut Connection)
        .unwrap_or(std::ptr::null_mut())
}

/// Retrieve the user data associated with this connection parser.
/// Returns user data, or NULL if there isn't any.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_user_data(connp: *const ConnectionParser) -> *mut libc::c_void {
    connp
        .as_ref()
        .and_then(|val| val.user_data::<*mut libc::c_void>())
        .map(|val| *val)
        .unwrap_or(std::ptr::null_mut())
}

/// Associate user data with the supplied parser.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_set_user_data(
    connp: *mut ConnectionParser,
    user_data: *mut libc::c_void,
) {
    if let Some(connp) = connp.as_mut() {
        connp.set_user_data(Box::new(user_data));
    }
}

/// Opens connection.
///
/// timestamp is optional
#[no_mangle]
pub unsafe extern "C" fn htp_connp_open(
    connp: *mut ConnectionParser,
    client_addr: *const libc::c_char,
    client_port: libc::c_int,
    server_addr: *const libc::c_char,
    server_port: libc::c_int,
    timestamp: *const libc::timeval,
) {
    let connp = if let Some(connp) = connp.as_mut() {
        connp
    } else {
        return;
    };
    let client_addr = client_addr.as_ref().and_then(|client_addr| {
        CStr::from_ptr(client_addr)
            .to_str()
            .ok()
            .and_then(|val| val.parse().ok())
    });
    let client_port = if client_port >= 0 || client_port <= std::u16::MAX as libc::c_int {
        Some(client_port as u16)
    } else {
        None
    };
    let server_addr = server_addr.as_ref().and_then(|server_addr| {
        CStr::from_ptr(server_addr)
            .to_str()
            .ok()
            .and_then(|val| val.parse().ok())
    });
    let server_port = if server_port >= 0 || server_port <= std::u16::MAX as libc::c_int {
        Some(server_port as u16)
    } else {
        None
    };
    let timestamp = timestamp.as_ref().map(|timestamp| {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(timestamp.tv_sec, timestamp.tv_usec as u32),
            Utc,
        )
    });
    connp.open(
        client_addr,
        client_port,
        server_addr,
        server_port,
        timestamp,
    )
}

/// Closes the connection associated with the supplied parser.
///
/// timestamp is optional
#[no_mangle]
pub unsafe extern "C" fn htp_connp_req_close(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
) {
    if let Some(connp) = connp.as_mut() {
        connp.req_close(timestamp.as_ref().map(|val| {
            DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(val.tv_sec, val.tv_usec as u32),
                Utc,
            )
        }))
    }
}

/// Process a chunk of inbound client request data
///
/// timestamp is optional
/// Returns HTP_STREAM_STATE_DATA, HTP_STREAM_STATE_ERROR or HTP_STREAM_STATE_DATA_OTHER (see QUICK_START).
///         HTP_STREAM_STATE_CLOSED and HTP_STREAM_STATE_TUNNEL are also possible.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_req_data(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
    data: *const libc::c_void,
    len: libc::size_t,
) -> HtpStreamState {
    if let Some(connp) = connp.as_mut() {
        connp.req_data(
            timestamp.as_ref().map(|val| {
                DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(val.tv_sec, val.tv_usec as u32),
                    Utc,
                )
            }),
            data,
            len,
        )
    } else {
        HtpStreamState::ERROR
    }
}

/// Process a chunk of outbound (server or response) data.
///
/// timestamp is optional.
/// Returns HTP_STREAM_STATE_OK on state change, HTP_STREAM_STATE_ERROR on error, or HTP_STREAM_STATE_DATA when more data is needed
#[no_mangle]
pub unsafe extern "C" fn htp_connp_res_data(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
    data: *const libc::c_void,
    len: libc::size_t,
) -> HtpStreamState {
    if let Some(connp) = connp.as_mut() {
        connp.res_data(
            timestamp.as_ref().map(|val| {
                DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(val.tv_sec, val.tv_usec as u32),
                    Utc,
                )
            }),
            data,
            len,
        )
    } else {
        HtpStreamState::ERROR
    }
}

/// Returns the LibHTP version string.
#[no_mangle]
pub unsafe extern "C" fn htp_get_version() -> *const libc::c_char {
    get_version()
}

/// Get the first header value matching the key.
///
/// headers: Header table.
/// ckey: Header name to match.
///
/// Returns the header or NULL when not found or on error
#[no_mangle]
pub unsafe extern "C" fn htp_headers_get(
    headers: *const Headers,
    ckey: *const libc::c_char,
) -> *const Header {
    if let (Some(headers), Some(ckey)) = (headers.as_ref(), ckey.as_ref()) {
        if let Some((_, value)) =
            headers.get_nocase_nozero(std::ffi::CStr::from_ptr(ckey).to_bytes())
        {
            value
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

/// Get the header at a given index.
///
/// headers: Header table.
/// index: Index into the table.
///
/// Returns the header or NULL when not found or on error
#[no_mangle]
pub unsafe extern "C" fn htp_headers_get_index(
    headers: *const Headers,
    index: usize,
) -> *const Header {
    if let Some(headers) = headers.as_ref() {
        if let Some((_, value)) = headers.get(index) {
            value
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

/// Get the size of the headers table.
///
/// headers: Headers table.
///
/// Returns the size or -1 on error
#[no_mangle]
pub unsafe extern "C" fn htp_headers_size(headers: *const Headers) -> isize {
    if let Some(headers) = headers.as_ref() {
        isize::try_from(headers.size()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Get the name of a header.
///
/// header: Header pointer.
///
/// Returns the name or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_header_name(header: *const Header) -> *const Bstr {
    if let Some(header) = header.as_ref() {
        &header.name
    } else {
        std::ptr::null()
    }
}

/// Get the name of a header as a ptr.
///
/// header: Header pointer.
///
/// Returns the pointer or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_header_name_ptr(header: *const Header) -> *const u8 {
    if let Some(header) = header.as_ref() {
        bstr::bstr_ptr(&header.name)
    } else {
        std::ptr::null()
    }
}

/// Get the length of a header name.
///
/// tx: Header pointer.
///
/// Returns the length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_header_name_len(header: *const Header) -> isize {
    if let Some(header) = header.as_ref() {
        isize::try_from(header.name.len()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Get the value of a header.
///
/// tx: Header pointer.
///
/// Returns the value or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_header_value(header: *const Header) -> *const Bstr {
    if let Some(header) = header.as_ref() {
        &header.value
    } else {
        std::ptr::null()
    }
}

/// Get the value of a header as a ptr.
///
/// tx: Header pointer.
///
/// Returns the pointer or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_header_value_ptr(header: *const Header) -> *const u8 {
    if let Some(header) = header.as_ref() {
        bstr::bstr_ptr(&header.value)
    } else {
        std::ptr::null()
    }
}

/// Get the length of a header value.
///
/// tx: Header pointer.
///
/// Returns the length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_header_value_len(header: *const Header) -> isize {
    if let Some(header) = header.as_ref() {
        isize::try_from(header.value.len()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Configures whether to attempt to decode a double encoded query in the normalized uri
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_double_decode_normalized_query(
    cfg: *mut Config,
    set: bool,
) {
    if !cfg.is_null() {
        (*cfg).set_double_decode_normalized_query(set);
    }
}

/// Configures whether to attempt to decode a double encoded path in the normalized uri
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_double_decode_normalized_path(cfg: *mut Config, set: bool) {
    if !cfg.is_null() {
        (*cfg).set_double_decode_normalized_path(set);
    }
}

/// Configures whether to normalize URIs into a complete or partial form.
/// Pass `true` to use complete normalized URI or `false` to use partials.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_normalized_uri_include_all(cfg: *mut Config, set: bool) {
    if !cfg.is_null() {
        (*cfg).set_normalized_uri_include_all(set);
    }
}

/// Configures whether transactions will be automatically destroyed once they
/// are processed and all callbacks invoked. This option is appropriate for
/// programs that process transactions as they are processed.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_tx_auto_destroy(
    cfg: *mut Config,
    tx_auto_destroy: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_tx_auto_destroy(tx_auto_destroy == 1)
    }
}

/// Registers a callback that is invoked every time there is a log message with
/// severity equal and higher than the configured log level.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_log(cfg: *mut Config, cbk_fn: LogExternalCallbackFn) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_log.register_extern(cbk_fn)
    }
}

/// Enable or Disable the built-in Multipart parser to the configuration. Disabled by default.
/// This parser will extract information stored in request bodies, when they are in multipart/form-data format.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_multipart(
    cfg: *mut Config,
    parse_multipart: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_parse_multipart(parse_multipart == 1)
    }
}

/// Enable or disable the built-in Urlencoded parser. Disabled by default.
/// The parser will parse query strings and request bodies with the appropriate MIME type.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_urlencoded(
    cfg: *mut Config,
    parse_urlencoded: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_parse_urlencoded(parse_urlencoded == 1)
    }
}

/// Get the number of transactions processed on this connection.
///
/// Returns the number of transactions or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_tx_size(connp: *const ConnectionParser) -> isize {
    if let Some(connp) = connp.as_ref() {
        isize::try_from(connp.tx_size()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Get a transaction.
///
/// Returns the transaction or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_tx(
    connp: *mut ConnectionParser,
    tx_id: usize,
) -> *mut Transaction {
    if let Some(connp) = connp.as_mut() {
        if let Some(tx) = connp.tx_mut(tx_id) {
            tx as *mut Transaction
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Retrieves the pointer to the active response transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Can be NULL.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_response_tx(
    connp: *mut ConnectionParser,
) -> *mut Transaction {
    connp
        .as_mut()
        .map(|connp| connp.response_mut() as *mut Transaction)
        .unwrap_or(std::ptr::null_mut())
}

/// Retrieves the pointer to the active request transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Call be NULL.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_request_tx(
    connp: *mut ConnectionParser,
) -> *mut Transaction {
    connp
        .as_mut()
        .map(|connp| connp.request_mut() as *mut Transaction)
        .unwrap_or(std::ptr::null_mut())
}

/// Invoke the transaction complete callback for each incomplete transaction.
/// The transactions passed to the callback will not have their request and
/// response state set to complete - they will simply be passed with the state
/// they have within the parser at the time of the call.
///
/// This function is intended to be used when a connection is closing and we want
/// to process any incomplete transactions that were in flight, or which never
/// completed due to packet loss or parsing errors.
///
/// This function will also cause these transactions to be removed from the parser.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_flush_incomplete_transactions(connp: *mut ConnectionParser) {
    connp
        .as_mut()
        .map(|connp| connp.flush_incomplete_transactions());
}

/// Returns the number of bytes consumed from the current data chunks so far or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_req_data_consumed(connp: *const ConnectionParser) -> i64 {
    if let Some(connp) = connp.as_ref() {
        (*connp).req_data_consumed()
    } else {
        -1
    }
}

/// Returns the number of bytes consumed from the most recent outbound data chunk. Normally, an invocation
/// of htp_connp_res_data() will consume all data from the supplied buffer, but there are circumstances
/// where only partial consumption is possible. In such cases HTP_STREAM_DATA_OTHER will be returned.
/// Consumed bytes are no longer necessary, but the remainder of the buffer will be need to be saved
/// for later.
/// Returns the number of bytes consumed from the last data chunk sent for outbound processing
/// or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_res_data_consumed(connp: *const ConnectionParser) -> i64 {
    if let Some(connp) = connp.as_ref() {
        (*connp).res_data_consumed()
    } else {
        -1
    }
}

/// Free rust allocated cstring
#[no_mangle]
pub unsafe extern "C" fn htp_free_cstring(input: *mut libc::c_char) {
    if !input.is_null() {
        CString::from_raw(input);
    }
}
