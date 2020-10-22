use crate::bstr;
use crate::config;
use crate::connection::Connection;
use crate::connection_parser::{ConnectionParser, HtpStreamState, Time};
use crate::hook::{DataExternalCallbackFn, LogExternalCallbackFn, TxExternalCallbackFn};
use crate::list;
use crate::log::{self, *};
use crate::transaction::{Header, Headers, Transaction};
use crate::util;
use crate::HtpStatus;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};

pub mod transaction;
pub mod uri;

/// Creates a new configuration structure. Configuration structures created at
/// configuration time must not be changed afterwards in order to support lock-less
/// copying.
#[no_mangle]
pub unsafe extern "C" fn htp_config_create() -> *mut config::Config {
    config::create()
}

/// Destroy a configuration structure.
#[no_mangle]
pub unsafe extern "C" fn htp_config_destroy(cfg: *mut config::Config) {
    if !cfg.is_null() {
        (*cfg).destroy()
    }
}

/// Registers a REQUEST_BODY_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_body_data(
    cfg: *mut config::Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_body_data.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_COMPLETE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_complete(
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_complete.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_HEADERS callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_headers(
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_headers.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_HEADER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_header_data(
    cfg: *mut config::Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_header_data.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_LINE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_line(
    cfg: *mut config::Config,
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
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_start.register_extern(cbk_fn)
    }
}

/// Registers a HTP_REQUEST_TRAILER callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer(
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_trailer.register_extern(cbk_fn)
    }
}

/// Registers a REQUEST_TRAILER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer_data(
    cfg: *mut config::Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_request_trailer_data.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_BODY_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_body_data(
    cfg: *mut config::Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_body_data.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_COMPLETE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_complete(
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_complete.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_HEADERS callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_headers(
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_headers.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_HEADER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_header_data(
    cfg: *mut config::Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_header_data.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_START callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_start(
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_start.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_TRAILER callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer(
    cfg: *mut config::Config,
    cbk_fn: TxExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_trailer.register_extern(cbk_fn)
    }
}

/// Registers a RESPONSE_TRAILER_DATA callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer_data(
    cfg: *mut config::Config,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_response_trailer_data.register_extern(cbk_fn)
    }
}

/// Registers a TRANSACTION_COMPLETE callback.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_transaction_complete(
    cfg: *mut config::Config,
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
    cfg: *mut config::Config,
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
pub unsafe extern "C" fn htp_config_set_bestfit_replacement_byte(
    cfg: *mut config::Config,
    b: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_bestfit_replacement_byte(b as u8)
    }
}

/// Configures the maximum layers LibHTP will pass to liblzma.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_lzma_layers(cfg: *mut config::Config, layer: libc::c_int) {
    if !cfg.is_null() {
        (*cfg).set_lzma_layers(layer)
    }
}

/// Configures the maximum compression bomb size LibHTP will decompress.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_compression_bomb_limit(
    cfg: *mut config::Config,
    bomblimit: libc::size_t,
) {
    if !cfg.is_null() {
        (*cfg).set_compression_bomb_limit(bomblimit)
    }
}

/// Configures whether input data will be converted to lowercase. Useful for handling servers with
/// case-insensitive filesystems.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_convert_lowercase(
    cfg: *mut config::Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_convert_lowercase(enabled == 1)
    }
}

/// Configures the maximum size of the buffer LibHTP will use when all data is not available
/// in the current buffer (e.g., a very long header line that might span several packets). This
/// limit is controlled by the hard_limit parameter. The soft_limit parameter is not implemented.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_field_limit(
    cfg: *mut config::Config,
    field_limit: libc::size_t,
) {
    if !cfg.is_null() {
        (*cfg).set_field_limit(field_limit)
    }
}

/// Configures the maximum memlimit LibHTP will pass to liblzma.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_lzma_memlimit(
    cfg: *mut config::Config,
    memlimit: libc::size_t,
) {
    if !cfg.is_null() {
        (*cfg).set_lzma_memlimit(memlimit)
    }
}

/// Configures how the server reacts to encoded NUL bytes. Some servers will stop at
/// at NUL, while some will respond with 400 or 404. When the termination option is not
/// used, the NUL byte will remain in the path.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_encoded_terminates(
    cfg: *mut config::Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_nul_encoded_terminates(enabled == 1)
    }
}

