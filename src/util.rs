//! Utility functions for http parsing.

use crate::{
    bstr::Bstr,
    config::{DecoderConfig, HtpServerPersonality, HtpUnwanted, HtpUrlEncodingHandling},
    error::Result,
    hook::FileDataHook,
    transaction::Transaction,
    utf8_decoder::Utf8Decoder,
    HtpStatus,
};
use nom::{
    branch::alt,
    bytes::complete::{
        is_not, tag, tag_no_case, take, take_till, take_until, take_while, take_while1,
        take_while_m_n,
    },
    bytes::streaming::take_till as streaming_take_till,
    bytes::streaming::take_while as streaming_take_while,
    character::complete::{char, digit1},
    character::is_space as nom_is_space,
    combinator::{map, not, opt},
    multi::fold_many0,
    number::complete::be_u8,
    sequence::tuple,
    IResult,
};

use std::{io::Write, rc::Rc, sync::Mutex};
use tempfile::{Builder, NamedTempFile};

/// String for the libhtp version.
pub const HTP_VERSION_STRING_FULL: &'_ str = concat!("LibHTP v", env!("CARGO_PKG_VERSION"), "\x00");

/// Trait to allow interacting with flags.
pub trait FlagOperations<T> {
    /// Inserts the specified flags in-place.
    fn set(&mut self, other: T);
    /// Removes the specified flags in-place.
    fn unset(&mut self, other: T);
    /// Determine if the specified flags are set
    fn is_set(&self, other: T) -> bool;
}

impl FlagOperations<u8> for u8 {
    /// Inserts the specified flags in-place.
    fn set(&mut self, other: u8) {
        *self |= other;
    }
    /// Removes the specified flags in-place.
    fn unset(&mut self, other: u8) {
        *self &= !other;
    }
    /// Determine if the specified flags are set
    fn is_set(&self, other: u8) -> bool {
        self & other != 0
    }
}

impl FlagOperations<u64> for u64 {
    /// Inserts the specified flags in-place.
    fn set(&mut self, other: u64) {
        *self |= other;
    }
    /// Removes the specified flags in-place.
    fn unset(&mut self, other: u64) {
        *self &= !other;
    }
    /// Determine if the specified flags are set
    fn is_set(&self, other: u64) -> bool {
        self & other != 0
    }
}

/// Various flag bits. Even though we have a flag field in several places
/// (header, transaction, connection), these fields are all in the same namespace
/// because we may want to set the same flag in several locations. For example, we
/// may set HTP_FIELD_FOLDED on the actual folded header, but also on the transaction
/// that contains the header. Both uses are useful.
pub struct HtpFlags;

impl HtpFlags {
    /// Field cannot be parsed.
    pub const FIELD_UNPARSEABLE: u64 = 0x0000_0000_0004;
    /// Field is invalid.
    pub const FIELD_INVALID: u64 = 0x0000_0000_0008;
    /// Field is folded.
    pub const FIELD_FOLDED: u64 = 0x0000_0000_0010;
    /// Field has been seen more than once.
    pub const FIELD_REPEATED: u64 = 0x0000_0000_0020;
    /// Field is too long.
    pub const FIELD_LONG: u64 = 0x0000_0000_0040;
    /// Field contains raw null byte.
    pub const FIELD_RAW_NUL: u64 = 0x0000_0000_0080;
    /// Detect HTTP request smuggling.
    pub const REQUEST_SMUGGLING: u64 = 0x0000_0000_0100;
    /// Invalid header folding.
    pub const INVALID_FOLDING: u64 = 0x0000_0000_0200;
    /// Invalid request transfer-encoding.
    pub const REQUEST_INVALID_T_E: u64 = 0x0000_0000_0400;
    /// Multiple chunks.
    pub const MULTI_PACKET_HEAD: u64 = 0x0000_0000_0800;
    /// No host information in header.
    pub const HOST_MISSING: u64 = 0x0000_0000_1000;
    /// Inconsistent host or port information.
    pub const HOST_AMBIGUOUS: u64 = 0x0000_0000_2000;
    /// Encoded path contains null.
    pub const PATH_ENCODED_NUL: u64 = 0x0000_0000_4000;
    /// Url encoded contains raw null.
    pub const PATH_RAW_NUL: u64 = 0x0000_0000_8000;
    /// Url encoding is invalid.
    pub const PATH_INVALID_ENCODING: u64 = 0x0000_0001_0000;
    /// Path is invalid.
    pub const PATH_INVALID: u64 = 0x0000_0002_0000;
    /// Overlong usage in path.
    pub const PATH_OVERLONG_U: u64 = 0x0000_0004_0000;
    /// Encoded path separators present.
    pub const PATH_ENCODED_SEPARATOR: u64 = 0x0000_0008_0000;
    /// At least one valid UTF-8 character and no invalid ones.
    pub const PATH_UTF8_VALID: u64 = 0x0000_0010_0000;
    /// Invalid utf8 in path.
    pub const PATH_UTF8_INVALID: u64 = 0x0000_0020_0000;
    /// Invalid utf8 overlong character.
    pub const PATH_UTF8_OVERLONG: u64 = 0x0000_0040_0000;
    /// Range U+FF00 - U+FFEF detected.
    pub const PATH_HALF_FULL_RANGE: u64 = 0x0000_0080_0000;
    /// Status line is invalid.
    pub const STATUS_LINE_INVALID: u64 = 0x0000_0100_0000;
    /// Host in the URI.
    pub const HOSTU_INVALID: u64 = 0x0000_0200_0000;
    /// Host in the Host header.
    pub const HOSTH_INVALID: u64 = 0x0000_0400_0000;
    /// Uri / host header invalid.
    pub const HOST_INVALID: u64 = (Self::HOSTU_INVALID | Self::HOSTH_INVALID);
    /// Contains null.
    pub const URLEN_ENCODED_NUL: u64 = 0x0000_0800_0000;
    /// Invalid encoding.
    pub const URLEN_INVALID_ENCODING: u64 = 0x0000_1000_0000;
    /// Overlong usage.
    pub const URLEN_OVERLONG_U: u64 = 0x0000_2000_0000;
    /// Range U+FF00 - U+FFEF detected.
    pub const URLEN_HALF_FULL_RANGE: u64 = 0x0000_4000_0000;
    /// Raw null byte.
    pub const URLEN_RAW_NUL: u64 = 0x0000_8000_0000;
    /// Request invalid.
    pub const REQUEST_INVALID: u64 = 0x0001_0000_0000;
    /// Request content-length invalid.
    pub const REQUEST_INVALID_C_L: u64 = 0x0002_0000_0000;
    /// Authorization is invalid.
    pub const AUTH_INVALID: u64 = 0x0004_0000_0000;
}

