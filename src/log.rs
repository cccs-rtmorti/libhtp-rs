use crate::{htp_connection_parser, htp_hooks, htp_list};
use std::ffi::CStr;

/// cbindgen:prefix-with-name=true
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_log_code {
    UNKNOWN,
    GZIP_DECOMPRESSION_FAILED,
    REQUEST_FIELD_MISSING_COLON,
    RESPONSE_FIELD_MISSING_COLON,
    INVALID_REQUEST_CHUNK_LEN,
    INVALID_RESPONSE_CHUNK_LEN,
    INVALID_TRANSFER_ENCODING_VALUE_IN_REQUEST,
    INVALID_TRANSFER_ENCODING_VALUE_IN_RESPONSE,
    INVALID_CONTENT_LENGTH_FIELD_IN_REQUEST,
    INVALID_CONTENT_LENGTH_FIELD_IN_RESPONSE,
    DUPLICATE_CONTENT_LENGTH_FIELD_IN_REQUEST,
    DUPLICATE_CONTENT_LENGTH_FIELD_IN_RESPONSE,
    CONTINUE_ALREADY_SEEN,
    UNABLE_TO_MATCH_RESPONSE_TO_REQUEST,
    INVALID_SERVER_PORT_IN_REQUEST,
    INVALID_AUTHORITY_PORT,
    REQUEST_HEADER_INVALID,
    RESPONSE_HEADER_INVALID,
    MISSING_HOST_HEADER,
    HOST_HEADER_AMBIGUOUS,
    INVALID_REQUEST_FIELD_FOLDING,
    INVALID_RESPONSE_FIELD_FOLDING,
    REQUEST_FIELD_TOO_LONG,
    RESPONSE_FIELD_TOO_LONG,
    REQUEST_SERVER_PORT_TCP_PORT_MISMATCH,
    URI_HOST_INVALID,
    HEADER_HOST_INVALID,
    METHOD_DELIM_NON_COMPLIANT,
    URI_DELIM_NON_COMPLIANT,
    REQUEST_LINE_LEADING_WHITESPACE,
    TOO_MANY_ENCODING_LAYERS,
    ABNORMAL_CE_HEADER,
    AUTH_UNRECOGNIZED,
    REQUEST_HEADER_REPETITION,
    RESPONSE_HEADER_REPETITION,
    RESPONSE_MULTIPART_BYTERANGES,
    RESPONSE_ABNORMAL_TRANSFER_ENCODING,
    RESPONSE_CHUNKED_OLD_PROTO,
    RESPONSE_INVALID_PROTOCOL,
    RESPONSE_INVALID_STATUS,
    REQUEST_LINE_INCOMPLETE,
    DOUBLE_ENCODED_URI,
    REQUEST_LINE_INVALID,
    REQUEST_BODY_UNEXPECTED,
    LZMA_MEMLIMIT_REACHED,
    COMPRESSION_BOMB,
    RESPONSE_BODY_UNEXPECTED,
    CONTENT_LENGTH_EXTRA_DATA_START,
    CONTENT_LENGTH_EXTRA_DATA_END,
    SWITCHING_PROTO_WITH_CONTENT_LENGTH,
    DEFORMED_EOL,
    PARSER_STATE_ERROR,
    MISSING_OUTBOUND_TRANSACTION_DATA,
    MISSING_INBOUND_TRANSACTION_DATA,
    ZERO_LENGTH_DATA_CHUNKS,
    REQUEST_LINE_UNKNOWN_METHOD,
    REQUEST_LINE_UNKNOWN_METHOD_NO_PROTOCOL,
    REQUEST_LINE_UNKNOWN_METHOD_INVALID_PROTOCOL,
    REQUEST_LINE_NO_PROTOCOL,
    RESPONSE_LINE_INVALID_PROTOCOL,
    RESPONSE_LINE_INVALID_RESPONSE_STATUS,
    RESPONSE_BODY_INTERNAL_ERROR,
    REQUEST_BODY_DATA_CALLBACK_ERROR,
    RESPONSE_INVALID_EMPTY_NAME,
    REQUEST_INVALID_EMPTY_NAME,
    RESPONSE_INVALID_LWS_AFTER_NAME,
    RESPONSE_HEADER_NAME_NOT_TOKEN,
    REQUEST_INVALID_LWS_AFTER_NAME,
    LZMA_DECOMPRESSION_DISABLED,
    CONNECTION_ALREADY_OPEN,
}

