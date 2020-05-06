use crate::{
    bstr, htp_config, htp_connection_parser, htp_hooks, htp_list, htp_request, htp_response,
    htp_transaction, htp_utf8_decoder, Status,
};
use ::libc;
use bitflags;

pub const HTP_VERSION_STRING: &'static str = concat!(env!("CARGO_PKG_VERSION"), "\x00");
pub const HTP_VERSION_STRING_FULL: &'static str =
    concat!("LibHTP v", env!("CARGO_PKG_VERSION"), "\x00");

// Various flag bits. Even though we have a flag field in several places
// (header, transaction, connection), these fields are all in the same namespace
// because we may want to set the same flag in several locations. For example, we
// may set HTP_FIELD_FOLDED on the actual folded header, but also on the transaction
// that contains the header. Both uses are useful.

// Connection flags are 8 bits wide.
bitflags::bitflags! {
    #[repr(C)]
    pub struct ConnectionFlags: u8 {
        const HTP_CONN_PIPELINED      = 0x01;
        const HTP_CONN_HTTP_0_9_EXTRA = 0x02;
    }
}

// All other flags are 64 bits wide.
bitflags::bitflags! {
    #[repr(C)]
    pub struct Flags: u64 {
        const HTP_FIELD_UNPARSEABLE      = 0x000000004;
        const HTP_FIELD_INVALID          = 0x000000008;
        const HTP_FIELD_FOLDED           = 0x000000010;
        const HTP_FIELD_REPEATED         = 0x000000020;
        const HTP_FIELD_LONG             = 0x000000040;
        const HTP_FIELD_RAW_NUL          = 0x000000080;
        const HTP_REQUEST_SMUGGLING      = 0x000000100;
        const HTP_INVALID_FOLDING        = 0x000000200;
        const HTP_REQUEST_INVALID_T_E    = 0x000000400;
        const HTP_MULTI_PACKET_HEAD      = 0x000000800;
        const HTP_HOST_MISSING           = 0x000001000;
        const HTP_HOST_AMBIGUOUS         = 0x000002000;
        const HTP_PATH_ENCODED_NUL       = 0x000004000;
        const HTP_PATH_RAW_NUL           = 0x000008000;
        const HTP_PATH_INVALID_ENCODING  = 0x000010000;
        const HTP_PATH_INVALID           = 0x000020000;
        const HTP_PATH_OVERLONG_U        = 0x000040000;
        const HTP_PATH_ENCODED_SEPARATOR = 0x000080000;
        /// At least one valid UTF-8 character and no invalid ones.
        const HTP_PATH_UTF8_VALID        = 0x000100000;
        const HTP_PATH_UTF8_INVALID      = 0x000200000;
        const HTP_PATH_UTF8_OVERLONG     = 0x000400000;
        /// Range U+FF00 - U+FFEF detected.
        const HTP_PATH_HALF_FULL_RANGE   = 0x000800000;
        const HTP_STATUS_LINE_INVALID    = 0x001000000;
        /// Host in the URI.
        const HTP_HOSTU_INVALID          = 0x002000000;
        /// Host in the Host header.
        const HTP_HOSTH_INVALID          = 0x004000000;
        const HTP_HOST_INVALID           = ( Self::HTP_HOSTU_INVALID.bits | Self::HTP_HOSTH_INVALID.bits );
        const HTP_URLEN_ENCODED_NUL      = 0x008000000;
        const HTP_URLEN_INVALID_ENCODING = 0x010000000;
        const HTP_URLEN_OVERLONG_U       = 0x020000000;
        /// Range U+FF00 - U+FFEF detected.
        const HTP_URLEN_HALF_FULL_RANGE  = 0x040000000;
        const HTP_URLEN_RAW_NUL          = 0x080000000;
        const HTP_REQUEST_INVALID        = 0x100000000;
        const HTP_REQUEST_INVALID_C_L    = 0x200000000;
        const HTP_AUTH_INVALID           = 0x400000000;
    }
}

extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlcat(dst: *mut libc::c_char, src: *const libc::c_char, size: size_t) -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum htp_file_source_t {
    HTP_FILE_MULTIPART = 1,
    HTP_FILE_PUT = 2,
}

/// Used to represent files that are seen during the processing of HTTP traffic. Most
/// commonly this refers to files seen in multipart/form-data payloads. In addition, PUT
/// request bodies can be treated as files.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_file_t {
    /// Where did this file come from? Possible values: HTP_FILE_MULTIPART and HTP_FILE_PUT.
    pub source: htp_file_source_t,
    /// File name, as provided (e.g., in the Content-Disposition multipart part header.
    pub filename: *mut bstr::bstr_t,
    /// File length.
    pub len: int64_t,
    /// The unique filename in which this file is stored on the filesystem, when applicable.
    pub tmpname: *mut libc::c_char,
    /// The file descriptor used for external storage, or -1 if unused.
    pub fd: libc::c_int,
}

/// URI structure. Each of the fields provides access to a single
/// URI element. Where an element is not present in a URI, the
/// corresponding field will be set to NULL or -1, depending on the
/// field type.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct htp_uri_t {
    /// Scheme, e.g., "http".
    pub scheme: *mut bstr::bstr_t,
    /// Username.
    pub username: *mut bstr::bstr_t,
    /// Password.
    pub password: *mut bstr::bstr_t,
    /// Hostname.
    pub hostname: *mut bstr::bstr_t,
    /// Port, as string.
    pub port: *mut bstr::bstr_t,
    /// Port, as number. This field will contain HTP_PORT_NONE if there was
    /// no port information in the URI and HTP_PORT_INVALID if the port information
    /// was invalid (e.g., it's not a number or it falls out of range.
    pub port_number: libc::c_int,
    /// The path part of this URI.
    pub path: *mut bstr::bstr_t,
    /// Query string.
    pub query: *mut bstr::bstr_t,
    /// Fragment identifier. This field will rarely be available in a server-side
    /// setting, but it's not impossible to see it.
    pub fragment: *mut bstr::bstr_t,
}

pub type htp_time_t = libc::timeval;

/// Enumerates all log levels.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_log_t {
    /// The connection parser associated with this log message.
    pub connp: *mut htp_connection_parser::htp_connp_t,
    /// The transaction associated with this log message, if any.
    pub tx: *mut htp_transaction::htp_tx_t,
    /// Log message.
    pub msg: *const libc::c_char,
    /// Message level.
    pub level: htp_log_level_t,
    /// Message code.
    pub code: libc::c_int,
    /// File in which the code that emitted the message resides.
    pub file: *const libc::c_char,
    /// Line number on which the code that emitted the message resides.
    pub line: libc::c_uint,
}

/// Represents a chunk of file data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_file_data_t {
    /// File information.
    pub file: *mut htp_file_t,
    /// Pointer to the data buffer.
    pub data: *const libc::c_uchar,
    /// Buffer length.
    pub len: size_t,
}