/// Enumerates file sources.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum HtpFileSource {
    /// File from a multipart request.
    MULTIPART = 1,
    /// File from a PUT request.
    PUT = 2,
}

/// Used to represent files that are seen during the processing of HTTP traffic. Most
/// commonly this refers to files seen in multipart/form-data payloads. In addition, PUT
/// request bodies can be treated as files.
#[derive(Debug, Clone)]
pub struct File {
    /// Where did this file come from? Possible values: MULTIPART and PUT.
    pub source: HtpFileSource,
    /// File name, as provided (e.g., in the Content-Disposition multipart part header.
    pub filename: Option<Bstr>,
    /// File length.
    pub len: usize,
    /// The file used for external storage.
    //TODO: Remove this mem management by making File not cloneable
    pub tmpfile: Option<Rc<Mutex<NamedTempFile>>>,
}

impl File {
    /// Construct new File.
    pub fn new(source: HtpFileSource, filename: Option<Bstr>) -> File {
        File {
            source,
            filename,
            len: 0,
            tmpfile: None,
        }
    }

    /// Set new tmpfile.
    pub fn create(&mut self, tmpfile: &str) -> Result<()> {
        self.tmpfile = Some(Rc::new(Mutex::new(
            Builder::new()
                .prefix("libhtp-multipart-file-")
                .rand_bytes(5)
                .tempfile_in(tmpfile)?,
        )));
        Ok(())
    }

    /// Write data to tmpfile.
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        if let Some(mutex) = &self.tmpfile {
            if let Ok(mut tmpfile) = mutex.lock() {
                tmpfile.write_all(data)?;
            }
        }
        Ok(())
    }

    /// Update file length and invoke any file data callbacks on the provided cfg.
    pub fn handle_file_data(
        &mut self,
        hook: FileDataHook,
        data: *const u8,
        len: usize,
    ) -> Result<()> {
        self.len = self.len.wrapping_add(len);
        // Package data for the callbacks.
        let mut file_data = FileData::new(&self, data, len);
        // Send data to callbacks
        hook.run_all(&mut file_data)
    }
}

/// Represents a chunk of file data.
pub struct FileData<'a> {
    /// File information.
    pub file: &'a File,
    /// Pointer to the data buffer.
    pub data: *const u8,
    /// Buffer length.
    pub len: usize,
}

impl FileData<'_> {
    /// Construct new FileData.
    pub fn new(file: &File, data: *const u8, len: usize) -> FileData {
        FileData { file, data, len }
    }
}

/// Determines if character in a seperator.
/// separators = "(" | ")" | "<" | ">" | "@"
/// | "," | ";" | ":" | "\" | <">
/// | "/" | "[" | "]" | "?" | "="
/// | "{" | "}" | SP | HT
pub fn is_separator(c: u8) -> bool {
    match c as char {
        '(' | ')' | '<' | '>' | '@' | ',' | ';' | ':' | '\\' | '"' | '/' | '[' | ']' | '?'
        | '=' | '{' | '}' | ' ' | '\t' => true,
        _ => false,
    }
}