/// Enumerates all log levels.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum htp_log_level_t {
    HTP_LOG_NONE,
    HTP_LOG_ERROR,
    HTP_LOG_WARNING,
    HTP_LOG_NOTICE,
    HTP_LOG_INFO,
    HTP_LOG_DEBUG,
    HTP_LOG_DEBUG2,
}

/// Represents a single log entry.
#[derive(Clone)]
pub struct htp_log_t {
    /// Client IP address.
    pub client_addr: String,
    /// Client port.
    pub client_port: i32,
    /// Server IP address.
    pub server_addr: String,
    /// Server port.
    pub server_port: i32,

    /// Log message.
    pub msg: String,
    /// Message level.
    pub level: htp_log_level_t,
    /// Message code.
    pub code: htp_log_code,
    /// File in which the code that emitted the message resides.
    pub file: String,
    /// Line number on which the code that emitted the message resides.
    pub line: u32,
}

impl htp_log_t {
    pub unsafe fn new(
        connp: *mut htp_connection_parser::htp_connp_t,
        file: &str,
        line: u32,
        level: htp_log_level_t,
        code: htp_log_code,
        msg: String,
    ) -> htp_log_t {
        let mut client_addr = String::new();
        let mut server_addr = String::new();

        if !(*(*connp).conn).client_addr.is_null() {
            client_addr = CStr::from_ptr((*(*connp).conn).client_addr)
                .to_string_lossy()
                .into_owned();
        }
        if !(*(*connp).conn).server_addr.is_null() {
            server_addr = CStr::from_ptr((*(*connp).conn).server_addr)
                .to_string_lossy()
                .into_owned();
        }

        Self {
            client_addr,
            client_port: (*(*connp).conn).client_port,
            server_addr,
            server_port: (*(*connp).conn).server_port,
            file: file.to_string(),
            line,
            level,
            code,
            msg,
        }
    }

    pub unsafe fn add_log(connp: *mut htp_connection_parser::htp_connp_t, log: Box<htp_log_t>) {
        if !connp.is_null() && !(*connp).conn.is_null() {
            htp_list::htp_list_array_push(
                (*(*connp).conn).messages,
                Box::into_raw(log) as *mut ::libc::c_void,
            );
            htp_hooks::htp_hook_run_all(
                (*(*connp).cfg).hook_log,
                htp_list::htp_list_array_get_last((*(*connp).conn).messages) as *mut ::libc::c_void,
            );
        }
    }
}

#[macro_export]
macro_rules! htp_log {
    ($connp:ident, $level:expr, $code:expr, $msg:expr) => {
        if !$connp.is_null() {
            use $crate::log::*;
            // Ignore messages below our log level.
            if !(*$connp).cfg.is_null() && $level <= (*(*$connp).cfg).log_level {
                let log = Box::new(htp_log_t::new(
                    $connp,
                    file!(),
                    line!(),
                    $level,
                    $code,
                    $msg.to_string(),
                ));
                htp_log_t::add_log($connp, log);
            }
        }
    };
}

pub unsafe fn htp_logs_free(messages: *mut htp_list::htp_list_array_t) {
    let size = htp_list::htp_list_array_size(messages);
    for i in 0..size {
        let log: *mut htp_log_t = htp_list::htp_list_array_get(messages, i) as *mut htp_log_t;
        if !log.is_null() {
            Box::from_raw(log);
        }
    }
    return htp_list::htp_list_array_destroy(messages);
}
