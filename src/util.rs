//! Utility functions for http parsing.

use crate::{
    bstr::Bstr,
    config::{DecoderConfig, HtpServerPersonality, HtpUnwanted, HtpUrlEncodingHandling},
    error::{NomError, Result},
    hook::FileDataHook,
    utf8_decoder::Utf8Decoder,
};
use nom::{
    branch::alt,
    bytes::complete::{
        is_not, tag, tag_no_case, take, take_till, take_until, take_while, take_while1,
        take_while_m_n,
    },
    bytes::streaming::{
        tag as streaming_tag, take_till as streaming_take_till, take_while as streaming_take_while,
    },
    character::complete::{char, digit1},
    character::is_space as nom_is_space,
    combinator::{map, not, opt, peek},
    multi::fold_many0,
    number::complete::be_u8,
    sequence::tuple,
    Err::Incomplete,
    IResult, Needed,
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
    /// Missing bytes in request and/or response data.
    pub const MISSING_BYTES: u64 = 0x0008_0000_0000;
    /// Missing bytes in request data.
    pub const REQUEST_MISSING_BYTES: u64 = (0x0010_0000_0000 | Self::MISSING_BYTES);
    /// Missing bytes in the response data.
    pub const RESPONSE_MISSING_BYTES: u64 = (0x0020_0000_0000 | Self::MISSING_BYTES);
}

/// Enumerates file sources.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum HtpFileSource {
    /// File from a multipart request.
    MULTIPART = 1,
    /// File from a request body.
    REQUEST_BODY = 2,
}