/// Configures the handling of raw NUL bytes. If enabled, raw NUL terminates strings.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_raw_terminates(
    cfg: *mut config::Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_nul_raw_terminates(enabled == 1)
    }
}

/// Enable or disable request cookie parsing. Enabled by default.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_request_cookies(
    cfg: *mut config::Config,
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
    cfg: *mut config::Config,
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
pub unsafe extern "C" fn htp_config_set_plusspace_decode(
    cfg: *mut config::Config,
    enabled: libc::c_int,
) {
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
    cfg: *mut config::Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_path_separators_decode(enabled == 1)
    }
}

/// Configures many layers of compression we try to decompress.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_response_decompression_layer_limit(
    cfg: *mut config::Config,
    limit: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_response_decompression_layer_limit(limit)
    }
}

/// Configure desired server personality.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_server_personality(
    cfg: *mut config::Config,
    personality: config::HtpServerPersonality,
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
pub unsafe extern "C" fn htp_config_set_u_encoding_decode(
    cfg: *mut config::Config,
    enabled: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_u_encoding_decode(enabled == 1)
    }
}

/// Configures how the server handles to invalid URL encoding.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_url_encoding_invalid_handling(
    cfg: *mut config::Config,
    handling: config::HtpUrlEncodingHandling,
) {
    if !cfg.is_null() {
        (*cfg).set_url_encoding_invalid_handling(handling)
    }
}

/// Controls whether the data should be treated as UTF-8 and converted to a single-byte
/// stream using best-fit mapping.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_utf8_convert_bestfit(
    cfg: *mut config::Config,
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
pub unsafe extern "C" fn htp_connp_close(connp: *mut ConnectionParser, timestamp: *const Time) {
    if let Some(connp) = connp.as_mut() {
        connp.close(timestamp.as_ref().map(|val| val.clone()))
    }
}

