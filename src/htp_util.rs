use crate::htp_config::htp_decoder_cfg_t;
use crate::htp_config::htp_url_encoding_handling_t;
use crate::{
    bstr, htp_config, htp_connection_parser, htp_hooks, htp_request, htp_transaction, utf8_decoder,
    Status,
};
use bitflags;
use nom::{
    branch::alt,
    bytes::complete::{
        is_not, tag, tag_no_case, take, take_till, take_until, take_while, take_while1,
        take_while_m_n,
    },
    character::complete::{char, digit1},
    character::is_space,
    combinator::{map, not, opt, peek},
    multi::{fold_many0, many0},
    number::complete::be_u8,
    sequence::tuple,
    IResult,
};
use std::cmp::Ordering;

pub const HTP_VERSION_STRING_FULL: &'static str =
    concat!("LibHTP v", env!("CARGO_PKG_VERSION"), "\x00");

// Various flag bits. Even though we have a flag field in several places
// (header, transaction, connection), these fields are all in the same namespace
// because we may want to set the same flag in several locations. For example, we
// may set HTP_FIELD_FOLDED on the actual folded header, but also on the transaction
// that contains the header. Both uses are useful.

// Connection flags are 8 bits wide.
bitflags::bitflags! {
    pub struct ConnectionFlags: u8 {
        const HTP_CONN_UNKNOWN        = 0x00;
        const HTP_CONN_PIPELINED      = 0x01;
        const HTP_CONN_HTTP_0_9_EXTRA = 0x02;
    }
}

// All other flags are 64 bits wide.
bitflags::bitflags! {
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
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum htp_file_source_t {
    HTP_FILE_MULTIPART = 1,
    HTP_FILE_PUT = 2,
}

/// Used to represent files that are seen during the processing of HTTP traffic. Most
/// commonly this refers to files seen in multipart/form-data payloads. In addition, PUT
/// request bodies can be treated as files.
#[derive(Copy, Clone)]
pub struct htp_file_t {
    /// Where did this file come from? Possible values: HTP_FILE_MULTIPART and HTP_FILE_PUT.
    pub source: htp_file_source_t,
    /// File name, as provided (e.g., in the Content-Disposition multipart part header.
    pub filename: *mut bstr::bstr_t,
    /// File length.
    pub len: usize,
    /// The unique filename in which this file is stored on the filesystem, when applicable.
    pub tmpname: *mut i8,
    /// The file descriptor used for external storage, or -1 if unused.
    pub fd: i32,
}

/// URI structure. Each of the fields provides access to a single
/// URI element. Where an element is not present in a URI, the
/// corresponding field will be set to NULL or -1, depending on the
/// field type.
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
    pub port_number: i32,
    /// The path part of this URI.
    pub path: *mut bstr::bstr_t,
    /// Query string.
    pub query: *mut bstr::bstr_t,
    /// Fragment identifier. This field will rarely be available in a server-side
    /// setting, but it's not impossible to see it.
    pub fragment: *mut bstr::bstr_t,
}

pub struct uri_t<'a> {
    pub scheme: Option<&'a [u8]>,
    pub username: Option<&'a [u8]>,
    pub password: Option<&'a [u8]>,
    pub hostname: Option<&'a [u8]>,
    pub port: Option<&'a [u8]>,
    pub path: Option<&'a [u8]>,
    pub query: Option<&'a [u8]>,
    pub fragment: Option<&'a [u8]>,
}

/// Represents a chunk of file data.
#[derive(Copy, Clone)]
pub struct htp_file_data_t {
    /// File information.
    pub file: *mut htp_file_t,
    /// Pointer to the data buffer.
    pub data: *const u8,
    /// Buffer length.
    pub len: usize,
}

/// Is character a linear white space character?
///
/// Returns true or false
pub fn htp_is_lws(c: u8) -> bool {
    match c as char {
        ' ' | '\t' => true,
        _ => false,
    }
}

/// Is character a separator character?
///
/// Returns true or false
pub fn htp_is_separator(c: u8) -> bool {
    // separators = "(" | ")" | "<" | ">" | "@"
    // | "," | ";" | ":" | "\" | <">
    // | "/" | "[" | "]" | "?" | "="
    // | "{" | "}" | SP | HT
    match c as char {
        '(' | ')' | '<' | '>' | '@' | ',' | ';' | ':' | '\\' | '"' | '/' | '[' | ']' | '?'
        | '=' | '{' | '}' | ' ' | '\t' => true,
        _ => false,
    }
}

/// Is character a text character?
///
/// Returns 0 or 1
pub unsafe fn htp_is_text(c: i32) -> i32 {
    if c == '\t' as i32 {
        return 1;
    }
    if c < 32 {
        return 0;
    }
    1
}

/// Is character a token character?
///
/// Returns true or false
pub fn htp_is_token(c: u8) -> bool {
    // token = 1*<any CHAR except CTLs or separators>
    // CHAR  = <any US-ASCII character (octets 0 - 127)>
    if c < 32 || c > 126 {
        return false;
    }
    if htp_is_separator(c) {
        return false;
    }
    true
}

pub fn take_ascii_whitespace<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| take_while(|c: u8| c.is_ascii_whitespace())(input)
}

/// Remove all line terminators (LF, CR or CRLF) from
/// the end of the line provided as input.
///
/// Returns a slice with all line terminators removed
pub fn htp_chomp<'a>(mut data: &'a [u8]) -> &'a [u8] {
    loop {
        let last_char = data.last();
        if last_char == Some(&('\n' as u8)) || last_char == Some(&('\r' as u8)) {
            data = &data[..data.len() - 1];
        } else {
            break;
        }
    }
    data
}

/// Is character a white space character?
///
/// Returns true or false
pub fn htp_is_space(c: u8) -> bool {
    match c as char {
        ' ' | '\t' | '\r' | '\n' | '\x0b' | '\x0c' => true,
        _ => false,
    }
}

/// Helper function that mimics the functionality of bytes::complete::take_until, ignoring tag case
/// Returns the longest input slice till it case insensitively matches the pattern. It doesn't consume the pattern.
///
/// Returns a tuple of the unconsumed data and the data up to but not including the input tag (if present)
pub fn take_until_no_case<'a>(tag: &'a [u8]) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        if tag.len() == 0 {
            return Ok((b"", input));
        }
        let mut new_input = input;
        let mut bytes_consumed: usize = 0;
        while new_input.len() > 0 {
            let (left, consumed) = take_till::<_, _, (&[u8], nom::error::ErrorKind)>(|c: u8| {
                c.to_ascii_lowercase() == tag[0] || c.to_ascii_uppercase() == tag[0]
            })(new_input)?;
            new_input = left;
            bytes_consumed = bytes_consumed.wrapping_add(consumed.len());
            if tag_no_case::<_, _, (&[u8], nom::error::ErrorKind)>(tag)(new_input).is_ok() {
                return Ok((new_input, &input[..bytes_consumed]));
            } else if let Ok((left, consumed)) =
                take::<_, _, (&[u8], nom::error::ErrorKind)>(1usize)(new_input)
            {
                bytes_consumed = bytes_consumed.wrapping_add(consumed.len());
                new_input = left;
            }
        }
        Ok((b"", input))
    }
}
/// Converts request method, given as a string, into a number.
///
/// Returns Method or M_UNKNOWN
pub fn htp_convert_bstr_to_method(method: &bstr::bstr_t) -> htp_request::htp_method_t {
    // TODO Optimize using parallel matching, or something similar.
    if method.cmp("GET") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_GET;
    }
    if method.cmp("PUT") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_PUT;
    }
    if method.cmp("POST") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_POST;
    }
    if method.cmp("DELETE") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_DELETE;
    }
    if method.cmp("CONNECT") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_CONNECT;
    }
    if method.cmp("OPTIONS") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_OPTIONS;
    }
    if method.cmp("TRACE") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_TRACE;
    }
    if method.cmp("PATCH") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_PATCH;
    }
    if method.cmp("PROPFIND") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_PROPFIND;
    }
    if method.cmp("PROPPATCH") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_PROPPATCH;
    }
    if method.cmp("MKCOL") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_MKCOL;
    }
    if method.cmp("COPY") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_COPY;
    }
    if method.cmp("MOVE") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_MOVE;
    }
    if method.cmp("LOCK") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_LOCK;
    }
    if method.cmp("UNLOCK") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_UNLOCK;
    }
    if method.cmp("VERSION-CONTROL") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_VERSION_CONTROL;
    }
    if method.cmp("CHECKOUT") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_CHECKOUT;
    }
    if method.cmp("UNCHECKOUT") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_UNCHECKOUT;
    }
    if method.cmp("CHECKIN") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_CHECKIN;
    }
    if method.cmp("UPDATE") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_UPDATE;
    }
    if method.cmp("LABEL") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_LABEL;
    }
    if method.cmp("REPORT") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_REPORT;
    }
    if method.cmp("MKWORKSPACE") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_MKWORKSPACE;
    }
    if method.cmp("MKACTIVITY") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_MKACTIVITY;
    }
    if method.cmp("BASELINE-CONTROL") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_BASELINE_CONTROL;
    }
    if method.cmp("MERGE") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_MERGE;
    }
    if method.cmp("INVALID") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_INVALID;
    }
    if method.cmp("HEAD") == Ordering::Equal {
        return htp_request::htp_method_t::HTP_M_HEAD;
    }
    htp_request::htp_method_t::HTP_M_UNKNOWN
}