/// Enumerates possible EOLs
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Eol {
    /// No specific EOL found
    None,
    /// '\n'
    LF,
    /// '\r'
    CR,
    /// "\r\n"
    CRLF,
    /// "\n\r"
    LFCR,
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

    /// Update file length and invoke any file data callbacks on the provided cfg
    pub fn handle_file_data(
        &mut self,
        hook: FileDataHook,
        data: *const u8,
        len: usize,
    ) -> Result<()> {
        self.len = self.len.wrapping_add(len);
        // Package data for the callbacks.
        let mut file_data = FileData::new(self, data, len);
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
    matches!(
        c as char,
        '(' | ')'
            | '<'
            | '>'
            | '@'
            | ','
            | ';'
            | ':'
            | '\\'
            | '"'
            | '/'
            | '['
            | ']'
            | '?'
            | '='
            | '{'
            | '}'
            | ' '
            | '\t'
    )
}

/// Determines if character is a token.
/// token = 1*<any CHAR except CTLs or separators>
/// CHAR  = <any US-ASCII character (octets 0 - 127)>
pub fn is_token(c: u8) -> bool {
    (32..=126).contains(&c) && !is_separator(c)
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
    matches!(c as char, ' ' | '\t' | '\r' | '\n' | '\x0b' | '\x0c')
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
            let (left, consumed) = take_till::<_, _, NomError<&[u8]>>(|c: u8| {
                c.to_ascii_lowercase() == tag[0] || c.to_ascii_uppercase() == tag[0]
            })(new_input)?;
            new_input = left;
            bytes_consumed = bytes_consumed.wrapping_add(consumed.len());
            if tag_no_case::<_, _, NomError<&[u8]>>(tag)(new_input).is_ok() {
                return Ok((new_input, &input[..bytes_consumed]));
            } else if let Ok((left, consumed)) = take::<_, _, NomError<&[u8]>>(1usize)(new_input) {
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
    matches!(data, b"\x0d" | b"\x0a" | b"\x0d\x0a")
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
                take_while(|c: u8| c.is_ascii_hexdigit()),
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
    let port_number = std::str::from_utf8(port).ok()?.parse::<u16>().ok()?;
    if port_number == 0 {
        None
    } else {
        Some(port_number)
    }
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
    let (i, c1) = x2c(i)?;
    let (i, c2) = x2c(i)?;
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
    let (i, c1) = x2c(i)?;
    let (i, c2) = x2c(i)?;
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
                let (_, (b, f, c)) = decode_u_encoding_path(hex, cfg)?;
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
        let (_, mut byte) = x2c(hex)?;
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
            let (_, b) = x2c(hex)?;
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
    cfg: &DecoderConfig,
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
        path_decode(path.as_slice(), decoder_cfg)
    {
        path.clear();
        path.add(consumed.as_slice());
        *status = expected_status_code;
        flag.set(flags);
    }
}

/// Performs decoding of the uri string, according to the configuration specified
/// by cfg. Various flags might be set.
pub fn urldecode_uri(decoder_cfg: &DecoderConfig, flags: &mut u64, input: &[u8]) -> Result<Bstr> {
    let (_, (consumed, f, _)) = urldecode_ex(input, decoder_cfg)?;
    if f.is_set(HtpFlags::URLEN_INVALID_ENCODING) {
        flags.set(HtpFlags::PATH_INVALID_ENCODING)
    }
    if f.is_set(HtpFlags::URLEN_ENCODED_NUL) {
        flags.set(HtpFlags::PATH_ENCODED_NUL)
    }
    if f.is_set(HtpFlags::URLEN_RAW_NUL) {
        flags.set(HtpFlags::PATH_RAW_NUL);
    }
    Ok(Bstr::from(consumed))
}

/// Performs in-place decoding of the input string, according to the configuration specified by cfg and ctx.
///
/// Returns OK on success, ERROR on failure.
pub fn urldecode_inplace(cfg: &DecoderConfig, input: &mut Bstr) -> Result<()> {
    let (_, (consumed, _, _)) = urldecode_ex(input.as_slice(), cfg)?;
    (*input).clear();
    input.add(consumed.as_slice());
    Ok(())
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
fn url_decode_valid_hex() -> impl Fn(&[u8]) -> IResult<&[u8], (u8, HtpUnwanted, u64, bool)> {
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
/// Returns decoded bytes, flags set during decoding, and corresponding status code
pub fn urldecode_ex<'a>(
    input: &'a [u8],
    cfg: &DecoderConfig,
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

    tuple((opt(take_is_space_or_null), tag_no_case("http")))(data).is_err()
}

/// Implements relaxed (not strictly RFC) hostname validation.
///
/// Returns true if the supplied hostname is valid; false if it is not.
pub fn validate_hostname(input: &[u8]) -> bool {
    if input.is_empty() || input.len() > 255 {
        return false;
    }
    if char::<_, NomError<&[u8]>>('[')(input).is_ok() {
        if let Ok((input, _)) = is_not::<_, _, NomError<&[u8]>>("#?/]")(input) {
            return char::<_, NomError<&[u8]>>(']')(input).is_ok();
        } else {
            return false;
        }
    }
    if tag::<_, _, NomError<&[u8]>>(".")(input).is_ok()
        || take_until::<_, _, NomError<&[u8]>>("..")(input).is_ok()
    {
        return false;
    }
    for section in input.split(|&c| c == b'.') {
        if section.len() > 63 {
            return false;
        }
        if take_while_m_n::<_, _, NomError<&[u8]>>(section.len(), section.len(), |c| {
            c == b'-' || (c as char).is_alphanumeric()
        })(section)
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

/// Take leading null characters or spaces as defined by util::is_space
pub fn take_is_space_or_null(data: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| is_space(c) || c == b'\0')(data)
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
    let (_, line) = streaming_take_till(|c| c == b'\n' || c == 0)(data)?;
    Ok((&data[line.len() + 1..], &data[0..line.len() + 1]))
}

/// Returns all data up to and including the first new line
/// Returns Err if not found
pub fn take_till_lf(data: &[u8]) -> IResult<&[u8], &[u8]> {
    let (_, line) = streaming_take_till(|c| c == b'\n')(data)?;
    Ok((&data[line.len() + 1..], &data[0..line.len() + 1]))
}

/// Returns all data up to and including the first EOL and which EOL was seen
///
/// Returns Err if not found
pub fn take_till_eol(data: &[u8]) -> IResult<&[u8], (&[u8], Eol)> {
    let (_, (line, eol)) = tuple((
        streaming_take_till(|c| c == b'\n' || c == b'\r'),
        alt((
            streaming_tag("\r\n"),
            map(
                alt((
                    tuple((streaming_tag("\r"), not(streaming_tag("\n")))),
                    tuple((streaming_tag("\n\r"), not(streaming_tag("\n")))),
                    tuple((streaming_tag("\n"), not(streaming_tag("\r")))),
                )),
                |(eol, _)| eol,
            ),
            map(
                tuple((streaming_tag("\n"), peek(streaming_tag("\r\n")))),
                |(eol, _)| eol,
            ),
        )),
    ))(data)?;
    match eol {
        b"\n" => Ok((&data[line.len() + 1..], (&data[0..line.len() + 1], Eol::LF))),
        b"\r" => Ok((&data[line.len() + 1..], (&data[0..line.len() + 1], Eol::CR))),
        b"\r\n" => Ok((
            &data[line.len() + 2..],
            (&data[0..line.len() + 2], Eol::CRLF),
        )),
        b"\n\r" => Ok((
            &data[line.len() + 2..],
            (&data[0..line.len() + 2], Eol::LFCR),
        )),
        _ => Err(Incomplete(Needed::Size(1))),
    }
}

/// Returns all data up to and including the first lf or cr character
/// Returns Err if not found
pub fn take_not_eol(data: &[u8]) -> IResult<&[u8], &[u8]> {
    let (_, line) = streaming_take_while(|c: u8| c != b'\n' && c != b'\r')(data)?;
    Ok((&data[line.len() + 1..], &data[0..line.len() + 1]))
}

/// Skip control characters
pub fn take_chunked_ctl_chars(data: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_chunked_ctl_char)(data)
}

/// Check if the data contains valid chunked length chars, i.e. leading chunked ctl chars and ascii hexdigits
///
/// Returns true if valid, false otherwise
pub fn is_valid_chunked_length_data(data: &[u8]) -> bool {
    tuple((
        take_chunked_ctl_chars,
        take_while1(|c: u8| !c.is_ascii_hexdigit()),
    ))(data)
    .is_err()
}

fn is_chunked_ctl_char(c: u8) -> bool {
    matches!(c, 0x0d | 0x0a | 0x20 | 0x09 | 0x0b | 0x0c)
}

#[cfg(test)]
mod test {
    use crate::{config::Config, util::*};
    use nom::{
        error::ErrorKind::TakeUntil,
        Err::{Error, Incomplete},
        Needed,
    };

    #[test]
    fn TakeUntilNull() {
        assert_eq!(
            Ok(("\0   ".as_bytes(), "hello_world  ".as_bytes())),
            take_until_null(b"hello_world  \0   ")
        );
        assert_eq!(
            Ok(("\0\0\0\0".as_bytes(), "hello".as_bytes())),
            take_until_null(b"hello\0\0\0\0")
        );
        assert_eq!(Ok(("\0".as_bytes(), "".as_bytes())), take_until_null(b"\0"));
    }

    #[test]
    fn TakeIsSpaceTrailing() {
        assert_eq!(
            Ok(("w0rd".as_bytes(), "   ".as_bytes())),
            take_is_space_trailing(b"w0rd   ")
        );
        assert_eq!(
            Ok(("word".as_bytes(), "   \t".as_bytes())),
            take_is_space_trailing(b"word   \t")
        );
        assert_eq!(
            Ok(("w0rd".as_bytes(), "".as_bytes())),
            take_is_space_trailing(b"w0rd")
        );
        assert_eq!(
            Ok(("\t  w0rd".as_bytes(), "   ".as_bytes())),
            take_is_space_trailing(b"\t  w0rd   ")
        );
        assert_eq!(
            Ok(("".as_bytes(), "     ".as_bytes())),
            take_is_space_trailing(b"     ")
        );
    }

    #[test]
    fn TakeIsSpace() {
        assert_eq!(
            Ok(("hello".as_bytes(), "   ".as_bytes())),
            take_is_space(b"   hello")
        );
        assert_eq!(
            Ok(("hell o".as_bytes(), "   \t".as_bytes())),
            take_is_space(b"   \thell o")
        );
        assert_eq!(
            Ok(("hell o".as_bytes(), "".as_bytes())),
            take_is_space(b"hell o")
        );
        assert_eq!(
            Ok(("hell o".as_bytes(), "\r\x0b".as_bytes())),
            take_is_space(b"\r\x0bhell o")
        );
        assert_eq!(
            Ok(("hell \to".as_bytes(), "\r\x0b  \t".as_bytes())),
            take_is_space(b"\r\x0b  \thell \to")
        )
    }

    #[test]
    fn TreatResponseLineAsBody() {
        assert!(!treat_response_line_as_body(b"   http 1.1"));
        assert!(!treat_response_line_as_body(b"\0 http 1.1"));
        assert!(!treat_response_line_as_body(b"http"));
        assert!(!treat_response_line_as_body(b"HTTP"));
        assert!(!treat_response_line_as_body(b"    HTTP"));
        assert!(treat_response_line_as_body(b"test"));
        assert!(treat_response_line_as_body(b"     test"));
        assert!(treat_response_line_as_body(b""));
        assert!(treat_response_line_as_body(b"kfgjl  hTtp "));
    }

    #[test]
    fn RemoveLWS() {
        assert_eq!(
            Ok(("hello".as_bytes(), "   ".as_bytes())),
            take_is_space(b"   hello")
        );
        assert_eq!(
            Ok(("hell o".as_bytes(), "   \t".as_bytes())),
            take_is_space(b"   \thell o")
        );
        assert_eq!(
            Ok(("hell o".as_bytes(), "".as_bytes())),
            take_is_space(b"hell o")
        );
    }

    #[test]
    fn SplitByColon() {
        assert_eq!(
            Ok(("Content-Length".as_bytes(), "230".as_bytes())),
            split_by_colon(b"Content-Length: 230")
        );
        assert_eq!(
            Ok(("".as_bytes(), "No header name".as_bytes())),
            split_by_colon(b":No header name")
        );
        assert_eq!(
            Ok(("Header@Name".as_bytes(), "Not Token".as_bytes())),
            split_by_colon(b"Header@Name: Not Token")
        );
        assert_eq!(
            Err(Error(("No colon".as_bytes(), TakeUntil))),
            split_by_colon(b"No colon")
        );
    }

    #[test]
    fn IsWordToken() {
        assert!(is_word_token(b"allalpha"));
        assert!(is_word_token(b"alpha567numeric1234"));
        assert!(!is_word_token(b"alpha{}"));
        assert!(!is_word_token(b"\n"));
        assert!(is_word_token(b"234543"));
        assert!(!is_word_token(b"abcdeg\t"));
        assert!(is_word_token(b"content-length"));
    }

    #[test]
    fn TakeNotEol() {
        assert_eq!(
            Ok(("\n".as_bytes(), "header:value\r".as_bytes())),
            take_not_eol(b"header:value\r\n")
        );
        assert_eq!(
            Err(Incomplete(Needed::Size(1))),
            take_not_eol(b"header:value")
        );
    }

    #[test]
    fn TakeTillLF() {
        assert_eq!(
            Ok(("hijk".as_bytes(), "abcdefg\n".as_bytes())),
            take_till_lf(b"abcdefg\nhijk")
        );
        assert_eq!(Err(Incomplete(Needed::Size(1))), take_till_lf(b"abcdefg"));
    }

    #[test]
    fn TakeTillEol() {
        assert_eq!(
            Ok(("hijk".as_bytes(), ("abcdefg\n".as_bytes(), Eol::LF))),
            take_till_eol(b"abcdefg\nhijk")
        );
        assert_eq!(
            Ok(("\r\nhijk".as_bytes(), ("abcdefg\n".as_bytes(), Eol::LF))),
            take_till_eol(b"abcdefg\n\r\nhijk")
        );
        assert_eq!(
            Ok(("hijk".as_bytes(), ("abcdefg\r".as_bytes(), Eol::CR))),
            take_till_eol(b"abcdefg\rhijk")
        );

        assert_eq!(
            Ok(("hijk".as_bytes(), ("abcdefg\r\n".as_bytes(), Eol::CRLF))),
            take_till_eol(b"abcdefg\r\nhijk")
        );
        assert_eq!(
            Ok(("".as_bytes(), ("abcdefg\r\n".as_bytes(), Eol::CRLF))),
            take_till_eol(b"abcdefg\r\n")
        );

        assert_eq!(
            Ok(("hijk".as_bytes(), ("abcdefg\n\r".as_bytes(), Eol::LFCR))),
            take_till_eol(b"abcdefg\n\rhijk")
        );
        assert_eq!(
            Ok(("\r\nhijk".as_bytes(), ("abcdefg\n\r".as_bytes(), Eol::LFCR))),
            take_till_eol(b"abcdefg\n\r\r\nhijk")
        );
        assert_eq!(
            Err(Incomplete(Needed::Size(2))),
            take_till_eol(b"abcdefg\n")
        );

        assert_eq!(
            Err(Incomplete(Needed::Size(1))),
            take_till_eol(b"abcdefg\n\r")
        );
        assert_eq!(
            Err(Incomplete(Needed::Size(2))),
            take_till_eol(b"abcdefg\r")
        );
        assert_eq!(Err(Incomplete(Needed::Size(1))), take_till_eol(b"abcdefg"));
    }

    #[test]
    fn Separator() {
        assert!(!is_separator(b'a'));
        assert!(!is_separator(b'^'));
        assert!(!is_separator(b'-'));
        assert!(!is_separator(b'_'));
        assert!(!is_separator(b'&'));
        assert!(is_separator(b'('));
        assert!(is_separator(b'\\'));
        assert!(is_separator(b'/'));
        assert!(is_separator(b'='));
        assert!(is_separator(b'\t'));
    }

    #[test]
    fn Token() {
        assert!(is_token(b'a'));
        assert!(is_token(b'&'));
        assert!(is_token(b'+'));
        assert!(!is_token(b'\t'));
        assert!(!is_token(b'\n'));
    }

    #[test]
    fn Chomp() {
        assert_eq!(chomp(b"test\r\n"), b"test");
        assert_eq!(chomp(b"test\r\n\n"), b"test");
        assert_eq!(chomp(b"test\r\n\r\n"), b"test");
        assert_eq!(chomp(b"te\nst"), b"te\nst");
        assert_eq!(chomp(b"foo\n"), b"foo");
        assert_eq!(chomp(b"arfarf"), b"arfarf");
        assert_eq!(chomp(b""), b"");
    }

    #[test]
    fn Space() {
        assert!(!is_space(0x61)); // a
        assert!(is_space(0x20)); // space
        assert!(is_space(0x0c)); // Form feed
        assert!(is_space(0x0a)); // newline
        assert!(is_space(0x0d)); // carriage return
        assert!(is_space(0x09)); // tab
        assert!(is_space(0x0b)); // Vertical tab
    }

    #[test]
    fn IsLineEmpty() {
        let data = b"arfarf";
        assert!(!is_line_empty(data));
        assert!(is_line_empty(b"\x0d\x0a"));
        assert!(is_line_empty(b"\x0d"));
        assert!(is_line_empty(b"\x0a"));
        assert!(!is_line_empty(b"\x0a\x0d"));
        assert!(!is_line_empty(b"\x0dabc"));
    }

    #[test]
    fn IsLineWhitespace() {
        let data = b"arfarf";
        assert!(!is_line_whitespace(data));
        assert!(is_line_whitespace(b"\x0d\x0a"));
        assert!(is_line_whitespace(b"\x0d"));
        assert!(!is_line_whitespace(b"\x0dabc"));
    }

    #[test]
    fn IsLineFolded() {
        assert!(is_line_folded(b"\tline"));
        assert!(is_line_folded(b" line"));
        assert!(!is_line_folded(b"line "));
    }

    #[test]
    fn ValidateHostname_1() {
        assert!(validate_hostname(b"www.example.com"));
    }

    #[test]
    fn ValidateHostname_2() {
        assert!(!validate_hostname(b".www.example.com"));
    }

    #[test]
    fn ValidateHostname_3() {
        assert!(!validate_hostname(b"www..example.com"));
    }

    #[test]
    fn ValidateHostname_4() {
        assert!(!validate_hostname(b"www.example.com.."));
    }

    #[test]
    fn ValidateHostname_5() {
        assert!(!validate_hostname(b"www example com"));
    }

    #[test]
    fn ValidateHostname_6() {
        assert!(!validate_hostname(b""));
    }

    #[test]
    fn ValidateHostname_7() {
        // Label over 63 characters.
        assert!(!validate_hostname(
            b"www.exampleexampleexampleexampleexampleexampleexampleexampleexampleexample.com"
        ));
    }

    #[test]
    fn ValidateHostname_8() {
        assert!(validate_hostname(b"www.ExAmplE-1984.com"));
    }

    #[test]
    fn ValidateHostname_9() {
        assert!(validate_hostname(b"[:::]"));
    }

    #[test]
    fn ValidateHostname_10() {
        assert!(!validate_hostname(b"[:::"));
    }

    #[test]
    fn ValidateHostname_11() {
        assert!(!validate_hostname(b"[:::/path[0]"));
    }

    #[test]
    fn ValidateHostname_12() {
        assert!(!validate_hostname(b"[:::#garbage]"));
    }

    #[test]
    fn ValidateHostname_13() {
        assert!(!validate_hostname(b"[:::?]"));
    }

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
        assert_eq!(
            Ok((b".....".as_ref(), b"".as_ref())),
            hex_digits()(b"  .....")
        );
    }

    #[test]
    fn TakeChunkedCtlChars() {
        assert_eq!(
            Ok((b"no chunked ctl chars here".as_ref(), b"".as_ref())),
            take_chunked_ctl_chars(b"no chunked ctl chars here")
        );
        assert_eq!(
            Ok((
                b"no chunked ctl chars here".as_ref(),
                b"\x0d\x0a\x20\x09\x0b\x0c".as_ref()
            )),
            take_chunked_ctl_chars(b"\x0d\x0a\x20\x09\x0b\x0cno chunked ctl chars here")
        );
        assert_eq!(
            Ok((
                b"no chunked ctl chars here\x0d\x0a".as_ref(),
                b"\x20\x09\x0b\x0c".as_ref()
            )),
            take_chunked_ctl_chars(b"\x20\x09\x0b\x0cno chunked ctl chars here\x0d\x0a")
        );
    }

    #[test]
    fn IsValidChunkedLengthData() {
        assert!(is_valid_chunked_length_data(b"68656c6c6f"));
        assert!(is_valid_chunked_length_data(
            b"\x0d\x0a\x20\x09\x0b\x0c68656c6c6f"
        ));
        assert!(!is_valid_chunked_length_data(b"X5O!P%@AP"));
        assert!(!is_valid_chunked_length_data(
            b"\x0d\x0a\x20\x09\x0b\x0cX5O!P%@AP"
        ));
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
        let (remaining, _) = tag_no_case::<_, _, NomError<&[u8]>>("TAG")(remaining).unwrap();

        res_consumed = b", but what about this ";
        res_remaining = b"TaG, or this TAG, or another tag. GO FISH.";
        let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
        assert_eq!(res_consumed, consumed);
        assert_eq!(res_remaining, remaining);
        let (remaining, _) = tag_no_case::<_, _, NomError<&[u8]>>("TAG")(remaining).unwrap();

        res_consumed = b", or this ";
        res_remaining = b"TAG, or another tag. GO FISH.";
        let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
        assert_eq!(res_consumed, consumed);
        assert_eq!(res_remaining, remaining);
        let (remaining, _) = tag_no_case::<_, _, NomError<&[u8]>>("TAG")(remaining).unwrap();

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
        let (remaining, _) = tag_no_case::<_, _, NomError<&[u8]>>("TAG")(remaining).unwrap();

        res_consumed = b". GO FISH.";
        res_remaining = b"";
        let (remaining, consumed) = take_until_no_case(b"TAG")(remaining).unwrap();
        assert_eq!(res_consumed, consumed);
        assert_eq!(res_remaining, remaining);
    }

    #[test]
    fn DecodeUrlencodedEx1_Identity() {
        let i = Bstr::from("/dest");
        let e = "/dest".as_bytes();
        let cfg = DecoderConfig::default();
        assert_eq!(e, urldecode_ex(&i, &cfg).unwrap().1 .0);
    }

    #[test]
    fn DecodeUrlencodedEx2_Urlencoded() {
        let i = Bstr::from("/%64est");
        let e = "/dest".as_bytes();
        let cfg = DecoderConfig::default();
        assert_eq!(e, urldecode_ex(&i, &cfg).unwrap().1 .0);
    }

    #[test]
    fn DecodeUrlencodedEx3_UrlencodedInvalidPreserve() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let i = Bstr::from("/%xxest");
        let e = "/%xxest".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx4_UrlencodedInvalidRemove() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        let i = Bstr::from("/%xxest");
        let e = "/xxest".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx5_UrlencodedInvalidDecode() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let i = Bstr::from("/%}9est");
        let e = "/iest".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx6_UrlencodedInvalidNotEnoughBytes() {
        let cfg = DecoderConfig::default();
        let i = Bstr::from("/%a");
        let e = "/%a".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx7_UrlencodedInvalidNotEnoughBytes() {
        let cfg = DecoderConfig::default();
        let i = Bstr::from("/%");
        let e = "/%".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx8_Uencoded() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        let i = Bstr::from("/%u0064");
        let e = "/d".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx9_UencodedDoNotDecode() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        cfg.set_u_encoding_decode(false);
        let i = Bstr::from("/%u0064");
        let e = "/%u0064".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx10_UencodedInvalidNotEnoughBytes() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let i = Bstr::from("/%u006");
        let e = "/%u006".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx11_UencodedInvalidPreserve() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let i = Bstr::from("/%u006");
        let e = "/%u006".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx12_UencodedInvalidRemove() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        let i = Bstr::from("/%uXXXX");
        let e = "/uXXXX".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx13_UencodedInvalidDecode() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let i = Bstr::from("/%u00}9");
        let e = "/i".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx14_UencodedInvalidPreserve() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let i = Bstr::from("/%u00");
        let e = "/%u00".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx15_UencodedInvalidPreserve() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let i = Bstr::from("/%u0");
        let e = "/%u0".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx16_UencodedInvalidPreserve() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let i = Bstr::from("/%u");
        let e = "/%u".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx17_UrlencodedNul() {
        let cfg = DecoderConfig::default();
        let i = Bstr::from("/%00");
        let e = "/\0".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx18_UrlencodedNulTerminates() {
        let mut cfg = Config::default();
        cfg.set_nul_encoded_terminates(true);
        let i = Bstr::from("/%00ABC");
        let e = "/".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx19_RawNulTerminates() {
        let mut cfg = Config::default();
        cfg.set_nul_raw_terminates(true);
        let i = Bstr::from("/\0ABC");
        let e = "/".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx20_UencodedBestFit() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        let i = Bstr::from("/%u0107");
        let e = "/c".as_bytes();
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodeUrlencodedEx21_UencodedCaseInsensitive() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        let i_lower = Bstr::from("/%u0064");
        let i_upper = Bstr::from("/%U0064");
        let e = "/d".as_bytes();
        assert_eq!(urldecode_ex(&i_upper, &cfg.decoder_cfg).unwrap().1 .0, e);
        assert_eq!(urldecode_ex(&i_lower, &cfg.decoder_cfg).unwrap().1 .0, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace1_UrlencodedInvalidNotEnoughBytes() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let mut i = Bstr::from("/%a");
        let e = Bstr::from("/%a");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace2_UencodedInvalidNotEnoughBytes() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%uX");
        let e = Bstr::from("/%uX");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace3_UencodedValid() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%u0107");
        let e = Bstr::from("/c");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace4_UencodedInvalidNotHexDigits_Remove() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%uXXXX");
        let e = Bstr::from("/uXXXX");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace5_UencodedInvalidNotHexDigits_Preserve() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%uXXXX");
        let e = Bstr::from("/%uXXXX");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace6_UencodedInvalidNotHexDigits_Process() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%u00}9");
        let e = Bstr::from("/i");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace7_UencodedNul() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%u0000");
        let e = Bstr::from("/\0");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_ENCODED_NUL));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace8_UencodedNotEnough_Remove() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%uXXX");
        let e = Bstr::from("/uXXX");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace9_UencodedNotEnough_Preserve() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        cfg.set_u_encoding_decode(true);
        let mut i = Bstr::from("/%uXXX");
        let e = Bstr::from("/%uXXX");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace10_UrlencodedNul() {
        let mut i = Bstr::from("/%00123");
        let e = Bstr::from("/\x00123");
        let cfg = DecoderConfig::default();
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_ENCODED_NUL));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace11_UrlencodedNul_Terminates() {
        let mut cfg = Config::default();
        cfg.set_nul_encoded_terminates(true);
        let mut i = Bstr::from("/%00123");
        let e = Bstr::from("/");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_ENCODED_NUL));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace12_EncodedSlash() {
        let mut cfg = Config::default();
        cfg.set_path_separators_decode(false);
        let mut i = Bstr::from("/one%2ftwo");
        let e = Bstr::from("/one%2ftwo");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_ENCODED_SEPARATOR));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace13_EncodedSlash_Decode() {
        let mut cfg = Config::default();
        cfg.set_path_separators_decode(true);
        let mut i = Bstr::from("/one%2ftwo");
        let e = Bstr::from("/one/two");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_ENCODED_SEPARATOR));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace14_Urlencoded_Invalid_Preserve() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let mut i = Bstr::from("/%HH");
        let e = Bstr::from("/%HH");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace15_Urlencoded_Invalid_Remove() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        let mut i = Bstr::from("/%HH");
        let e = Bstr::from("/HH");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace16_Urlencoded_Invalid_Process() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let mut i = Bstr::from("/%}9");
        let e = Bstr::from("/i");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace17_Urlencoded_NotEnough_Remove() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        let mut i = Bstr::from("/%H");
        let e = Bstr::from("/H");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace18_Urlencoded_NotEnough_Preserve() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let mut i = Bstr::from("/%H");
        let e = Bstr::from("/%H");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace19_Urlencoded_NotEnough_Process() {
        let mut cfg = Config::default();
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let mut i = Bstr::from("/%H");
        let e = Bstr::from("/%H");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace20_RawNul1() {
        let mut cfg = Config::default();
        cfg.set_nul_raw_terminates(true);
        let mut i = Bstr::from("/\x00123");
        let e = Bstr::from("/");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace21_RawNul1() {
        let mut cfg = Config::default();
        cfg.set_nul_raw_terminates(false);
        let mut i = Bstr::from("/\x00123");
        let e = Bstr::from("/\x00123");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace22_ConvertBackslash1() {
        let mut cfg = Config::default();
        cfg.set_backslash_convert_slashes(true);
        let mut i = Bstr::from("/one\\two");
        let e = Bstr::from("/one/two");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace23_ConvertBackslash2() {
        let mut cfg = Config::default();
        cfg.set_backslash_convert_slashes(false);
        let mut i = Bstr::from("/one\\two");
        let e = Bstr::from("/one\\two");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_DecodePathInplace24_CompressSeparators() {
        let mut cfg = Config::default();
        cfg.set_path_separators_compress(true);
        let mut i = Bstr::from("/one//two");
        let e = Bstr::from("/one/two");
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, e);
    }

    #[test]
    fn DecodingTest_InvalidUtf8() {
        let mut cfg = Config::default();
        cfg.set_utf8_convert_bestfit(true);
        let mut i = Bstr::from(b"\xf1.\xf1\xef\xbd\x9dabcd".to_vec());
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        utf8_decode_and_validate_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(i.eq_slice("?.?}abcd"));
    }

    #[test]
    fn DecodingTest_InvalidUtf8_IncompleteInvalidSequence() {
        let mut cfg = Config::default();
        cfg.set_utf8_convert_bestfit(true);
        //1111 0000 1001 0000 1000 1101 1111 1111
        let mut i = Bstr::from(b"\xf0\x90\x8d\xff".to_vec());
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        utf8_decode_and_validate_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(i.eq_slice("??"));
    }

    #[test]
    fn DecodingTest_InvalidUtf8_IncompleteInvalidSequence2() {
        let mut cfg = Config::default();
        cfg.set_utf8_convert_bestfit(true);
        //1110 0010 1000 0010
        let mut i = Bstr::from(b"\xe2\x82".to_vec());
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        utf8_decode_and_validate_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(i.eq_slice("?"));
    }

    #[test]
    fn DecodingTest_InvalidUtf8_IncompleteInvalidSequence3() {
        let mut cfg = Config::default();
        cfg.set_utf8_convert_bestfit(true);
        //1100 0010 1111 1111 1111 0000
        let mut i = Bstr::from(b"\xc2\xff\xf0".to_vec());
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        utf8_decode_and_validate_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(i.eq_slice("??"));
    }

    #[test]
    fn DecodingTest_InvalidUtf8_IncompleteInvalidSequence4() {
        let mut cfg = Config::default();
        cfg.set_utf8_convert_bestfit(true);
        //1111 0000 1001 0000 0010 1000 1011 1100
        let mut i = Bstr::from(b"\xf0\x90\x28\xbc".to_vec());
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        utf8_decode_and_validate_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert!(i.eq_slice("?(?"));
    }

    #[test]
    fn UrlDecode() {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let mut s = Bstr::from("/one/tw%u006f/three/%u123");
        let mut e = Bstr::from("/one/two/three/%u123");

        urldecode_inplace(&cfg.decoder_cfg, &mut s).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%uXXXX");
        e = Bstr::from("/one/two/three/%uXXXX");
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        urldecode_inplace(&cfg.decoder_cfg, &mut s).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%u123");
        e = Bstr::from("/one/two/three/u123");
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        urldecode_inplace(&cfg.decoder_cfg, &mut s).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%3");
        e = Bstr::from("/one/two/three/3");
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        urldecode_inplace(&cfg.decoder_cfg, &mut s).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%3");
        e = Bstr::from("/one/two/three/%3");
        cfg.set_u_encoding_decode(true);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        urldecode_inplace(&cfg.decoder_cfg, &mut s).unwrap();
        assert_eq!(e, s);
    }
}