/// Determines if character is a token.
/// token = 1*<any CHAR except CTLs or separators>
/// CHAR  = <any US-ASCII character (octets 0 - 127)>
pub fn is_token(c: u8) -> bool {
    !(c < 32 || c > 126 || is_separator(c))
}

/// This parser takes leading whitespace as defined by is_ascii_whitespace.
pub fn take_ascii_whitespace() -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    move |input| take_while(|c: u8| c.is_ascii_whitespace())(input)
}

/// Remove all line terminators (LF, CR or CRLF) from
/// the end of the line provided as input.
pub fn chomp(mut data: &[u8]) -> &[u8] {
    loop {
        let last_char = data.last();
        if last_char == Some(&(b'\n')) || last_char == Some(&(b'\r')) {
            data = &data[..data.len() - 1];
        } else {
            break;
        }
    }
    data
}

/// Determines if character is a whitespace character.
/// whitespace = ' ' | '\t' | '\r' | '\n' | '\x0b' | '\x0c'
pub fn is_space(c: u8) -> bool {
    match c as char {
        ' ' | '\t' | '\r' | '\n' | '\x0b' | '\x0c' => true,
        _ => false,
    }
}

/// Helper function that mimics the functionality of bytes::complete::take_until, ignoring tag case
/// Returns the longest input slice till it case insensitively matches the pattern. It doesn't consume the pattern.
///
/// Returns a tuple of the unconsumed data and the data up to but not including the input tag (if present)
pub fn take_until_no_case(tag: &[u8]) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> + '_ {
    move |input| {
        if tag.is_empty() {
            return Ok((b"", input));
        }
        let mut new_input = input;
        let mut bytes_consumed: usize = 0;
        while !new_input.is_empty() {
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

/// Is the given line empty?
///
/// Returns true or false
pub fn is_line_empty(data: &[u8]) -> bool {
    match data {
        b"\x0d" | b"\x0a" | b"\x0d\x0a" => true,
        _ => false,
    }
}

/// Determine if entire line is whitespace as defined by
/// util::is_space.
pub fn is_line_whitespace(data: &[u8]) -> bool {
    !data.iter().any(|c| !is_space(*c))
}

/// Searches for and extracts the next set of ascii digits from the input slice if present
/// Parses over leading and trailing LWS characters.
///
/// Returns (any trailing non-LWS characters, (non-LWS leading characters, ascii digits))
pub fn ascii_digits() -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    move |input| {
        map(
            tuple((
                nom_take_is_space,
                take_till(|c: u8| c.is_ascii_digit()),
                digit1,
                nom_take_is_space,
            )),
            |(_, leading_data, digits, _)| (leading_data, digits),
        )(input)
    }
}

/// Searches for and extracts the next set of hex digits from the input slice if present
/// Parses over leading and trailing LWS characters.
///
/// Returns a tuple of any trailing non-LWS characters and the found hex digits
pub fn hex_digits() -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    move |input| {
        map(
            tuple((
                nom_take_is_space,
                take_while1(|c: u8| c.is_ascii_hexdigit()),
                nom_take_is_space,
            )),
            |(_, digits, _)| digits,
        )(input)
    }
}

/// Determines if the given line is a continuation (of some previous line).
pub fn is_line_folded(data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }
    is_folding_char(data[0])
}

/// Determines if given character is folding.
/// folding characters = /t, ' ', '\0'
pub fn is_folding_char(c: u8) -> bool {
    nom_is_space(c) || c == 0
}

/// Determines if the given line is a request terminator.
pub fn is_line_terminator(
    server_personality: HtpServerPersonality,
    data: &[u8],
    next_no_lf: bool,
) -> bool {
    // Is this the end of request headers?
    if server_personality == HtpServerPersonality::IIS_5_0 {
        // IIS 5 will accept a whitespace line as a terminator
        if is_line_whitespace(data) {
            return true;
        }
    }

    // Treat an empty line as terminator
    if is_line_empty(data) {
        return true;
    }
    if data.len() == 2 && nom_is_space(data[0]) && data[1] == b'\n' {
        return next_no_lf;
    }
    false
}

/// Determines if the given line can be ignored when it appears before a request.
pub fn is_line_ignorable(server_personality: HtpServerPersonality, data: &[u8]) -> bool {
    is_line_terminator(server_personality, data, false)
}