/// Is the given line empty?
///
/// Returns true or false
pub fn htp_is_line_empty(data: &[u8]) -> bool {
    match data {
        b"\x0d" | b"\x0a" | b"\x0d\x0a" => true,
        _ => false,
    }
}

/// Does line consist entirely of whitespace characters?
///
/// Returns bool
pub fn htp_is_line_whitespace(data: &[u8]) -> bool {
    for c in data {
        if !htp_is_space(*c) {
            return false;
        }
    }
    true
}

/// Searches for and extracts the next set of ascii digits from the input slice if present
/// Parses over leading and trailing LWS characters.
///
/// Returns (any trailing non-LWS characters, (non-LWS leading characters, ascii digits))
pub fn ascii_digits<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (&'a [u8], &'a [u8])> {
    move |input| {
        map(
            tuple((
                take_while(|c| is_space(c)),
                take_till(|c: u8| c.is_ascii_digit()),
                digit1,
                take_while(|c| is_space(c)),
            )),
            |(_, leading_data, digits, _)| (leading_data, digits),
        )(input)
    }
}

/// Searches for and extracts the next set of hex digits from the input slice if present
/// Parses over leading and trailing LWS characters.
///
/// Returns a tuple of any trailing non-LWS characters and the found hex digits
fn hex_digits<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        map(
            tuple((
                take_while(|c| is_space(c)),
                take_while1(|c: u8| c.is_ascii_hexdigit()),
                take_while(|c| is_space(c)),
            )),
            |(_, digits, _)| digits,
        )(input)
    }
}

/// Parses Content-Length string (positive decimal number).
/// White space is allowed before and after the number.
///
/// Returns Content-Length as a number or None if parsing failed.
pub fn htp_parse_content_length<'a>(
    input: &'a [u8],
    connp: Option<&mut htp_connection_parser::htp_connp_t>,
) -> Option<i64> {
    if let Ok((trailing_data, (leading_data, content_length))) = ascii_digits()(input) {
        if let Some(connp) = connp {
            if leading_data.len() > 0 {
                // Contains invalid characters! But still attempt to process
                unsafe {
                    htp_warn!(
                        connp as *mut htp_connection_parser::htp_connp_t,
                        htp_log_code::CONTENT_LENGTH_EXTRA_DATA_START,
                        "C-L value with extra data in the beginning"
                    );
                };
            }

            if trailing_data.len() > 0 {
                // Ok to have junk afterwards
                unsafe {
                    htp_warn!(
                        connp as *mut htp_connection_parser::htp_connp_t,
                        htp_log_code::CONTENT_LENGTH_EXTRA_DATA_END,
                        "C-L value with extra data in the end"
                    );
                };
            }
        }
        if let Ok(content_length) = std::str::from_utf8(content_length) {
            if let Ok(content_length) = i64::from_str_radix(content_length, 10) {
                return Some(content_length);
            }
        }
    }
    None
}

/// Parses chunk length (positive hexadecimal number). White space is allowed before
/// and after the number.
///
/// Returns a chunked_length or None if empty.
pub fn htp_parse_chunked_length<'a>(input: &'a [u8]) -> Result<Option<i32>, &'static str> {
    if let Ok((trailing_data, chunked_length)) = hex_digits()(input) {
        if trailing_data.len() == 0 && chunked_length.len() == 0 {
            return Ok(None);
        }
        if let Ok(chunked_length) = std::str::from_utf8(chunked_length) {
            if let Ok(chunked_length) = i32::from_str_radix(chunked_length, 16) {
                return Ok(Some(chunked_length));
            }
        }
    }
    Err("Invalid Chunk Length")
}

/// Determines if the given line is a continuation (of some previous line).
///
/// Returns false or true, respectively.
pub fn htp_connp_is_line_folded(data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }
    htp_is_folding_char(data[0])
}

pub fn htp_is_folding_char(c: u8) -> bool {
    if htp_is_lws(c) || c == 0 {
        return true;
    }
    false
}

/// Determines if the given line is a request terminator.
///
/// Returns true or false
pub fn htp_connp_is_line_terminator(
    server_personality: htp_config::htp_server_personality_t,
    data: &[u8],
    next_no_lf: bool,
) -> bool {
    // Is this the end of request headers?
    if server_personality == htp_config::htp_server_personality_t::HTP_SERVER_IIS_5_0 {
        // IIS 5 will accept a whitespace line as a terminator
        if htp_is_line_whitespace(data) {
            return true;
        }
    }

    // Treat an empty line as terminator
    if htp_is_line_empty(data) {
        return true;
    }
    if data.len() == 2 && htp_is_lws(data[0]) && data[1] == '\n' as u8 {
        return next_no_lf;
    }
    false
}

/// Determines if the given line can be ignored when it appears before a request.
///
/// Returns true or false
pub fn htp_connp_is_line_ignorable(
    server_personality: htp_config::htp_server_personality_t,
    data: &[u8],
) -> bool {
    htp_connp_is_line_terminator(server_personality, data, false)
}

/// Attempts to convert the provided port slice to a u16
///
/// Returns port number if a valid one is found. None if fails to convert or the result is 0
fn convert_port(port: &[u8]) -> Option<u16> {
    if port.len() == 0 {
        return None;
    }
    if let Ok(res) = std::str::from_utf8(port) {
        if let Ok(port_number) = u16::from_str_radix(res, 10) {
            if port_number == 0 {
                return None;
            }
            return Some(port_number);
        }
    }
    None
}

/// Parses an authority string, which consists of a hostname with an optional port number
///
/// Returns a remaining unparsed data, parsed hostname, parsed port, converted port number,
/// and a flag indicating whether the parsed data is valid
pub fn htp_parse_hostport<'a>(
    hostport: &'a mut bstr::bstr_t,
) -> IResult<&'a [u8], (&'a [u8], Option<(&'a [u8], Option<u16>)>, bool)> {
    let (input, host) = hostname()((hostport).as_slice())?;
    let mut valid = htp_validate_hostname(host);
    if let Ok((_, p)) = port()(input) {
        if let Some(port) = convert_port(p) {
            return Ok((input, (host, Some((p, Some(port))), valid)));
        } else {
            return Ok((input, (host, Some((p, None)), false)));
        }
    } else if input.len() > 0 {
        //Trailing data after the hostname that is invalid e.g. [::1]xxxxx
        valid = false;
    }
    Ok((input, (host, None, valid)))
}

/// Parses hostport provided in the URI.
///
/// Returns HTP_OK on success or HTP_ERROR error.
pub unsafe fn htp_parse_uri_hostport(
    hostport: &mut bstr::bstr_t,
    uri: &mut htp_uri_t,
    flags: &mut Flags,
) -> Status {
    if let Ok((_, (host, port_nmb, mut valid))) = htp_parse_hostport(hostport) {
        (*uri).hostname = bstr::bstr_dup_str(host);
        if (*uri).hostname.is_null() {
            return Status::ERROR;
        }
        bstr::bstr_to_lowercase((*uri).hostname);
        if let Some((port, port_nmb)) = port_nmb {
            (*uri).port = bstr::bstr_dup_str(port);
            if (*uri).port.is_null() {
                bstr::bstr_free((*uri).hostname);
                return Status::ERROR;
            }
            if let Some(port_number) = port_nmb {
                (*uri).port_number = port_number as i32;
            } else {
                valid = false;
            }
        }
        if !valid {
            *flags |= Flags::HTP_HOSTU_INVALID
        }
    }
    Status::OK
}