/// Is character a linear white space character?
///
/// Returns 0 or 1
pub unsafe fn htp_is_lws(mut c: libc::c_int) -> libc::c_int {
    if c == ' ' as i32 || c == '\t' as i32 {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}

/// Is character a separator character?
///
/// Returns 0 or 1
pub unsafe fn htp_is_separator(mut c: libc::c_int) -> libc::c_int {
    // separators = "(" | ")" | "<" | ">" | "@"
    // | "," | ";" | ":" | "\" | <">
    // | "/" | "[" | "]" | "?" | "="
    // | "{" | "}" | SP | HT

    match c {
        40 | 41 | 60 | 62 | 64 | 44 | 59 | 58 | 92 | 34 | 47 | 91 | 93 | 63 | 61 | 123 | 125
        | 32 | 9 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}

/// Is character a text character?
///
/// Returns 0 or 1
pub unsafe fn htp_is_text(mut c: libc::c_int) -> libc::c_int {
    if c == '\t' as i32 {
        return 1 as libc::c_int;
    }
    if c < 32 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

/// Is character a token character?
///
/// Returns 0 or 1
pub unsafe fn htp_is_token(mut c: libc::c_int) -> libc::c_int {
    // token = 1*<any CHAR except CTLs or separators>
    // CHAR  = <any US-ASCII character (octets 0 - 127)>
    if c < 32 as libc::c_int || c > 126 as libc::c_int {
        return 0 as libc::c_int;
    }
    if htp_is_separator(c) != 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

/// Remove all line terminators (LF, CR or CRLF) from
/// the end of the line provided as input.
///
/// Returns 0 if nothing was removed, 1 if one or more LF characters were removed, or
///         2 if one or more CR and/or LF characters were removed.
pub unsafe fn htp_chomp(mut data: *mut libc::c_uchar, mut len: *mut size_t) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    // Loop until there's no more stuff in the buffer
    while *len > 0 as libc::c_int as libc::c_ulong {
        // Try one LF first
        if *data.offset((*len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '\n' as i32
        {
            *len = (*len).wrapping_sub(1);
            r = 1 as libc::c_int;
            if *len == 0 as libc::c_int as libc::c_ulong {
                return r;
            }
            // A CR is allowed before LF
            if *data.offset((*len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '\r' as i32
            {
                *len = (*len).wrapping_sub(1);
                r = 2 as libc::c_int
            }
        } else if *data.offset((*len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '\r' as i32
        {
            *len = (*len).wrapping_sub(1);
            r = 1 as libc::c_int
        } else {
            return r;
        }
    }
    return r;
}

/// Is character a white space character?
///
/// Returns 0 or 1
pub unsafe fn htp_is_space(mut c: libc::c_int) -> libc::c_int {
    match c {
        32 | 12 | 11 | 9 | 13 | 10 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}

/// Converts request method, given as a string, into a number.
///
/// Returns Method number of M_UNKNOWN
pub unsafe fn htp_convert_method_to_number(mut method: *mut bstr::bstr_t) -> libc::c_int {
    if method.is_null() {
        return htp_request::htp_method_t::HTP_M_UNKNOWN as libc::c_int;
    }
    // TODO Optimize using parallel matching, or something similar.
    if bstr::bstr_cmp_c(method, b"GET\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_GET as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"PUT\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_PUT as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"POST\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_POST as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"DELETE\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_DELETE as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"CONNECT\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_CONNECT as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"OPTIONS\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_OPTIONS as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"TRACE\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_TRACE as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"PATCH\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_PATCH as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"PROPFIND\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_PROPFIND as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"PROPPATCH\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_PROPPATCH as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"MKCOL\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_MKCOL as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"COPY\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_COPY as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"MOVE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_MOVE as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"LOCK\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_LOCK as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"UNLOCK\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_UNLOCK as libc::c_int;
    }
    if bstr::bstr_cmp_c(
        method,
        b"VERSION-CONTROL\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_VERSION_CONTROL as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"CHECKOUT\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_CHECKOUT as libc::c_int;
    }
    if bstr::bstr_cmp_c(
        method,
        b"UNCHECKOUT\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_UNCHECKOUT as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"CHECKIN\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_CHECKIN as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"UPDATE\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_UPDATE as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"LABEL\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_LABEL as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"REPORT\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_REPORT as libc::c_int;
    }
    if bstr::bstr_cmp_c(
        method,
        b"MKWORKSPACE\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_MKWORKSPACE as libc::c_int;
    }
    if bstr::bstr_cmp_c(
        method,
        b"MKACTIVITY\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_MKACTIVITY as libc::c_int;
    }
    if bstr::bstr_cmp_c(
        method,
        b"BASELINE-CONTROL\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_BASELINE_CONTROL as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"MERGE\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_MERGE as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"INVALID\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_INVALID as libc::c_int;
    }
    if bstr::bstr_cmp_c(method, b"HEAD\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return htp_request::htp_method_t::HTP_M_HEAD as libc::c_int;
    }
    return htp_request::htp_method_t::HTP_M_UNKNOWN as libc::c_int;
}

/// Is the given line empty?
///
/// Returns 0 or 1
pub unsafe fn htp_is_line_empty(mut data: *mut libc::c_uchar, mut len: size_t) -> libc::c_int {
    if len == 1 as libc::c_int as libc::c_ulong
        || len == 2 as libc::c_int as libc::c_ulong
            && *data.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
            && *data.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

/// Does line consist entirely of whitespace characters?
///
/// Returns 0 or 1
pub unsafe fn htp_is_line_whitespace(mut data: *mut libc::c_uchar, mut len: size_t) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        if *(*__ctype_b_loc()).offset(*data.offset(i as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}

/// Parses Content-Length string (positive decimal number).
/// White space is allowed before and after the number.
///
/// Returns Content-Length as a number, or -1 on error.
pub unsafe fn htp_parse_content_length(
    b: *const bstr::bstr_t,
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> int64_t {
    let mut len: size_t = (*b).len;
    let mut data: *mut libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut r: int64_t = 0 as libc::c_int as int64_t;
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(1003 as libc::c_int) as int64_t;
    }
    // Ignore junk before
    while pos < len
        && ((*data.offset(pos as isize) as libc::c_int) < '0' as i32
            || *data.offset(pos as isize) as libc::c_int > '9' as i32)
    {
        if htp_is_lws(*data.offset(pos as isize) as libc::c_int) == 0
            && !connp.is_null()
            && r == 0 as libc::c_int as libc::c_long
        {
            htp_log(
                connp,
                b"htp_util.c\x00" as *const u8 as *const libc::c_char,
                267 as libc::c_int,
                htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"C-L value with extra data in the beginnning\x00" as *const u8
                    as *const libc::c_char,
            );
            r = -(1 as libc::c_int) as int64_t
        }
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return -(1001 as libc::c_int) as int64_t;
    }
    r = bstr::bstr_util_mem_to_pint(
        data.offset(pos as isize) as *const libc::c_void,
        len.wrapping_sub(pos),
        10 as libc::c_int,
        &mut pos,
    );
    // Ok to have junk afterwards
    if pos < len && !connp.is_null() {
        htp_log(
            connp,
            b"htp_util.c\x00" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"C-L value with extra data in the end\x00" as *const u8 as *const libc::c_char,
        );
    }
    return r;
}

/// Parses chunk length (positive hexadecimal number). White space is allowed before
/// and after the number. An error will be returned if the chunk length is greater than
/// INT32_MAX.
///
/// Returns Chunk length, or a negative number on error.
pub unsafe fn htp_parse_chunked_length(mut data: *mut libc::c_uchar, mut len: size_t) -> int64_t {
    // skip leading line feeds and other control chars
    while len != 0 {
        let mut c: libc::c_uchar = *data;
        if !(c as libc::c_int == 0xd as libc::c_int
            || c as libc::c_int == 0xa as libc::c_int
            || c as libc::c_int == 0x20 as libc::c_int
            || c as libc::c_int == 0x9 as libc::c_int
            || c as libc::c_int == 0xb as libc::c_int
            || c as libc::c_int == 0xc as libc::c_int)
        {
            break;
        }
        data = data.offset(1);
        len = len.wrapping_sub(1)
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(1004 as libc::c_int) as int64_t;
    }
    // find how much of the data is correctly formatted
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        let mut c_0: libc::c_uchar = *data.offset(i as isize);
        if !(*(*__ctype_b_loc()).offset(c_0 as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || c_0 as libc::c_int >= 'a' as i32 && c_0 as libc::c_int <= 'f' as i32
            || c_0 as libc::c_int >= 'A' as i32 && c_0 as libc::c_int <= 'F' as i32)
        {
            break;
        }
        i = i.wrapping_add(1)
    }
    // cut off trailing junk
    if i != len {
        len = i
    }
    let mut chunk_len: int64_t =
        htp_parse_positive_integer_whitespace(data, len, 16 as libc::c_int);
    if chunk_len < 0 as libc::c_int as libc::c_long {
        return chunk_len;
    }
    if chunk_len > 2147483647 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as int64_t;
    }
    return chunk_len;
}

/// A somewhat forgiving parser for a positive integer in a given base.
/// Only LWS is allowed before and after the number.
///
/// Returns The parsed number on success; a negative number on error.
pub unsafe fn htp_parse_positive_integer_whitespace(
    data: *const libc::c_uchar,
    mut len: size_t,
    mut base: libc::c_int,
) -> int64_t {
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(1003 as libc::c_int) as int64_t;
    }
    let mut last_pos: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    // Ignore LWS before
    while pos < len && htp_is_lws(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return -(1001 as libc::c_int) as int64_t;
    }
    let mut r: int64_t = bstr::bstr_util_mem_to_pint(
        data.offset(pos as isize) as *const libc::c_void,
        len.wrapping_sub(pos),
        base,
        &mut last_pos,
    );
    if r < 0 as libc::c_int as libc::c_long {
        return r;
    }
    // Move after the last digit
    pos = (pos as libc::c_ulong).wrapping_add(last_pos) as size_t as size_t;
    // Ignore LWS after
    while pos < len {
        if htp_is_lws(*data.offset(pos as isize) as libc::c_int) == 0 {
            return -(1002 as libc::c_int) as int64_t;
        }
        pos = pos.wrapping_add(1)
    }
    return r;
}

/// Records one log message.
pub unsafe extern "C" fn htp_log(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut level: htp_log_level_t,
    mut code: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if connp.is_null() {
        return;
    }
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: ::std::ffi::VaListImpl;
    // Ignore messages below our log level.
    if ((*(*connp).cfg).log_level as libc::c_uint) < level as libc::c_uint {
        return;
    }
    args_0 = args.clone();
    let mut r: libc::c_int = vsnprintf(
        buf.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        fmt,
        args_0.as_va_list(),
    );
    if r < 0 as libc::c_int {
        snprintf(
            buf.as_mut_ptr(),
            1024 as libc::c_int as libc::c_ulong,
            b"[vnsprintf returned error %d]\x00" as *const u8 as *const libc::c_char,
            r,
        );
    } else if r >= 1024 as libc::c_int {
        // Indicate overflow with a '+' at the end.
        buf[1022 as libc::c_int as usize] = '+' as i32 as libc::c_char;
        buf[1023 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    // Create a new log entry.
    let mut log: *mut htp_log_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_log_t>() as libc::c_ulong,
    ) as *mut htp_log_t;
    if log.is_null() {
        return;
    }
    (*log).connp = connp;
    (*log).file = file;
    (*log).line = line as libc::c_uint;
    (*log).level = level;
    (*log).code = code;
    (*log).msg = strdup(buf.as_mut_ptr());
    htp_list::htp_list_array_push((*(*connp).conn).messages, log as *mut libc::c_void);
    if level == htp_log_level_t::HTP_LOG_ERROR {
        (*connp).last_error = log
    }
    // coverity[check_return]
    htp_hooks::htp_hook_run_all((*(*connp).cfg).hook_log, log as *mut libc::c_void);
}

/// Determines if the given line is a continuation (of some previous line).
///
/// Returns 0 or 1 for false and true, respectively. Returns -1 on error (NULL pointer or length zero).
pub unsafe fn htp_connp_is_line_folded(data: *const libc::c_uchar, mut len: size_t) -> libc::c_int {
    if data.is_null() || len == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    return htp_is_folding_char(*data.offset(0 as libc::c_int as isize) as libc::c_int);
}

pub unsafe fn htp_is_folding_char(mut c: libc::c_int) -> libc::c_int {
    if htp_is_lws(c) != 0 || c == 0 as libc::c_int {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}

/// Determines if the given line is a request terminator.
///
/// Returns 0 or 1
pub unsafe fn htp_connp_is_line_terminator(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> libc::c_int {
    // Is this the end of request headers?
    match (*(*connp).cfg).server_personality as libc::c_uint {
        5 => {
            // IIS 5 will accept a whitespace line as a terminator
            if htp_is_line_whitespace(data, len) != 0 {
                return 1 as libc::c_int;
            }
        }
        _ => {}
    }
    // Fall through
    // Treat an empty line as terminator
    if htp_is_line_empty(data, len) != 0 {
        return 1 as libc::c_int;
    }
    if len == 2 as libc::c_int as libc::c_ulong
        && htp_is_lws(*data.offset(0 as libc::c_int as isize) as libc::c_int) != 0
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
    {
        if (*connp).out_current_read_offset < (*connp).out_current_len
            && *(*connp)
                .out_current_data
                .offset((*connp).out_current_read_offset as isize) as libc::c_int
                != '\n' as i32
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}

/// Determines if the given line can be ignored when it appears before a request.
///
/// Returns 0 or 1
pub unsafe fn htp_connp_is_line_ignorable(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> libc::c_int {
    return htp_connp_is_line_terminator(connp, data, len);
}

unsafe fn htp_parse_port(
    mut data: *mut libc::c_uchar,
    mut len: size_t,
    mut port: *mut libc::c_int,
    mut invalid: *mut libc::c_int,
) -> Status {
    if len == 0 as libc::c_int as libc::c_ulong {
        *port = -(1 as libc::c_int);
        *invalid = 1 as libc::c_int;
        return Status::OK;
    }
    let mut port_parsed: int64_t =
        htp_parse_positive_integer_whitespace(data, len, 10 as libc::c_int);
    if port_parsed < 0 as libc::c_int as libc::c_long {
        // Failed to parse the port number.
        *port = -(1 as libc::c_int);
        *invalid = 1 as libc::c_int
    } else if port_parsed > 0 as libc::c_int as libc::c_long
        && port_parsed < 65536 as libc::c_int as libc::c_long
    {
        // Valid port number.
        *port = port_parsed as libc::c_int
    } else {
        // Port number out of range.
        *port = -(1 as libc::c_int);
        *invalid = 1 as libc::c_int
    }
    return Status::OK;
}

/// Parses an authority string, which consists of a hostname with an optional port number; username
/// and password are not allowed and will not be handled.
///
/// Returns in hostname: A bstring containing the hostname, or NULL if the hostname is invalid. If
///                      this value is not NULL, the caller assumes responsibility for memory
///                      management.
/// Returns in port: Port as text, or NULL if not provided.
/// Returns in port_number: Port number, or -1 if the port is not present or invalid.
/// Returns in invalid: Set to 1 if any part of the authority is invalid.
///
/// Returns HTP_OK on success, HTP_ERROR on memory allocation failure.
pub unsafe fn htp_parse_hostport(
    mut hostport: *mut bstr::bstr_t,
    mut hostname: *mut *mut bstr::bstr_t,
    mut port: *mut *mut bstr::bstr_t,
    mut port_number: *mut libc::c_int,
    mut invalid: *mut libc::c_int,
) -> Status {
    if hostport.is_null() || hostname.is_null() || port_number.is_null() || invalid.is_null() {
        return Status::ERROR;
    }
    *hostname = 0 as *mut bstr::bstr_t;
    if !port.is_null() {
        *port = 0 as *mut bstr::bstr_t
    }
    *port_number = -(1 as libc::c_int);
    *invalid = 0 as libc::c_int;
    let mut data: *mut libc::c_uchar = if (*hostport).realptr.is_null() {
        (hostport as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*hostport).realptr
    };
    let mut len: size_t = (*hostport).len;
    bstr::bstr_util_mem_trim(&mut data, &mut len);
    if len == 0 as libc::c_int as libc::c_ulong {
        *invalid = 1 as libc::c_int;
        return Status::OK;
    }
    // Check for an IPv6 address.
    if *data.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
        // IPv6 host.
        // Find the end of the IPv6 address.
        let mut pos: size_t = 0 as libc::c_int as size_t;
        while pos < len && *data.offset(pos as isize) as libc::c_int != ']' as i32 {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            *invalid = 1 as libc::c_int;
            return Status::OK;
        }
        *hostname = bstr::bstr_dup_mem(
            data as *const libc::c_void,
            pos.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if (*hostname).is_null() {
            return Status::ERROR;
        }
        // Over the ']'.
        pos = pos.wrapping_add(1);
        if pos == len {
            return Status::OK;
        }
        // Handle port.
        if *data.offset(pos as isize) as libc::c_int == ':' as i32 {
            if !port.is_null() {
                *port = bstr::bstr_dup_mem(
                    data.offset(pos as isize).offset(1 as libc::c_int as isize)
                        as *const libc::c_void,
                    len.wrapping_sub(pos)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if (*port).is_null() {
                    bstr::bstr_free(*hostname);
                    return Status::ERROR;
                }
            }
            return htp_parse_port(
                data.offset(pos as isize).offset(1 as libc::c_int as isize),
                len.wrapping_sub(pos)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                port_number,
                invalid,
            );
        } else {
            *invalid = 1 as libc::c_int;
            return Status::OK;
        }
    } else {
        // Not IPv6 host.
        // Is there a colon?
        let mut colon: *mut libc::c_uchar =
            memchr(data as *const libc::c_void, ':' as i32, len) as *mut libc::c_uchar;
        if colon.is_null() {
            // Hostname alone, no port.
            *hostname = bstr::bstr_dup_mem(data as *const libc::c_void, len);
            if (*hostname).is_null() {
                return Status::ERROR;
            }
            bstr::bstr_to_lowercase(*hostname);
        } else {
            // Hostname and port.
            // Ignore whitespace at the end of hostname.
            let mut hostend: *mut libc::c_uchar = colon;
            while hostend > data
                && *(*__ctype_b_loc())
                    .offset(*hostend.offset(-(1 as libc::c_int as isize)) as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
            {
                hostend = hostend.offset(-1)
            }
            *hostname = bstr::bstr_dup_mem(
                data as *const libc::c_void,
                hostend.wrapping_offset_from(data) as libc::c_long as size_t,
            );
            if (*hostname).is_null() {
                return Status::ERROR;
            }
            if !port.is_null() {
                *port = bstr::bstr_dup_mem(
                    colon.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    len.wrapping_sub(
                        colon
                            .offset(1 as libc::c_int as isize)
                            .wrapping_offset_from(data) as libc::c_long
                            as libc::c_ulong,
                    ),
                );
                if (*port).is_null() {
                    bstr::bstr_free(*hostname);
                    return Status::ERROR;
                }
            }
            return htp_parse_port(
                colon.offset(1 as libc::c_int as isize),
                len.wrapping_sub(
                    colon
                        .offset(1 as libc::c_int as isize)
                        .wrapping_offset_from(data) as libc::c_long
                        as libc::c_ulong,
                ),
                port_number,
                invalid,
            );
        }
    }
    return Status::OK;
}

/// Parses hostport provided in the URI.
///
/// Returns HTP_OK on success or HTP_ERROR error.
pub unsafe fn htp_parse_uri_hostport(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut hostport: *mut bstr::bstr_t,
    mut uri: *mut htp_uri_t,
) -> Status {
    let mut invalid: libc::c_int = 0;
    let mut rc: Status = htp_parse_hostport(
        hostport,
        &mut (*uri).hostname,
        &mut (*uri).port,
        &mut (*uri).port_number,
        &mut invalid,
    );
    if rc != Status::OK {
        return rc;
    }
    if invalid != 0 {
        (*(*connp).in_tx).flags |= Flags::HTP_HOSTU_INVALID
    }
    if !(*uri).hostname.is_null() {
        if htp_validate_hostname((*uri).hostname) == 0 as libc::c_int {
            (*(*connp).in_tx).flags |= Flags::HTP_HOSTU_INVALID
        }
    }
    return Status::OK;
}

/// Parses hostport provided in the Host header.
///
/// Returns HTP_OK on success or HTP_ERROR error.
pub unsafe fn htp_parse_header_hostport(
    mut hostport: *mut bstr::bstr_t,
    mut hostname: *mut *mut bstr::bstr_t,
    mut port: *mut *mut bstr::bstr_t,
    mut port_number: *mut libc::c_int,
    mut flags: *mut Flags,
) -> Status {
    let mut invalid: libc::c_int = 0;
    let mut rc: Status = htp_parse_hostport(hostport, hostname, port, port_number, &mut invalid);
    if rc != Status::OK {
        return rc;
    }
    if invalid != 0 {
        *flags |= Flags::HTP_HOSTH_INVALID
    }
    if !(*hostname).is_null() {
        if htp_validate_hostname(*hostname) == 0 as libc::c_int {
            *flags |= Flags::HTP_HOSTH_INVALID
        }
    }
    return Status::OK;
}

/// Parses request URI, making no attempt to validate the contents.
///
/// Returns HTP_ERROR on memory allocation failure, HTP_OK otherwise
pub unsafe fn htp_parse_uri(mut input: *mut bstr::bstr_t, mut uri: *mut *mut htp_uri_t) -> Status {
    // Allow a htp_uri_t structure to be provided on input,
    // but allocate a new one if the structure is NULL.
    if (*uri).is_null() {
        *uri = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<htp_uri_t>() as libc::c_ulong,
        ) as *mut htp_uri_t;
        if (*uri).is_null() {
            return Status::ERROR;
        }
    }
    if input.is_null() {
        // The input might be NULL on requests that don't actually
        // contain the URI. We allow that.
        return Status::OK;
    }
    let mut data: *mut libc::c_uchar = if (*input).realptr.is_null() {
        (input as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*input).realptr
    };
    let mut len: size_t = (*input).len;
    let mut start: size_t = 0;
    let mut pos: size_t = 0;
    if len == 0 as libc::c_int as libc::c_ulong {
        // Empty string.
        return Status::OK;
    }
    pos = 0 as libc::c_int as size_t;
    // Scheme test: if it doesn't start with a forward slash character (which it must
    // for the contents to be a path or an authority, then it must be the scheme part
    if *data.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        // Parse scheme
        // Find the colon, which marks the end of the scheme part
        start = pos;
        while pos < len && *data.offset(pos as isize) as libc::c_int != ':' as i32 {
            pos = pos.wrapping_add(1)
        }
        if pos >= len {
            // We haven't found a colon, which means that the URI
            // is invalid. Apache will ignore this problem and assume
            // the URI contains an invalid path so, for the time being,
            // we are going to do the same.
            pos = 0 as libc::c_int as size_t
        } else {
            // Make a copy of the scheme
            (**uri).scheme = bstr::bstr_dup_mem(
                data.offset(start as isize) as *const libc::c_void,
                pos.wrapping_sub(start),
            );
            if (**uri).scheme.is_null() {
                return Status::ERROR;
            }
            // Go over the colon
            pos = pos.wrapping_add(1)
        }
    }
    // Authority test: two forward slash characters and it's an authority.
    // One, three or more slash characters, and it's a path. We, however,
    // only attempt to parse authority if we've seen a scheme.
    if !(**uri).scheme.is_null() {
        if pos.wrapping_add(2 as libc::c_int as libc::c_ulong) < len
            && *data.offset(pos as isize) as libc::c_int == '/' as i32
            && *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '/' as i32
            && *data.offset(pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != '/' as i32
        {
            // Parse authority
            // Go over the two slash characters
            pos = pos.wrapping_add(2 as libc::c_int as libc::c_ulong);
            start = pos;
            // Authority ends with a question mark, forward slash or hash
            while pos < len
                && *data.offset(pos as isize) as libc::c_int != '?' as i32
                && *data.offset(pos as isize) as libc::c_int != '/' as i32
                && *data.offset(pos as isize) as libc::c_int != '#' as i32
            {
                pos = pos.wrapping_add(1)
            }
            let mut hostname_start: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut hostname_len: size_t = 0;
            // Are the credentials included in the authority?
            let mut m: *mut libc::c_uchar = memchr(
                data.offset(start as isize) as *const libc::c_void,
                '@' as i32,
                pos.wrapping_sub(start),
            ) as *mut libc::c_uchar;
            if !m.is_null() {
                // Credentials present
                let mut credentials_start: *mut libc::c_uchar = data.offset(start as isize);
                let mut credentials_len: size_t = (m.wrapping_offset_from(data) as libc::c_long
                    as libc::c_ulong)
                    .wrapping_sub(start);
                // Figure out just the hostname part
                hostname_start = data
                    .offset(start as isize)
                    .offset(credentials_len as isize)
                    .offset(1 as libc::c_int as isize);
                hostname_len = pos
                    .wrapping_sub(start)
                    .wrapping_sub(credentials_len)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                // Extract the username and the password
                m = memchr(
                    credentials_start as *const libc::c_void,
                    ':' as i32,
                    credentials_len,
                ) as *mut libc::c_uchar;
                if !m.is_null() {
                    // Username and password
                    (**uri).username = bstr::bstr_dup_mem(
                        credentials_start as *const libc::c_void,
                        m.wrapping_offset_from(credentials_start) as libc::c_long as size_t,
                    );
                    if (**uri).username.is_null() {
                        return Status::ERROR;
                    }
                    (**uri).password = bstr::bstr_dup_mem(
                        m.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        credentials_len
                            .wrapping_sub(m.wrapping_offset_from(credentials_start) as libc::c_long
                                as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    if (**uri).password.is_null() {
                        return Status::ERROR;
                    }
                } else {
                    // Username alone
                    (**uri).username = bstr::bstr_dup_mem(
                        credentials_start as *const libc::c_void,
                        credentials_len,
                    );
                    if (**uri).username.is_null() {
                        return Status::ERROR;
                    }
                }
            } else {
                // No credentials
                hostname_start = data.offset(start as isize);
                hostname_len = pos.wrapping_sub(start)
            }
            // Parsing authority without credentials.
            if hostname_len > 0 as libc::c_int as libc::c_ulong
                && *hostname_start.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32
            {
                // IPv6 address.
                m = memchr(
                    hostname_start as *const libc::c_void,
                    ']' as i32,
                    hostname_len,
                ) as *mut libc::c_uchar;
                if m.is_null() {
                    // Invalid IPv6 address; use the entire string as hostname.
                    (**uri).hostname =
                        bstr::bstr_dup_mem(hostname_start as *const libc::c_void, hostname_len);
                    if (**uri).hostname.is_null() {
                        return Status::ERROR;
                    }
                } else {
                    (**uri).hostname = bstr::bstr_dup_mem(
                        hostname_start as *const libc::c_void,
                        (m.wrapping_offset_from(hostname_start) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t,
                    );
                    if (**uri).hostname.is_null() {
                        return Status::ERROR;
                    }
                    // Is there a port?
                    hostname_len = hostname_len.wrapping_sub(
                        (m.wrapping_offset_from(hostname_start) as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong,
                    );
                    hostname_start = m.offset(1 as libc::c_int as isize);
                    // Port string
                    m = memchr(
                        hostname_start as *const libc::c_void,
                        ':' as i32,
                        hostname_len,
                    ) as *mut libc::c_uchar;
                    if !m.is_null() {
                        let mut port_len: size_t = hostname_len
                            .wrapping_sub(m.wrapping_offset_from(hostname_start) as libc::c_long
                                as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        (**uri).port = bstr::bstr_dup_mem(
                            m.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            port_len,
                        );
                        if (**uri).port.is_null() {
                            return Status::ERROR;
                        }
                    }
                }
            } else {
                // Not IPv6 address.
                m = memchr(
                    hostname_start as *const libc::c_void,
                    ':' as i32,
                    hostname_len,
                ) as *mut libc::c_uchar;
                if !m.is_null() {
                    let mut port_len_0: size_t = hostname_len
                        .wrapping_sub(
                            m.wrapping_offset_from(hostname_start) as libc::c_long as libc::c_ulong
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    hostname_len = hostname_len
                        .wrapping_sub(port_len_0)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    // Port string
                    (**uri).port = bstr::bstr_dup_mem(
                        m.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        port_len_0,
                    );
                    if (**uri).port.is_null() {
                        return Status::ERROR;
                    }
                }
                // Hostname
                (**uri).hostname =
                    bstr::bstr_dup_mem(hostname_start as *const libc::c_void, hostname_len);
                if (**uri).hostname.is_null() {
                    return Status::ERROR;
                }
            }
        }
    }
    // Path
    start = pos;
    // The path part will end with a question mark or a hash character, which
    // mark the beginning of the query part or the fragment part, respectively.
    while pos < len
        && *data.offset(pos as isize) as libc::c_int != '?' as i32
        && *data.offset(pos as isize) as libc::c_int != '#' as i32
    {
        pos = pos.wrapping_add(1)
    }
    // Path
    (**uri).path = bstr::bstr_dup_mem(
        data.offset(start as isize) as *const libc::c_void,
        pos.wrapping_sub(start),
    );
    if (**uri).path.is_null() {
        return Status::ERROR;
    }
    if pos == len {
        return Status::OK;
    }
    // Query
    if *data.offset(pos as isize) as libc::c_int == '?' as i32 {
        // Step over the question mark
        start = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
        // The query part will end with the end of the input
        // or the beginning of the fragment part
        while pos < len && *data.offset(pos as isize) as libc::c_int != '#' as i32 {
            pos = pos.wrapping_add(1)
        }
        // Query string
        (**uri).query = bstr::bstr_dup_mem(
            data.offset(start as isize) as *const libc::c_void,
            pos.wrapping_sub(start),
        );
        if (**uri).query.is_null() {
            return Status::ERROR;
        }
        if pos == len {
            return Status::OK;
        }
    }
    // Fragment
    if *data.offset(pos as isize) as libc::c_int == '#' as i32 {
        // Step over the hash character
        start = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
        // Fragment; ends with the end of the input
        (**uri).fragment = bstr::bstr_dup_mem(
            data.offset(start as isize) as *const libc::c_void,
            len.wrapping_sub(start),
        );
        if (**uri).fragment.is_null() {
            return Status::ERROR;
        }
    }
    return Status::OK;
}

/// Convert two input bytes, pointed to by the pointer parameter,
/// into a single byte by assuming the input consists of hexadecimal
/// characters. This function will happily convert invalid input.
///
/// Returns hex-decoded byte
unsafe fn x2c(mut what: *mut libc::c_uchar) -> libc::c_uchar {
    let mut digit: libc::c_uchar = 0;
    digit = if *what.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32 {
        ((*what.offset(0 as libc::c_int as isize) as libc::c_int & 0xdf as libc::c_int)
            - 'A' as i32)
            + 10 as libc::c_int
    } else {
        (*what.offset(0 as libc::c_int as isize) as libc::c_int) - '0' as i32
    } as libc::c_uchar;
    digit = (digit as libc::c_int * 16 as libc::c_int) as libc::c_uchar;
    digit = (digit as libc::c_int
        + if *what.offset(1 as libc::c_int as isize) as libc::c_int >= 'A' as i32 {
            ((*what.offset(1 as libc::c_int as isize) as libc::c_int & 0xdf as libc::c_int)
                - 'A' as i32)
                + 10 as libc::c_int
        } else {
            (*what.offset(1 as libc::c_int as isize) as libc::c_int) - '0' as i32
        }) as libc::c_uchar;
    return digit;
}

/// Convert a Unicode codepoint into a single-byte, using best-fit
/// mapping (as specified in the provided configuration structure).
///
/// Returns converted single byte
unsafe fn bestfit_codepoint(
    mut cfg: *mut htp_config::htp_cfg_t,
    mut ctx: htp_config::htp_decoder_ctx_t,
    mut codepoint: uint32_t,
) -> uint8_t {
    // Is it a single-byte codepoint?
    if codepoint < 0x100 as libc::c_int as libc::c_uint {
        return codepoint as uint8_t;
    }
    // Our current implementation converts only the 2-byte codepoints.
    if codepoint > 0xffff as libc::c_int as libc::c_uint {
        return (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte;
    }
    let mut p: *mut uint8_t = (*cfg).decoder_cfgs[ctx as usize].bestfit_map;
    loop
    // TODO Optimize lookup.
    {
        let mut x: uint32_t =
            (((*p.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
                + *p.offset(1 as libc::c_int as isize) as libc::c_int) as uint32_t;
        if x == 0 as libc::c_int as libc::c_uint {
            return (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte;
        }
        if x == codepoint {
            return *p.offset(2 as libc::c_int as isize);
        }
        // Move to the next triplet
        p = p.offset(3 as libc::c_int as isize)
    }
}

/// Decode a UTF-8 encoded path. Overlong characters will be decoded, invalid
/// characters will be left as-is. Best-fit mapping will be used to convert
/// UTF-8 into a single-byte stream.
pub unsafe fn htp_utf8_decode_path_inplace(
    mut cfg: *mut htp_config::htp_cfg_t,
    mut tx: *mut htp_transaction::htp_tx_t,
    mut path: *mut bstr::bstr_t,
) {
    if path.is_null() {
        return;
    }
    let mut data: *mut uint8_t = if (*path).realptr.is_null() {
        (path as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*path).realptr
    };
    if data.is_null() {
        return;
    }
    let mut len: size_t = (*path).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut codepoint: uint32_t = 0 as libc::c_int as uint32_t;
    let mut state: uint32_t = 0 as libc::c_int as uint32_t;
    let mut counter: uint32_t = 0 as libc::c_int as uint32_t;
    let mut seen_valid: uint8_t = 0 as libc::c_int as uint8_t;
    while rpos < len && wpos < len {
        counter = counter.wrapping_add(1);
        match htp_utf8_decoder::htp_utf8_decode_allow_overlong(
            &mut state,
            &mut codepoint,
            *data.offset(rpos as isize) as uint32_t,
        ) {
            0 => {
                if counter == 1 as libc::c_int as libc::c_uint {
                    // ASCII character, which we just copy.
                    let fresh0 = wpos;
                    wpos = wpos.wrapping_add(1);
                    *data.offset(fresh0 as isize) = codepoint as uint8_t
                } else {
                    // A valid UTF-8 character, which we need to convert.
                    seen_valid = 1 as libc::c_int as uint8_t;
                    // Check for overlong characters and set the flag accordingly.
                    match counter {
                        2 => {
                            if codepoint < 0x80 as libc::c_int as libc::c_uint {
                                (*tx).flags |= Flags::HTP_PATH_UTF8_OVERLONG
                            }
                        }
                        3 => {
                            if codepoint < 0x800 as libc::c_int as libc::c_uint {
                                (*tx).flags |= Flags::HTP_PATH_UTF8_OVERLONG
                            }
                        }
                        4 => {
                            if codepoint < 0x10000 as libc::c_int as libc::c_uint {
                                (*tx).flags |= Flags::HTP_PATH_UTF8_OVERLONG
                            }
                        }
                        _ => {}
                    }
                    // Special flag for half-width/full-width evasion.
                    if codepoint >= 0xff00 as libc::c_int as libc::c_uint
                        && codepoint <= 0xffef as libc::c_int as libc::c_uint
                    {
                        (*tx).flags |= Flags::HTP_PATH_HALF_FULL_RANGE
                    }
                    // Use best-fit mapping to convert to a single byte.
                    let fresh1 = wpos;
                    wpos = wpos.wrapping_add(1);
                    *data.offset(fresh1 as isize) = bestfit_codepoint(
                        cfg,
                        htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
                        codepoint,
                    )
                }
                // Advance over the consumed byte and reset the byte counter.
                rpos = rpos.wrapping_add(1);
                counter = 0 as libc::c_int as uint32_t
            }
            1 => {
                // Invalid UTF-8 character.
                (*tx).flags |= Flags::HTP_PATH_UTF8_INVALID;
                // Is the server expected to respond with 400?
                if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .utf8_invalid_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                        [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                        .utf8_invalid_unwanted
                        as libc::c_int
                }
                // Output the replacement byte, replacing one or more invalid bytes.
                let fresh2 = wpos;
                wpos = wpos.wrapping_add(1);
                *data.offset(fresh2 as isize) = (*cfg).decoder_cfgs
                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .bestfit_replacement_byte;
                // If the invalid byte was first in a sequence, consume it. Otherwise,
                // assume it's the starting byte of the next character.
                if counter == 1 as libc::c_int as libc::c_uint {
                    rpos = rpos.wrapping_add(1)
                }
                // Reset the decoder state and continue decoding.
                state = 0 as libc::c_int as uint32_t;
                codepoint = 0 as libc::c_int as uint32_t;
                counter = 0 as libc::c_int as uint32_t
            }
            _ => {
                // Keep going; the character is not yet formed.
                rpos = rpos.wrapping_add(1)
            }
        }
    }
    // Did the input stream seem like a valid UTF-8 string?
    if seen_valid as libc::c_int != 0 && !(*tx).flags.contains(Flags::HTP_PATH_UTF8_INVALID) {
        (*tx).flags |= Flags::HTP_PATH_UTF8_VALID
    }
    // Adjust the length of the string, because
    // we're doing in-place decoding.
    bstr::bstr_adjust_len(path, wpos);
}

/// Validate a path that is quite possibly UTF-8 encoded.
pub unsafe fn htp_utf8_validate_path(
    mut tx: *mut htp_transaction::htp_tx_t,
    mut path: *mut bstr::bstr_t,
) {
    let mut data: *mut libc::c_uchar = if (*path).realptr.is_null() {
        (path as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*path).realptr
    }; // How many bytes used by a UTF-8 character.
    let mut len: size_t = (*path).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut codepoint: uint32_t = 0 as libc::c_int as uint32_t;
    let mut state: uint32_t = 0 as libc::c_int as uint32_t;
    let mut counter: uint32_t = 0 as libc::c_int as uint32_t;
    let mut seen_valid: uint8_t = 0 as libc::c_int as uint8_t;
    while rpos < len {
        counter = counter.wrapping_add(1);
        match htp_utf8_decoder::htp_utf8_decode_allow_overlong(
            &mut state,
            &mut codepoint,
            *data.offset(rpos as isize) as uint32_t,
        ) {
            0 => {
                // We have a valid character.
                if counter > 1 as libc::c_int as libc::c_uint {
                    // A valid UTF-8 character, consisting of 2 or more bytes.
                    seen_valid = 1 as libc::c_int as uint8_t;
                    // Check for overlong characters and set the flag accordingly.
                    match counter {
                        2 => {
                            if codepoint < 0x80 as libc::c_int as libc::c_uint {
                                (*tx).flags |= Flags::HTP_PATH_UTF8_OVERLONG
                            }
                        }
                        3 => {
                            if codepoint < 0x800 as libc::c_int as libc::c_uint {
                                (*tx).flags |= Flags::HTP_PATH_UTF8_OVERLONG
                            }
                        }
                        4 => {
                            if codepoint < 0x10000 as libc::c_int as libc::c_uint {
                                (*tx).flags |= Flags::HTP_PATH_UTF8_OVERLONG
                            }
                        }
                        _ => {}
                    }
                }
                // Special flag for half-width/full-width evasion.
                if codepoint > 0xfeff as libc::c_int as libc::c_uint
                    && codepoint < 0x10000 as libc::c_int as libc::c_uint
                {
                    (*tx).flags |= Flags::HTP_PATH_HALF_FULL_RANGE
                }
                // Advance over the consumed byte and reset the byte counter.
                rpos = rpos.wrapping_add(1);
                counter = 0 as libc::c_int as uint32_t
            }
            1 => {
                // Invalid UTF-8 character.
                (*tx).flags |= Flags::HTP_PATH_UTF8_INVALID;
                // Override the decoder state because we want to continue decoding.
                state = 0 as libc::c_int as uint32_t;
                // Advance over the consumed byte and reset the byte counter.
                rpos = rpos.wrapping_add(1);
                counter = 0 as libc::c_int as uint32_t
            }
            _ => {
                // Keep going; the character is not yet formed.
                rpos = rpos.wrapping_add(1)
            }
        }
    }
    // Did the input stream seem like a valid UTF-8 string?
    if seen_valid as libc::c_int != 0 && !(*tx).flags.contains(Flags::HTP_PATH_UTF8_INVALID) {
        (*tx).flags |= Flags::HTP_PATH_UTF8_VALID
    };
}

/// Decode a %u-encoded character, using best-fit mapping as necessary. Path version.
///
/// Returns decoded byte
unsafe fn decode_u_encoding_path(
    mut cfg: *mut htp_config::htp_cfg_t,
    mut tx: *mut htp_transaction::htp_tx_t,
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut c1: libc::c_uint = x2c(data) as libc::c_uint;
    let mut c2: libc::c_uint = x2c(data.offset(2 as libc::c_int as isize)) as libc::c_uint;
    let mut r: libc::c_int = (*cfg).decoder_cfgs
        [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
        .bestfit_replacement_byte as libc::c_int;
    if c1 == 0 as libc::c_int as libc::c_uint {
        r = c2 as libc::c_int;
        (*tx).flags |= Flags::HTP_PATH_OVERLONG_U
    } else {
        // Check for fullwidth form evasion
        if c1 == 0xff as libc::c_int as libc::c_uint {
            (*tx).flags |= Flags::HTP_PATH_HALF_FULL_RANGE
        }
        if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
            .u_encoding_unwanted
            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
        {
            (*tx).response_status_expected_number =
                (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .u_encoding_unwanted as libc::c_int
        }
        // Use best-fit mapping
        let mut p: *mut libc::c_uchar = (*cfg).decoder_cfgs
            [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
            .bestfit_map;
        // TODO Optimize lookup.
        // Have we reached the end of the map?
        while !(*p.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
        {
            // Have we found the mapping we're looking for?
            if *p.offset(0 as libc::c_int as isize) as libc::c_uint == c1
                && *p.offset(1 as libc::c_int as isize) as libc::c_uint == c2
            {
                r = *p.offset(2 as libc::c_int as isize) as libc::c_int;
                break;
            } else {
                // Move to the next triplet
                p = p.offset(3 as libc::c_int as isize)
            }
        }
    }
    // Check for encoded path separators
    if r == '/' as i32
        || (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
            .backslash_convert_slashes
            != 0
            && r == '\\' as i32
    {
        (*tx).flags |= Flags::HTP_PATH_ENCODED_SEPARATOR
    }
    return r;
}

/// Decode a %u-encoded character, using best-fit mapping as necessary. Params version.
///
/// Returns decoded byte
unsafe fn decode_u_encoding_params(
    mut cfg: *mut htp_config::htp_cfg_t,
    mut ctx: htp_config::htp_decoder_ctx_t,
    mut data: *mut libc::c_uchar,
    mut flags: *mut Flags,
) -> libc::c_int {
    let mut c1: libc::c_uint = x2c(data) as libc::c_uint;
    let mut c2: libc::c_uint = x2c(data.offset(2 as libc::c_int as isize)) as libc::c_uint;
    // Check for overlong usage first.
    if c1 == 0 as libc::c_int as libc::c_uint {
        *flags |= Flags::HTP_URLEN_OVERLONG_U;
        return c2 as libc::c_int;
    }
    // Both bytes were used.
    // Detect half-width and full-width range.
    if c1 == 0xff as libc::c_int as libc::c_uint && c2 <= 0xef as libc::c_int as libc::c_uint {
        *flags |= Flags::HTP_URLEN_HALF_FULL_RANGE
    }
    // Use best-fit mapping.
    let mut p: *mut libc::c_uchar = (*cfg).decoder_cfgs[ctx as usize].bestfit_map;
    let mut r: libc::c_int =
        (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte as libc::c_int;
    // TODO Optimize lookup.
    // Have we reached the end of the map?
    while !(*p.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
    {
        // Have we found the mapping we're looking for?
        if *p.offset(0 as libc::c_int as isize) as libc::c_uint == c1
            && *p.offset(1 as libc::c_int as isize) as libc::c_uint == c2
        {
            r = *p.offset(2 as libc::c_int as isize) as libc::c_int;
            break;
        } else {
            // Move to the next triplet
            p = p.offset(3 as libc::c_int as isize)
        }
    }
    return r;
}

/// Decode a request path according to the settings in the
/// provided configuration structure.
pub unsafe fn htp_decode_path_inplace(
    mut tx: *mut htp_transaction::htp_tx_t,
    mut path: *mut bstr::bstr_t,
) -> libc::c_int {
    if path.is_null() {
        return -(1 as libc::c_int);
    }
    let mut data: *mut libc::c_uchar = if (*path).realptr.is_null() {
        (path as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*path).realptr
    };
    if data.is_null() {
        return -(1 as libc::c_int);
    }
    let mut len: size_t = (*path).len;
    let mut cfg: *mut htp_config::htp_cfg_t = (*tx).cfg;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut previous_was_separator: libc::c_int = 0 as libc::c_int;
    let mut current_block_104: u64;
    while rpos < len && wpos < len {
        let mut c: libc::c_int = *data.offset(rpos as isize) as libc::c_int;
        // Decode encoded characters
        if c == '%' as i32 {
            if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) < len {
                let mut handled: libc::c_int = 0 as libc::c_int;
                if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .u_encoding_decode
                    != 0
                {
                    // Check for the %u encoding
                    if *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        == 'u' as i32
                        || *data
                            .offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                            as libc::c_int
                            == 'U' as i32
                    {
                        handled = 1 as libc::c_int;
                        if (*cfg).decoder_cfgs
                            [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                            .u_encoding_unwanted
                            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                        {
                            (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .u_encoding_unwanted
                                as libc::c_int
                        }
                        if rpos.wrapping_add(5 as libc::c_int as libc::c_ulong) < len {
                            if *(*__ctype_b_loc()).offset(*data.offset(
                                rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                            )
                                as libc::c_int
                                as isize) as libc::c_int
                                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                                && *(*__ctype_b_loc()).offset(
                                    *data.offset(
                                        rpos.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int as isize,
                                ) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                && *(*__ctype_b_loc()).offset(
                                    *data.offset(
                                        rpos.wrapping_add(4 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int as isize,
                                ) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                && *(*__ctype_b_loc()).offset(
                                    *data.offset(
                                        rpos.wrapping_add(5 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int as isize,
                                ) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                            {
                                // Decode a valid %u encoding
                                c = decode_u_encoding_path(
                                    cfg,
                                    tx,
                                    &mut *data.offset(
                                        rpos.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ),
                                );
                                rpos = (rpos as libc::c_ulong)
                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                    as size_t as size_t;
                                if c == 0 as libc::c_int {
                                    (*tx).flags |= Flags::HTP_PATH_ENCODED_NUL;
                                    if (*cfg).decoder_cfgs
                                        [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH
                                            as usize]
                                        .nul_encoded_unwanted
                                        != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                                    {
                                        (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                                            [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH
                                                as usize]
                                            .nul_encoded_unwanted
                                            as libc::c_int
                                    }
                                }
                            } else {
                                // Invalid %u encoding
                                (*tx).flags |= Flags::HTP_PATH_INVALID_ENCODING;
                                if (*cfg).decoder_cfgs
                                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                    .url_encoding_invalid_unwanted
                                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                                {
                                    (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                                        [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH
                                            as usize]
                                        .url_encoding_invalid_unwanted
                                        as libc::c_int
                                }
                                match (*cfg).decoder_cfgs
                                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                    .url_encoding_invalid_handling
                                    as libc::c_uint
                                {
                                    1 => {
                                        current_block_104 = 5193467589189724848;
                                        match current_block_104 {
                                            15044848815912959287 => {
                                                // Leave the percent character in output
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            5193467589189724848 => {
                                                // Do not place anything in output; eat
                                                // the percent character
                                                rpos = rpos.wrapping_add(1);
                                                continue;
                                            }
                                            _ => {
                                                // Decode invalid %u encoding
                                                c = decode_u_encoding_path(
                                                    cfg,
                                                    tx,
                                                    &mut *data.offset(rpos.wrapping_add(
                                                        2 as libc::c_int as libc::c_ulong,
                                                    )
                                                        as isize),
                                                );
                                                rpos = (rpos as libc::c_ulong)
                                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t
                                            }
                                        }
                                    }
                                    0 => {
                                        current_block_104 = 15044848815912959287;
                                        match current_block_104 {
                                            15044848815912959287 => rpos = rpos.wrapping_add(1),
                                            5193467589189724848 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue;
                                            }
                                            _ => {
                                                c = decode_u_encoding_path(
                                                    cfg,
                                                    tx,
                                                    &mut *data.offset(rpos.wrapping_add(
                                                        2 as libc::c_int as libc::c_ulong,
                                                    )
                                                        as isize),
                                                );
                                                rpos = (rpos as libc::c_ulong)
                                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t
                                            }
                                        }
                                    }
                                    2 => {
                                        current_block_104 = 3531489836707249550;
                                        match current_block_104 {
                                            15044848815912959287 => rpos = rpos.wrapping_add(1),
                                            5193467589189724848 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue;
                                            }
                                            _ => {
                                                c = decode_u_encoding_path(
                                                    cfg,
                                                    tx,
                                                    &mut *data.offset(rpos.wrapping_add(
                                                        2 as libc::c_int as libc::c_ulong,
                                                    )
                                                        as isize),
                                                );
                                                rpos = (rpos as libc::c_ulong)
                                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            // Invalid %u encoding (not enough data)
                            (*tx).flags |= Flags::HTP_PATH_INVALID_ENCODING;
                            if (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .url_encoding_invalid_unwanted
                                != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                            {
                                (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                    .url_encoding_invalid_unwanted
                                    as libc::c_int
                            }
                            match (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .url_encoding_invalid_handling
                                as libc::c_uint
                            {
                                1 => {
                                    current_block_104 = 15984154738040588190;
                                    match current_block_104 {
                                        11934984557441853882 => {
                                            // Leave the percent character in output
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        15984154738040588190 => {
                                            // Do not place anything in output; eat
                                            // the percent character
                                            rpos = rpos.wrapping_add(1);
                                            continue;
                                        }
                                        _ => {
                                            // Cannot decode, because there's not enough data.
                                            // Leave the percent character in output
                                            rpos = rpos.wrapping_add(1)
                                        }
                                    }
                                }
                                0 => {
                                    current_block_104 = 11934984557441853882;
                                    match current_block_104 {
                                        11934984557441853882 => rpos = rpos.wrapping_add(1),
                                        15984154738040588190 => {
                                            rpos = rpos.wrapping_add(1);
                                            continue;
                                        }
                                        _ => rpos = rpos.wrapping_add(1),
                                    }
                                }
                                2 => {
                                    current_block_104 = 14856184476078576297;
                                    match current_block_104 {
                                        11934984557441853882 => rpos = rpos.wrapping_add(1),
                                        15984154738040588190 => {
                                            rpos = rpos.wrapping_add(1);
                                            continue;
                                        }
                                        _ => rpos = rpos.wrapping_add(1),
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                // Handle standard URL encoding
                if handled == 0 {
                    if *(*__ctype_b_loc()).offset(
                        *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                            as libc::c_int as isize,
                    ) as libc::c_int
                        & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                        && *(*__ctype_b_loc()).offset(
                            *data.offset(
                                rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize
                            ) as libc::c_int as isize,
                        ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        c = x2c(&mut *data
                            .offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                            as libc::c_int;
                        if c == 0 as libc::c_int {
                            (*tx).flags |= Flags::HTP_PATH_ENCODED_NUL;
                            if (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .nul_encoded_unwanted
                                != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                            {
                                (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                    .nul_encoded_unwanted
                                    as libc::c_int
                            }
                            if (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .nul_encoded_terminates
                                != 0
                            {
                                bstr::bstr_adjust_len(path, wpos);
                                return 1 as libc::c_int;
                            }
                        }
                        if c == '/' as i32
                            || (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .backslash_convert_slashes
                                != 0
                                && c == '\\' as i32
                        {
                            (*tx).flags |= Flags::HTP_PATH_ENCODED_SEPARATOR;
                            if (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .path_separators_encoded_unwanted
                                != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                            {
                                (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                    .path_separators_encoded_unwanted
                                    as libc::c_int
                            }
                            if (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .path_separators_decode
                                != 0
                            {
                                // Decode
                                rpos = (rpos as libc::c_ulong)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    as size_t as size_t
                            } else {
                                // Leave encoded
                                c = '%' as i32;
                                rpos = rpos.wrapping_add(1)
                            }
                        } else {
                            // Decode
                            rpos = (rpos as libc::c_ulong)
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as size_t as size_t
                        }
                    } else {
                        // Invalid encoding
                        (*tx).flags |= Flags::HTP_PATH_INVALID_ENCODING;
                        if (*cfg).decoder_cfgs
                            [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                            .url_encoding_invalid_unwanted
                            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                        {
                            (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                                [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                                .url_encoding_invalid_unwanted
                                as libc::c_int
                        }
                        match (*cfg).decoder_cfgs
                            [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                            .url_encoding_invalid_handling
                            as libc::c_uint
                        {
                            1 => {
                                // Do not place anything in output; eat
                                // the percent character
                                rpos = rpos.wrapping_add(1);
                                continue;
                            }
                            0 => {
                                // Leave the percent character in output
                                rpos = rpos.wrapping_add(1)
                            }
                            2 => {
                                // Decode
                                c = x2c(&mut *data
                                    .offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize))
                                    as libc::c_int;
                                rpos = (rpos as libc::c_ulong)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                    as size_t as size_t
                            }
                            _ => {
                                // Unknown setting
                                return -(1 as libc::c_int);
                            }
                        }
                    }
                }
            } else {
                // Invalid URL encoding (not enough data)
                (*tx).flags |= Flags::HTP_PATH_INVALID_ENCODING;
                if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .url_encoding_invalid_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                        [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                        .url_encoding_invalid_unwanted
                        as libc::c_int
                }
                match (*cfg).decoder_cfgs
                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .url_encoding_invalid_handling as libc::c_uint
                {
                    1 => {
                        current_block_104 = 5650022063725743123;
                        match current_block_104 {
                            10614498797110429124 => {
                                // Cannot decode, because there's not enough data.
                                // Leave the percent character in output.
                                // TODO Configurable handling.
                                rpos = rpos.wrapping_add(1)
                            }
                            5986777620604961003 => {
                                // Leave the percent character in output
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => {
                                // Do not place anything in output; eat
                                // the percent character
                                rpos = rpos.wrapping_add(1);
                                continue;
                            }
                        }
                    }
                    0 => {
                        current_block_104 = 5986777620604961003;
                        match current_block_104 {
                            10614498797110429124 => rpos = rpos.wrapping_add(1),
                            5986777620604961003 => rpos = rpos.wrapping_add(1),
                            _ => {
                                rpos = rpos.wrapping_add(1);
                                continue;
                            }
                        }
                    }
                    2 => {
                        current_block_104 = 10614498797110429124;
                        match current_block_104 {
                            10614498797110429124 => rpos = rpos.wrapping_add(1),
                            5986777620604961003 => rpos = rpos.wrapping_add(1),
                            _ => {
                                rpos = rpos.wrapping_add(1);
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else {
            // One non-encoded character
            // Is it a NUL byte?
            if c == 0 as libc::c_int {
                if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .nul_raw_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                        [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                        .nul_raw_unwanted
                        as libc::c_int
                }
                if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .nul_raw_terminates
                    != 0
                {
                    // Terminate path with a raw NUL byte
                    bstr::bstr_adjust_len(path, wpos);
                    return 1 as libc::c_int;
                }
            }
            rpos = rpos.wrapping_add(1)
        }
        // Note: What if an invalid encoding decodes into a path
        //       separator? This is theoretical at the moment, because
        //       the only platform we know doesn't convert separators is
        //       Apache, who will also respond with 400 if invalid encoding
        //       is encountered. Thus no check for a separator here.
        // Place the character into output
        // Check for control characters
        if c < 0x20 as libc::c_int {
            if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                .control_chars_unwanted
                != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
            {
                (*tx).response_status_expected_number = (*cfg).decoder_cfgs
                    [htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                    .control_chars_unwanted
                    as libc::c_int
            }
        }
        // Convert backslashes to forward slashes, if necessary
        if c == '\\' as i32
            && (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
                .backslash_convert_slashes
                != 0
        {
            c = '/' as i32
        }
        // Lowercase characters, if necessary
        if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
            .convert_lowercase
            != 0
        {
            c = tolower(c)
        }
        // If we're compressing separators then we need
        // to track if the previous character was a separator
        if (*cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
            .path_separators_compress
            != 0
        {
            if c == '/' as i32 {
                if previous_was_separator == 0 {
                    let fresh3 = wpos;
                    wpos = wpos.wrapping_add(1);
                    *data.offset(fresh3 as isize) = c as libc::c_uchar;
                    previous_was_separator = 1 as libc::c_int
                }
            } else {
                let fresh4 = wpos;
                wpos = wpos.wrapping_add(1);
                *data.offset(fresh4 as isize) = c as libc::c_uchar;
                previous_was_separator = 0 as libc::c_int
            }
        } else {
            let fresh5 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh5 as isize) = c as libc::c_uchar
        }
    }
    bstr::bstr_adjust_len(path, wpos);
    return 1 as libc::c_int;
}

pub unsafe fn htp_tx_urldecode_uri_inplace(
    mut tx: *mut htp_transaction::htp_tx_t,
    mut input: *mut bstr::bstr_t,
) -> Status {
    let mut flags: Flags = Flags::empty();
    let mut rc: Status = htp_urldecode_inplace_ex(
        (*tx).cfg,
        htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH,
        input,
        &mut flags,
        &mut (*tx).response_status_expected_number,
    );
    if flags.contains(Flags::HTP_URLEN_INVALID_ENCODING) {
        (*tx).flags |= Flags::HTP_PATH_INVALID_ENCODING
    }
    if flags.contains(Flags::HTP_URLEN_ENCODED_NUL) {
        (*tx).flags |= Flags::HTP_PATH_ENCODED_NUL
    }
    if flags.contains(Flags::HTP_URLEN_RAW_NUL) {
        (*tx).flags |= Flags::HTP_PATH_RAW_NUL;
    }
    return rc;
}

pub unsafe fn htp_tx_urldecode_params_inplace(
    mut tx: *mut htp_transaction::htp_tx_t,
    mut input: *mut bstr::bstr_t,
) -> Status {
    return htp_urldecode_inplace_ex(
        (*tx).cfg,
        htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
        input,
        &mut (*tx).flags,
        &mut (*tx).response_status_expected_number,
    );
}

/// Performs in-place decoding of the input string, according to the configuration specified
/// by cfg and ctx. On output, various flags (HTP_URLEN_*) might be set.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub unsafe fn htp_urldecode_inplace(
    mut cfg: *mut htp_config::htp_cfg_t,
    mut ctx: htp_config::htp_decoder_ctx_t,
    mut input: *mut bstr::bstr_t,
    mut flags: *mut Flags,
) -> Status {
    let mut expected_status_code: libc::c_int = 0 as libc::c_int;
    return htp_urldecode_inplace_ex(cfg, ctx, input, flags, &mut expected_status_code);
}

/// Performs in-place decoding of the input string, according to the configuration specified
/// by cfg and ctx. On output, various flags (HTP_URLEN_*) might be set. If something in the
/// input would cause a particular server to respond with an error, the appropriate status
/// code will be set.
///
/// Returns in expected_status_code: 0 by default, or status code as necessary
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub unsafe fn htp_urldecode_inplace_ex(
    mut cfg: *mut htp_config::htp_cfg_t,
    mut ctx: htp_config::htp_decoder_ctx_t,
    mut input: *mut bstr::bstr_t,
    mut flags: *mut Flags,
    mut expected_status_code: *mut libc::c_int,
) -> Status {
    if input.is_null() {
        return Status::ERROR;
    }
    let mut data: *mut libc::c_uchar = if (*input).realptr.is_null() {
        (input as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*input).realptr
    };
    if data.is_null() {
        return Status::ERROR;
    }
    let mut len: size_t = (*input).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut current_block_74: u64;
    while rpos < len && wpos < len {
        let mut c: libc::c_int = *data.offset(rpos as isize) as libc::c_int;
        // Decode encoded characters.
        if c == '%' as i32 {
            // Need at least 2 additional bytes for %HH.
            if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) < len {
                let mut handled: libc::c_int = 0 as libc::c_int;
                // Decode %uHHHH encoding, but only if allowed in configuration.
                if (*cfg).decoder_cfgs[ctx as usize].u_encoding_decode != 0 {
                    // The next character must be a case-insensitive u.
                    if *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        == 'u' as i32
                        || *data
                            .offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                            as libc::c_int
                            == 'U' as i32
                    {
                        handled = 1 as libc::c_int;
                        if (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted
                            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                        {
                            *expected_status_code =
                                (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted as libc::c_int
                        }
                        // Need at least 5 additional bytes for %uHHHH.
                        if rpos.wrapping_add(5 as libc::c_int as libc::c_ulong) < len {
                            if *(*__ctype_b_loc()).offset(*data.offset(
                                rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                            )
                                as libc::c_int
                                as isize) as libc::c_int
                                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                                && *(*__ctype_b_loc()).offset(
                                    *data.offset(
                                        rpos.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int as isize,
                                ) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                && *(*__ctype_b_loc()).offset(
                                    *data.offset(
                                        rpos.wrapping_add(4 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int as isize,
                                ) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                && *(*__ctype_b_loc()).offset(
                                    *data.offset(
                                        rpos.wrapping_add(5 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int as isize,
                                ) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                            {
                                // Decode a valid %u encoding.
                                c = decode_u_encoding_params(
                                    cfg,
                                    ctx,
                                    &mut *data.offset(
                                        rpos.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ),
                                    flags,
                                );
                                rpos = (rpos as libc::c_ulong)
                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                    as size_t as size_t
                            } else {
                                // Invalid %u encoding (could not find 4 xdigits).
                                *flags |= Flags::HTP_URLEN_INVALID_ENCODING;
                                if (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_unwanted
                                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                                {
                                    *expected_status_code = (*cfg).decoder_cfgs[ctx as usize]
                                        .url_encoding_invalid_unwanted
                                        as libc::c_int
                                }
                                match (*cfg).decoder_cfgs[ctx as usize]
                                    .url_encoding_invalid_handling
                                    as libc::c_uint
                                {
                                    1 => {
                                        current_block_74 = 15769233237055051138;
                                        match current_block_74 {
                                            10436515788539709011 => {
                                                // Leave the % in output.
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            15769233237055051138 => {
                                                // Do not place anything in output; consume the %.
                                                rpos = rpos.wrapping_add(1);
                                                continue;
                                            }
                                            _ => {
                                                // Decode invalid %u encoding.
                                                c = decode_u_encoding_params(
                                                    cfg,
                                                    ctx,
                                                    &mut *data.offset(rpos.wrapping_add(
                                                        2 as libc::c_int as libc::c_ulong,
                                                    )
                                                        as isize),
                                                    flags,
                                                );
                                                rpos = (rpos as libc::c_ulong)
                                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t
                                            }
                                        }
                                    }
                                    0 => {
                                        current_block_74 = 10436515788539709011;
                                        match current_block_74 {
                                            10436515788539709011 => rpos = rpos.wrapping_add(1),
                                            15769233237055051138 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue;
                                            }
                                            _ => {
                                                c = decode_u_encoding_params(
                                                    cfg,
                                                    ctx,
                                                    &mut *data.offset(rpos.wrapping_add(
                                                        2 as libc::c_int as libc::c_ulong,
                                                    )
                                                        as isize),
                                                    flags,
                                                );
                                                rpos = (rpos as libc::c_ulong)
                                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t
                                            }
                                        }
                                    }
                                    2 => {
                                        current_block_74 = 16443981440205402410;
                                        match current_block_74 {
                                            10436515788539709011 => rpos = rpos.wrapping_add(1),
                                            15769233237055051138 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue;
                                            }
                                            _ => {
                                                c = decode_u_encoding_params(
                                                    cfg,
                                                    ctx,
                                                    &mut *data.offset(rpos.wrapping_add(
                                                        2 as libc::c_int as libc::c_ulong,
                                                    )
                                                        as isize),
                                                    flags,
                                                );
                                                rpos = (rpos as libc::c_ulong)
                                                    .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                                    as size_t
                                                    as size_t
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            // Invalid %u encoding; not enough data.
                            *flags |= Flags::HTP_URLEN_INVALID_ENCODING;
                            if (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_unwanted
                                != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                            {
                                *expected_status_code = (*cfg).decoder_cfgs[ctx as usize]
                                    .url_encoding_invalid_unwanted
                                    as libc::c_int
                            }
                            match (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_handling
                                as libc::c_uint
                            {
                                1 => {
                                    current_block_74 = 16383797545558020236;
                                    match current_block_74 {
                                        16032006980801283503 => {
                                            // Cannot decode because there's not enough data.
                                            // Leave the % in output.
                                            // TODO Configurable handling of %, u, etc.
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        8223123178938535296 => {
                                            // Leave the % in output.
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        _ => {
                                            // Do not place anything in output; consume the %.
                                            rpos = rpos.wrapping_add(1);
                                            continue;
                                        }
                                    }
                                }
                                0 => {
                                    current_block_74 = 8223123178938535296;
                                    match current_block_74 {
                                        16032006980801283503 => rpos = rpos.wrapping_add(1),
                                        8223123178938535296 => rpos = rpos.wrapping_add(1),
                                        _ => {
                                            rpos = rpos.wrapping_add(1);
                                            continue;
                                        }
                                    }
                                }
                                2 => {
                                    current_block_74 = 16032006980801283503;
                                    match current_block_74 {
                                        16032006980801283503 => rpos = rpos.wrapping_add(1),
                                        8223123178938535296 => rpos = rpos.wrapping_add(1),
                                        _ => {
                                            rpos = rpos.wrapping_add(1);
                                            continue;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                // Handle standard URL encoding.
                if handled == 0 {
                    // Need 2 hexadecimal digits.
                    if *(*__ctype_b_loc()).offset(
                        *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                            as libc::c_int as isize,
                    ) as libc::c_int
                        & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                        && *(*__ctype_b_loc()).offset(
                            *data.offset(
                                rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize
                            ) as libc::c_int as isize,
                        ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        // Decode %HH encoding.
                        c = x2c(&mut *data
                            .offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                            as libc::c_int;
                        rpos = (rpos as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            as size_t as size_t
                    } else {
                        // Invalid encoding (enough bytes, but not hexadecimal digits).
                        *flags |= Flags::HTP_URLEN_INVALID_ENCODING;
                        if (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_unwanted
                            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                        {
                            *expected_status_code = (*cfg).decoder_cfgs[ctx as usize]
                                .url_encoding_invalid_unwanted
                                as libc::c_int
                        }
                        match (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_handling
                            as libc::c_uint
                        {
                            1 => {
                                current_block_74 = 15028968826697170054;
                                match current_block_74 {
                                    7617508444621897972 => {
                                        // Leave the % in output.
                                        rpos = rpos.wrapping_add(1)
                                    }
                                    15028968826697170054 => {
                                        // Do not place anything in output; consume the %.
                                        rpos = rpos.wrapping_add(1);
                                        continue;
                                    }
                                    _ => {
                                        // Decode.
                                        c = x2c(&mut *data.offset(
                                            rpos.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ))
                                            as libc::c_int;
                                        rpos = (rpos as libc::c_ulong)
                                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                            as size_t
                                            as size_t
                                    }
                                }
                            }
                            0 => {
                                current_block_74 = 7617508444621897972;
                                match current_block_74 {
                                    7617508444621897972 => rpos = rpos.wrapping_add(1),
                                    15028968826697170054 => {
                                        rpos = rpos.wrapping_add(1);
                                        continue;
                                    }
                                    _ => {
                                        c = x2c(&mut *data.offset(
                                            rpos.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ))
                                            as libc::c_int;
                                        rpos = (rpos as libc::c_ulong)
                                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                            as size_t
                                            as size_t
                                    }
                                }
                            }
                            2 => {
                                current_block_74 = 3516197883607697062;
                                match current_block_74 {
                                    7617508444621897972 => rpos = rpos.wrapping_add(1),
                                    15028968826697170054 => {
                                        rpos = rpos.wrapping_add(1);
                                        continue;
                                    }
                                    _ => {
                                        c = x2c(&mut *data.offset(
                                            rpos.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ))
                                            as libc::c_int;
                                        rpos = (rpos as libc::c_ulong)
                                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                            as size_t
                                            as size_t
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                // Invalid encoding; not enough data (at least 2 bytes required).
                *flags |= Flags::HTP_URLEN_INVALID_ENCODING;
                if (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    *expected_status_code = (*cfg).decoder_cfgs[ctx as usize]
                        .url_encoding_invalid_unwanted
                        as libc::c_int
                }
                match (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_handling
                    as libc::c_uint
                {
                    1 => {
                        current_block_74 = 8697558811166951253;
                        match current_block_74 {
                            13503558473217943653 => {
                                // Cannot decode because there's not enough data.
                                // Leave the % in output.
                                // TODO Configurable handling of %, etc.
                                rpos = rpos.wrapping_add(1)
                            }
                            821486359641935908 => {
                                // Leave the % in output.
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => {
                                // Do not place anything in output; consume the %.
                                rpos = rpos.wrapping_add(1);
                                continue;
                            }
                        }
                    }
                    0 => {
                        current_block_74 = 821486359641935908;
                        match current_block_74 {
                            13503558473217943653 => rpos = rpos.wrapping_add(1),
                            821486359641935908 => rpos = rpos.wrapping_add(1),
                            _ => {
                                rpos = rpos.wrapping_add(1);
                                continue;
                            }
                        }
                    }
                    2 => {
                        current_block_74 = 13503558473217943653;
                        match current_block_74 {
                            13503558473217943653 => rpos = rpos.wrapping_add(1),
                            821486359641935908 => rpos = rpos.wrapping_add(1),
                            _ => {
                                rpos = rpos.wrapping_add(1);
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
            // Did we get an encoded NUL byte?
            if c == 0 as libc::c_int {
                if (*cfg).decoder_cfgs[ctx as usize].nul_encoded_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    *expected_status_code =
                        (*cfg).decoder_cfgs[ctx as usize].nul_encoded_unwanted as libc::c_int
                }
                *flags |= Flags::HTP_URLEN_ENCODED_NUL;
                if (*cfg).decoder_cfgs[ctx as usize].nul_encoded_terminates != 0 {
                    // Terminate the path at the raw NUL byte.
                    bstr::bstr_adjust_len(input, wpos);
                    return Status::OK;
                }
            }
            let fresh6 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh6 as isize) = c as libc::c_uchar
        } else if c == '+' as i32 {
            // Decoding of the plus character is conditional on the configuration.
            if (*cfg).decoder_cfgs[ctx as usize].plusspace_decode != 0 {
                c = 0x20 as libc::c_int
            }
            rpos = rpos.wrapping_add(1);
            let fresh7 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh7 as isize) = c as libc::c_uchar
        } else {
            // One non-encoded byte.
            // Did we get a raw NUL byte?
            if c == 0 as libc::c_int {
                if (*cfg).decoder_cfgs[ctx as usize].nul_raw_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    *expected_status_code =
                        (*cfg).decoder_cfgs[ctx as usize].nul_raw_unwanted as libc::c_int
                }
                *flags |= Flags::HTP_URLEN_RAW_NUL;
                if (*cfg).decoder_cfgs[ctx as usize].nul_raw_terminates != 0 {
                    // Terminate the path at the encoded NUL byte.
                    bstr::bstr_adjust_len(input, wpos);
                    return Status::OK;
                }
            }
            rpos = rpos.wrapping_add(1);
            let fresh8 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh8 as isize) = c as libc::c_uchar
        }
    }
    bstr::bstr_adjust_len(input, wpos);
    return Status::OK;
}

/// Normalize a previously-parsed request URI.
///
/// Returns HTP_OK or HTP_ERROR
pub unsafe fn htp_normalize_parsed_uri(
    mut tx: *mut htp_transaction::htp_tx_t,
    mut incomplete: *mut htp_uri_t,
    mut normalized: *mut htp_uri_t,
) -> libc::c_int {
    // Scheme.
    if !(*incomplete).scheme.is_null() {
        // Duplicate and convert to lowercase.
        (*normalized).scheme = bstr::bstr_dup_lower((*incomplete).scheme);
        if (*normalized).scheme.is_null() {
            return -(1 as libc::c_int);
        }
    }
    // Username.
    if !(*incomplete).username.is_null() {
        (*normalized).username = bstr::bstr_dup((*incomplete).username);
        if (*normalized).username.is_null() {
            return -(1 as libc::c_int);
        }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).username);
    }
    // Password.
    if !(*incomplete).password.is_null() {
        (*normalized).password = bstr::bstr_dup((*incomplete).password);
        if (*normalized).password.is_null() {
            return -(1 as libc::c_int);
        }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).password);
    }
    // Hostname.
    if !(*incomplete).hostname.is_null() {
        // We know that incomplete->hostname does not contain
        // port information, so no need to check for it here.
        (*normalized).hostname = bstr::bstr_dup((*incomplete).hostname);
        if (*normalized).hostname.is_null() {
            return -(1 as libc::c_int);
        }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).hostname);
        htp_normalize_hostname_inplace((*normalized).hostname);
    }
    // Port.
    if !(*incomplete).port.is_null() {
        let mut port_parsed: int64_t = htp_parse_positive_integer_whitespace(
            if (*(*incomplete).port).realptr.is_null() {
                ((*incomplete).port as *mut libc::c_uchar)
                    .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
            } else {
                (*(*incomplete).port).realptr
            },
            (*(*incomplete).port).len,
            10 as libc::c_int,
        );
        if port_parsed < 0 as libc::c_int as libc::c_long {
            // Failed to parse the port number.
            (*normalized).port_number = -(1 as libc::c_int);
            (*tx).flags |= Flags::HTP_HOSTU_INVALID
        } else if port_parsed > 0 as libc::c_int as libc::c_long
            && port_parsed < 65536 as libc::c_int as libc::c_long
        {
            // Valid port number.
            (*normalized).port_number = port_parsed as libc::c_int
        } else {
            // Port number out of range.
            (*normalized).port_number = -(1 as libc::c_int);
            (*tx).flags |= Flags::HTP_HOSTU_INVALID
        }
    } else {
        (*normalized).port_number = -(1 as libc::c_int)
    }
    // Path.
    if !(*incomplete).path.is_null() {
        // Make a copy of the path, so that we can work on it.
        (*normalized).path = bstr::bstr_dup((*incomplete).path);
        if (*normalized).path.is_null() {
            return -(1 as libc::c_int);
        }
        // Decode URL-encoded (and %u-encoded) characters, as well as lowercase,
        // compress separators and convert backslashes.
        htp_decode_path_inplace(tx, (*normalized).path);
        // Handle UTF-8 in the path.
        if (*(*tx).cfg).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
            .utf8_convert_bestfit
            != 0
        {
            // Decode Unicode characters into a single-byte stream, using best-fit mapping.
            htp_utf8_decode_path_inplace((*tx).cfg, tx, (*normalized).path);
        } else {
            // No decoding, but try to validate the path as a UTF-8 stream.
            htp_utf8_validate_path(tx, (*normalized).path);
        }
        // RFC normalization.
        htp_normalize_uri_path_inplace((*normalized).path);
    }
    // Query string.
    if !(*incomplete).query.is_null() {
        (*normalized).query = bstr::bstr_dup((*incomplete).query);
        if (*normalized).query.is_null() {
            return -(1 as libc::c_int);
        }
    }
    // Fragment.
    if !(*incomplete).fragment.is_null() {
        (*normalized).fragment = bstr::bstr_dup((*incomplete).fragment);
        if (*normalized).fragment.is_null() {
            return -(1 as libc::c_int);
        }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).fragment);
    }
    return 1 as libc::c_int;
}

/// Normalize request hostname. Convert all characters to lowercase and
/// remove trailing dots from the end, if present.
///
/// Returns Normalized hostname.
pub unsafe fn htp_normalize_hostname_inplace(mut hostname: *mut bstr::bstr_t) -> *mut bstr::bstr_t {
    if hostname.is_null() {
        return 0 as *mut bstr::bstr_t;
    }
    bstr::bstr_to_lowercase(hostname);
    // Remove dots from the end of the string.
    while bstr::bstr_char_at_end(hostname, 0 as libc::c_int as size_t) == '.' as i32 {
        bstr::bstr_chop(hostname);
    }
    return hostname;
}

/// Normalize URL path. This function implements the remove dot segments algorithm
/// specified in RFC 3986, section 5.2.4.
pub unsafe fn htp_normalize_uri_path_inplace(mut s: *mut bstr::bstr_t) {
    if s.is_null() {
        return;
    }
    let mut data: *mut libc::c_uchar = if (*s).realptr.is_null() {
        (s as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*s).realptr
    };
    if data.is_null() {
        return;
    }
    let mut len: size_t = (*s).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut c: libc::c_int = -(1 as libc::c_int);
    while rpos < len && wpos < len {
        if c == -(1 as libc::c_int) {
            let fresh9 = rpos;
            rpos = rpos.wrapping_add(1);
            c = *data.offset(fresh9 as isize) as libc::c_int
        }
        // A. If the input buffer begins with a prefix of "../" or "./",
        //    then remove that prefix from the input buffer; otherwise,
        if c == '.' as i32 {
            if rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len
                && *data.offset(rpos as isize) as libc::c_int == '.' as i32
                && *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '/' as i32
            {
                c = -(1 as libc::c_int);
                rpos = (rpos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                continue;
            } else if rpos < len && *data.offset(rpos as isize) as libc::c_int == '/' as i32 {
                c = -(1 as libc::c_int);
                rpos = (rpos as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                continue;
            }
        }
        if c == '/' as i32 {
            // B. if the input buffer begins with a prefix of "/./" or "/.",
            //    where "." is a complete path segment, then replace that
            //    prefix with "/" in the input buffer; otherwise,
            if rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len
                && *data.offset(rpos as isize) as libc::c_int == '.' as i32
                && *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '/' as i32
            {
                c = '/' as i32;
                rpos = (rpos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                continue;
            } else if rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) == len
                && *data.offset(rpos as isize) as libc::c_int == '.' as i32
            {
                c = '/' as i32;
                rpos = (rpos as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                continue;
            } else if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) < len
                && *data.offset(rpos as isize) as libc::c_int == '.' as i32
                && *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '.' as i32
                && *data.offset(rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '/' as i32
            {
                c = '/' as i32;
                rpos = (rpos as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                // C. if the input buffer begins with a prefix of "/../" or "/..",
                //    where ".." is a complete path segment, then replace that
                //    prefix with "/" in the input buffer and remove the last
                //    segment and its preceding "/" (if any) from the output
                //    buffer; otherwise,
                // Remove the last segment
                while wpos > 0 as libc::c_int as libc::c_ulong
                    && *data.offset(wpos.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        != '/' as i32
                {
                    wpos = wpos.wrapping_sub(1)
                }
                if wpos > 0 as libc::c_int as libc::c_ulong {
                    wpos = wpos.wrapping_sub(1)
                }
                continue;
            } else if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) == len
                && *data.offset(rpos as isize) as libc::c_int == '.' as i32
                && *data.offset(rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '.' as i32
            {
                c = '/' as i32;
                rpos = (rpos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                // Remove the last segment
                while wpos > 0 as libc::c_int as libc::c_ulong
                    && *data.offset(wpos.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        != '/' as i32
                {
                    wpos = wpos.wrapping_sub(1)
                }
                if wpos > 0 as libc::c_int as libc::c_ulong {
                    wpos = wpos.wrapping_sub(1)
                }
                continue;
            }
        }
        // D.  if the input buffer consists only of "." or "..", then remove
        // that from the input buffer; otherwise,
        if c == '.' as i32 && rpos == len {
            rpos = rpos.wrapping_add(1)
        } else if c == '.' as i32
            && rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) == len
            && *data.offset(rpos as isize) as libc::c_int == '.' as i32
        {
            rpos = (rpos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        } else {
            // E.  move the first path segment in the input buffer to the end of
            // the output buffer, including the initial "/" character (if
            // any) and any subsequent characters up to, but not including,
            // the next "/" character or the end of the input buffer.
            let fresh10 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh10 as isize) = c as libc::c_uchar;
            while rpos < len
                && *data.offset(rpos as isize) as libc::c_int != '/' as i32
                && wpos < len
            {
                let fresh11 = rpos;
                rpos = rpos.wrapping_add(1);
                let fresh12 = wpos;
                wpos = wpos.wrapping_add(1);
                *data.offset(fresh12 as isize) = *data.offset(fresh11 as isize)
            }
            c = -(1 as libc::c_int)
        }
    }
    bstr::bstr_adjust_len(s, wpos);
}

pub unsafe fn htp_connp_in_state_as_string(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> *mut libc::c_char {
    if connp.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_IDLE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_IDLE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_LINE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_LINE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_PROTOCOL
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_PROTOCOL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_HEADERS
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_HEADERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_CONNECT_CHECK
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_CONNECT_CHECK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_CONNECT_WAIT_RESPONSE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_CONNECT_WAIT_RESPONSE\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_BODY_DETERMINE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_BODY_DETERMINE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_BODY_IDENTITY
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_BODY_IDENTITY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_BODY_CHUNKED_LENGTH
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_BODY_CHUNKED_LENGTH\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_BODY_CHUNKED_DATA
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_BODY_CHUNKED_DATA\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_BODY_CHUNKED_DATA_END
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_BODY_CHUNKED_DATA_END\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_FINALIZE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_FINALIZE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).in_state
        == Some(
            htp_request::htp_connp_REQ_IGNORE_DATA_AFTER_HTTP_0_9
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"REQ_IGNORE_DATA_AFTER_HTTP_0_9\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    return b"UNKNOWN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}

pub unsafe fn htp_connp_out_state_as_string(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> *mut libc::c_char {
    if connp.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_IDLE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_IDLE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_LINE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_LINE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_HEADERS
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_HEADERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_BODY_DETERMINE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_BODY_DETERMINE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_BODY_IDENTITY_CL_KNOWN
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_BODY_IDENTITY_CL_KNOWN\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_BODY_IDENTITY_STREAM_CLOSE\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_BODY_CHUNKED_LENGTH
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_BODY_CHUNKED_LENGTH\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_BODY_CHUNKED_DATA
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_BODY_CHUNKED_DATA\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_BODY_CHUNKED_DATA_END
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_BODY_CHUNKED_DATA_END\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*connp).out_state
        == Some(
            htp_response::htp_connp_RES_FINALIZE
                as unsafe extern "C" fn(_: *mut htp_connection_parser::htp_connp_t) -> Status,
        )
    {
        return b"RES_BODY_FINALIZE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return b"UNKNOWN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}

pub unsafe fn htp_tx_request_progress_as_string(
    mut tx: *mut htp_transaction::htp_tx_t,
) -> *mut libc::c_char {
    if tx.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    match (*tx).request_progress as libc::c_uint {
        0 => return b"NOT_STARTED\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 => return b"REQ_LINE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"REQ_HEADERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => return b"REQ_BODY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 => return b"REQ_TRAILER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 => return b"COMPLETE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => {}
    }
    return b"INVALID\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}

pub unsafe fn htp_tx_response_progress_as_string(
    mut tx: *mut htp_transaction::htp_tx_t,
) -> *mut libc::c_char {
    if tx.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    match (*tx).response_progress as libc::c_uint {
        0 => return b"NOT_STARTED\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 => return b"RES_LINE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"RES_HEADERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => return b"RES_BODY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 => return b"RES_TRAILER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 => return b"COMPLETE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => {}
    }
    return b"INVALID\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}

pub unsafe fn htp_unparse_uri_noencode(mut uri: *mut htp_uri_t) -> *mut bstr::bstr_t {
    if uri.is_null() {
        return 0 as *mut bstr::bstr_t;
    }
    // On the first pass determine the length of the final string
    let mut len: size_t = 0 as libc::c_int as size_t;
    if !(*uri).scheme.is_null() {
        len = (len as libc::c_ulong).wrapping_add((*(*uri).scheme).len) as size_t as size_t;
        len = (len as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
            as size_t
        // "://"
    }
    if !(*uri).username.is_null() || !(*uri).password.is_null() {
        if !(*uri).username.is_null() {
            len = (len as libc::c_ulong).wrapping_add((*(*uri).username).len) as size_t as size_t
        }
        // "@"
        len = (len as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
            as size_t; // ":"
        if !(*uri).password.is_null() {
            len = (len as libc::c_ulong).wrapping_add((*(*uri).password).len) as size_t as size_t
        } // ":"
        len = (len as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    } // "?"
    if !(*uri).hostname.is_null() {
        len = (len as libc::c_ulong).wrapping_add((*(*uri).hostname).len) as size_t as size_t
    } // "#"
    if !(*uri).port.is_null() {
        len = (len as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        len = (len as libc::c_ulong).wrapping_add((*(*uri).port).len) as size_t as size_t
    }
    if !(*uri).path.is_null() {
        len = (len as libc::c_ulong).wrapping_add((*(*uri).path).len) as size_t as size_t
    }
    if !(*uri).query.is_null() {
        len = (len as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        len = (len as libc::c_ulong).wrapping_add((*(*uri).query).len) as size_t as size_t
    }
    if !(*uri).fragment.is_null() {
        len = (len as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        len = (len as libc::c_ulong).wrapping_add((*(*uri).fragment).len) as size_t as size_t
    }
    // On the second pass construct the string
    let mut r: *mut bstr::bstr = bstr::bstr_alloc(len);
    if r.is_null() {
        return 0 as *mut bstr::bstr_t;
    }
    if !(*uri).scheme.is_null() {
        bstr::bstr_add_noex(r, (*uri).scheme);
        bstr::bstr_add_c_noex(r, b"://\x00" as *const u8 as *const libc::c_char);
    }
    if !(*uri).username.is_null() || !(*uri).password.is_null() {
        if !(*uri).username.is_null() {
            bstr::bstr_add_noex(r, (*uri).username);
        }
        bstr::bstr_add_c_noex(r, b":\x00" as *const u8 as *const libc::c_char);
        if !(*uri).password.is_null() {
            bstr::bstr_add_noex(r, (*uri).password);
        }
        bstr::bstr_add_c_noex(r, b"@\x00" as *const u8 as *const libc::c_char);
    }
    if !(*uri).hostname.is_null() {
        bstr::bstr_add_noex(r, (*uri).hostname);
    }
    if !(*uri).port.is_null() {
        bstr::bstr_add_c_noex(r, b":\x00" as *const u8 as *const libc::c_char);
        bstr::bstr_add_noex(r, (*uri).port);
    }
    if !(*uri).path.is_null() {
        bstr::bstr_add_noex(r, (*uri).path);
    }
    if !(*uri).query.is_null() {
        bstr::bstr_add_c_noex(r, b"?\x00" as *const u8 as *const libc::c_char);
        bstr::bstr_add_noex(r, (*uri).query);

        //bstr::bstr *query = bstr::bstr_dup(uri->query);
        //if (query == NULL) {
        //    bstr::bstr_free(r);
        //    return NULL;
        //}

        //htp_uriencoding_normalize_inplace(query);

        //bstr::bstr_add_c_noex(r, "?");
        //bstr::bstr_add_noex(r, query);

        //bstr::bstr_free(query);
    }
    if !(*uri).fragment.is_null() {
        bstr::bstr_add_c_noex(r, b"#\x00" as *const u8 as *const libc::c_char);
        bstr::bstr_add_noex(r, (*uri).fragment);
    }
    return r;
}

/// Determine if the information provided on the response line
/// is good enough. Browsers are lax when it comes to response
/// line parsing. In most cases they will only look for the
/// words "http" at the beginning.
///
/// Returns 1 for good enough or 0 for not good enough
pub unsafe fn htp_treat_response_line_as_body(
    mut data: *const uint8_t,
    mut len: size_t,
) -> libc::c_int {
    // Browser behavior:
    //      Firefox 3.5.x: (?i)^\s*http
    //      IE: (?i)^\s*http\s*/
    //      Safari: ^HTTP/\d+\.\d+\s+\d{3}
    let mut pos: size_t = 0 as libc::c_int as size_t;
    if data.is_null() {
        return 1 as libc::c_int;
    }
    while pos < len
        && (htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0
            || *data.offset(pos as isize) as libc::c_int == 0 as libc::c_int)
    {
        pos = pos.wrapping_add(1)
    }
    if len < pos.wrapping_add(4 as libc::c_int as libc::c_ulong) {
        return 1 as libc::c_int;
    }
    if *data.offset(pos as isize) as libc::c_int != 'H' as i32
        && *data.offset(pos as isize) as libc::c_int != 'h' as i32
    {
        return 1 as libc::c_int;
    }
    if *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
        != 'T' as i32
        && *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
            != 't' as i32
    {
        return 1 as libc::c_int;
    }
    if *data.offset(pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
        != 'T' as i32
        && *data.offset(pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
            != 't' as i32
    {
        return 1 as libc::c_int;
    }
    if *data.offset(pos.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
        != 'P' as i32
        && *data.offset(pos.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
            != 'p' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

/// Run the REQUEST_BODY_DATA hook.
pub unsafe fn htp_req_run_hook_body_data(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut d: *mut htp_transaction::htp_tx_data_t,
) -> Status {
    // Do not invoke callbacks with an empty data chunk
    if !(*d).data.is_null() && (*d).len == 0 as libc::c_int as libc::c_ulong {
        return Status::OK;
    }
    // Do not invoke callbacks without a transaction.
    if (*connp).in_tx.is_null() {
        return Status::OK;
    }
    // Run transaction hooks first
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*connp).in_tx).hook_request_body_data,
        d as *mut libc::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // Run configuration hooks second
    rc = htp_hooks::htp_hook_run_all(
        (*(*connp).cfg).hook_request_body_data,
        d as *mut libc::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // On PUT requests, treat request body as file
    if !(*connp).put_file.is_null() {
        let mut file_data: htp_file_data_t = htp_file_data_t {
            file: 0 as *mut htp_file_t,
            data: 0 as *const libc::c_uchar,
            len: 0,
        };
        file_data.data = (*d).data;
        file_data.len = (*d).len;
        file_data.file = (*connp).put_file;
        (*file_data.file).len =
            ((*file_data.file).len as libc::c_ulong).wrapping_add((*d).len) as int64_t as int64_t;
        rc = htp_hooks::htp_hook_run_all(
            (*(*connp).cfg).hook_request_file_data,
            &mut file_data as *mut htp_file_data_t as *mut libc::c_void,
        );
        if rc != Status::OK {
            return rc;
        }
    }
    return Status::OK;
}

/// Run the RESPONSE_BODY_DATA hook.
pub unsafe fn htp_res_run_hook_body_data(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut d: *mut htp_transaction::htp_tx_data_t,
) -> Status {
    // Do not invoke callbacks with an empty data chunk.
    if !(*d).data.is_null() && (*d).len == 0 as libc::c_int as libc::c_ulong {
        return Status::OK;
    }
    // Run transaction hooks first
    let mut rc: Status = htp_hooks::htp_hook_run_all(
        (*(*connp).out_tx).hook_response_body_data,
        d as *mut libc::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // Run configuration hooks second
    rc = htp_hooks::htp_hook_run_all(
        (*(*connp).cfg).hook_response_body_data,
        d as *mut libc::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    return Status::OK;
}

/// Parses the provided memory region, extracting the double-quoted string.
///
/// Returns HTP_OK on success, HTP_DECLINED if the input is not well formed,
///         and HTP_ERROR on fatal errors.
pub unsafe fn htp_extract_quoted_string_as_bstr(
    mut data: *mut libc::c_uchar,
    mut len: size_t,
    mut out: *mut *mut bstr::bstr_t,
    mut endoffset: *mut size_t,
) -> Status {
    if data.is_null() || out.is_null() {
        return Status::ERROR;
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return Status::DECLINED;
    }
    let mut pos: size_t = 0 as libc::c_int as size_t;
    // Check that the first character is a double quote.
    if *data.offset(pos as isize) as libc::c_int != '\"' as i32 {
        return Status::DECLINED;
    }
    // Step over the double quote.
    pos = pos.wrapping_add(1);
    if pos == len {
        return Status::DECLINED;
    }
    // Calculate the length of the resulting string.
    let mut escaped_chars: size_t = 0 as libc::c_int as size_t;
    while pos < len {
        if *data.offset(pos as isize) as libc::c_int == '\\' as i32 {
            if pos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len {
                escaped_chars = escaped_chars.wrapping_add(1);
                pos = (pos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                continue;
            }
        } else if *data.offset(pos as isize) as libc::c_int == '\"' as i32 {
            break;
        }
        pos = pos.wrapping_add(1)
    }
    // Have we reached the end of input without seeing the terminating double quote?
    if pos == len {
        return Status::DECLINED;
    }
    // Copy the data and unescape it as necessary.
    let mut outlen: size_t = pos
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_sub(escaped_chars);
    *out = bstr::bstr_alloc(outlen);
    if (*out).is_null() {
        return Status::ERROR;
    }
    let mut outptr: *mut libc::c_uchar = if (**out).realptr.is_null() {
        (*out as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (**out).realptr
    };
    let mut outpos: size_t = 0 as libc::c_int as size_t;
    pos = 1 as libc::c_int as size_t;
    while pos < len && outpos < outlen {
        // TODO We are not properly unescaping test here, we're only
        //      handling escaped double quotes.
        if *data.offset(pos as isize) as libc::c_int == '\\' as i32 {
            if pos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len {
                let fresh17 = outpos;
                outpos = outpos.wrapping_add(1);
                *outptr.offset(fresh17 as isize) =
                    *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                pos = (pos as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                continue;
            }
        } else if *data.offset(pos as isize) as libc::c_int == '\"' as i32 {
            break;
        }
        let fresh18 = pos;
        pos = pos.wrapping_add(1);
        let fresh19 = outpos;
        outpos = outpos.wrapping_add(1);
        *outptr.offset(fresh19 as isize) = *data.offset(fresh18 as isize)
    }
    bstr::bstr_adjust_len(*out, outlen);
    if !endoffset.is_null() {
        *endoffset = pos
    }
    return Status::OK;
}

pub unsafe fn htp_parse_ct_header(
    mut header: *mut bstr::bstr_t,
    mut ct: *mut *mut bstr::bstr_t,
) -> Status {
    if header.is_null() || ct.is_null() {
        return Status::ERROR;
    }
    let mut data: *mut libc::c_uchar = if (*header).realptr.is_null() {
        (header as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*header).realptr
    };
    let mut len: size_t = (*header).len;
    // The assumption here is that the header value we receive
    // here has been left-trimmed, which means the starting position
    // is on the media type. On some platforms that may not be the
    // case, and we may need to do the left-trim ourselves.
    // Find the end of the MIME type, using the same approach PHP 5.4.3 uses.
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len
        && *data.offset(pos as isize) as libc::c_int != ';' as i32
        && *data.offset(pos as isize) as libc::c_int != ',' as i32
        && *data.offset(pos as isize) as libc::c_int != ' ' as i32
    {
        pos = pos.wrapping_add(1)
    }
    *ct = bstr::bstr_dup_ex(header, 0 as libc::c_int as size_t, pos);
    if (*ct).is_null() {
        return Status::ERROR;
    }
    bstr::bstr_to_lowercase(*ct);
    return Status::OK;
}

/// Implements relaxed (not strictly RFC) hostname validation.
///
/// Returns 1 if the supplied hostname is valid; 0 if it is not.
pub unsafe fn htp_validate_hostname(mut hostname: *mut bstr::bstr_t) -> libc::c_int {
    let mut data: *mut libc::c_uchar = if (*hostname).realptr.is_null() {
        (hostname as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*hostname).realptr
    };
    let mut len: size_t = (*hostname).len;
    let mut startpos: size_t = 0 as libc::c_int as size_t;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    if len == 0 as libc::c_int as libc::c_ulong || len > 255 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while pos < len {
        // Validate label characters.
        startpos = pos;
        while pos < len && *data.offset(pos as isize) as libc::c_int != '.' as i32 {
            let mut c: libc::c_uchar = *data.offset(pos as isize);
            // Exactly one dot expected.
            // According to the RFC, the underscore is not allowed in a label, but
            // we allow it here because we think it's often seen in practice.
            if !(c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
                || c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32
                || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
                || c as libc::c_int == '-' as i32
                || c as libc::c_int == '_' as i32)
            {
                return 0 as libc::c_int;
            }
            pos = pos.wrapping_add(1)
        }
        if pos.wrapping_sub(startpos) == 0 as libc::c_int as libc::c_ulong
            || pos.wrapping_sub(startpos) > 63 as libc::c_int as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        if pos >= len {
            return 1 as libc::c_int;
        }
        startpos = pos;
        while pos < len && *data.offset(pos as isize) as libc::c_int == '.' as i32 {
            pos = pos.wrapping_add(1)
        }
        if pos.wrapping_sub(startpos) != 1 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}

/// Frees all data contained in the uri, and then the uri itself.
pub unsafe fn htp_uri_free(mut uri: *mut htp_uri_t) {
    if uri.is_null() {
        return;
    }
    bstr::bstr_free((*uri).scheme);
    bstr::bstr_free((*uri).username);
    bstr::bstr_free((*uri).password);
    bstr::bstr_free((*uri).hostname);
    bstr::bstr_free((*uri).port);
    bstr::bstr_free((*uri).path);
    bstr::bstr_free((*uri).query);
    bstr::bstr_free((*uri).fragment);
    free(uri as *mut libc::c_void);
}

/// Allocates and initializes a new htp_uri_t structure.
///
/// Returns New structure, or NULL on memory allocation failure.
pub unsafe fn htp_uri_alloc() -> *mut htp_uri_t {
    let mut u: *mut htp_uri_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_uri_t>() as libc::c_ulong,
    ) as *mut htp_uri_t;
    if u.is_null() {
        return 0 as *mut htp_uri_t;
    }
    (*u).port_number = -(1 as libc::c_int);
    return u;
}

/// Returns the LibHTP version string.
pub unsafe fn htp_get_version() -> *const libc::c_char {
    HTP_VERSION_STRING_FULL.as_ptr() as *const libc::c_char
}