/// Attempts to convert the provided port slice to a u16
///
/// Returns port number if a valid one is found. None if fails to convert or the result is 0
pub fn convert_port(port: &[u8]) -> Option<u16> {
    if port.is_empty() {
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

/// Convert two input bytes, pointed to by the pointer parameter,
/// into a single byte by assuming the input consists of hexadecimal
/// characters. This function will happily convert invalid input.
///
/// Returns hex-decoded byte
fn x2c(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, (c1, c2)) = tuple((be_u8, be_u8))(input)?;
    let mut decoded_byte = if c1 >= b'A' {
        ((c1 & 0xdf) - b'A') + 10
    } else {
        c1 - b'0'
    };
    decoded_byte = (decoded_byte as i32 * 16) as u8;
    decoded_byte += if c2 >= b'A' {
        ((c2 & 0xdf) - b'A') + 10
    } else {
        c2 - b'0'
    };
    Ok((input, decoded_byte))
}

/// Decode a UTF-8 encoded path. Replaces a possibly-invalid utf8 byte stream with
/// an ascii stream. Overlong characters will be decoded and invalid characters will
/// be replaced with the replacement byte specified in the cfg. Best-fit mapping will
/// be used to convert UTF-8 into a single-byte stream. The resulting decoded path will
/// be stored in the input path if the transaction cfg indicates it
pub fn utf8_decode_and_validate_uri_path_inplace(
    cfg: &DecoderConfig,
    flags: &mut u64,
    status: &mut HtpUnwanted,
    path: &mut Bstr,
) {
    let mut decoder = Utf8Decoder::new(cfg.bestfit_map);
    decoder.decode_and_validate(path.as_slice());
    if cfg.utf8_convert_bestfit {
        path.clear();
        path.add(decoder.decoded_bytes.as_slice());
    }
    flags.set(decoder.flags);

    if flags.is_set(HtpFlags::PATH_UTF8_INVALID) && cfg.utf8_invalid_unwanted != HtpUnwanted::IGNORE
    {
        *status = cfg.utf8_invalid_unwanted;
    }
}

/// Decode a %u-encoded character, using best-fit mapping as necessary. Path version.
///
/// Sets i to decoded byte
fn decode_u_encoding_path<'a>(
    i: &'a [u8],
    cfg: &DecoderConfig,
) -> IResult<&'a [u8], (u8, u64, HtpUnwanted)> {
    let mut flags = 0;
    let mut expected_status_code = HtpUnwanted::IGNORE;
    let (i, c1) = x2c(&i)?;
    let (i, c2) = x2c(&i)?;
    let mut r = c2;
    if c1 == 0 {
        flags.set(HtpFlags::PATH_OVERLONG_U)
    } else {
        // Check for fullwidth form evasion
        if c1 == 0xff {
            flags.set(HtpFlags::PATH_HALF_FULL_RANGE)
        }
        expected_status_code = cfg.u_encoding_unwanted;
        // Use best-fit mapping
        r = cfg.bestfit_map.get(bestfit_key!(c1, c2));
    }
    // Check for encoded path separators
    if r == b'/' || cfg.backslash_convert_slashes && r == b'\\' {
        flags.set(HtpFlags::PATH_ENCODED_SEPARATOR)
    }
    Ok((i, (r, flags, expected_status_code)))
}

/// Decode a %u-encoded character, using best-fit mapping as necessary. Params version.
///
/// Returns decoded byte
fn decode_u_encoding_params<'a>(i: &'a [u8], cfg: &DecoderConfig) -> IResult<&'a [u8], (u8, u64)> {
    let (i, c1) = x2c(&i)?;
    let (i, c2) = x2c(&i)?;
    let mut flags = 0;
    // Check for overlong usage first.
    if c1 == 0 {
        flags.set(HtpFlags::URLEN_OVERLONG_U);
        return Ok((i, (c2, flags)));
    }
    // Both bytes were used.
    // Detect half-width and full-width range.
    if c1 == 0xff && c2 <= 0xef {
        flags.set(HtpFlags::URLEN_HALF_FULL_RANGE)
    }
    // Use best-fit mapping.
    Ok((i, (cfg.bestfit_map.get(bestfit_key!(c1, c2)), flags)))
}

/// Decodes path valid uencoded params according to the given cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_valid_uencoding(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |remaining_input| {
        let (left, _) = tag_no_case("u")(remaining_input)?;
        let mut output = remaining_input;
        let mut byte = b'%';
        let mut flags = 0;
        let mut expected_status_code = HtpUnwanted::IGNORE;
        if cfg.u_encoding_decode {
            let (left, hex) = take_while_m_n(4, 4, |c: u8| c.is_ascii_hexdigit())(left)?;
            output = left;
            expected_status_code = cfg.u_encoding_unwanted;
            // Decode a valid %u encoding.
            let (_, (b, f, c)) = decode_u_encoding_path(hex, cfg)?;
            byte = b;
            flags.set(f);
            if c != HtpUnwanted::IGNORE {
                expected_status_code = c;
            }
            if byte == 0 {
                flags.set(HtpFlags::PATH_ENCODED_NUL);
                if cfg.nul_encoded_unwanted != HtpUnwanted::IGNORE {
                    expected_status_code = cfg.nul_encoded_unwanted
                }
                if cfg.nul_encoded_terminates {
                    // Terminate the path at the raw NUL byte.
                    return Ok((b"", (byte, expected_status_code, flags, false)));
                }
            }
        }
        let (byte, code) = path_decode_control(byte, cfg);
        if code != HtpUnwanted::IGNORE {
            expected_status_code = code;
        }
        Ok((output, (byte, expected_status_code, flags, true)))
    }
}