/// Attempts to extract the scheme from a given input URI.
/// e.g. input: http://user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag
/// e.g. output: (//user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag, http)
///
/// Returns a tuple of the unconsumed data and the matched scheme
pub fn scheme<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        // Scheme test: if it doesn't start with a forward slash character (which it must
        // for the contents to be a path or an authority), then it must be the scheme part
        map(
            tuple((peek(not(tag("/"))), take_until(":"), tag(":"))),
            |(_, scheme, _)| scheme,
        )(input)
    }
}

/// Attempts to extract the credentials from a given input URI, assuming the scheme has already been extracted.
/// e.g. input: //user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag
/// e.g. output: (www.example.com:1234/path1/path2?a=b&c=d#frag, (user, pass))
///
/// Returns a tuple of the remaining unconsumed data and a tuple of the matched username and password
pub fn credentials<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (&'a [u8], Option<&'a [u8]>)> {
    move |input| {
        // Authority test: two forward slash characters and it's an authority.
        // One, three or more slash characters, and it's a path.
        // Note: we only attempt to parse authority if we've seen a scheme.
        let (input, (_, _, credentials, _)) =
            tuple((tag("//"), peek(not(tag("/"))), take_until("@"), tag("@")))(input)?;
        let (password, username) = opt(tuple((take_until(":"), tag(":"))))(credentials)?;
        if let Some((username, _)) = username {
            Ok((input, (username, Some(password))))
        } else {
            Ok((input, (credentials, None)))
        }
    }
}

/// Attempts to extract an IPv6 hostname from a given input URI,
/// assuming any scheme, credentials, hostname, port, and path have been already parsed out.
/// e.g. input: [:::]/path1?a=b&c=d#frag
/// e.g. output: ([:::], /path?a=b&c=d#frag)
///
/// Returns a tuple of the remaining unconsumed data and the matched ipv6 hostname
pub fn ipv6<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| -> IResult<&'a [u8], &'a [u8]> {
        let (rest, (_, _, _)) = tuple((tag("["), is_not("/?#]"), opt(tag("]"))))(input)?;
        Ok((rest, &input[..input.len() - rest.len()]))
    }
}

/// Attempts to extract the hostname from a given input URI
/// e.g. input: http://user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag
/// e.g. output: (:1234/path1/path2?a=b&c=d#frag, www.example.com)
///
/// Returns a tuple of the remaining unconsumed data and the matched hostname
pub fn hostname<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        let (input, mut hostname) = map(
            tuple((
                opt(tag("//")), //If it starts with "//", skip (might have parsed a scheme and no creds)
                peek(not(tag("/"))), //If it starts with '/', this is a path, not a hostname
                many0(tag(" ")),
                alt((ipv6(), is_not("/?#:"))),
            )),
            |(_, _, _, hostname)| hostname,
        )(input)?;
        //There may be spaces in the middle of a hostname, so much trim only at the end
        while hostname.ends_with(&[' ' as u8]) {
            hostname = &hostname[..hostname.len() - 1];
        }
        Ok((input, hostname))
    }
}

/// Attempts to extract the port from a given input URI,
/// assuming any scheme, credentials, or hostname have been already parsed out.
/// e.g. input: :1234/path1/path2?a=b&c=d#frag
/// e.g. output: (/path1/path2?a=b&c=d#frag, 1234)
///
/// Returns a tuple of the remaining unconsumed data and the matched port
pub fn port<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        // Must start with ":" for there to be a port to parse
        let (input, (_, _, port, _)) =
            tuple((tag(":"), many0(tag(" ")), is_not("/?#"), many0(tag(" "))))(input)?;
        let (_, port) = is_not(" ")(port)?; //we assume there never will be a space in the middle of a port
        Ok((input, port))
    }
}

/// Attempts to extract the path from a given input URI,
/// assuming any scheme, credentials, hostname, and port have been already parsed out.
/// e.g. input: /path1/path2?a=b&c=d#frag
/// e.g. output: (?a=b&c=d#frag, /path1/path2)
///
/// Returns a tuple of the remaining unconsumed data and the matched path
pub fn path<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| is_not("#?")(input)
}

/// Attempts to extract the query from a given input URI,
/// assuming any scheme, credentials, hostname, port, and path have been already parsed out.
/// e.g. input: ?a=b&c=d#frag
/// e.g. output: (#frag, ?a=b&c=d)
///
/// Returns a tuple of the remaining unconsumed data and the matched query
pub fn query<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        // Skip the starting '?'
        map(tuple((tag("?"), is_not("#"))), |(_, query)| query)(input)
    }
}

/// Attempts to extract the fragment from a given input URI,
/// assuming any other components have been parsed out
/// e.g. input: ?a=b&c=d#frag
/// e.g. output: ("", #frag)
///
/// Returns a tuple of the remaining unconsumed data and the matched fragment
pub fn fragment<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        // Skip the starting '#'
        let (input, _) = tag("#")(input)?;
        Ok((b"", input))
    }
}

/// Parses request URI, making no attempt to validate the contents.
///
/// It attempts, but is not guaranteed to successfully parse out a scheme, username, password, hostname, port, query, and fragment.
/// If it fails to parse a path, it will return an error.
/// Note: only attempts to extract a username, password, and hostname and subsequently port if it successfully parsed a scheme.
/// e.g. input: "http:://user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag"
/// e.g. output: (Some("http", Some("user", "pass"), Some("www.example.com", Some("1234"))), "/path1/path2", Some("a=b&c=d"), Some("frag"))
///
/// Returns parsed scheme, username, password, hostname, port, path, query and fragment as a tuple containing optional values.
pub fn parse_uri<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], uri_t> {
    move |input| {
        map(
            tuple((
                opt(tuple((
                    scheme(),
                    opt(credentials()),
                    opt(tuple((hostname(), opt(port())))),
                ))),
                opt(path()),
                opt(query()),
                opt(fragment()),
            )),
            |(scheme_authority, path, query, fragment)| {
                let mut uri = uri_t {
                    scheme: None,
                    username: None,
                    password: None,
                    hostname: None,
                    port: None,
                    path,
                    query,
                    fragment,
                };
                if let Some((scheme, authority, hostname_port)) = scheme_authority {
                    uri.scheme = Some(scheme);
                    if let Some((username, password)) = authority {
                        uri.username = Some(username);
                        uri.password = password;
                    }
                    if let Some((hostname, port)) = hostname_port {
                        uri.hostname = Some(hostname);
                        uri.port = port;
                    }
                }
                uri
            },
        )(input)
    }
}

/// Parses request URI, making no attempt to validate the contents.
///
/// Returns HTP_ERROR on memory allocation failure, HTP_OK otherwise
pub unsafe fn htp_parse_uri(input: *mut bstr::bstr_t, mut uri: *mut *mut htp_uri_t) -> Status {
    // Allow a htp_uri_t structure to be provided on input,
    // but allocate a new one if the structure is NULL.
    if (*uri).is_null() {
        *uri = calloc(1, ::std::mem::size_of::<htp_uri_t>()) as *mut htp_uri_t;
        if (*uri).is_null() {
            return Status::ERROR;
        }
    }
    if input.is_null() || bstr::bstr_len(input) == 0 {
        // The input might be NULL or empty on requests that don't actually
        // contain the URI. We allow that.
        return Status::OK;
    }
    if let Ok((_, parsed_uri)) = parse_uri()((*input).as_slice()) {
        if let Some(scheme) = parsed_uri.scheme {
            (*(*uri)).scheme = bstr::bstr_dup_str(scheme);
        }
        if let Some(username) = parsed_uri.username {
            (*(*uri)).username = bstr::bstr_dup_str(username);
        }
        if let Some(password) = parsed_uri.password {
            (*(*uri)).password = bstr::bstr_dup_str(password);
        }
        if let Some(hostname) = parsed_uri.hostname {
            (*(*uri)).hostname = bstr::bstr_dup_str(hostname);
        }
        if let Some(port) = parsed_uri.port {
            (*(*uri)).port = bstr::bstr_dup_str(port);
        }
        if let Some(path) = parsed_uri.path {
            (*(*uri)).path = bstr::bstr_dup_str(path);
        }
        if let Some(query) = parsed_uri.query {
            (*(*uri)).query = bstr::bstr_dup_str(query);
        }
        if let Some(fragment) = parsed_uri.fragment {
            (*(*uri)).fragment = bstr::bstr_dup_str(fragment);
        }
    }
    Status::OK
}