/// Creates a new connection parser using the provided configuration or a default configuration if NULL provided.
/// Note the provided config will be copied into the created connection parser. Therefore, subsequent modification
/// to the original config will have no effect.
///
/// Returns a new connection parser instance, or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_create(cfg: *mut config::Config) -> *mut ConnectionParser {
    if let Some(cfg) = cfg.as_ref() {
        Box::into_raw(Box::new(ConnectionParser::new(cfg.clone())))
    } else {
        Box::into_raw(Box::new(ConnectionParser::new(config::Config::default())))
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
pub unsafe extern "C" fn htp_connp_user_data(connp: *mut ConnectionParser) -> *mut libc::c_void {
    connp
        .as_mut()
        .map(|val| val.user_data)
        .unwrap_or(std::ptr::null_mut())
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
    timestamp: *const Time,
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
    let timestamp = timestamp.as_ref().map(|timestamp| timestamp.clone());
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
pub unsafe extern "C" fn htp_connp_req_close(connp: *mut ConnectionParser, timestamp: *const Time) {
    if let Some(connp) = connp.as_mut() {
        connp.req_close(timestamp.as_ref().map(|val| val.clone()))
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
    timestamp: *const Time,
    data: *const libc::c_void,
    len: libc::size_t,
) -> HtpStreamState {
    if let Some(connp) = connp.as_mut() {
        connp.req_data(timestamp.as_ref().map(|val| val.clone()), data, len)
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
    timestamp: *const Time,
    data: *const libc::c_void,
    len: libc::size_t,
) -> HtpStreamState {
    if let Some(connp) = connp.as_mut() {
        connp.res_data(timestamp.as_ref().map(|val| val.clone()), data, len)
    } else {
        HtpStreamState::ERROR
    }
}

/// Associate user data with the supplied parser.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_set_user_data(
    connp: *mut ConnectionParser,
    user_data: *mut libc::c_void,
) {
    if let Some(connp) = connp.as_mut() {
        connp.user_data = user_data;
    }
}

/// Returns the LibHTP version string.
#[no_mangle]
pub unsafe extern "C" fn htp_get_version() -> *const libc::c_char {
    util::get_version()
}

/// Get a log message's log message
///
/// Returns the log message as a cstring or NULL on error
/// The caller must free this result with htp_log_free
#[no_mangle]
pub unsafe extern "C" fn htp_conn_message_log(
    conn: *const Connection,
    msg_id: usize,
) -> *mut std::os::raw::c_char {
    conn.as_ref()
        .and_then(|conn| conn.message(msg_id))
        .and_then(|msg| CString::new(msg.msg.clone()).ok())
        .map(|msg| msg.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Get a log message's file
///
/// Returns the file as a cstring or NULL on error
/// The caller must free this result with htp_log_free
#[no_mangle]
pub unsafe extern "C" fn htp_conn_message_file(
    conn: *const Connection,
    msg_id: usize,
) -> *mut std::os::raw::c_char {
    conn.as_ref()
        .and_then(|conn| conn.message(msg_id))
        .and_then(|msg| CString::new(msg.file.clone()).ok())
        .map(|msg| msg.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Get a log message's code
///
/// Returns a code or HTP_LOG_CODE_ERROR on error
#[no_mangle]
pub unsafe extern "C" fn htp_conn_message_code(
    conn: *const Connection,
    msg_id: usize,
) -> log::HtpLogCode {
    conn.as_ref()
        .and_then(|conn| conn.message(msg_id))
        .map(|msg| msg.code)
        .unwrap_or(HtpLogCode::ERROR)
}

/// Get the number of messages in a connection.
///
/// Returns the number of messages or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_conn_message_size(conn: *const Connection) -> isize {
    if let Some(conn) = conn.as_ref() {
        isize::try_from(conn.message_size()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Get the number of transactions in a connection
///
/// Returns the number of transactions or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_conn_tx_size(conn: *const Connection) -> isize {
    if let Some(conn) = conn.as_ref() {
        isize::try_from(conn.tx_size()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Get a transaction in a connection.
///
/// Returns the transaction or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_conn_tx(conn: *mut Connection, tx_id: usize) -> *mut Transaction {
    if let Some(conn) = conn.as_mut() {
        conn.tx_mut_ptr(tx_id)
    } else {
        std::ptr::null_mut()
    }
}

/// Returns the in_data_counter
#[no_mangle]
pub unsafe extern "C" fn htp_conn_in_data_counter(conn: *const Connection) -> i64 {
    nullcheck!(conn);
    (*conn).in_data_counter
}

/// Returns the out_data_counter
#[no_mangle]
pub unsafe extern "C" fn htp_conn_out_data_counter(conn: *const Connection) -> i64 {
    nullcheck!(conn);
    (*conn).out_data_counter
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
/// tx: Header pointer.
///
/// Returns the name or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_header_name(header: *const Header) -> *const bstr::Bstr {
    if let Some(header) = header.as_ref() {
        &header.name
    } else {
        std::ptr::null()
    }
}

/// Get the name of a header as a ptr.
///
/// tx: Header pointer.
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
pub unsafe extern "C" fn htp_header_value(header: *const Header) -> *const bstr::Bstr {
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

/// Performs in-place decoding of the input string, according to the configuration specified
/// by cfg and ctx. On output, various flags (HTP_URLEN_*) might be set.
///
/// Returns HTP_STATUS_OK on success, HTP_STATUS_ERROR on failure.
#[no_mangle]
pub unsafe extern "C" fn htp_urldecode_inplace(
    cfg: *mut config::Config,
    input: *mut bstr::Bstr,
    flags: *mut u64,
) -> HtpStatus {
    if input.is_null() || flags.is_null() || cfg.is_null() {
        return HtpStatus::ERROR;
    }
    let mut f = util::Flags::from_bits_truncate(*flags);
    let res = util::urldecode_inplace(&(*cfg).decoder_cfg, &mut *input, &mut f);
    *flags = f.bits();
    res.into()
}

/// Configures whether transactions will be automatically destroyed once they
/// are processed and all callbacks invoked. This option is appropriate for
/// programs that process transactions as they are processed.
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_tx_auto_destroy(
    cfg: *mut config::Config,
    tx_auto_destroy: libc::c_int,
) {
    if !cfg.is_null() {
        (*cfg).set_tx_auto_destroy(tx_auto_destroy == 1)
    }
}

/// Registers a callback that is invoked every time there is a log message with
/// severity equal and higher than the configured log level.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_log(
    cfg: *mut config::Config,
    cbk_fn: LogExternalCallbackFn,
) {
    if let Some(cfg) = cfg.as_mut() {
        cfg.hook_log.register_extern(cbk_fn)
    }
}

/// Adds the built-in Multipart parser to the configuration. This parser will extract information
/// stored in request bodies, when they are in multipart/form-data format.
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_multipart_parser(cfg: *mut config::Config) {
    if !cfg.is_null() {
        (*cfg).register_multipart_parser()
    }
}

/// Retrieves the pointer to the active outbound transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Can be NULL.
///
/// Returns active outbound transaction, or NULL if there isn't one.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_out_tx(connp: *mut ConnectionParser) -> *mut Transaction {
    connp
        .as_mut()
        .map(|connp| connp.out_tx_mut_ptr())
        .unwrap_or(std::ptr::null_mut())
}

/// Retrieves the pointer to the active inbound transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Can be NULL.
///
/// Returns active inbound transaction, or NULL if there isn't one.
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_in_tx(connp: *mut ConnectionParser) -> *mut Transaction {
    connp
        .as_mut()
        .map(|connp| connp.in_tx_mut_ptr())
        .unwrap_or(std::ptr::null_mut())
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

/// Append as many bytes from the source to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
#[no_mangle]
pub unsafe extern "C" fn bstr_add_c_noex(
    destination: *mut bstr::Bstr,
    source: *const libc::c_char,
) -> *mut bstr::Bstr {
    bstr::bstr_add_c_noex(destination, source)
}

/// Append as many bytes from the source bstring to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
#[no_mangle]
pub unsafe extern "C" fn bstr_add_noex(
    destination: *mut bstr::Bstr,
    source: *const bstr::Bstr,
) -> *mut bstr::Bstr {
    bstr::bstr_add_noex(destination, source)
}

/// Allocate a zero-length bstring, reserving space for at least size bytes.
///
/// Returns New string instance
#[no_mangle]
pub unsafe extern "C" fn bstr_alloc(len: libc::size_t) -> *mut bstr::Bstr {
    bstr::bstr_alloc(len)
}

/// Create a new bstring by copying the provided NUL-terminated string.
///
/// Returns New bstring, or NULL if memory allocation failed.
#[no_mangle]
pub unsafe extern "C" fn bstr_dup_c(cstr: *const libc::c_char) -> *mut bstr::Bstr {
    bstr::bstr_dup_c(cstr)
}

/// Create a new bstring by copying a part of the provided bstring.
/// returns New bstring, or NULL if memory allocation failed.
#[no_mangle]
pub unsafe extern "C" fn bstr_dup_ex(
    b: *const bstr::Bstr,
    offset: libc::size_t,
    len: libc::size_t,
) -> *mut bstr::Bstr {
    bstr::bstr_dup_ex(b, offset, len)
}

/// Case-sensitive comparison of a bstring and a NUL-terminated string.
/// returns Zero on string match, 1 if b is greater than cstr, and -1 if cstr is
///         greater than b.
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_c(b: *const bstr::Bstr, c: *const libc::c_char) -> libc::c_int {
    bstr::bstr_cmp_c(b, c)
}

/// Create a new bstring by copying the provided bstring.
/// returns New bstring, or NULL if memory allocation failed.
#[no_mangle]
pub unsafe extern "C" fn bstr_dup(b: *const bstr::Bstr) -> *mut bstr::Bstr {
    bstr::bstr_dup(b)
}

/// Deallocate the supplied bstring instance and set it to NULL. Allows NULL on
/// input.
#[no_mangle]
pub unsafe extern "C" fn bstr_free(b: *mut bstr::Bstr) {
    bstr::bstr_free(b)
}

/// This function was a macro in libhtp
/// #define bstr_len(X) ((*(X)).len)
#[no_mangle]
pub unsafe extern "C" fn bstr_len(x: *const bstr::Bstr) -> libc::size_t {
    bstr::bstr_len(x)
}

/// This function was a macro in libhtp
/// #define bstr_ptr(X) ( ((*(X)).realptr == NULL) ? ((unsigned char *)(X) + sizeof(bstr)) : (unsigned char *)(*(X)).realptr )
#[no_mangle]
pub unsafe extern "C" fn bstr_ptr(x: *const bstr::Bstr) -> *mut libc::c_uchar {
    bstr::bstr_ptr(x)
}

/// This function was a macro in libhtp
/// #define bstr_size(X) ((*(X)).size)
#[no_mangle]
pub unsafe extern "C" fn bstr_size(x: *const bstr::Bstr) -> libc::size_t {
    bstr::bstr_size(x)
}

/// Convert contents of a memory region to a positive integer.
/// base: The desired number base.
/// lastlen: Points to the first unused byte in the region
/// returns If the conversion was successful, this function returns the
/// number. When the conversion fails, -1 will be returned when not
/// one valid digit was found, and -2 will be returned if an overflow
/// occurred.
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_to_pint(
    data: *const libc::c_void,
    len: libc::size_t,
    base: libc::c_int,
    lastlen: *mut libc::size_t,
) -> libc::c_long {
    bstr::bstr_util_mem_to_pint(data, len, base, lastlen)
}

/// Create a new NUL-terminated string out of the provided bstring. If NUL bytes
/// are contained in the bstring, each will be replaced with "\0" (two characters).
/// The caller is responsible to keep track of the allocated memory area and free
/// it once it is no longer needed.
/// returns The newly created NUL-terminated string, or NULL in case of memory
///         allocation failure.
#[no_mangle]
pub unsafe extern "C" fn bstr_util_strdup_to_c(b: *const bstr::Bstr) -> *mut libc::c_char {
    if b.is_null() {
        return std::ptr::null_mut();
    }
    let src = std::slice::from_raw_parts(bstr_ptr(b), bstr_len(b));

    // Since the memory returned here is just a char* and the caller will
    // free() it we have to use malloc() here.
    // So we allocate enough space for doubled NULL bytes plus the trailing NULL.
    let mut null_count = 1;
    for byte in src {
        if *byte == 0 {
            null_count += 1;
        }
    }
    let newlen = bstr_len(b) + null_count;
    let mem = libc::malloc(newlen) as *mut i8;
    let dst: &mut [i8] = std::slice::from_raw_parts_mut(mem, newlen);
    let mut dst_idx = 0;
    for byte in src {
        if *byte == 0 {
            dst[dst_idx] = '\\' as i8;
            dst_idx += 1;
            dst[dst_idx] = '0' as i8;
            dst_idx += 1;
        } else {
            dst[dst_idx] = *byte as i8;
            dst_idx += 1;
        }
    }
    dst[dst_idx] = 0;

    mem
}

// Get the log message
// returns a pointer to a null-terminated string
// The caller is responsible for freeing the memory with htp_log_free
#[no_mangle]
pub unsafe extern "C" fn htp_log_get(
    messages: *mut core::ffi::c_void,
    idx: libc::size_t,
) -> *mut libc::c_char {
    let messages = messages as *mut list::List<*mut core::ffi::c_void>;
    if let Some(log) = (*messages).get(idx) {
        let log = *log as *mut Log;
        if let Ok(msg_cstr) = CString::new((*log).msg.clone()) {
            return msg_cstr.into_raw();
        }
    }
    std::ptr::null_mut()
}

// Free the message
#[no_mangle]
pub unsafe extern "C" fn htp_log_free(msg: *mut libc::c_char) -> () {
    if !msg.is_null() {
        CString::from_raw(msg);
    }
}

// Get the message code
#[no_mangle]
pub unsafe extern "C" fn htp_log_get_code(
    messages: *mut core::ffi::c_void,
    idx: libc::size_t,
) -> HtpLogCode {
    let messages = messages as *mut list::List<*mut core::ffi::c_void>;
    if let Some(log) = (*messages).get(idx) {
        let log = *log as *mut Log;
        if !log.is_null() {
            return (*log).code;
        }
    }
    HtpLogCode::UNKNOWN
}

// Get the log filename
// returns a pointer to a null-terminated string
// The called is responsible for freeing the memory with htp_log_free
#[no_mangle]
pub unsafe extern "C" fn htp_log_get_file(
    messages: *mut core::ffi::c_void,
    idx: libc::size_t,
) -> *mut libc::c_char {
    let messages = messages as *mut list::List<*mut core::ffi::c_void>;
    if let Some(log) = (*messages).get(idx) {
        let log = *log as *mut Log;
        if let Ok(file_cstr) = CString::new((*log).file.clone()) {
            file_cstr.into_raw()
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

#[test]
fn bstr_tUtilDupToC() {
    unsafe {
        let c: *mut i8;
        let str: *mut bstr::Bstr = bstr::bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );

        c = bstr_util_strdup_to_c(str);
        assert_eq!(
            0,
            libc::strcmp(CString::new("ABCDEFGHIJKL\\0NOPQRST").unwrap().as_ptr(), c)
        );

        libc::free(c as *mut core::ffi::c_void);
        bstr::bstr_free(str);
    }
}