/// Decodes path invalid uencoded params according to the given cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_invalid_uencoding(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |remaining_input| {
        let mut output = remaining_input;
        let mut byte = b'%';
        let mut flags = 0;
        let mut expected_status_code = HtpUnwanted::IGNORE;
        let (left, _) = tag_no_case("u")(remaining_input)?;
        if cfg.u_encoding_decode {
            let (left, hex) = take(4usize)(left)?;
            // Invalid %u encoding
            flags = HtpFlags::PATH_INVALID_ENCODING;
            expected_status_code = cfg.url_encoding_invalid_unwanted;
            if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::REMOVE_PERCENT {
                // Do not place anything in output; consume the %.
                return Ok((remaining_input, (byte, expected_status_code, flags, false)));
            } else if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::PROCESS_INVALID {
                let (_, (b, f, c)) = decode_u_encoding_path(&hex, cfg)?;
                if c != HtpUnwanted::IGNORE {
                    expected_status_code = c;
                }
                flags.set(f);
                byte = b;
                output = left;
            }
        }
        let (byte, code) = path_decode_control(byte, cfg);
        if code != HtpUnwanted::IGNORE {
            expected_status_code = code;
        }
        Ok((output, (byte, expected_status_code, flags, true)))
    }
}

/// Decodes path valid hex according to the given cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn path_decode_valid_hex(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |remaining_input| {
        let original_remaining = remaining_input;
        // Valid encoding (2 xbytes)
        not(tag_no_case("u"))(remaining_input)?;
        let (mut left, hex) = take_while_m_n(2, 2, |c: u8| c.is_ascii_hexdigit())(remaining_input)?;
        let mut flags = 0;
        // Convert from hex.
        let (_, mut byte) = x2c(&hex)?;
        if byte == 0 {
            flags.set(HtpFlags::PATH_ENCODED_NUL);
            if cfg.nul_encoded_terminates {
                // Terminate the path at the raw NUL byte.
                return Ok((b"", (byte, cfg.nul_encoded_unwanted, flags, false)));
            }
        }
        if byte == b'/' || (cfg.backslash_convert_slashes && byte == b'\\') {
            flags.set(HtpFlags::PATH_ENCODED_SEPARATOR);
            if !cfg.path_separators_decode {
                // Leave encoded
                byte = b'%';
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
fn path_decode_invalid_hex(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |remaining_input| {
        let mut remaining = remaining_input;
        // Valid encoding (2 xbytes)
        not(tag_no_case("u"))(remaining_input)?;
        let (left, hex) = take(2usize)(remaining_input)?;
        let mut byte = b'%';
        // Invalid encoding
        let flags = HtpFlags::PATH_INVALID_ENCODING;
        let expected_status_code = cfg.url_encoding_invalid_unwanted;
        if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::REMOVE_PERCENT {
            // Do not place anything in output; consume the %.
            return Ok((remaining_input, (byte, expected_status_code, flags, false)));
        } else if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::PROCESS_INVALID {
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
fn path_decode_percent(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |i| {
        let (remaining_input, c) = char('%')(i)?;
        let byte = c as u8;
        alt((
            path_decode_valid_uencoding(cfg),
            path_decode_invalid_uencoding(cfg),
            move |remaining_input| {
                let (_, _) = tag_no_case("u")(remaining_input)?;
                // Invalid %u encoding (not enough data)
                let flags = HtpFlags::PATH_INVALID_ENCODING;
                let expected_status_code = cfg.url_encoding_invalid_unwanted;
                if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::REMOVE_PERCENT {
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
                        cfg.url_encoding_invalid_unwanted,
                        HtpFlags::PATH_INVALID_ENCODING,
                        cfg.url_encoding_invalid_handling != HtpUrlEncodingHandling::REMOVE_PERCENT,
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
fn path_parse_other(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |i| {
        let (remaining_input, byte) = be_u8(i)?;
        // One non-encoded byte.
        // Did we get a raw NUL byte?
        if byte == 0 && cfg.nul_raw_terminates {
            // Terminate the path at the encoded NUL byte.
            return Ok((b"", (byte, cfg.nul_raw_unwanted, 0, false)));
        }
        let (byte, expected_status_code) = path_decode_control(byte, cfg);
        Ok((remaining_input, (byte, expected_status_code, 0, true)))
    }
}
/// Checks for control characters and converts them according to the cfg settings
///
/// Returns decoded byte and expected_status_code
fn path_decode_control(mut byte: u8, cfg: &DecoderConfig) -> (u8, HtpUnwanted) {
    // Note: What if an invalid encoding decodes into a path
    //       separator? This is theoretical at the moment, because
    //       the only platform we know doesn't convert separators is
    //       Apache, who will also respond with 400 if invalid encoding
    //       is encountered. Thus no check for a separator here.
    // Place the character into output
    // Check for control characters
    let expected_status_code = if byte < 0x20 {
        cfg.control_chars_unwanted
    } else {
        HtpUnwanted::IGNORE
    };
    // Convert backslashes to forward slashes, if necessary
    if byte == b'\\' && cfg.backslash_convert_slashes {
        byte = b'/'
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
    cfg: &'a DecoderConfig,
) -> IResult<&'a [u8], (Vec<u8>, u64, HtpUnwanted)> {
    fold_many0(
        alt((path_decode_percent(cfg), path_parse_other(cfg))),
        (Vec::new(), 0, HtpUnwanted::IGNORE),
        |mut acc: (Vec<_>, u64, HtpUnwanted), (byte, code, flag, insert)| {
            // If we're compressing separators then we need
            // to check if the previous character was a separator
            if insert {
                if byte == b'/' && cfg.path_separators_compress {
                    if !acc.0.is_empty() {
                        if acc.0[acc.0.len() - 1] != b'/' {
                            acc.0.push(byte);
                        }
                    } else {
                        acc.0.push(byte);
                    }
                } else {
                    acc.0.push(byte);
                }
            }
            acc.1.set(flag);
            acc.2 = code;
            acc
        },
    )(input)
}

/// Decode the parsed uri path inplace according to the settings in the
/// transaction configuration structure.
pub fn decode_uri_path_inplace(
    decoder_cfg: &DecoderConfig,
    flag: &mut u64,
    status: &mut HtpUnwanted,
    path: &mut Bstr,
) {
    if let Ok((_, (consumed, flags, expected_status_code))) =
        path_decode(path.as_slice(), &decoder_cfg)
    {
        path.clear();
        path.add(consumed.as_slice());
        *status = expected_status_code;
        flag.set(flags);
    }
}

/// Performs decoding of the uri string, according to the configuration specified
/// by cfg. Various flags might be set.
pub fn urldecode_uri_inplace(
    decoder_cfg: &DecoderConfig,
    flags: &mut u64,
    input: &mut Bstr,
) -> Result<()> {
    if let Ok((_, (consumed, f, _))) = urldecode_ex(input.as_slice(), decoder_cfg) {
        (*input).clear();
        input.add(consumed.as_slice());
        if f.is_set(HtpFlags::URLEN_INVALID_ENCODING) {
            flags.set(HtpFlags::PATH_INVALID_ENCODING)
        }
        if f.is_set(HtpFlags::URLEN_ENCODED_NUL) {
            flags.set(HtpFlags::PATH_ENCODED_NUL)
        }
        if f.is_set(HtpFlags::URLEN_RAW_NUL) {
            flags.set(HtpFlags::PATH_RAW_NUL);
        }
        Ok(())
    } else {
        Err(HtpStatus::ERROR)
    }
}

/// Performs inplace url decoding of the input string and sets appropriate transaction flags.
pub fn tx_urldecode_params_inplace(tx: &mut Transaction, input: &mut Bstr) -> Result<()> {
    if let Ok((_, (consumed, flags, expected_status))) =
        urldecode_ex(input.as_slice(), &tx.cfg.decoder_cfg)
    {
        (*input).clear();
        input.add(consumed.as_slice());
        tx.flags.set(flags);
        tx.response_status_expected_number = expected_status;
        Ok(())
    } else {
        Err(HtpStatus::ERROR)
    }
}

/// Performs in-place decoding of the input string, according to the configuration specified
/// by cfg and ctx. On output, various flags (HTP_URLEN_*) might be set.
///
/// Returns OK on success, ERROR on failure.
pub fn urldecode_inplace(cfg: &DecoderConfig, input: &mut Bstr, flags: &mut u64) -> Result<()> {
    if let Ok((_, (consumed, flag, _))) = urldecode_ex(input.as_slice(), cfg) {
        (*input).clear();
        input.add(consumed.as_slice());
        flags.set(flag);
        Ok(())
    } else {
        Err(HtpStatus::ERROR)
    }
}

/// Decodes valid uencoded hex bytes according to the given cfg settings.
/// e.g. "u0064" -> "d"
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_valid_uencoding(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |input| {
        let (left, _) = alt((char('u'), char('U')))(input)?;
        if cfg.u_encoding_decode {
            let (input, hex) = take_while_m_n(4, 4, |c: u8| c.is_ascii_hexdigit())(left)?;
            let (_, (byte, flags)) = decode_u_encoding_params(hex, cfg)?;
            return Ok((input, (byte, cfg.u_encoding_unwanted, flags, true)));
        }
        Ok((input, (b'%', HtpUnwanted::IGNORE, 0, true)))
    }
}

/// Decodes invalid uencoded params according to the given cfg settings.
/// e.g. "u00}9" -> "i"
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_invalid_uencoding(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |mut input| {
        let (left, _) = alt((char('u'), char('U')))(input)?;
        let mut byte = b'%';
        let mut code = HtpUnwanted::IGNORE;
        let mut flags = 0;
        let mut insert = true;
        if cfg.u_encoding_decode {
            // Invalid %u encoding (could not find 4 xdigits).
            let (left, invalid_hex) = take(4usize)(left)?;
            flags.set(HtpFlags::URLEN_INVALID_ENCODING);
            code = if cfg.url_encoding_invalid_unwanted != HtpUnwanted::IGNORE {
                cfg.url_encoding_invalid_unwanted
            } else {
                cfg.u_encoding_unwanted
            };
            if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::REMOVE_PERCENT {
                // Do not place anything in output; consume the %.
                insert = false;
            } else if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::PROCESS_INVALID {
                let (_, (b, f)) = decode_u_encoding_params(invalid_hex, cfg)?;
                flags.set(f);
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
fn url_decode_valid_hex<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], (u8, HtpUnwanted, u64, bool)>
{
    move |input| {
        // Valid encoding (2 xbytes)
        not(alt((char('u'), char('U'))))(input)?;
        let (input, hex) = take_while_m_n(2, 2, |c: u8| c.is_ascii_hexdigit())(input)?;
        let (_, byte) = x2c(hex)?;
        Ok((input, (byte, HtpUnwanted::IGNORE, 0, true)))
    }
}

/// Decodes invalid hex byte according to the given cfg settings.
/// e.g. "}9" -> "i"
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_decode_invalid_hex(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |mut input| {
        not(alt((char('u'), char('U'))))(input)?;
        // Invalid encoding (2 bytes, but not hexadecimal digits).
        let mut byte = b'%';
        let mut insert = true;
        if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::REMOVE_PERCENT {
            // Do not place anything in output; consume the %.
            insert = false;
        } else if cfg.url_encoding_invalid_handling == HtpUrlEncodingHandling::PROCESS_INVALID {
            let (left, b) = x2c(input)?;
            input = left;
            byte = b;
        }
        Ok((
            input,
            (
                byte,
                cfg.url_encoding_invalid_unwanted,
                HtpFlags::URLEN_INVALID_ENCODING,
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
fn url_decode_percent(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |i| {
        let (input, _) = char('%')(i)?;
        let (input, (byte, mut expected_status_code, mut flags, insert)) = alt((
            url_decode_valid_uencoding(cfg),
            url_decode_invalid_uencoding(cfg),
            url_decode_valid_hex(),
            url_decode_invalid_hex(cfg),
            move |input| {
                // Invalid %u encoding; not enough data. (not even 2 bytes)
                // Do not place anything in output if REMOVE_PERCENT; consume the %.
                Ok((
                    input,
                    (
                        b'%',
                        cfg.url_encoding_invalid_unwanted,
                        HtpFlags::URLEN_INVALID_ENCODING,
                        !(cfg.url_encoding_invalid_handling
                            == HtpUrlEncodingHandling::REMOVE_PERCENT),
                    ),
                ))
            },
        ))(input)?;
        //Did we get an encoded NUL byte?
        if byte == 0 {
            flags.set(HtpFlags::URLEN_ENCODED_NUL);
            if cfg.nul_encoded_unwanted != HtpUnwanted::IGNORE {
                expected_status_code = cfg.nul_encoded_unwanted
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
fn url_decode_plus(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |input| {
        let (input, byte) = map(char('+'), |byte| {
            // Decoding of the plus character is conditional on the configuration.
            if cfg.plusspace_decode {
                0x20
            } else {
                byte as u8
            }
        })(input)?;
        Ok((input, (byte, HtpUnwanted::IGNORE, 0, true)))
    }
}

/// Consumes the next byte in the input string and treats it as an unencoded byte.
/// Handles raw null bytes according to the input cfg settings.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be output.
fn url_parse_unencoded_byte(
    cfg: &DecoderConfig,
) -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> + '_ {
    move |input| {
        let (input, byte) = be_u8(input)?;
        // One non-encoded byte.
        // Did we get a raw NUL byte?
        if byte == 0 {
            return Ok((
                if cfg.nul_raw_terminates { b"" } else { input },
                (
                    byte,
                    cfg.nul_raw_unwanted,
                    HtpFlags::URLEN_RAW_NUL,
                    !cfg.nul_raw_terminates,
                ),
            ));
        }
        Ok((input, (byte, HtpUnwanted::IGNORE, 0, true)))
    }
}

/// Performs decoding of the input string, according to the configuration specified
/// by cfg. Various flags (HTP_URLEN_*) might be set. If something in the input would
/// cause a particular server to respond with an error, the appropriate status
/// code will be set.
///
/// Returns decoded byte, corresponding status code, appropriate flags and whether the byte should be consumed or output.
fn urldecode_ex<'a>(
    input: &'a [u8],
    cfg: &'a DecoderConfig,
) -> IResult<&'a [u8], (Vec<u8>, u64, HtpUnwanted)> {
    fold_many0(
        alt((
            url_decode_percent(cfg),
            url_decode_plus(cfg),
            url_parse_unencoded_byte(cfg),
        )),
        (Vec::new(), 0, HtpUnwanted::IGNORE),
        |mut acc: (Vec<_>, u64, HtpUnwanted), (byte, code, flag, insert)| {
            if insert {
                acc.0.push(byte);
            }
            acc.1.set(flag);
            if code != HtpUnwanted::IGNORE {
                acc.2 = code;
            }
            acc
        },
    )(input)
}

/// Determine if the information provided on the response line
/// is good enough. Browsers are lax when it comes to response
/// line parsing. In most cases they will only look for the
/// words "http" at the beginning.
///
/// Returns true for good enough (treat as response body) or false for not good enough
pub fn treat_response_line_as_body(data: &[u8]) -> bool {
    // Browser behavior:
    //      Firefox 3.5.x: (?i)^\s*http
    //      IE: (?i)^\s*http\s*/
    //      Safari: ^HTTP/\d+\.\d+\s+\d{3}

    tuple((opt(take_is_space), tag_no_case("http")))(data).is_err()
}

/// Implements relaxed (not strictly RFC) hostname validation.
///
/// Returns true if the supplied hostname is valid; false if it is not.
pub fn validate_hostname(input: &[u8]) -> bool {
    if input.is_empty() || input.len() > 255 {
        return false;
    }
    if char::<_, (&[u8], nom::error::ErrorKind)>('[')(input).is_ok() {
        if let Ok((input, _)) = is_not::<_, _, (&[u8], nom::error::ErrorKind)>("#?/]")(input) {
            return char::<_, (&[u8], nom::error::ErrorKind)>(']')(input).is_ok();
        } else {
            return false;
        }
    }
    if tag::<_, _, (&[u8], nom::error::ErrorKind)>(".")(input).is_ok()
        || take_until::<_, _, (&[u8], nom::error::ErrorKind)>("..")(input).is_ok()
    {
        return false;
    }
    for section in input.split(|&c| c == b'.') {
        if section.len() > 63 {
            return false;
        }
        if take_while_m_n::<_, _, (&[u8], nom::error::ErrorKind)>(
            section.len(),
            section.len(),
            |c| c == b'-' || (c as char).is_alphanumeric(),
        )(section)
        .is_err()
        {
            return false;
        }
    }
    true
}

/// Returns the LibHTP version string.
pub fn get_version() -> *const i8 {
    HTP_VERSION_STRING_FULL.as_ptr() as *const i8
}

/// Splits by colon and removes leading whitespace from value
/// Returns header,value pair if succeeds.
pub fn split_by_colon(data: &[u8]) -> IResult<&[u8], &[u8]> {
    let (value, (header, _)) = tuple((take_until(":"), char(':')))(data)?;
    let (value, _) = nom_take_is_space(value)?;
    Ok((header, value))
}

/// Take leading whitespace as defined by nom_is_space.
pub fn nom_take_is_space(data: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(nom_is_space)(data)
}

/// Take data before the first null character if it exists.
pub fn take_until_null(data: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c != b'\0')(data)
}

/// Returns data without trailing whitespace as defined by util::is_space.
pub fn take_is_space_trailing(data: &[u8]) -> IResult<&[u8], &[u8]> {
    if let Some(index) = data.iter().rposition(|c| !is_space(*c)) {
        Ok((&data[..(index + 1)], &data[(index + 1)..]))
    } else {
        Ok((b"", data))
    }
}

/// Take leading space as defined by util::is_space.
pub fn take_is_space(data: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_space)(data)
}

/// Take any non-space character as defined by is_space.
pub fn take_not_is_space(data: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c: u8| !is_space(c))(data)
}

/// Returns true if each character is a token
pub fn is_word_token(data: &[u8]) -> bool {
    !data.iter().any(|c| !is_token(*c))
}

/// Returns all data up to and including the first new line or null
/// Returns Err if not found
pub fn take_till_lf_null(data: &[u8]) -> IResult<&[u8], &[u8]> {
    let res = streaming_take_till(|c| c == b'\n' || c == 0)(data);
    if let Ok((_, line)) = res {
        Ok((&data[line.len() + 1..], &data[0..line.len() + 1]))
    } else {
        res
    }
}

/// Returns all data up to and including the first new line
/// Returns Err if not found
pub fn take_till_lf(data: &[u8]) -> IResult<&[u8], &[u8]> {
    let res = streaming_take_till(|c| c == b'\n')(data);
    if let Ok((_, line)) = res {
        Ok((&data[line.len() + 1..], &data[0..line.len() + 1]))
    } else {
        res
    }
}

/// Returns all data up to and including the first lf or cr character
/// Returns Err if not found
pub fn take_not_eol(data: &[u8]) -> IResult<&[u8], &[u8]> {
    let res = streaming_take_while(|c: u8| c != b'\n' && c != b'\r')(data);
    if let Ok((_, line)) = res {
        Ok((&data[line.len() + 1..], &data[0..line.len() + 1]))
    } else {
        res
    }
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