/// Convert two input bytes, pointed to by the pointer parameter,
/// into a single byte by assuming the input consists of hexadecimal
/// characters. This function will happily convert invalid input.
///
/// Returns hex-decoded byte
fn x2c(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, (c1, c2)) = tuple((be_u8, be_u8))(input)?;
    let mut decoded_byte: u8 = 0;
    decoded_byte = if c1 >= 'A' as u8 {
        ((c1 & 0xdf) - 'A' as u8) + 10
    } else {
        c1 - '0' as u8
    };
    decoded_byte = (decoded_byte as i32 * 16) as u8;
    decoded_byte = decoded_byte
        + if c2 >= 'A' as u8 {
            ((c2 & 0xdf) - 'A' as u8) + 10
        } else {
            c2 - '0' as u8
        };
    Ok((input, decoded_byte))
}

/// Decode a UTF-8 encoded path. Replaces a possibly-invalid utf8 byte stream with
/// an ascii stream. Overlong characters will be decoded and invalid characters will
/// be replaced with the replacement byte specified in the cfg. Best-fit mapping will
/// be used to convert UTF-8 into a single-byte stream. The resulting decoded path will
/// be stored in the input path if the cfg indicates it
pub fn utf8_decode_and_validate_path_inplace(
    tx: &mut htp_transaction::htp_tx_t,
    path: &mut bstr::bstr_t,
) {
    let cfg = unsafe {
        (*(tx.cfg)).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
    };
    let mut decoder = utf8_decoder::Utf8Decoder::new(cfg);
    decoder.decode_and_validate(path.as_slice());
    if cfg.utf8_convert_bestfit {
        path.clear();
        path.add(decoder.decoded_bytes.as_slice());
    }
    tx.flags |= decoder.flags;

    if tx.flags.contains(Flags::HTP_PATH_UTF8_INVALID)
        && cfg.utf8_invalid_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
    {
        tx.response_status_expected_number = cfg.utf8_invalid_unwanted as i32;
    }
}

/// Decode a %u-encoded character, using best-fit mapping as necessary. Path version.
///
/// Returns decoded byte
fn decode_u_encoding_path<'a>(
    i: &'a [u8],
    cfg: htp_decoder_cfg_t,
) -> IResult<&'a [u8], (u8, Flags, i32)> {
    let mut flags = Flags::empty();
    let mut expected_status_code: i32 = 0;
    let (i, c1) = x2c(&i)?;
    let (i, c2) = x2c(&i)?;
    let mut r = cfg.bestfit_replacement_byte;
    if c1 == 0 {
        r = c2;
        flags |= Flags::HTP_PATH_OVERLONG_U
    } else {
        // Check for fullwidth form evasion
        if c1 == 0xff {
            flags |= Flags::HTP_PATH_HALF_FULL_RANGE
        }
        if cfg.u_encoding_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
            expected_status_code = cfg.u_encoding_unwanted as i32
        }
        // Use best-fit mapping
        let p = cfg.bestfit_map;
        // TODO Optimize lookup.
        // Have we reached the end of the map?
        let mut index: usize = 0;
        while index + 3 < p.len() {
            if p[index] == c1 && p[index + 1] == c2 {
                r = p[index + 2];
                break;
            } else {
                // Move to the next triplet
                index = index.wrapping_add(3)
            }
        }
    }
    // Check for encoded path separators
    if r == '/' as u8 || cfg.backslash_convert_slashes && r == '\\' as u8 {
        flags |= Flags::HTP_PATH_ENCODED_SEPARATOR
    }
    Ok((i, (r, flags, expected_status_code)))
}

/// Decode a %u-encoded character, using best-fit mapping as necessary. Params version.
///
/// Returns decoded byte
fn decode_u_encoding_params<'a>(
    i: &'a [u8],
    cfg: htp_decoder_cfg_t,
) -> IResult<&'a [u8], (u8, Flags)> {
    let (i, c1) = x2c(&i)?;
    let (i, c2) = x2c(&i)?;
    let mut r = cfg.bestfit_replacement_byte;
    let mut flags = Flags::empty();
    // Check for overlong usage first.
    if c1 == 0 {
        flags |= Flags::HTP_URLEN_OVERLONG_U;
        return Ok((i, (c2, flags)));
    }
    // Both bytes were used.
    // Detect half-width and full-width range.
    if c1 == 0xff && c2 <= 0xef {
        flags |= Flags::HTP_URLEN_HALF_FULL_RANGE
    }
    // Use best-fit mapping.
    let p = cfg.bestfit_map;
    // TODO Optimize lookup.
    // Have we reached the end of the map?
    let mut index: usize = 0;
    while index + 3 < p.len() {
        if p[index] == c1 && p[index + 1] == c2 {
            r = p[index + 2];
            break;
        } else {
            // Move to the next triplet
            index = index.wrapping_add(3)
        }
    }
    Ok((i, (r, flags)))
}

/// Decodes path valid uencoded params according to the given cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_valid_uencoding<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |remaining_input| {
        let (left, _) = tag_no_case("u")(remaining_input)?;
        let mut output = remaining_input;
        let mut byte = '%' as u8;
        let mut flags = Flags::empty();
        let mut expected_status_code: i32 = 0;
        if cfg.u_encoding_decode {
            let (left, hex) = take_while_m_n(4, 4, |c: u8| c.is_ascii_hexdigit())(left)?;
            output = left;
            if cfg.u_encoding_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                expected_status_code = cfg.u_encoding_unwanted as i32
            }
            // Decode a valid %u encoding.
            let (_, (b, f, c)) = decode_u_encoding_path(hex, cfg)?;
            byte = b;
            flags |= f;
            if c != 0 {
                expected_status_code = c;
            }
            if byte == 0 {
                flags |= Flags::HTP_PATH_ENCODED_NUL;
                if cfg.nul_encoded_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                    expected_status_code = cfg.nul_encoded_unwanted as i32
                }
                if cfg.nul_encoded_terminates {
                    // Terminate the path at the raw NUL byte.
                    return Ok((b"", (byte, expected_status_code, flags, false)));
                }
            }
        }
        let (byte, code) = path_decode_control(byte, cfg);
        if code != 0 {
            expected_status_code = code;
        }
        Ok((output, (byte, expected_status_code, flags, true)))
    }
}

/// Decodes path invalid uencoded params according to the given cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_invalid_uencoding<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |remaining_input| {
        let mut output = remaining_input;
        let mut byte = '%' as u8;
        let mut flags = Flags::empty();
        let mut expected_status_code: i32 = 0;
        let (left, _) = tag_no_case("u")(remaining_input)?;
        if cfg.u_encoding_decode {
            let (left, hex) = take(4usize)(left)?;
            // Invalid %u encoding
            flags = Flags::HTP_PATH_INVALID_ENCODING;
            if cfg.url_encoding_invalid_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
            {
                expected_status_code = cfg.url_encoding_invalid_unwanted as i32
            }
            if cfg.url_encoding_invalid_handling
                == htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT
            {
                // Do not place anything in output; consume the %.
                return Ok((remaining_input, (byte, expected_status_code, flags, false)));
            } else if cfg.url_encoding_invalid_handling
                == htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID
            {
                let (_, (b, f, c)) = decode_u_encoding_path(&hex, cfg)?;
                if c != 0 {
                    expected_status_code = c;
                }
                flags |= f;
                byte = b;
                output = left;
            }
        }
        let (byte, code) = path_decode_control(byte, cfg);
        if code != 0 {
            expected_status_code = code;
        }
        Ok((output, (byte, expected_status_code, flags, true)))
    }
}

