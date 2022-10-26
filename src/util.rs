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

use std::{io::Write, str::FromStr};
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
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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
#[derive(Debug)]
pub struct File {
    /// Where did this file come from? Possible values: MULTIPART and PUT.
    pub source: HtpFileSource,
    /// File name, as provided (e.g., in the Content-Disposition multipart part header.
    pub filename: Option<Bstr>,
    /// File length.
    pub len: usize,
    /// The file used for external storage.
    pub tmpfile: Option<NamedTempFile>,
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
        self.tmpfile = Some(
            Builder::new()
                .prefix("libhtp-multipart-file-")
                .rand_bytes(5)
                .tempfile_in(tmpfile)?,
        );
        Ok(())
    }

    /// Write data to tmpfile.
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        if let Some(tmpfile) = &mut self.tmpfile {
            tmpfile.write_all(data)?;
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

/// Trim the leading whitespace
pub fn trim_start(input: &[u8]) -> &[u8] {
    let mut result = input;
    while let Some(x) = result.first() {
        if is_space(*x) {
            result = &result[1..]
        } else {
            break;
        }
    }
    result
}

/// Trim the trailing whitespace
pub fn trim_end(input: &[u8]) -> &[u8] {
    let mut result = input;
    while let Some(x) = result.last() {
        if is_space(*x) {
            result = &result[..(result.len() - 1)]
        } else {
            break;
        }
    }
    result
}

/// Trim the leading and trailing whitespace from this byteslice.
pub fn trimmed(input: &[u8]) -> &[u8] {
    trim_end(trim_start(input))
}

/// Splits the given input into two halves using the given predicate.
/// The `reverse` parameter determines whether or not to split on the
/// first match or the second match.
/// The `do_trim` parameter will return results with leading and trailing
/// whitespace trimmed.
/// If the predicate does not match, then the entire input is returned
/// in the first predicate element and an empty binary string is returned
/// in the second element.
pub fn split_on_predicate<F>(
    input: &[u8],
    reverse: bool,
    do_trim: bool,
    predicate: F,
) -> (&[u8], &[u8])
where
    F: FnMut(&u8) -> bool,
{
    let (first, second) = if reverse {
        let mut iter = input.rsplitn(2, predicate);
        let mut second = iter.next();
        let mut first = iter.next();
        // If we do not get two results, then put the only result first
        if first.is_none() {
            first = second;
            second = None;
        }
        (first.unwrap_or(b""), second.unwrap_or(b""))
    } else {
        let mut iter = input.splitn(2, predicate);
        let first = iter.next();
        let second = iter.next();
        (first.unwrap_or(b""), second.unwrap_or(b""))
    };

    if do_trim {
        (trimmed(first), trimmed(second))
    } else {
        (first, second)
    }
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
            let (left, consumed) = take_till(|c: u8| {
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
        || (Vec::new(), 0, HtpUnwanted::IGNORE),
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
        || (Vec::new(), 0, HtpUnwanted::IGNORE),
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

    // Check IPv6
    if let Ok((_rest, (_left_br, addr, _right_br))) = tuple((
        char::<_, NomError<&[u8]>>('['),
        is_not::<_, _, NomError<&[u8]>>("#?/]"),
        char::<_, NomError<&[u8]>>(']'),
    ))(input)
    {
        if let Ok(str) = std::str::from_utf8(addr) {
            return std::net::Ipv6Addr::from_str(str).is_ok();
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
        // According to the RFC, an underscore it not allowed in the label, but
        // we allow it here because we think it's often seen in practice.
        if take_while_m_n::<_, _, NomError<&[u8]>>(section.len(), section.len(), |c| {
            c == b'_' || c == b'-' || (c as char).is_alphanumeric()
        })(section)
        .is_err()
        {
            return false;
        }
    }
    true
}

/// Returns the LibHTP version string.
pub fn get_version() -> &'static str {
    HTP_VERSION_STRING_FULL
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
        _ => Err(Incomplete(Needed::new(1))),
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

/// Check if the entire input line is chunked control characters
pub fn is_chunked_ctl_line(l: &[u8]) -> bool {
    for c in l {
        if !is_chunked_ctl_char(*c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, util::*};
    use rstest::rstest;

    #[rstest]
    #[case("", "", "")]
    #[case("hello world", "", "hello world")]
    #[case("\0", "\0", "")]
    #[case("hello_world  \0   ", "\0   ", "hello_world  ")]
    #[case("hello\0\0\0\0", "\0\0\0\0", "hello")]
    fn test_take_until_null(#[case] input: &str, #[case] remaining: &str, #[case] parsed: &str) {
        assert_eq!(
            take_until_null(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), parsed.as_bytes())
        );
    }

    #[rstest]
    #[case("", "", "")]
    #[case("word   \t", "word", "   \t")]
    #[case("word", "word", "")]
    #[case("\t  word   ", "\t  word", "   ")]
    #[case("     ", "", "     ")]
    fn test_is_space_trailing(#[case] input: &str, #[case] remaining: &str, #[case] parsed: &str) {
        assert_eq!(
            take_is_space_trailing(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), parsed.as_bytes())
        );
    }

    #[rstest]
    #[case("", "", "")]
    #[case("   hell o", "hell o", "   ")]
    #[case("   \thell o", "hell o", "   \t")]
    #[case("hell o", "hell o", "")]
    #[case("\r\x0b  \thell \to", "hell \to", "\r\x0b  \t")]
    fn test_take_is_space(#[case] input: &str, #[case] remaining: &str, #[case] parsed: &str) {
        assert_eq!(
            take_is_space(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), parsed.as_bytes())
        );
    }

    #[rstest]
    #[case("   http 1.1", false)]
    #[case("\0 http 1.1", false)]
    #[case("http", false)]
    #[case("HTTP", false)]
    #[case("    HTTP", false)]
    #[case("test", true)]
    #[case("     test", true)]
    #[case("", true)]
    #[case("kfgjl  hTtp ", true)]
    fn test_treat_response_line_as_body(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(treat_response_line_as_body(input.as_bytes()), expected);
    }

    #[rstest]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Error(Error { input: [], code: TakeUntil })"
    )]
    #[case("", "", "")]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Error(Error { input: [78, 111, 32, 99, 111, 108, 111, 110], code: TakeUntil })"
    )]
    #[case("No colon", "", "")]
    #[case("Content-Length: 230", "Content-Length", "230")]
    #[case(":No header name", "", "No header name")]
    fn test_split_by_colon(#[case] input: &str, #[case] remaining: &str, #[case] parsed: &str) {
        assert_eq!(
            split_by_colon(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), parsed.as_bytes())
        );
    }

    #[rstest]
    #[case("", true)]
    #[case("allalpha", true)]
    #[case("alpha567numeric1234", true)]
    #[case("234543", true)]
    #[case("content-length", true)]
    #[case("alpha{}", false)]
    #[case("\n", false)]
    #[case("abcdeg\t", false)]
    fn test_is_word_token(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_word_token(input.as_bytes()), expected);
    }

    #[rstest]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("header:value", "", "")]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("", "", "")]
    #[case("\nheader:value\r\n", "header:value\r\n", "\n")]
    #[case("header:value\r\n", "\n", "header:value\r")]
    #[case("header:value\n\r", "\r", "header:value\n")]
    #[case("header:value\n\n", "\n", "header:value\n")]
    #[case("header:value\r\r", "\r", "header:value\r")]
    #[case("abcdefg\nhijk", "hijk", "abcdefg\n")]
    fn test_take_not_eol(#[case] input: &str, #[case] remaining: &str, #[case] parsed: &str) {
        assert_eq!(
            take_not_eol(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), parsed.as_bytes())
        );
    }

    #[rstest]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("", "", "")]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("header:value\r\r", "", "")]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("header:value", "", "")]
    #[case("\nheader:value\r\n", "header:value\r\n", "\n")]
    #[case("header:value\r\n", "", "header:value\r\n")]
    #[case("header:value\n\r", "\r", "header:value\n")]
    #[case("header:value\n\n", "\n", "header:value\n")]
    #[case("abcdefg\nhijk", "hijk", "abcdefg\n")]
    fn test_take_till_lf(#[case] input: &str, #[case] remaining: &str, #[case] parsed: &str) {
        assert_eq!(
            take_till_lf(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), parsed.as_bytes())
        );
    }

    #[rstest]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("", "", "", Eol::CR)]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("abcdefg\n", "", "", Eol::CR)]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("abcdefg\n\r", "", "", Eol::CR)]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("abcdefg\r", "", "", Eol::CR)]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Incomplete(Size(1))")]
    #[case("abcdefg", "", "", Eol::CR)]
    #[case("abcdefg\nhijk", "hijk", "abcdefg\n", Eol::LF)]
    #[case("abcdefg\n\r\nhijk", "\r\nhijk", "abcdefg\n", Eol::LF)]
    #[case("abcdefg\rhijk", "hijk", "abcdefg\r", Eol::CR)]
    #[case("abcdefg\r\nhijk", "hijk", "abcdefg\r\n", Eol::CRLF)]
    #[case("abcdefg\r\n", "", "abcdefg\r\n", Eol::CRLF)]
    #[case("abcdefg\n\rhijk", "hijk", "abcdefg\n\r", Eol::LFCR)]
    #[case("abcdefg\n\r\r\nhijk", "\r\nhijk", "abcdefg\n\r", Eol::LFCR)]
    fn test_take_till_eol(
        #[case] input: &str,
        #[case] remaining: &str,
        #[case] parsed: &str,
        #[case] eol: Eol,
    ) {
        assert_eq!(
            take_till_eol(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), (parsed.as_bytes(), eol))
        );
    }

    #[rstest]
    #[case(b'a', false)]
    #[case(b'^', false)]
    #[case(b'-', false)]
    #[case(b'_', false)]
    #[case(b'&', false)]
    #[case(b'(', true)]
    #[case(b'\\', true)]
    #[case(b'/', true)]
    #[case(b'=', true)]
    #[case(b'\t', true)]
    fn test_is_separator(#[case] input: u8, #[case] expected: bool) {
        assert_eq!(is_separator(input), expected);
    }

    #[rstest]
    #[case(b'a', true)]
    #[case(b'&', true)]
    #[case(b'+', true)]
    #[case(b'\t', false)]
    #[case(b'\n', false)]
    fn test_is_token(#[case] input: u8, #[case] expected: bool) {
        assert_eq!(is_token(input), expected);
    }

    #[rstest]
    #[case("", "")]
    #[case("test\n", "test")]
    #[case("test\r\n", "test")]
    #[case("test\r\n\n", "test")]
    #[case("test\n\r\r\n\r", "test")]
    #[case("test", "test")]
    #[case("te\nst", "te\nst")]
    fn test_chomp(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(chomp(input.as_bytes()), expected.as_bytes());
    }

    #[rstest]
    #[case::trimmed(b"notrim", b"notrim")]
    #[case::trim_start(b"\t trim", b"trim")]
    #[case::trim_both(b" trim ", b"trim")]
    #[case::trim_both_ignore_middle(b" trim trim ", b"trim trim")]
    #[case::trim_end(b"trim \t", b"trim")]
    #[case::trim_empty(b"", b"")]
    fn test_trim(#[case] input: &[u8], #[case] expected: &[u8]) {
        assert_eq!(trimmed(input), expected);
    }

    #[rstest]
    #[case::non_space(0x61, false)]
    #[case::space(0x20, true)]
    #[case::form_feed(0x0c, true)]
    #[case::newline(0x0a, true)]
    #[case::carriage_return(0x0d, true)]
    #[case::tab(0x09, true)]
    #[case::vertical_tab(0x0b, true)]
    fn test_is_space(#[case] input: u8, #[case] expected: bool) {
        assert_eq!(is_space(input), expected);
    }

    #[rstest]
    #[case("", false)]
    #[case("arfarf", false)]
    #[case("\n\r", false)]
    #[case("\rabc", false)]
    #[case("\r\n", true)]
    #[case("\r", true)]
    #[case("\n", true)]
    fn test_is_line_empty(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_line_empty(input.as_bytes()), expected);
    }

    #[rstest]
    #[case("", false)]
    #[case("\tline", true)]
    #[case(" \t  line", true)]
    #[case(" line", true)]
    #[case("line ", false)]
    fn test_is_line_folded(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_line_folded(input.as_bytes()), expected);
    }

    #[rstest]
    #[case("", false)]
    #[case("www.ExAmplE-1984.com", true)]
    #[case("[::]", true)]
    #[case("[2001:3db8:0000:0000:0000:ff00:d042:8530]", true)]
    #[case("www.example.com", true)]
    #[case("www.exa-mple.com", true)]
    #[case("www.exa_mple.com", true)]
    #[case(".www.example.com", false)]
    #[case("www..example.com", false)]
    #[case("www.example.com..", false)]
    #[case("www example com", false)]
    #[case("[::", false)]
    #[case("[::/path[0]", false)]
    #[case("[::#garbage]", false)]
    #[case("[::?]", false)]
    #[case::over64_char(
        "www.exampleexampleexampleexampleexampleexampleexampleexampleexampleexample.com",
        false
    )]
    fn test_validate_hostname(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(validate_hostname(input.as_bytes()), expected);
    }

    #[rstest]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Error(Error { input: [], code: Digit })"
    )]
    #[case("   garbage no ascii ", "", "", "")]
    #[case("    a200 \t  bcd ", "bcd ", "a", "200")]
    #[case("   555555555    ", "", "", "555555555")]
    #[case("   555555555    500", "500", "", "555555555")]
    fn test_ascii_digits(
        #[case] input: &str,
        #[case] remaining: &str,
        #[case] leading: &str,
        #[case] digits: &str,
    ) {
        // Returns (any trailing non-LWS characters, (non-LWS leading characters, ascii digits))
        assert_eq!(
            ascii_digits()(input.as_bytes()).unwrap(),
            (
                remaining.as_bytes(),
                (leading.as_bytes(), digits.as_bytes())
            )
        );
    }

    #[rstest]
    #[case("", "", "")]
    #[case("12a5", "", "12a5")]
    #[case("12a5   .....", ".....", "12a5")]
    #[case("    \t12a5.....    ", ".....    ", "12a5")]
    #[case(" 68656c6c6f   12a5", "12a5", "68656c6c6f")]
    #[case("  .....", ".....", "")]
    fn test_hex_digits(#[case] input: &str, #[case] remaining: &str, #[case] digits: &str) {
        //(trailing non-LWS characters, found hex digits)
        assert_eq!(
            hex_digits()(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), digits.as_bytes())
        );
    }

    #[rstest]
    #[case("", "", "")]
    #[case("no chunked ctl chars here", "no chunked ctl chars here", "")]
    #[case(
        "\x0d\x0a\x20\x09\x0b\x0cno chunked ctl chars here",
        "no chunked ctl chars here",
        "\x0d\x0a\x20\x09\x0b\x0c"
    )]
    #[case(
        "no chunked ctl chars here\x20\x09\x0b\x0c",
        "no chunked ctl chars here\x20\x09\x0b\x0c",
        ""
    )]
    #[case(
        "\x20\x09\x0b\x0cno chunked ctl chars here\x20\x09\x0b\x0c",
        "no chunked ctl chars here\x20\x09\x0b\x0c",
        "\x20\x09\x0b\x0c"
    )]
    fn test_take_chunked_ctl_chars(
        #[case] input: &str,
        #[case] remaining: &str,
        #[case] hex_digits: &str,
    ) {
        //(trailing non-LWS characters, found hex digits)
        assert_eq!(
            take_chunked_ctl_chars(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), hex_digits.as_bytes())
        );
    }

    #[rstest]
    #[case("", true)]
    #[case("68656c6c6f", true)]
    #[case("\x0d\x0a\x20\x09\x0b\x0c68656c6c6f", true)]
    #[case("X5O!P%@AP", false)]
    #[case("\x0d\x0a\x20\x09\x0b\x0cX5O!P%@AP", false)]
    fn test_is_valid_chunked_length_data(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_valid_chunked_length_data(input.as_bytes()), expected);
    }

    #[rstest]
    #[case(
        "Let's fish for a Tag, but what about this TaG, or this TAG, or another tag. GO FISH.",
        "Tag, but what about this TaG, or this TAG, or another tag. GO FISH.",
        "Let's fish for a "
    )]
    #[case(
        ", but what about this TaG, or this TAG, or another tag. GO FISH.",
        "TaG, or this TAG, or another tag. GO FISH.",
        ", but what about this "
    )]
    #[case(
        ", or this TAG, or another tag. GO FISH.",
        "TAG, or another tag. GO FISH.",
        ", or this "
    )]
    #[case(", or another tag. GO FISH.", "tag. GO FISH.", ", or another ")]
    #[case(". GO FISH.", "", ". GO FISH.")]
    fn test_take_until_no_case(#[case] input: &str, #[case] remaining: &str, #[case] parsed: &str) {
        assert_eq!(
            take_until_no_case(b"TAG")(input.as_bytes()).unwrap(),
            (remaining.as_bytes(), parsed.as_bytes())
        );
    }

    #[rstest]
    #[case("/dest", "/dest", "/dest", "/dest")]
    #[case("/%64est", "/dest", "/dest", "/dest")]
    #[case("/%xxest", "/1est", "/%xxest", "/xxest")]
    #[case("/%a", "/%a", "/%a", "/a")]
    #[case("/%00ABC", "/\0ABC", "/\0ABC", "/\0ABC")]
    #[case("/%u0064", "/%u0064", "/%u0064", "/%u0064")]
    #[case("/%u006", "/%u006", "/%u006", "/%u006")]
    #[case("/%uXXXX", "/%uXXXX", "/%uXXXX", "/%uXXXX")]
    #[case("/%u0000ABC", "/%u0000ABC", "/%u0000ABC", "/%u0000ABC")]
    #[case("/\0ABC", "/\0ABC", "/\0ABC", "/\0ABC")]
    #[case("/one%2ftwo", "/one/two", "/one/two", "/one/two")]
    fn test_urldecode_ex(
        #[case] input: &str,
        #[case] expected_process: &str,
        #[case] expected_preserve: &str,
        #[case] expected_remove: &str,
    ) {
        let i = Bstr::from(input);
        let mut cfg = Config::default();

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        assert_eq!(
            urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0,
            expected_process.as_bytes()
        );

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        assert_eq!(
            urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0,
            expected_preserve.as_bytes()
        );

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        assert_eq!(
            urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0,
            expected_remove.as_bytes()
        );
    }

    #[rstest]
    #[case("/dest", "/dest", "/dest", "/dest")]
    #[case("/%64est", "/dest", "/dest", "/dest")]
    #[case("/%xxest", "/1est", "/%xxest", "/xxest")]
    #[case("/%a", "/%a", "/%a", "/a")]
    #[case("/%00ABC", "/\0ABC", "/\0ABC", "/\0ABC")]
    #[case("/%u0064", "/d", "/d", "/d")]
    #[case("/%U0064", "/d", "/d", "/d")]
    #[case("/%u006", "/%u006", "/%u006", "/u006")]
    #[case("/%uXXXX", "/?", "/%uXXXX", "/uXXXX")]
    #[case("/%u0000ABC", "/\0ABC", "/\0ABC", "/\0ABC")]
    #[case("/\0ABC", "/\0ABC", "/\0ABC", "/\0ABC")]
    #[case("/one%2ftwo", "/one/two", "/one/two", "/one/two")]
    fn test_urldecode_ex_decode(
        #[case] input: &str,
        #[case] expected_process: &str,
        #[case] expected_preserve: &str,
        #[case] expected_remove: &str,
    ) {
        let i = Bstr::from(input);
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        assert_eq!(
            urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0,
            expected_process.as_bytes()
        );

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        assert_eq!(
            urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0,
            expected_preserve.as_bytes()
        );

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        assert_eq!(
            urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0,
            expected_remove.as_bytes()
        );
    }

    #[rstest]
    #[case("/%u0000ABC")]
    #[case("/%00ABC")]
    #[case("/\0ABC")]
    fn test_urldecode_ex_nul_terminates(#[case] input: &str) {
        let i = Bstr::from(input);
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_nul_encoded_terminates(true);
        cfg.set_nul_raw_terminates(true);
        assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, b"/");
    }

    #[rstest]
    #[case("/dest", "/dest", "/dest", "/dest", 0)]
    #[case("/%64est", "/dest", "/dest", "/dest", 0)]
    #[case(
        "/%xxest",
        "/1est",
        "/%xxest",
        "/xxest",
        HtpFlags::PATH_INVALID_ENCODING
    )]
    #[case("/%a", "/%a", "/%a", "/a", HtpFlags::PATH_INVALID_ENCODING)]
    #[case("/%00ABC", "/\0ABC", "/\0ABC", "/\0ABC", HtpFlags::PATH_ENCODED_NUL)]
    #[case("/%u0064", "/%u0064", "/%u0064", "/%u0064", 0)]
    #[case("/%u006", "/%u006", "/%u006", "/%u006", 0)]
    #[case("/%uXXXX", "/%uXXXX", "/%uXXXX", "/%uXXXX", 0)]
    #[case("/%u0000ABC", "/%u0000ABC", "/%u0000ABC", "/%u0000ABC", 0)]
    #[case("/\0ABC", "/\0ABC", "/\0ABC", "/\0ABC", 0)]
    #[case(
        "/one%2ftwo",
        "/one%2ftwo",
        "/one%2ftwo",
        "/one%2ftwo",
        HtpFlags::PATH_ENCODED_SEPARATOR
    )]
    fn test_decode_uri_path_inplace(
        #[case] input: &str,
        #[case] expected_process: &str,
        #[case] expected_preserve: &str,
        #[case] expected_remove: &str,
        #[case] flags: u64,
    ) {
        let mut cfg = Config::default();
        let mut response_status_expected_number = HtpUnwanted::IGNORE;

        let mut input_process = Bstr::from(input);
        let mut flags_process = 0;
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags_process,
            &mut response_status_expected_number,
            &mut input_process,
        );
        assert_eq!(input_process, Bstr::from(expected_process));
        assert_eq!(flags_process, flags);

        let mut input_preserve = Bstr::from(input);
        let mut flags_preserve = 0;
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags_preserve,
            &mut response_status_expected_number,
            &mut input_preserve,
        );
        assert_eq!(input_preserve, Bstr::from(expected_preserve));
        assert_eq!(flags_preserve, flags);

        let mut input_remove = Bstr::from(input);
        let mut flags_remove = 0;
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags_remove,
            &mut response_status_expected_number,
            &mut input_remove,
        );
        assert_eq!(input_remove, Bstr::from(expected_remove));
        assert_eq!(flags_remove, flags);
    }

    #[rstest]
    #[case("/dest", "/dest", "/dest", "/dest", 0)]
    #[case("/%64est", "/dest", "/dest", "/dest", 0)]
    #[case(
        "/%xxest",
        "/1est",
        "/%xxest",
        "/xxest",
        HtpFlags::PATH_INVALID_ENCODING
    )]
    #[case("/%a", "/%a", "/%a", "/a", HtpFlags::PATH_INVALID_ENCODING)]
    #[case("/%00ABC", "/\0ABC", "/\0ABC", "/\0ABC", HtpFlags::PATH_ENCODED_NUL)]
    #[case("/%u0064", "/d", "/d", "/d", HtpFlags::PATH_OVERLONG_U)]
    #[case("/%U0064", "/d", "/d", "/d", HtpFlags::PATH_OVERLONG_U)]
    #[case("/%u006", "/%u006", "/%u006", "/u006", HtpFlags::PATH_INVALID_ENCODING)]
    #[case("/%uXXXX", "/?", "/%uXXXX", "/uXXXX", HtpFlags::PATH_INVALID_ENCODING)]
    #[case("/%u0000ABC", "/\0ABC", "/\0ABC", "/\0ABC", HtpFlags::PATH_ENCODED_NUL | HtpFlags::PATH_OVERLONG_U)]
    #[case("/\0ABC", "/\0ABC", "/\0ABC", "/\0ABC", 0)]
    #[case(
        "/one%2ftwo",
        "/one%2ftwo",
        "/one%2ftwo",
        "/one%2ftwo",
        HtpFlags::PATH_ENCODED_SEPARATOR
    )]
    fn test_decode_uri_path_inplace_decode(
        #[case] input: &str,
        #[case] expected_process: &str,
        #[case] expected_preserve: &str,
        #[case] expected_remove: &str,
        #[case] flags: u64,
    ) {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        let mut response_status_expected_number = HtpUnwanted::IGNORE;

        let mut input_process = Bstr::from(input);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let mut flags_process = 0;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags_process,
            &mut response_status_expected_number,
            &mut input_process,
        );
        assert_eq!(input_process, Bstr::from(expected_process));
        assert_eq!(flags_process, flags);

        let mut input_preserve = Bstr::from(input);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let mut flags_preserve = 0;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags_preserve,
            &mut response_status_expected_number,
            &mut input_preserve,
        );
        assert_eq!(input_preserve, Bstr::from(expected_preserve));
        assert_eq!(flags_preserve, flags);

        let mut input_remove = Bstr::from(input);
        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        let mut flags_remove = 0;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags_remove,
            &mut response_status_expected_number,
            &mut input_remove,
        );
        assert_eq!(input_remove, Bstr::from(expected_remove));
        assert_eq!(flags_remove, flags);
    }

    #[rstest]
    #[case("/%u0000ABC", HtpFlags::PATH_ENCODED_NUL | HtpFlags::PATH_OVERLONG_U)]
    #[case("/%00ABC", HtpFlags::PATH_ENCODED_NUL)]
    #[case("/\0ABC", 0)]
    fn test_decode_uri_path_inplace_nul_terminates(
        #[case] input: &str,
        #[case] expected_flags: u64,
    ) {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);
        cfg.set_nul_encoded_terminates(true);
        cfg.set_nul_raw_terminates(true);
        let mut i = Bstr::from(input);
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, Bstr::from("/"));
        assert_eq!(flags, expected_flags);
    }

    #[rstest]
    #[case::encoded("/one%2ftwo")]
    #[case::convert("/one\\two")]
    #[case::compress("/one//two")]
    fn test_decode_uri_path_inplace_seps(#[case] input: &str) {
        let mut cfg = Config::default();
        cfg.set_backslash_convert_slashes(true);
        cfg.set_path_separators_decode(true);
        cfg.set_path_separators_compress(true);
        let mut i = Bstr::from(input);
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        decode_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, Bstr::from("/one/two"));
    }

    #[rstest]
    #[case(b"\xf1.\xf1\xef\xbd\x9dabcd", "?.?}abcd")]
    //1111 0000 1001 0000 1000 1101 1111 1111
    #[case::invalid_incomplete_seq(b"\xf0\x90\x8d\xff", "??")]
    //1110 0010 1000 0010
    #[case::invalid_incomplete_seq(b"\xe2\x82", "?")]
    //1100 0010 1111 1111 1111 0000
    #[case::invalid_incomplete_seq(b"\xc2\xff\xf0", "??")]
    //1111 0000 1001 0000 0010 1000 1011 1100
    #[case::invalid_incomplete_seq(b"\xf0\x90\x28\xbc", "?(?")]
    fn test_utf8_decode_and_validate_uri_path_inplace(
        #[case] input: &[u8],
        #[case] expected: &str,
    ) {
        let mut cfg = Config::default();
        cfg.set_utf8_convert_bestfit(true);
        let mut i = Bstr::from(input);
        let mut flags = 0;
        let mut response_status_expected_number = HtpUnwanted::IGNORE;
        utf8_decode_and_validate_uri_path_inplace(
            &cfg.decoder_cfg,
            &mut flags,
            &mut response_status_expected_number,
            &mut i,
        );
        assert_eq!(i, Bstr::from(expected));
    }

    #[rstest]
    #[case(
        "/one/tw%u006f/three/%u123",
        "/one/two/three/%u123",
        "/one/two/three/%u123",
        "/one/two/three/u123"
    )]
    #[case(
        "/one/tw%u006f/three/%3",
        "/one/two/three/%3",
        "/one/two/three/%3",
        "/one/two/three/3"
    )]
    #[case(
        "/one/tw%u006f/three/%uXXXX",
        "/one/two/three/?",
        "/one/two/three/%uXXXX",
        "/one/two/three/uXXXX"
    )]
    fn test_urldecode_inplace(
        #[case] input: &str,
        #[case] expected_process: &str,
        #[case] expected_preserve: &str,
        #[case] expected_remove: &str,
    ) {
        let mut cfg = Config::default();
        cfg.set_u_encoding_decode(true);

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        let mut input_process = Bstr::from(input);
        urldecode_inplace(&cfg.decoder_cfg, &mut input_process).unwrap();
        assert_eq!(input_process, Bstr::from(expected_process));

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        let mut input_preserve = Bstr::from(input);
        urldecode_inplace(&cfg.decoder_cfg, &mut input_preserve).unwrap();
        assert_eq!(input_preserve, Bstr::from(expected_preserve));

        cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        let mut input_remove = Bstr::from(input);
        urldecode_inplace(&cfg.decoder_cfg, &mut input_remove).unwrap();
        assert_eq!(input_remove, Bstr::from(expected_remove));
    }

    #[rstest]
    #[case("", false, true, ("", ""))]
    #[case("ONE TWO THREE", false, true, ("ONE", "TWO THREE"))]
    #[case("ONE TWO THREE", true, true, ("ONE TWO", "THREE"))]
    #[case("ONE   TWO   THREE", false, true, ("ONE", "TWO   THREE"))]
    #[case("ONE   TWO   THREE", true, true, ("ONE   TWO", "THREE"))]
    #[case("ONE", false, true, ("ONE", ""))]
    #[case("ONE", true, true, ("ONE", ""))]
    fn test_split_on_predicate(
        #[case] input: &str,
        #[case] reverse: bool,
        #[case] trim: bool,
        #[case] expected: (&str, &str),
    ) {
        assert_eq!(
            split_on_predicate(input.as_bytes(), reverse, trim, |c| *c == 0x20),
            (expected.0.as_bytes(), expected.1.as_bytes())
        );
    }
}