/// Decodes path valid hex according to the given cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_valid_hex<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |remaining_input| {
        let original_remaining = remaining_input;
        // Valid encoding (2 xbytes)
        not(tag_no_case("u"))(remaining_input)?;
        let (mut left, hex) = take_while_m_n(2, 2, |c: u8| c.is_ascii_hexdigit())(remaining_input)?;
        let mut flags = Flags::empty();
        let mut expected_status_code: i32 = 0;
        // Convert from hex.
        let (_, mut byte) = x2c(&hex)?;
        if byte == 0 {
            flags |= Flags::HTP_PATH_ENCODED_NUL;
            if cfg.nul_encoded_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                expected_status_code = cfg.nul_encoded_unwanted as i32
            }
            if cfg.nul_encoded_terminates {
                // Terminate the path at the raw NUL byte.
                return Ok((b"", (byte, expected_status_code, flags, false)));
            }
        }
        if byte == '/' as u8 || (cfg.backslash_convert_slashes && byte == '\\' as u8) {
            flags |= Flags::HTP_PATH_ENCODED_SEPARATOR;
            if cfg.path_separators_encoded_unwanted
                != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
            {
                expected_status_code = cfg.path_separators_encoded_unwanted as i32
            }
            if !cfg.path_separators_decode {
                // Leave encoded
                byte = '%' as u8;
                left = original_remaining;
            }
        }
        let (byte, expected_status_code) = path_decode_control(byte, cfg);
        Ok((left, (byte, expected_status_code, flags, true)))
    }
}

/// Decodes path invalid hex according to the given cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_invalid_hex<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |remaining_input| {
        let mut remaining = remaining_input;
        // Valid encoding (2 xbytes)
        not(tag_no_case("u"))(remaining_input)?;
        let (left, hex) = take(2usize)(remaining_input)?;
        let mut byte = '%' as u8;
        // Invalid encoding
        let flags = Flags::HTP_PATH_INVALID_ENCODING;
        let expected_status_code = if cfg.url_encoding_invalid_unwanted
            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
        {
            cfg.url_encoding_invalid_unwanted as i32
        } else {
            0
        };
        if cfg.url_encoding_invalid_handling
            == htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT
        {
            // Do not place anything in output; consume the %.
            return Ok((remaining_input, (byte, expected_status_code, flags, false)));
        } else if cfg.url_encoding_invalid_handling
            == htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID
        {
            // Decode
            let (_, b) = x2c(&hex)?;
            remaining = left;
            byte = b;
        }
        let (byte, expected_status_code) = path_decode_control(byte, cfg);
        Ok((remaining, (byte, expected_status_code, flags, true)))
    }
}
/// If the first byte of the input path string is a '%', it attempts to decode according to the
/// configuration specified by cfg. Various flags (HTP_PATH_*) might be set. If something in the
/// input would cause a particular server to respond with an error, the appropriate status
/// code will be set.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_percent<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |i| {
        let (remaining_input, c) = char('%')(i)?;
        let byte = c as u8;
        alt((
            path_decode_valid_uencoding(cfg),
            path_decode_invalid_uencoding(cfg),
            move |remaining_input| {
                let (_, _) = tag_no_case("u")(remaining_input)?;
                // Invalid %u encoding (not enough data)
                let flags = Flags::HTP_PATH_INVALID_ENCODING;
                let mut expected_status_code: i32 = 0;
                if cfg.url_encoding_invalid_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    expected_status_code = cfg.url_encoding_invalid_unwanted as i32
                }
                if cfg.url_encoding_invalid_handling
                    == htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT
                {
                    // Do not place anything in output; consume the %.
                    return Ok((remaining_input, (byte, expected_status_code, flags, false)));
                }
                Ok((remaining_input, (byte, expected_status_code, flags, true)))
            },
            path_decode_valid_hex(cfg),
            path_decode_invalid_hex(cfg),
            move |remaining_input| {
                // Invalid URL encoding (not even 2 bytes of data)
                Ok((
                    remaining_input,
                    (
                        byte,
                        if cfg.url_encoding_invalid_unwanted
                            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                        {
                            cfg.url_encoding_invalid_unwanted as i32
                        } else {
                            0
                        },
                        Flags::HTP_PATH_INVALID_ENCODING,
                        cfg.url_encoding_invalid_handling
                            != htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
                    ),
                ))
            },
        ))(remaining_input)
    }
}

/// Assumes the input is already decoded and checks if it is null byte or control character, handling each
/// according to the decoder configurations settings.
///
/// Returns parsed byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_parse_other<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |i| {
        let (remaining_input, byte) = be_u8(i)?;
        let mut expected_status_code: i32 = 0;
        // One non-encoded byte.
        // Did we get a raw NUL byte?
        if byte == 0 {
            if cfg.nul_raw_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                expected_status_code = cfg.nul_raw_unwanted as i32
            }
            if cfg.nul_raw_terminates {
                // Terminate the path at the encoded NUL byte.
                return Ok((b"", (byte, expected_status_code, Flags::empty(), false)));
            }
        }
        let (byte, expected_status_code) = path_decode_control(byte, cfg);
        Ok((
            remaining_input,
            (byte, expected_status_code, Flags::empty(), true),
        ))
    }
}
/// Checks for control characters and converts them according to the cfg settings
///
/// Returns decoded byte and expected_status_code
fn path_decode_control(mut byte: u8, cfg: htp_decoder_cfg_t) -> (u8, i32) {
    // Note: What if an invalid encoding decodes into a path
    //       separator? This is theoretical at the moment, because
    //       the only platform we know doesn't convert separators is
    //       Apache, who will also respond with 400 if invalid encoding
    //       is encountered. Thus no check for a separator here.
    // Place the character into output
    // Check for control characters
    let mut expected_status_code = 0;
    if byte < 0x20 && cfg.control_chars_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
    {
        expected_status_code = cfg.control_chars_unwanted as i32
    }
    // Convert backslashes to forward slashes, if necessary
    if byte == '\\' as u8 && cfg.backslash_convert_slashes {
        byte = '/' as u8
    }
    // Lowercase characters, if necessary
    if cfg.convert_lowercase {
        byte = byte.to_ascii_lowercase()
    }
    (byte, expected_status_code)
}

/// Decode a request path according to the settings in the
/// provided configuration structure.
fn path_decode<'a>(
    input: &'a [u8],
    cfg: htp_decoder_cfg_t,
) -> IResult<&'a [u8], (Vec<u8>, Flags, i32)> {
    fold_many0(
        alt((path_decode_percent(cfg), path_parse_other(cfg))),
        (Vec::new(), Flags::empty(), 0),
        |mut acc: (Vec<_>, Flags, i32), (byte, code, flag, insert)| {
            // If we're compressing separators then we need
            // to check if the previous character was a separator
            if insert {
                if byte == '/' as u8 && cfg.path_separators_compress {
                    if !acc.0.is_empty() {
                        if acc.0[acc.0.len() - 1] != '/' as u8 {
                            acc.0.push(byte);
                        }
                    } else {
                        acc.0.push(byte);
                    }
                } else {
                    acc.0.push(byte);
                }
            }
            acc.1 |= flag;
            acc.2 = code;
            acc
        },
    )(input)
}

/// Decode a request path inplace according to the settings in the
/// provided configuration structure.
pub fn htp_decode_path_inplace(
    tx: &mut htp_transaction::htp_tx_t,
    path: &mut bstr::bstr_t,
) -> Status {
    let mut rc = Status::ERROR;
    let decoder_cfg = unsafe {
        (*(tx.cfg)).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
    };
    if let Ok((_, (consumed, flags, expected_status_code))) =
        path_decode(path.as_slice(), decoder_cfg)
    {
        path.clear();
        path.add(consumed.as_slice());
        tx.response_status_expected_number = expected_status_code;
        tx.flags |= flags;
        rc = Status::OK;
    }
    rc
}

pub fn htp_tx_urldecode_uri_inplace(
    tx: &mut htp_transaction::htp_tx_t,
    input: &mut bstr::bstr_t,
) -> Status {
    let mut rc = Status::ERROR;
    let decoder_cfg = unsafe {
        (*(tx.cfg)).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH as usize]
    };
    if let Ok((_, (consumed, flags, _))) = htp_urldecode_ex(input.as_slice(), decoder_cfg) {
        (*input).clear();
        input.add(consumed.as_slice());
        if flags.contains(Flags::HTP_URLEN_INVALID_ENCODING) {
            tx.flags |= Flags::HTP_PATH_INVALID_ENCODING
        }
        if flags.contains(Flags::HTP_URLEN_ENCODED_NUL) {
            tx.flags |= Flags::HTP_PATH_ENCODED_NUL
        }
        if flags.contains(Flags::HTP_URLEN_RAW_NUL) {
            tx.flags |= Flags::HTP_PATH_RAW_NUL;
        }
        rc = Status::OK;
    }
    rc
}

pub fn htp_tx_urldecode_params_inplace(
    tx: &mut htp_transaction::htp_tx_t,
    input: &mut bstr::bstr_t,
) -> Status {
    let mut rc = Status::ERROR;
    let decoder_cfg = unsafe {
        (*(tx.cfg)).decoder_cfgs[htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED as usize]
    };
    if let Ok((_, (consumed, flags, expected_status))) =
        htp_urldecode_ex(input.as_slice(), decoder_cfg)
    {
        (*input).clear();
        input.add(consumed.as_slice());
        tx.flags |= flags;
        tx.response_status_expected_number = expected_status;
        rc = Status::OK;
    }
    rc
}

/// Performs in-place decoding of the input string, according to the configuration specified
/// by cfg and ctx. On output, various flags (HTP_URLEN_*) might be set.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub fn htp_urldecode_inplace(
    cfg: htp_decoder_cfg_t,
    input: &mut bstr::bstr_t,
    flags: &mut Flags,
) -> Status {
    let mut rc = Status::ERROR;
    if let Ok((_, (consumed, flag, _))) = htp_urldecode_ex(input.as_slice(), cfg) {
        (*input).clear();
        input.add(consumed.as_slice());
        *flags |= flag;
        rc = Status::OK;
    }
    rc
}

/// Decodes valid uencoded hex bytes according to the given cfg settings.
/// e.g. "u0064" -> "d"
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_valid_uencoding<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |input| {
        let (left, _) = alt((char('u'), char('U')))(input)?;
        if cfg.u_encoding_decode {
            let (input, hex) = take_while_m_n(4, 4, |c: u8| c.is_ascii_hexdigit())(left)?;
            let (_, (byte, flags)) = decode_u_encoding_params(hex, cfg)?;
            return Ok((
                input,
                (
                    byte,
                    if cfg.u_encoding_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                        cfg.u_encoding_unwanted as i32
                    } else {
                        0
                    },
                    flags,
                    true,
                ),
            ));
        }
        Ok((input, ('%' as u8, 0, Flags::empty(), true)))
    }
}

/// Decodes invalid uencoded params according to the given cfg settings.
/// e.g. "u00}9" -> "i"
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_invalid_uencoding<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |mut input| {
        let (left, _) = alt((char('u'), char('U')))(input)?;
        let mut byte = '%' as u8;
        let mut code: i32 = 0;
        let mut flags = Flags::empty();
        let mut insert = true;
        if cfg.u_encoding_decode {
            // Invalid %u encoding (could not find 4 xdigits).
            let (left, invalid_hex) = take(4usize)(left)?;
            flags |= Flags::HTP_URLEN_INVALID_ENCODING;
            code = if cfg.url_encoding_invalid_unwanted
                != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
            {
                cfg.url_encoding_invalid_unwanted as i32
            } else if cfg.u_encoding_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                cfg.u_encoding_unwanted as i32
            } else {
                0
            };
            if cfg.url_encoding_invalid_handling
                == htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT
            {
                // Do not place anything in output; consume the %.
                insert = false;
            } else if cfg.url_encoding_invalid_handling
                == htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID
            {
                let (_, (b, f)) = decode_u_encoding_params(invalid_hex, cfg)?;
                flags |= f;
                byte = b;
                input = left;
            }
        }
        Ok((input, (byte, code, flags, insert)))
    }
}

/// Decodes valid hex byte.
///  e.g. "2f" -> "/"
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_valid_hex<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |input| {
        // Valid encoding (2 xbytes)
        not(alt((char('u'), char('U'))))(input)?;
        let (input, hex) = take_while_m_n(2, 2, |c: u8| c.is_ascii_hexdigit())(input)?;
        let (_, byte) = x2c(hex)?;
        Ok((input, (byte, 0, Flags::empty(), true)))
    }
}

/// Decodes invalid hex byte according to the given cfg settings.
/// e.g. "}9" -> "i"
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_invalid_hex<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |mut input| {
        not(alt((char('u'), char('U'))))(input)?;
        // Invalid encoding (2 bytes, but not hexadecimal digits).
        let mut byte = '%' as u8;
        let mut insert = true;
        if cfg.url_encoding_invalid_handling
            == htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT
        {
            // Do not place anything in output; consume the %.
            insert = false;
        } else if cfg.url_encoding_invalid_handling
            == htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID
        {
            let (left, b) = x2c(input)?;
            input = left;
            byte = b;
        }
        Ok((
            input,
            (
                byte,
                if cfg.url_encoding_invalid_unwanted
                    != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                {
                    cfg.url_encoding_invalid_unwanted as i32
                } else {
                    0
                },
                Flags::HTP_URLEN_INVALID_ENCODING,
                insert,
            ),
        ))
    }
}

/// If the first byte of the input string is a '%', it attempts to decode according to the
/// configuration specified by cfg. Various flags (HTP_URLEN_*) might be set. If something in the
/// input would cause a particular server to respond with an error, the appropriate status
/// code will be set.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_percent<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |i| {
        let (input, _) = char('%')(i)?;
        let (input, (byte, mut expected_status_code, mut flags, insert)) = alt((
            url_decode_valid_uencoding(cfg),
            url_decode_invalid_uencoding(cfg),
            url_decode_valid_hex(),
            url_decode_invalid_hex(cfg),
            move |input| {
                // Invalid %u encoding; not enough data. (not even 2 bytes)
                // Do not place anything in output if HTP_URL_DECODE_REMOVE_PERCENT; consume the %.
                Ok((
                    input,
                    (
                        '%' as u8,
                        if cfg.url_encoding_invalid_unwanted
                            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
                        {
                            cfg.url_encoding_invalid_unwanted as i32
                        } else {
                            0
                        },
                        Flags::HTP_URLEN_INVALID_ENCODING,
                        !(cfg.url_encoding_invalid_handling
                            == htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT),
                    ),
                ))
            },
        ))(input)?;
        //Did we get an encoded NUL byte?
        if byte == 0 {
            flags |= Flags::HTP_URLEN_ENCODED_NUL;
            if cfg.nul_encoded_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                expected_status_code = cfg.nul_encoded_unwanted as i32
            }
            if cfg.nul_encoded_terminates {
                // Terminate the path at the encoded NUL byte.
                return Ok((b"", (byte, expected_status_code, flags, false)));
            }
        }
        Ok((input, (byte, expected_status_code, flags, insert)))
    }
}

/// Consumes the next nullbyte if it is a '+', decoding it according to the cfg
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_plus<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |input| {
        let (input, byte) = map(char('+'), |byte| {
            // Decoding of the plus character is conditional on the configuration.
            if cfg.plusspace_decode {
                0x20
            } else {
                byte as u8
            }
        })(input)?;
        Ok((input, (byte, 0, Flags::empty(), true)))
    }
}

/// Consumes the next byte in the input string and treats it as an unencoded byte.
/// Handles raw null bytes according to the input cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_parse_unencoded_byte<'a>(
    cfg: htp_decoder_cfg_t,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, i32, Flags, bool)> {
    move |input| {
        let (input, byte) = be_u8(input)?;
        // One non-encoded byte.
        // Did we get a raw NUL byte?
        if byte == 0 {
            return Ok((
                if cfg.nul_raw_terminates { b"" } else { input },
                (
                    byte,
                    if cfg.nul_raw_unwanted != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE {
                        cfg.nul_raw_unwanted as i32
                    } else {
                        0
                    },
                    Flags::HTP_URLEN_RAW_NUL,
                    !cfg.nul_raw_terminates,
                ),
            ));
        }
        Ok((input, (byte, 0, Flags::empty(), true)))
    }
}

/// Performs decoding of the input string, according to the configuration specified
/// by cfg. Various flags (HTP_URLEN_*) might be set. If something in the input would
/// cause a particular server to respond with an error, the appropriate status
/// code will be set.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be consumed or output.
fn htp_urldecode_ex<'a>(
    input: &'a [u8],
    cfg: htp_decoder_cfg_t,
) -> IResult<&'a [u8], (Vec<u8>, Flags, i32)> {
    fold_many0(
        alt((
            url_decode_percent(cfg),
            url_decode_plus(cfg),
            url_parse_unencoded_byte(cfg),
        )),
        (Vec::new(), Flags::empty(), 0),
        |mut acc: (Vec<_>, Flags, i32), (byte, code, flag, insert)| {
            if insert {
                acc.0.push(byte);
            }
            acc.1 |= flag;
            if code != 0 {
                acc.2 = code;
            }
            acc
        },
    )(input)
}

/// Normalize a previously-parsed request URI.
///
/// Returns HTP_OK or HTP_ERROR
pub unsafe fn htp_normalize_parsed_uri(
    tx: *mut htp_transaction::htp_tx_t,
    incomplete: *const htp_uri_t,
    mut normalized: *mut htp_uri_t,
) -> i32 {
    // Scheme.
    if !(*incomplete).scheme.is_null() {
        // Duplicate and convert to lowercase.
        (*normalized).scheme = bstr::bstr_dup_lower((*incomplete).scheme);
        if (*normalized).scheme.is_null() {
            return -1;
        }
    }
    // Username.
    if !(*incomplete).username.is_null() {
        (*normalized).username = bstr::bstr_dup((*incomplete).username);
        if (*normalized).username.is_null() {
            return -1;
        }
        htp_tx_urldecode_uri_inplace(&mut *tx, &mut *(*normalized).username);
    }
    // Password.
    if !(*incomplete).password.is_null() {
        (*normalized).password = bstr::bstr_dup((*incomplete).password);
        if (*normalized).password.is_null() {
            return -1;
        }
        htp_tx_urldecode_uri_inplace(&mut *tx, &mut *(*normalized).password);
    }
    // Hostname.
    if !(*incomplete).hostname.is_null() {
        // We know that incomplete->hostname does not contain
        // port information, so no need to check for it here.
        (*normalized).hostname = bstr::bstr_dup((*incomplete).hostname);
        if (*normalized).hostname.is_null() {
            return -1;
        }
        htp_tx_urldecode_uri_inplace(&mut *tx, &mut *(*normalized).hostname);
        htp_normalize_hostname_inplace(&mut *(*normalized).hostname);
    }
    // Port.
    if !(*incomplete).port.is_null() {
        if let Some(port) = convert_port((&*(*incomplete).port).as_slice()) {
            // Valid port number.
            (*normalized).port_number = port as i32;
        } else {
            // Failed to parse the port number.
            (*normalized).port_number = -1;
            (*tx).flags |= Flags::HTP_HOSTU_INVALID
        }
    } else {
        (*normalized).port_number = -1
    }
    // Path.
    if !(*incomplete).path.is_null() {
        // Make a copy of the path, so that we can work on it.
        (*normalized).path = bstr::bstr_dup((*incomplete).path);
        if (*normalized).path.is_null() {
            return -1;
        }
        // Decode URL-encoded (and %u-encoded) characters, as well as lowercase,
        // compress separators and convert backslashes.
        htp_decode_path_inplace(&mut *tx, &mut *(*normalized).path);
        // Handle UTF-8 in the path. Validate it first, and only save it if cfg specifies it
        utf8_decode_and_validate_path_inplace(&mut *tx, &mut *(*normalized).path);
        // RFC normalization.
        htp_normalize_uri_path_inplace(&mut *(*normalized).path);
    }
    // Query string.
    if !(*incomplete).query.is_null() {
        (*normalized).query = bstr::bstr_dup((*incomplete).query);
        if (*normalized).query.is_null() {
            return -1;
        }
    }
    // Fragment.
    if !(*incomplete).fragment.is_null() {
        (*normalized).fragment = bstr::bstr_dup((*incomplete).fragment);
        if (*normalized).fragment.is_null() {
            return -1;
        }
        htp_tx_urldecode_uri_inplace(&mut *tx, &mut *(*normalized).fragment);
    }
    1
}

/// Normalize request hostname inplace. Convert all characters to lowercase and
/// remove trailing dots from the end, if present.
fn htp_normalize_hostname_inplace(hostname: &mut bstr::bstr_t) {
    hostname.make_ascii_lowercase();
    // Remove dots from the end of the string.
    while hostname.last() == Some(&('.' as u8)) {
        hostname.pop();
    }
}

/// Normalize URL path. This function implements the remove dot segments algorithm
/// specified in RFC 3986, section 5.2.4.
fn normalize_uri_path(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::<&[u8]>::with_capacity(10);
    input
        .split(|c| *c == '/' as u8)
        .for_each(|segment| match segment {
            b"." => {}
            b".." => {
                if !(out.len() == 1 && out[0] == b"") {
                    out.pop();
                }
            }
            x => out.push(x),
        });
    out.join(b"/" as &[u8])
}

/// Normalize URL path in place. This function implements the remove dot segments algorithm
/// specified in RFC 3986, section 5.2.4.
pub fn htp_normalize_uri_path_inplace(s: &mut bstr::bstr_t) {
    let consumed = normalize_uri_path(s.as_slice());
    s.clear();
    s.add(consumed.as_slice());
}

/// Determine if the information provided on the response line
/// is good enough. Browsers are lax when it comes to response
/// line parsing. In most cases they will only look for the
/// words "http" at the beginning.
///
/// Returns 1 for good enough or 0 for not good enough
pub unsafe fn htp_treat_response_line_as_body(data: *const u8, len: usize) -> i32 {
    // Browser behavior:
    //      Firefox 3.5.x: (?i)^\s*http
    //      IE: (?i)^\s*http\s*/
    //      Safari: ^HTTP/\d+\.\d+\s+\d{3}
    let mut pos: usize = 0;
    if data.is_null() {
        return 1;
    }
    while pos < len
        && (htp_is_space(*data.offset(pos as isize)) || *data.offset(pos as isize) as i32 == 0)
    {
        pos = pos.wrapping_add(1)
    }
    if len < pos.wrapping_add(4) {
        return 1;
    }
    if *data.offset(pos as isize) != 'H' as u8 && *data.offset(pos as isize) != 'h' as u8 {
        return 1;
    }
    if *data.offset(pos.wrapping_add(1) as isize) != 'T' as u8
        && *data.offset(pos.wrapping_add(1) as isize) != 't' as u8
    {
        return 1;
    }
    if *data.offset(pos.wrapping_add(2) as isize) != 'T' as u8
        && *data.offset(pos.wrapping_add(2) as isize) != 't' as u8
    {
        return 1;
    }
    if *data.offset(pos.wrapping_add(3) as isize) != 'P' as u8
        && *data.offset(pos.wrapping_add(3) as isize) != 'p' as u8
    {
        return 1;
    }
    0
}

/// Run the REQUEST_BODY_DATA hook.
pub unsafe fn htp_req_run_hook_body_data(
    connp: *mut htp_connection_parser::htp_connp_t,
    d: *mut htp_transaction::htp_tx_data_t,
) -> Status {
    // Do not invoke callbacks with an empty data chunk
    if !(*d).data().is_null() && (*d).len() == 0 {
        return Status::OK;
    }
    // Do not invoke callbacks without a transaction.
    let mut rc = if let Some(in_tx) = (*connp).in_tx() {
        // Run transaction hooks first
        htp_hooks::htp_hook_run_all(in_tx.hook_request_body_data, d as *mut core::ffi::c_void)
    } else {
        return Status::OK;
    };

    if rc != Status::OK {
        return rc;
    }
    // Run configuration hooks second
    rc = htp_hooks::htp_hook_run_all(
        (*(*connp).cfg).hook_request_body_data,
        d as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    // On PUT requests, treat request body as file
    if !(*connp).put_file.is_null() {
        let mut file_data: htp_file_data_t = htp_file_data_t {
            file: 0 as *mut htp_file_t,
            data: 0 as *const u8,
            len: 0,
        };
        file_data.data = (*d).data();
        file_data.len = (*d).len();
        file_data.file = (*connp).put_file;
        (*file_data.file).len = ((*file_data.file).len).wrapping_add((*d).len());
        rc = htp_hooks::htp_hook_run_all(
            (*(*connp).cfg).hook_request_file_data,
            &mut file_data as *mut htp_file_data_t as *mut core::ffi::c_void,
        );
        if rc != Status::OK {
            return rc;
        }
    }
    Status::OK
}

/// Run the RESPONSE_BODY_DATA hook.
pub unsafe fn htp_res_run_hook_body_data(
    connp: *mut htp_connection_parser::htp_connp_t,
    d: *mut htp_transaction::htp_tx_data_t,
) -> Status {
    let out_tx = if let Some(out_tx) = (*connp).out_tx_mut() {
        out_tx
    } else {
        return Status::ERROR;
    };
    // Do not invoke callbacks with an empty data chunk.
    if !(*d).data().is_null() && (*d).len() == 0 {
        return Status::OK;
    }
    // Run transaction hooks first
    let mut rc: Status =
        htp_hooks::htp_hook_run_all(out_tx.hook_response_body_data, d as *mut core::ffi::c_void);
    if rc != Status::OK {
        return rc;
    }
    // Run configuration hooks second
    rc = htp_hooks::htp_hook_run_all(
        (*(*connp).cfg).hook_response_body_data,
        d as *mut core::ffi::c_void,
    );
    if rc != Status::OK {
        return rc;
    }
    Status::OK
}

/// Parses the content type header, trimming any leading whitespace.
/// Finds the end of the MIME type, using the same approach PHP 5.4.3 uses.
///
/// Returns a tuple of the remaining unparsed header data and the content type
fn content_type_header<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    move |input| {
        map(
            tuple((take_ascii_whitespace(), is_not(";, "))),
            |(_, content_type)| content_type,
        )(input)
    }
}

/// Parses the content type header from the given header value, lowercases it, and stores it in the provided ct bstr.
/// Finds the end of the MIME type, using the same approach PHP 5.4.3 uses.
///
/// Returns Status::OK if successful; Status::ERROR if not
pub fn htp_parse_ct_header<'a>(header: &'a bstr::bstr_t, ct: &mut bstr::bstr_t) -> Status {
    let mut rc = Status::ERROR;
    if let Ok((_, content_type)) = content_type_header()(header.as_slice()) {
        ct.clear();
        ct.add(content_type);
        ct.make_ascii_lowercase();
        rc = Status::OK;
    }
    rc
}

/// Implements relaxed (not strictly RFC) hostname validation.
///
/// Returns true if the supplied hostname is valid; false if it is not.
pub fn htp_validate_hostname<'a>(input: &'a [u8]) -> bool {
    if input.len() == 0 || input.len() > 255 {
        return false;
    }
    if char::<_, (&[u8], nom::error::ErrorKind)>('[')(input).is_ok() {
        if let Ok((input, _)) = is_not::<_, _, (&[u8], nom::error::ErrorKind)>("#?/]")(input) {
            if char::<_, (&[u8], nom::error::ErrorKind)>(']')(input).is_ok() {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    if tag::<_, _, (&[u8], nom::error::ErrorKind)>(".")(input).is_ok()
        || take_until::<_, _, (&[u8], nom::error::ErrorKind)>("..")(input).is_ok()
    {
        return false;
    }
    for section in input.split(|&c| c == '.' as u8) {
        if section.len() > 63 {
            return false;
        }
        if !take_while_m_n::<_, _, (&[u8], nom::error::ErrorKind)>(
            section.len(),
            section.len(),
            |c| c == '-' as u8 || (c as char).is_alphanumeric(),
        )(section)
        .is_ok()
        {
            return false;
        }
    }
    true
}

/// Frees all data contained in the uri, and then the uri itself.
pub unsafe fn htp_uri_free(uri: *mut htp_uri_t) {
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
    free(uri as *mut core::ffi::c_void);
}

/// Allocates and initializes a new htp_uri_t structure.
///
/// Returns New structure, or NULL on memory allocation failure.
pub unsafe fn htp_uri_alloc() -> *mut htp_uri_t {
    let mut u: *mut htp_uri_t = calloc(1, ::std::mem::size_of::<htp_uri_t>()) as *mut htp_uri_t;
    if u.is_null() {
        return 0 as *mut htp_uri_t;
    }
    (*u).port_number = -1;
    u
}

/// Returns the LibHTP version string.
pub unsafe fn htp_get_version() -> *const i8 {
    HTP_VERSION_STRING_FULL.as_ptr() as *const i8
}

// Tests

#[test]
fn AsciiDigits() {
    // Returns (any trailing non-LWS characters, (non-LWS leading characters, ascii digits))
    assert_eq!(
        Ok((b"bcd ".as_ref(), (b"a".as_ref(), b"200".as_ref()))),
        ascii_digits()(b"    a200 \t  bcd ")
    );
    assert_eq!(
        Ok((b"".as_ref(), (b"".as_ref(), b"555555555".as_ref()))),
        ascii_digits()(b"   555555555    ")
    );
    assert_eq!(
        Ok((b"500".as_ref(), (b"".as_ref(), b"555555555".as_ref()))),
        ascii_digits()(b"   555555555    500")
    );
    assert!(ascii_digits()(b"   garbage no ascii ").is_err());
}

#[test]
fn HexDigits() {
    //(trailing non-LWS characters, found hex digits)
    assert_eq!(Ok((b"".as_ref(), b"12a5".as_ref())), hex_digits()(b"12a5"));
    assert_eq!(
        Ok((b"".as_ref(), b"12a5".as_ref())),
        hex_digits()(b"    \t12a5    ")
    );
    assert_eq!(
        Ok((b".....".as_ref(), b"12a5".as_ref())),
        hex_digits()(b"12a5   .....")
    );
    assert_eq!(
        Ok((b".....    ".as_ref(), b"12a5".as_ref())),
        hex_digits()(b"    \t12a5.....    ")
    );
    assert_eq!(
        Ok((b"12a5".as_ref(), b"68656c6c6f".as_ref())),
        hex_digits()(b"68656c6c6f   12a5")
    );
    assert!(hex_digits()(b"  .....").is_err());
}

#[test]
fn TakeUntilNoCase() {
    let (remaining, consumed) = take_until_no_case(b"TAG")(
        b"Let's fish for a Tag, but what about this TaG, or this TAG, or another tag. GO FISH.",
    )
    .unwrap();

    let mut res_consumed: &[u8] = b"Let's fish for a ";
    let mut res_remaining: &[u8] =
        b"Tag, but what about this TaG, or this TAG, or another tag. GO FISH.";
    assert_eq!(res_consumed, consumed);
    assert_eq!(res_remaining, remaining);
    let (remaining, _) =
        tag_no_case::<_, _, (&[u8], nom::error::ErrorKind)>("TAG")(remaining).unwrap();

    res_consumed = b", but what about this ";
    res_remaining = b"TaG, or this TAG, or another tag. GO FISH.";
    let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
    assert_eq!(res_consumed, consumed);
    assert_eq!(res_remaining, remaining);
    let (remaining, _) =
        tag_no_case::<_, _, (&[u8], nom::error::ErrorKind)>("TAG")(remaining).unwrap();

    res_consumed = b", or this ";
    res_remaining = b"TAG, or another tag. GO FISH.";
    let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
    assert_eq!(res_consumed, consumed);
    assert_eq!(res_remaining, remaining);
    let (remaining, _) =
        tag_no_case::<_, _, (&[u8], nom::error::ErrorKind)>("TAG")(remaining).unwrap();

    res_consumed = b", or another ";
    res_remaining = b"tag. GO FISH.";
    let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
    assert_eq!(res_consumed, consumed);
    assert_eq!(res_remaining, remaining);

    res_consumed = b"";
    res_remaining = b"tag. GO FISH.";
    let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
    assert_eq!(res_consumed, consumed);
    assert_eq!(res_remaining, remaining);
    let (remaining, _) =
        tag_no_case::<_, _, (&[u8], nom::error::ErrorKind)>("TAG")(remaining).unwrap();

    res_consumed = b". GO FISH.";
    res_remaining = b"";
    let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
    assert_eq!(res_consumed, consumed);
    assert_eq!(res_remaining, remaining);
}
