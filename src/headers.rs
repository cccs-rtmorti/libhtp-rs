use crate::util::{is_token, take_until_null, Eol, FlagOperations};
use nom::{
    branch::alt,
    bytes::complete::tag as complete_tag,
    bytes::streaming::{tag, take_till, take_till1, take_while},
    character::is_space,
    character::streaming::{space0, space1},
    combinator::{map, not, peek},
    sequence::tuple,
    Err::Incomplete,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Flags;

impl Flags {
    pub const FOLDING: u64 = 0x0001;
    pub const FOLDING_SPECIAL_CASE: u64 = (0x0002 | Self::FOLDING);
    pub const NAME_EMPTY: u64 = 0x0004;
    pub const VALUE_EMPTY: u64 = 0x0008;
    pub const NAME_NON_TOKEN_CHARS: u64 = 0x0010;
    pub const NAME_TRAILING_WHITESPACE: u64 = 0x0020;
    pub const NAME_LEADING_WHITESPACE: u64 = 0x0040;
    pub const NULL_TERMINATED: u64 = 0x0080;
    pub const MISSING_COLON: u64 = (0x0100 | Self::NAME_EMPTY);
    pub const DEFORMED_EOL: u64 = 0x0200;
    pub const TERMINATOR_SPECIAL_CASE: u64 = 0x0400;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Name {
    pub name: Vec<u8>,
    pub flags: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub value: Vec<u8>,
    pub flags: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub name: Name,
    pub value: Value,
}

pub struct Parser {
    eol: Eol,
    null_terminates: bool,
}

impl Parser {
    pub fn new(eol: Eol, null_terminates: bool) -> Self {
        Self {
            eol,
            null_terminates,
        }
    }
    /// Returns true if c is a line feed character
    fn is_eol(&self) -> impl Fn(u8) -> bool + '_ {
        move |c| c == b'\n' || (self.eol == Eol::CR && c == b'\r')
    }

    /// Parse one complete end of line character or character set
    fn complete_eol_regular(&self) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> + '_ {
        move |input| {
            if self.eol == Eol::CR {
                alt((complete_tag("\r\n"), complete_tag("\n"), complete_tag("\r")))(input)
            } else {
                alt((complete_tag("\r\n"), complete_tag("\n")))(input)
            }
        }
    }

    /// Parse one complete deformed end of line character set
    fn complete_eol_deformed(&self) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> + '_ {
        move |input| {
            if self.eol == Eol::LFCR {
                alt((
                    map(
                        tuple((
                            complete_tag("\n\r\r\n"),
                            peek(alt((complete_tag("\n"), complete_tag("\r\n")))),
                        )),
                        |(eol, _)| eol,
                    ),
                    complete_tag("\n\r"),
                ))(input)
            } else {
                map(
                    alt((
                        tuple((
                            complete_tag("\n\r\r\n"),
                            peek(alt((complete_tag("\n"), complete_tag("\r\n")))),
                        )),
                        tuple((complete_tag("\n\r"), peek(complete_tag("\r\n")))),
                    )),
                    |(eol, _)| eol,
                )(input)
            }
        }
    }

    /// Parse one complete end of line character or character set
    fn complete_eol(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            alt((
                map(self.complete_eol_deformed(), |eol| {
                    (eol, Flags::DEFORMED_EOL)
                }),
                map(self.complete_eol_regular(), |eol| (eol, 0)),
            ))(input)
        }
    }

    /// Parse one header end of line, and guarantee that it is not folding
    fn eol(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            map(
                tuple((self.complete_eol(), not(folding_lws))),
                |(end, _)| end,
            )(input)
        }
    }

    /// Parse one null byte or one end of line, and guarantee that it is not folding
    fn null_or_eol(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| alt((null, self.eol()))(input)
    }

    /// Parse one null byte or complete end of line
    fn complete_null_or_eol(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| alt((null, self.complete_eol()))(input)
    }

    /// Parse header folding bytes (eol + whitespace or eol + special cases)
    fn folding(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], &[u8], u64)> + '_ {
        move |input| {
            map(
                tuple((self.complete_eol(), folding_lws)),
                |((eol, flags), (folding_lws, other_flags))| {
                    (eol, folding_lws, flags | other_flags)
                },
            )(input)
        }
    }

    /// Special case check for end of headers with space or tab seperating the EOLs
    fn terminator_special_case(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            if let Ok((remaining, ((eol1, flags1), space, (eol2, flags2), _))) =
                tuple((
                    self.complete_eol(),
                    alt((tag(" "), tag("\t"))),
                    self.complete_eol(),
                    peek(self.complete_eol()),
                ))(input)
            {
                Ok((
                    remaining,
                    (
                        &input[..eol1.len() + space.len() + eol2.len()],
                        flags1 | flags2 | Flags::TERMINATOR_SPECIAL_CASE,
                    ),
                ))
            } else {
                map(
                    tuple((
                        self.complete_eol(),
                        alt((tag(" "), tag("\t"))),
                        peek(self.complete_eol()),
                    )),
                    |((end, flags), _, _)| (end, flags | Flags::TERMINATOR_SPECIAL_CASE),
                )(input)
            }
        }
    }

    /// Parse folding bytes or a value terminator (eol or null)
    fn folding_or_terminator(
        &self,
    ) -> impl Fn(&[u8]) -> IResult<&[u8], ((&[u8], u64), Option<&[u8]>)> + '_ {
        move |input| {
            alt((
                map(self.terminator_special_case(), |result| (result, None)),
                map(self.folding(), |(end, fold, flags)| {
                    ((end, flags), Some(fold))
                }),
                map(self.null_or_eol(), |end| (end, None)),
            ))(input)
        }
    }

    /// Removes trailing unwanted characaters from input.
    /// If null terminates is set to true, it will remove all characters before the null character
    fn remove_trailing(&self, input: &mut Vec<u8>, flags: &mut u64) {
        if self.null_terminates {
            if let Ok((trailing_data, data)) = take_until_null(&input) {
                if trailing_data.first() == Some(&b'\0') {
                    flags.set(Flags::NULL_TERMINATED);
                }
                *input = data.to_vec();
            }
        }
        while let Some(end) = input.last() {
            if is_space(*end) {
                input.pop();
            } else {
                break;
            }
        }
    }

    /// Parse a header value.
    /// Returns the bytes and the value terminator; null, eol or folding
    /// eg. (bytes, (eol_bytes, Option<fold_bytes>))
    fn value_bytes(
        &self,
    ) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], ((&[u8], u64), Option<&[u8]>))> + '_ {
        move |input| {
            let (mut remaining, mut value) = take_till(self.is_eol())(input)?;
            if value.last() == Some(&b'\r') {
                value = &value[..value.len() - 1];
                remaining = &input[value.len()..];
            }
            let (remaining, result) = self.folding_or_terminator()(remaining)?;
            Ok((remaining, (value, result)))
        }
    }

    /// Parse a complete header value, including any folded headers
    fn value(&self) -> impl Fn(&[u8]) -> IResult<&[u8], Value> + '_ {
        move |input| {
            let (rest, (val_bytes, ((_eol, mut flags), fold))) = self.value_bytes()(input)?;
            let mut value = val_bytes.to_vec();
            if fold.is_some() {
                let mut i = rest;
                loop {
                    match self.value_bytes()(i) {
                        Ok((rest, (val_bytes, ((_eol, other_flags), fold)))) => {
                            i = rest;
                            flags.set(other_flags);
                            //If the value is empty, the value started with a fold and we don't want to push back a space
                            if !value.is_empty() {
                                value.push(b' ');
                            }
                            value.extend(val_bytes);
                            if fold.is_none() {
                                self.remove_trailing(&mut value, &mut flags);
                                return Ok((rest, Value { value, flags }));
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
            } else {
                if value.is_empty() {
                    flags.set(Flags::VALUE_EMPTY);
                } else {
                    self.remove_trailing(&mut value, &mut flags);
                }
                Ok((rest, Value { value, flags }))
            }
        }
    }

    /// Parse data before an eol with no colon as an empty name with the data as the value
    fn header_sans_colon(&self) -> impl Fn(&[u8]) -> IResult<&[u8], Header> + '_ {
        move |input| {
            let (mut remaining, (_, mut value)) = tuple((
                not(tag("\r\n")),
                take_till1(|c| c == b':' || is_terminator(c)),
            ))(input)?;
            if value.last() == Some(&b'\r') {
                value = &value[..value.len() - 1];
                remaining = &input[value.len()..];
            }
            let (remaining, (_, flags)) = self.complete_null_or_eol()(remaining)?;
            Ok((
                remaining,
                Header {
                    name: Name {
                        name: Vec::new(),
                        flags: Flags::MISSING_COLON | flags,
                    },
                    value: Value {
                        value: value.into(),
                        flags: Flags::MISSING_COLON | flags,
                    },
                },
            ))
        }
    }

    /// Parse a header name: value
    fn header_with_colon(&self) -> impl Fn(&[u8]) -> IResult<&[u8], Header> + '_ {
        move |input| {
            map(
                tuple((name, separator, self.value())),
                |(name, _, value)| Header { name, value },
            )(input)
        }
    }

    /// Parses a header name and value with, or without a colon separator
    fn header(&self) -> impl Fn(&[u8]) -> IResult<&[u8], Header> + '_ {
        move |input| alt((self.header_with_colon(), self.header_sans_colon()))(input)
    }

    /// Parse multiple headers and indicate if end of headers or null was found
    pub fn headers(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (Vec<Header>, bool)> + '_ {
        move |input| {
            let (rest, head) = self.header()(input)?;
            let is_null_terminated = head.value.flags.is_set(Flags::NULL_TERMINATED);
            let mut out = Vec::with_capacity(16);
            out.push(head);
            if is_null_terminated {
                return Ok((rest, (out, true)));
            }
            if let Ok((rest, _eoh)) = self.complete_eol()(rest) {
                return Ok((rest, (out, true)));
            }
            let mut i = rest;
            loop {
                match self.header()(i) {
                    Ok((rest, head)) => {
                        i = rest;
                        let is_null_terminated = head.value.flags.is_set(Flags::NULL_TERMINATED);
                        out.push(head);
                        if is_null_terminated {
                            return Ok((rest, (out, true)));
                        }
                        if let Ok((rest, _eoh)) = self.complete_eol()(rest) {
                            return Ok((rest, (out, true)));
                        }
                    }
                    Err(Incomplete(_)) => {
                        return Ok((i, (out, false)));
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }

    /// Set the parser eol type
    pub fn set_eol(&mut self, eol: Eol) {
        self.eol = eol;
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            eol: Eol::None,
            null_terminates: true,
        }
    }
}

/// Parse name containing non token characters
fn non_token_name(input: &[u8]) -> IResult<&[u8], (&[u8], u64)> {
    map(
        tuple((
            space0,
            take_till(|c| c == b':' || is_terminator(c)),
            peek(tag(":")),
        )),
        |(leading_spaces, mut name, _): (&[u8], &[u8], _)| {
            let mut flags = Flags::NAME_NON_TOKEN_CHARS;
            if !name.is_empty() {
                if !leading_spaces.is_empty() {
                    flags.set(Flags::NAME_LEADING_WHITESPACE)
                }
                while let Some(end) = name.last() {
                    if is_space(*end) {
                        flags.set(Flags::NAME_TRAILING_WHITESPACE);
                        name = &name[..name.len() - 1];
                    } else {
                        break;
                    }
                }
            } else {
                flags.set(Flags::NAME_EMPTY)
            }
            (name, flags)
        },
    )(input)
}

/// Parse name containing only token characters
fn token_name(input: &[u8]) -> IResult<&[u8], (&[u8], u64)> {
    // The name should consist only of token characters (i.e., no spaces, seperators, control characters, etc)
    map(
        tuple((space0, take_while(is_token), space0, peek(tag(":")))),
        |(leading_spaces, name, trailing_spaces, _): (&[u8], &[u8], &[u8], _)| {
            let mut flags: u64 = 0;
            if !name.is_empty() {
                if !leading_spaces.is_empty() {
                    flags.set(Flags::NAME_LEADING_WHITESPACE)
                }
                if !trailing_spaces.is_empty() {
                    flags.set(Flags::NAME_TRAILING_WHITESPACE)
                }
            } else {
                flags.set(Flags::NAME_EMPTY)
            }
            (name, flags)
        },
    )(input)
}

/// Parse one header name up to the :
fn name(input: &[u8]) -> IResult<&[u8], Name> {
    //We first attempt to parse a token name before we attempt a non token name
    map(alt((token_name, non_token_name)), |(name, flags)| Name {
        name: name.into(),
        flags,
    })(input)
}

/// Check if the byte is LF or null
fn is_terminator(c: u8) -> bool {
    c == b'\n' || c == b'\0'
}

/// Parse one null character and return it and the NULL_TERMINATED flag
fn null(input: &[u8]) -> IResult<&[u8], (&[u8], u64)> {
    map(complete_tag("\0"), |null| (null, Flags::NULL_TERMINATED))(input)
}

/// Handles any special cases that are exceptions to the spec
///
/// Currently handles the use of a single CR as folding LWS
fn folding_lws_special(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((tag("\r"), not(alt((tag("\r"), tag("\n")))), space0)),
        |(fold, _, spaces): (&[u8], _, &[u8])| &input[..fold.len() + spaces.len()],
    )(input)
}

/// Extracts any folding lws (whitespace or any special cases)
fn folding_lws(input: &[u8]) -> IResult<&[u8], (&[u8], u64)> {
    alt((
        map(space1, |fold| (fold, Flags::FOLDING)),
        map(folding_lws_special, |fold| {
            (fold, Flags::FOLDING_SPECIAL_CASE)
        }),
    ))(input)
}

/// Parse a separator (colon + space) between header name and value
fn separator(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    tuple((tag(":"), space0))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::{
        error::ErrorKind::{Not, Tag},
        Err::{Error, Incomplete},
        Needed,
    };
    macro_rules! b {
        ($b: literal) => {
            $b.as_bytes()
        };
    }

    macro_rules! assert_headers_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.headers()($i), "Failed to assert headers result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_headers_result_eq! { $r, $i, $p }
            assert_headers_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_header_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.header()($i), "Failed to assert header result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_header_result_eq! { $r, $i, $p }
            assert_header_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_eol_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.eol()($i), "Failed to assert eol result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_eol_result_eq! { $r, $i, $p }
            assert_eol_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_complete_eol_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.complete_eol()($i), "Failed to assert complete_eol result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_complete_eol_result_eq! { $r, $i, $p }
            assert_complete_eol_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_null_or_eol_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.null_or_eol()($i), "Failed to assert null_or_eol result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_null_or_eol_result_eq! { $r, $i, $p }
            assert_null_or_eol_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_folding_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.folding()($i), "Failed to assert folding result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_folding_result_eq! { $r, $i, $p }
            assert_folding_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_folding_or_terminator_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.folding_or_terminator()($i), "Failed to assert folding_or_terminator result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_folding_or_terminator_result_eq! { $r, $i, $p }
            assert_folding_or_terminator_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_value_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.value()($i), "Failed to assert value result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_value_result_eq! { $r, $i, $p }
            assert_value_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_value_bytes_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.value_bytes()($i), "Failed to assert value_bytes result on Eol::{:#?} parser", parser.eol);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_value_bytes_result_eq! { $r, $i, $p }
            assert_value_bytes_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! header {
        ($name:expr, $nameflag:expr, $value:expr, $valueflag:expr) => {
            Header {
                name: Name {
                    name: $name.to_vec(),
                    flags: $nameflag,
                },
                value: Value {
                    value: $value.to_vec(),
                    flags: $valueflag,
                },
            }
        };
    }

    #[test]
    fn NullTerminates() {
        let parser = Parser::new(Eol::None, false);
        let parser_null_terminates = Parser::new(Eol::None, true);

        let input = b"k1:v1\r\nk2:v2 before\0v2 after\r\n\r\n";
        let result = Ok((
            b!(""),
            (
                vec![
                    header!(b"k1", 0, b"v1", 0),
                    header!(b"k2", 0, b"v2 before\0v2 after", 0),
                ],
                true,
            ),
        ));
        let result_null_terminates = Ok((
            b!("\r\n"),
            (
                vec![
                    header!(b"k1", 0, b"v1", 0),
                    header!(b"k2", 0, b"v2 before", Flags::NULL_TERMINATED),
                ],
                true,
            ),
        ));
        assert_headers_result_eq!(result, input, parser);
        assert_headers_result_eq!(result_null_terminates, input, parser_null_terminates);
    }

    #[test]
    fn Headers() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);

        let input = b"k1:v1\r\n:v2\r\n v2+\r\nk3: v3\r\nk4 v4\r\nk\r5:v\r5\n\rmore\r\n\r\n";
        let common = vec![
            header!(b"k1", 0, b"v1", 0),
            header!(b"", Flags::NAME_EMPTY, b"v2 v2+", Flags::FOLDING),
            header!(b"k3", 0, b"v3", 0),
            header!(b"", Flags::MISSING_COLON, b"k4 v4", Flags::MISSING_COLON),
        ];
        let result = Ok((
            b!(""),
            (
                [
                    common.as_slice(),
                    vec![header!(
                        b"k\r5",
                        Flags::NAME_NON_TOKEN_CHARS,
                        b"v\r5 more",
                        Flags::FOLDING_SPECIAL_CASE
                    )]
                    .as_slice(),
                ]
                .concat(),
                true,
            ),
        ));
        let result_cr = Ok((
            b!("more\r\n\r\n"),
            (
                [
                    common.as_slice(),
                    vec![
                        header!(b"k\r5", Flags::NAME_NON_TOKEN_CHARS, b"v", 0),
                        header!(b"", Flags::MISSING_COLON, b"5", Flags::MISSING_COLON),
                    ]
                    .as_slice(),
                ]
                .concat(),
                true,
            ),
        ));
        let result_lfcr = Ok((
            b!(""),
            (
                [
                    common.as_slice(),
                    vec![
                        header!(
                            b"k\r5",
                            Flags::NAME_NON_TOKEN_CHARS,
                            b"v\r5",
                            Flags::DEFORMED_EOL
                        ),
                        header!(b"", Flags::MISSING_COLON, b"more", Flags::MISSING_COLON),
                    ]
                    .as_slice(),
                ]
                .concat(),
                true,
            ),
        ));
        assert_headers_result_eq!(result, input, parser);
        assert_headers_result_eq!(result_cr, input, parser_cr);
        assert_headers_result_eq!(result_lfcr, input, parser_lfcr);

        let input = b"k1:v1\r\nk2:v2\r";
        let result = Ok((b!("k2:v2\r"), (vec![header!(b"k1", 0, b"v1", 0)], false)));
        assert_headers_result_eq!(result, input, parser, parser_cr, parser_lfcr);

        let input = b"k1:v1\nk2:v2\0v2\r\nk3:v3\r";
        let result = Ok((
            b!("k3:v3\r"),
            (
                vec![
                    header!(b"k1", 0, b"v1", 0),
                    header!(b"k2", 0, b"v2", Flags::NULL_TERMINATED),
                ],
                true,
            ),
        ));
        assert_headers_result_eq!(result, input, parser, parser_cr, parser_lfcr);
        let common = vec![
            header!(b"Name1", 0, b"Value1", 0),
            header!(b"Name2", 0, b"Value2", 0),
            header!(b"Name3", 0, b"Val ue3", Flags::FOLDING),
            header!(b"Name4", 0, b"Value4 Value4.1 Value4.2", Flags::FOLDING),
        ];

        let result = Ok((b!(""), (common.clone(), true)));
        // Test only \n terminators (should be same result as above)
        let i = b"Name1: Value1\n\
                  Name2:Value2\n\
                  Name3: Val\n ue3\n\
                  Name4: Value4\n Value4.1\n Value4.2\n\
                  \n";
        assert_headers_result_eq!(result, i, parser, parser_cr, parser_lfcr);

        // Test only \r\n terminators (should be same result as above)
        let i = b"Name1: Value1\r\n\
                  Name2:Value2\r\n\
                  Name3: Val\r\n ue3\r\n\
                  Name4: Value4\r\n Value4.1\r\n Value4.2\r\n\
                  \r\n";
        assert_headers_result_eq!(result, i, parser, parser_cr, parser_lfcr);

        // Test a mix of \r\n and \n terminators (should be same result as above)
        let i = b"Name1: Value1\r\n\
                  Name2:Value2\n\
                  Name3: Val\r\n ue3\r\n\
                  Name4: Value4\r\n Value4.1\n Value4.2\r\n\
                  \n";
        assert_headers_result_eq!(result, i, parser, parser_cr, parser_lfcr);

        // Test only \r terminators (should be same result as above)
        let i = b"Name1: Value1\r\
                  Name2:Value2\r\
                  Name3: Val\r\n ue3\r\
                  Name4: Value4\r\n Value4.1\r\n Value4.2\r\
                  \r\n";
        assert_headers_result_eq!(result, i, parser_cr);

        // Test a mix of \r\n, \r, and \n terminators (should be same result as above)
        let i = b"Name1: Value1\r\
                  Name2:Value2\r\
                  Name3: Val\r\n ue3\r\n\
                  Name4: Value4\r\n Value4.1\n Value4.2\r\n\
                  \n";
        assert_headers_result_eq!(result, i, parser_cr);

        // Test a mix of \r\n, \n\r, \n terminators (should be same result as above
        // EXCEPT only ONE deformed warning)
        let mut one_deformed = common.clone();
        one_deformed
            .get_mut(2)
            .unwrap()
            .value
            .flags
            .set(Flags::DEFORMED_EOL);
        let result = Ok((b!(""), (one_deformed, true)));
        let i = b"Name1: Value1\r\n\
                  Name2:Value2\n\
                  Name3: Val\n\r ue3\n\r\
                  Name4: Value4\r\n Value4.1\n Value4.2\r\n\
                  \n";
        assert_headers_result_eq!(result, i, parser_lfcr);

        // Test only \n\r terminators (should be same result as above EXCEPT for
        // ALL deformed warning)
        let mut deformed = common;
        for header in deformed.iter_mut() {
            header.value.flags.set(Flags::DEFORMED_EOL)
        }
        let result = Ok((b!(""), (deformed, true)));
        let i = b"Name1: Value1\n\r\
                  Name2:Value2\n\r\
                  Name3: Val\n\r ue3\n\r\
                  Name4: Value4\n\r Value4.1\n\r Value4.2\n\r\
                  \n\r";
        assert_headers_result_eq!(result, i, parser_lfcr);
    }

    #[test]
    fn HeaderSansColon() {
        let parser = Parser::default();
        assert!(parser.header_sans_colon()(b"K V").is_err());
        assert!(parser.header_sans_colon()(b"K:V\r\n").is_err());
        assert!(parser.header_sans_colon()(b"\r\n").is_err());
        assert_eq!(
            parser.header_sans_colon()(b"K V\0alue\r\n"),
            Ok((
                b!("alue\r\n"),
                header!(
                    b"",
                    Flags::MISSING_COLON | Flags::NULL_TERMINATED,
                    b"K V",
                    Flags::MISSING_COLON | Flags::NULL_TERMINATED
                ),
            ))
        );
        assert_eq!(
            parser.header_sans_colon()(b"K V\ralue\r\n"),
            Ok((
                b!(""),
                header!(
                    b"",
                    Flags::MISSING_COLON,
                    b"K V\ralue",
                    Flags::MISSING_COLON
                ),
            ))
        );
        let result = Ok((
            b!("k1:v1\r\n"),
            header!(b"", Flags::MISSING_COLON, b"K V", Flags::MISSING_COLON),
        ));
        assert_eq!(result, parser.header_sans_colon()(b"K V\r\nk1:v1\r\n"));
        assert_eq!(result, parser.header_sans_colon()(b"K V\nk1:v1\r\n"));
    }

    #[test]
    fn HeaderWithColon() {
        let parser = Parser::default();
        assert!(parser.header_with_colon()(b"K: V").is_err());
        assert!(parser.header_with_colon()(b"K: V\r\n").is_err());
        assert!(parser.header_with_colon()(b"K V\r\n").is_err());
        assert!(parser.header_with_colon()(b"K V\r\nK:V\r\n").is_err());
        assert!(parser.header_with_colon()(b"K\0ey:Value\r\nK:V\r\n").is_err());
        assert_eq!(
            parser.header_with_colon()(b"K1:V1\nK2:V2\n\r\n"),
            Ok((b!("K2:V2\n\r\n"), header!(b"K1", 0, b"V1", 0),))
        );
        assert_eq!(
            parser.header_with_colon()(b":\r\n\r\n"),
            Ok((
                b!("\r\n"),
                header!(b"", Flags::NAME_EMPTY, b"", Flags::VALUE_EMPTY),
            ))
        );
        assert_eq!(
            parser.header_with_colon()(b"K:\r\n\r\n"),
            Ok((b!("\r\n"), header!(b"K", 0, b"", Flags::VALUE_EMPTY),))
        );
        assert_eq!(
            parser.header_with_colon()(b":V\r\n\r\n"),
            Ok((b!("\r\n"), header!(b"", Flags::NAME_EMPTY, b"V", 0),))
        );
        assert_eq!(
            parser.header_with_colon()(b"K:folded\r\n\rV\r\n\r\n"),
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"folded V", Flags::FOLDING_SPECIAL_CASE),
            ))
        );
        assert_eq!(
            parser.header_with_colon()(b"K: V\r\n\r\n"),
            Ok((b!("\r\n"), header!(b"K", 0, b"V", 0),))
        );
        assert_eq!(
            parser.header_with_colon()(b"K: V before\0 V after\r\n\r\n"),
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"V before", Flags::NULL_TERMINATED),
            ))
        );
        assert_eq!(
            parser.header_with_colon()(b"K: V\r\n a\r\n l\r\n u\r\n\te\r\n\r\n"),
            Ok((b!("\r\n"), header!(b"K", 0, b"V a l u e", Flags::FOLDING),))
        );
    }

    #[test]
    fn Header() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);
        assert_header_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"K: V",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"K: V\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"Host:www.google.com\rName: Value\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"Host", 0, b"www.google.com\rName: Value", 0),
            )),
            input,
            parser,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((
                b!("Name: Value\r\n\r\n"),
                header!(b"Host", 0, b"www.google.com", 0),
            )),
            input,
            parser_cr
        );
        assert_header_result_eq!(
            Ok((
                b!(""),
                header!(b"", Flags::MISSING_COLON, b"K1 V1", Flags::MISSING_COLON),
            )),
            b"K1 V1\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((
                b!("K2:V2\n\r\n"),
                header!(b"", Flags::MISSING_COLON, b"K1 V1", Flags::MISSING_COLON),
            )),
            b"K1 V1\r\nK2:V2\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((b!("K2:V2\n\r\n"), header!(b"K1", 0, b"V1", 0),)),
            b"K1:V1\nK2:V2\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"", Flags::NAME_EMPTY, b"", Flags::VALUE_EMPTY),
            )),
            b":\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"", Flags::VALUE_EMPTY),)),
            b"K:\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"", Flags::NAME_EMPTY, b"V", 0),)),
            b":V\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"folded V", Flags::FOLDING_SPECIAL_CASE),
            )),
            b"K:folded\r\n\rV\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V", 0),)),
            b"K: V\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"V before", Flags::NULL_TERMINATED),
            )),
            b"K: V before\0 V after\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V a l u e", Flags::FOLDING),)),
            b"K: V\n a\r\n l\n u\r\n\te\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"K: V\r a\r\n l\n u\r\n\te\r\n\r\n";
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V\r a l u e", Flags::FOLDING),)),
            input,
            parser,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V a l u e", Flags::FOLDING),)),
            input,
            parser_cr
        );

        let input = b"K:deformed folded\n\r V\n\r\r\n\n";
        assert_header_result_eq!(
            Ok((
                b!("\n"),
                header!(
                    b"K",
                    0,
                    b"deformed folded V",
                    Flags::FOLDING_SPECIAL_CASE | Flags::DEFORMED_EOL
                ),
            )),
            input,
            parser,
            parser_cr
        );
        assert_header_result_eq!(
            Ok((
                b!("\n"),
                header!(
                    b"K",
                    0,
                    b"deformed folded V",
                    Flags::FOLDING | Flags::DEFORMED_EOL
                ),
            )),
            input,
            parser_lfcr
        );

        let input = b"K:deformed folded\n\r V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"deformed folded V", Flags::FOLDING_SPECIAL_CASE),
            )),
            input,
            parser,
            parser_cr
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    0,
                    b"deformed folded V",
                    Flags::FOLDING | Flags::DEFORMED_EOL
                ),
            )),
            input,
            parser_lfcr
        );

        let input = b"K:deformed folded\n\r\r V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    0,
                    b"deformed folded V",
                    Flags::FOLDING_SPECIAL_CASE | Flags::DEFORMED_EOL
                ),
            )),
            input,
            parser_lfcr
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\r V\r\n\r\n"),
                header!(b"K", 0, b"deformed folded", 0),
            )),
            input,
            parser,
            parser_cr
        );
    }

    #[test]
    fn Separator() {
        assert!(separator(b" : ").is_err());
        assert!(separator(b" ").is_err());
        assert_eq!(separator(b":value"), Ok((b!("value"), (b!(":"), b!("")))));
        assert_eq!(separator(b": value"), Ok((b!("value"), (b!(":"), b!(" ")))));
        assert_eq!(
            separator(b":\t value"),
            Ok((b!("value"), (b!(":"), b!("\t "))))
        );
    }

    #[test]
    fn TokenName() {
        assert_eq!(
            token_name(b"Hello: world"),
            Ok((b!(": world"), (b!("Hello"), 0)))
        );
        assert_eq!(
            token_name(b" Hello: world"),
            Ok((b!(": world"), (b!("Hello"), Flags::NAME_LEADING_WHITESPACE)))
        );
        assert_eq!(
            token_name(b"Hello : world"),
            Ok((
                b!(": world"),
                (b!("Hello"), Flags::NAME_TRAILING_WHITESPACE)
            ))
        );
        assert_eq!(
            token_name(b" Hello : world"),
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE | Flags::NAME_TRAILING_WHITESPACE
                )
            ))
        );
        assert!(token_name(b"Hello Invalid: world").is_err());
        assert!(token_name(b"Hello;Invalid: world").is_err());
        assert!(token_name(b"Hello").is_err());
        assert!(token_name(b"Hello\rInvalid: world").is_err());
        assert!(token_name(b"Hello\nInvalid: world").is_err());
        assert!(token_name(b"Hello\0Invalid: world").is_err());
    }

    #[test]
    fn NonTokenName() {
        assert_eq!(
            non_token_name(b"Hello: world"),
            Ok((b!(": world"), (b!("Hello"), Flags::NAME_NON_TOKEN_CHARS)))
        );
        assert_eq!(
            non_token_name(b" Hello: world"),
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE | Flags::NAME_NON_TOKEN_CHARS
                )
            ))
        );
        assert_eq!(
            non_token_name(b"Hello : world"),
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_TRAILING_WHITESPACE | Flags::NAME_NON_TOKEN_CHARS
                )
            ))
        );
        assert_eq!(
            non_token_name(b" Hello : world"),
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE
                        | Flags::NAME_TRAILING_WHITESPACE
                        | Flags::NAME_NON_TOKEN_CHARS
                )
            ))
        );
        assert_eq!(
            non_token_name(b"Hello Invalid: world"),
            Ok((
                b!(": world"),
                (b!("Hello Invalid"), Flags::NAME_NON_TOKEN_CHARS)
            ))
        );
        assert_eq!(
            non_token_name(b"Hello;Invalid: world"),
            Ok((
                b!(": world"),
                (b!("Hello;Invalid"), Flags::NAME_NON_TOKEN_CHARS)
            ))
        );
        assert!(token_name(b"Hello\rInvalid: world").is_err());
        assert!(token_name(b"Hello\nInvalid: world").is_err());
        assert!(token_name(b"Hello\0Invalid: world").is_err());
        assert!(non_token_name(b"Hello").is_err());
    }
    #[test]
    fn Name() {
        assert_eq!(name(b"Hello: world").unwrap().1.name, b"Hello".to_vec());
        assert_eq!(name(b": world").unwrap().1.name, b"".to_vec());
        assert_eq!(
            name(b"Host:www.google.com\rName: Value"),
            Ok((
                b!(":www.google.com\rName: Value"),
                Name {
                    name: b"Host".to_vec(),
                    flags: 0
                }
            ))
        );
        assert_eq!(
            name(b"Hello : world"),
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello".to_vec(),
                    flags: Flags::NAME_TRAILING_WHITESPACE
                }
            ))
        );
        assert_eq!(
            name(b" Hello : world"),
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello".to_vec(),
                    flags: Flags::NAME_LEADING_WHITESPACE | Flags::NAME_TRAILING_WHITESPACE
                }
            ))
        );
        assert_eq!(
            name(b"Hello;invalid: world"),
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello;invalid".to_vec(),
                    flags: Flags::NAME_NON_TOKEN_CHARS
                }
            ))
        );
        assert_eq!(
            name(b"Hello invalid: world"),
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello invalid".to_vec(),
                    flags: Flags::NAME_NON_TOKEN_CHARS
                }
            ))
        );
        assert_eq!(
            name(b" Hello Invalid : world"),
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello Invalid".to_vec(),
                    flags: Flags::NAME_LEADING_WHITESPACE
                        | Flags::NAME_TRAILING_WHITESPACE
                        | Flags::NAME_NON_TOKEN_CHARS
                }
            ))
        );
        assert_eq!(
            name(b"  : world"),
            Ok((
                b!(": world"),
                Name {
                    name: b"".to_vec(),
                    flags: Flags::NAME_EMPTY
                }
            ))
        );
        assert!(name(b"Hello").is_err());
    }

    #[test]
    fn Null() {
        assert!(null(b"test").is_err());
        assert!(null(b"\r\n").is_err());
        assert!(null(b"\n").is_err());
        assert!(null(b"\r").is_err());
        assert_eq!(
            null(b"\0a"),
            Ok((b!("a"), (b!("\0"), Flags::NULL_TERMINATED)))
        );
    }

    #[test]
    fn Eol() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);
        assert_eol_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            parser,
            parser_cr,
            parser_lfcr
        );
        let input = b"\ra";
        assert_eol_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );
        assert_eol_result_eq!(Ok((b!("a"), (b!("\r"), 0))), input, parser_cr);

        let input = b"\r\r";
        assert_eol_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );
        assert_eol_result_eq!(Err(Incomplete(Needed::Size(1))), input, parser_cr);

        assert_eol_result_eq!(Err(Incomplete(Needed::Size(1))), b"\n\r", parser, parser_cr);

        let input = b"\n\ra";
        assert_eol_result_eq!(Err(Error((b!("\ra"), Not))), input, parser, parser_cr);
        assert_eol_result_eq!(
            Ok((b!("a"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            parser_lfcr
        );
        let input = b"\n\r\n";
        assert_eol_result_eq!(Ok((b!("\r\n"), (b!("\n"), 0))), input, parser, parser_cr);
        assert_eol_result_eq!(
            Ok((b!("\n"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            parser_lfcr
        );
        let input = b"\n\r\n\r";
        assert_eol_result_eq!(Ok((b!("\r\n\r"), (b!("\n"), 0))), input, parser, parser_cr);
        assert_eol_result_eq!(
            Ok((b!("\n\r"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            parser_lfcr
        );
        assert_eol_result_eq!(
            Ok((b!("a"), (b!("\n"), 0))),
            (b"\na"),
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_eol_result_eq!(
            Ok((b!("\r\na"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            b"\n\r\r\na",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_eol_result_eq!(
            Ok((b!("\r\na"), (b!("\r\n"), 0))),
            b"\r\n\r\na",
            parser,
            parser_cr,
            parser_lfcr
        );

        assert_complete_eol_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_complete_eol_result_eq!(
            Ok((b!(""), (b!("\r\n"), 0))),
            b"\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_complete_eol_result_eq!(
            Ok((b!(""), (b!("\n"), 0))),
            b"\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_complete_eol_result_eq!(
            Ok((b!("\r\n"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            b"\n\r\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_complete_eol_result_eq!(
            Ok((b!("\r\n"), (b!("\r\n"), 0))),
            b"\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
    }

    #[test]
    fn NullOrEol() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);
        assert_null_or_eol_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_null_or_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_null_or_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\n",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"\r";
        assert_null_or_eol_result_eq!(Err(Incomplete(Needed::Size(1))), input, parser_cr);
        assert_null_or_eol_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );

        let input = b"\ra";
        assert_null_or_eol_result_eq!(Ok((b!("a"), (b!("\r"), 0))), input, parser_cr);
        assert_null_or_eol_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );

        assert_null_or_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            parser,
            parser_cr,
            parser_lfcr
        );
        let input = b"\r\r";
        assert_null_or_eol_result_eq!(Err(Incomplete(Needed::Size(1))), input, parser_cr);
        assert_null_or_eol_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );

        assert_null_or_eol_result_eq!(
            Ok((b!("a"), (b!("\0"), Flags::NULL_TERMINATED))),
            b"\0a",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_null_or_eol_result_eq!(
            Ok((b!("a"), (b!("\n"), 0))),
            b"\na",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_null_or_eol_result_eq!(
            Ok((b!("\r\na"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            b"\n\r\r\na",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_null_or_eol_result_eq!(
            Ok((b!("\r\n"), (b!("\r\n"), 0))),
            b"\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        let input = b"\n\r\n";
        assert_null_or_eol_result_eq!(Ok((b!("\r\n"), (b!("\n"), 0))), input, parser, parser_cr);
        assert_null_or_eol_result_eq!(
            Ok((b!("\n"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            parser_lfcr
        );
        let input = b"\n\r\n\r";
        assert_null_or_eol_result_eq!(Ok((b!("\r\n\r"), (b!("\n"), 0))), input, parser, parser_cr);
        assert_null_or_eol_result_eq!(
            Ok((b!("\n\r"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            parser_lfcr
        );
    }

    #[test]
    fn IsTerminator() {
        assert!(is_terminator(b'\n'));
        assert!(is_terminator(b'\0'));
        assert!(!is_terminator(b'\t'));
        assert!(!is_terminator(b' '));
        assert!(!is_terminator(b'\r'));
    }

    #[test]
    fn FoldingLwsSpecial() {
        assert!(folding_lws_special(b"test").is_err());
        assert!(folding_lws_special(b"\r\n").is_err());
        assert!(folding_lws_special(b"\r").is_err());
        assert!(folding_lws_special(b"\r\r").is_err());
        assert!(folding_lws_special(b"\r\r\t next").is_err());
        assert!(folding_lws_special(b"\r\n\t next").is_err());
        assert_eq!(folding_lws_special(b"\rnext"), Ok((b!("next"), b!("\r"))));
        assert_eq!(
            folding_lws_special(b"\r\t next"),
            Ok((b!("next"), b!("\r\t ")))
        );
    }

    #[test]
    fn FoldingLws() {
        assert!(folding_lws(b"test").is_err());
        assert!(folding_lws(b"\r\n").is_err());
        assert!(folding_lws(b"\r").is_err());
        assert!(folding_lws(b"\r\r").is_err());
        assert_eq!(
            folding_lws(b"\rnext"),
            Ok((b!("next"), (b!("\r"), Flags::FOLDING_SPECIAL_CASE)))
        );
        assert_eq!(
            folding_lws(b"\r\t next"),
            Ok((b!("next"), (b!("\r\t "), Flags::FOLDING_SPECIAL_CASE)))
        );
        assert_eq!(
            folding_lws(b" next"),
            Ok((b!("next"), (b!(" "), Flags::FOLDING)))
        );
        assert_eq!(
            folding_lws(b"\tnext"),
            Ok((b!("next"), (b!("\t"), Flags::FOLDING)))
        );
        assert_eq!(
            folding_lws(b"\t next"),
            Ok((b!("next"), (b!("\t "), Flags::FOLDING)))
        );
        assert_eq!(
            folding_lws(b"\t\t\r\n"),
            Ok((b!("\r\n"), (b!("\t\t"), Flags::FOLDING)))
        );
        assert_eq!(
            folding_lws(b"\t \t\r"),
            Ok((b!("\r"), (b!("\t \t"), Flags::FOLDING)))
        );
        assert_eq!(
            folding_lws(b"     \n"),
            Ok((b!("\n"), (b!("     "), Flags::FOLDING)))
        );
    }

    #[test]
    fn Folding() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);
        assert_folding_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n \t",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Err(Error((b"\n".as_ref(), Not))),
            b"\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\r",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Err(Error((b"\r".as_ref(), Not))),
            b"\r\n\r\r",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((
                b!("next"),
                (b!("\r\n"), b!("\r"), Flags::FOLDING_SPECIAL_CASE)
            )),
            b"\r\n\rnext",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((
                b!("next"),
                (b!("\r\n"), b!("\r\t "), Flags::FOLDING_SPECIAL_CASE)
            )),
            b"\r\n\r\t next",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((b!("next"), (b!("\r\n"), b!(" "), Flags::FOLDING))),
            b"\r\n next",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((b!("next"), (b!("\r\n"), b!("\t"), Flags::FOLDING))),
            b"\r\n\tnext",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((b!("next"), (b!("\r\n"), b!("\t "), Flags::FOLDING))),
            b"\r\n\t next",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((b!("\r\n"), (b!("\r\n"), b!("\t\t"), Flags::FOLDING))),
            b"\r\n\t\t\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((b!("\r"), (b!("\r\n"), b!("\t \t"), Flags::FOLDING))),
            b"\r\n\t \t\r",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_result_eq!(
            Ok((b!("\n"), (b!("\r\n"), b!("     "), Flags::FOLDING))),
            b"\r\n     \n",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"\n\r     \n";
        assert_folding_result_eq!(
            Ok((
                b!("\n"),
                (b!("\n"), b!("\r     "), Flags::FOLDING_SPECIAL_CASE)
            )),
            input,
            parser,
            parser_cr
        );
        assert_folding_result_eq!(
            Ok((
                b!("\n"),
                (
                    b!("\n\r"),
                    b!("     "),
                    Flags::FOLDING | Flags::DEFORMED_EOL
                )
            )),
            input,
            parser_lfcr
        );

        let input = b"\r     \n";
        assert_folding_result_eq!(
            Ok((b!("\n"), (b!("\r"), b!("     "), Flags::FOLDING))),
            input,
            parser_cr
        );
        assert_folding_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );
    }

    #[test]
    fn FoldingOrTerminator() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);
        // All of these fail because they are incomplete.
        // We need more bytes before we can get the full fold
        // or decide there is no fold.
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n ",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\r",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"\r\r";
        assert_folding_or_terminator_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(Err(Incomplete(Needed::Size(1))), input, parser_cr);
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r\n"), Flags::FOLDING), Some(b!("\t"))))),
            b"\r\n\ta",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("a"),
                (
                    (b!("\r\n"), Flags::FOLDING | Flags::FOLDING_SPECIAL_CASE),
                    Some(b!("\r"))
                )
            )),
            b"\r\n\ra",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r\n"), Flags::FOLDING), Some(b!(" "))))),
            b"\r\n a",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r\n"), 0), None))),
            b"\r\na",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\na"), ((b!("\n"), 0), None))),
            b"\n\na",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\na"), ((b!("\r\n"), 0), None))),
            b"\r\n\r\na",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\na"), ((b!("\n\r"), Flags::DEFORMED_EOL), None))),
            b"\n\r\r\na",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\0"), Flags::NULL_TERMINATED), None))),
            b"\0a",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"\r a";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r"), Flags::FOLDING), Some(b!(" "))))),
            input,
            parser_cr
        );
        assert_folding_or_terminator_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            parser,
            parser_lfcr
        );
        let input = b"\n\r     \n";
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                ((b!("\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r     ")))
            )),
            input,
            parser,
            parser_cr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                (
                    (b!("\n\r"), Flags::FOLDING | Flags::DEFORMED_EOL),
                    Some(b!("     "))
                )
            )),
            input,
            parser_lfcr
        );

        let input = b"\n\r \n";
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                ((b!("\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r ")))
            )),
            input,
            parser,
            parser_cr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                (
                    (
                        b!("\n\r"),
                        Flags::TERMINATOR_SPECIAL_CASE | Flags::DEFORMED_EOL
                    ),
                    None
                )
            )),
            input,
            parser_lfcr
        );

        assert_folding_or_terminator_result_eq!(
            Ok((b!("\n"), ((b!("\n"), Flags::TERMINATOR_SPECIAL_CASE), None))),
            b"\n \n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                ((b!("\r\n"), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            b"\r\n \n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\r\n"),
                ((b!("\r\n"), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            b"\r\n \r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\r\n"),
                ((b!("\n"), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            b"\n \r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\n"), ((b!("\n"), Flags::TERMINATOR_SPECIAL_CASE), None))),
            b"\n\t\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                ((b!("\n \r\n"), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            b"\n \r\n\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                ((b!("\n\t\n"), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            b"\n\t\n\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                ((b!("\r\n \r\n"), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            b"\r\n \r\n\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                ((b!("\n\t\r\n"), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            b"\n\t\r\n\n",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"\n\r \n\n";
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n\n"),
                ((b!("\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r ")))
            )),
            input,
            parser,
            parser_cr
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                (
                    (
                        b!("\n\r \n"),
                        Flags::TERMINATOR_SPECIAL_CASE | Flags::DEFORMED_EOL
                    ),
                    None
                )
            )),
            input,
            parser_lfcr
        );
    }

    #[test]
    fn ValueBytes() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);
        // Expect fail because we need to see EOL
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b" ",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\tvalue",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b" value",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"value\rname2";
        assert_value_bytes_result_eq!(Err(Incomplete(Needed::Size(1))), input, parser, parser_lfcr);
        assert_value_bytes_result_eq!(
            Ok((b!("name2"), (b!("value"), ((b!("\r"), 0), None)))),
            input,
            parser_cr
        );
        // Expect fail because we need to see past EOL to check for folding
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"www.google.com\rName: Value\r\n\r\n";
        assert_value_bytes_result_eq!(
            Ok((
                b!("\r\n"),
                (b!("www.google.com\rName: Value"), ((b!("\r\n"), 0), None))
            )),
            input,
            parser,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("Name: Value\r\n\r\n"),
                (b!("www.google.com"), ((b!("\r"), 0), None))
            )),
            input,
            parser_cr
        );

        let input = b"www.google.com\rName: Value\n\r\n";
        assert_value_bytes_result_eq!(
            Ok((
                b!("\r\n"),
                (b!("www.google.com\rName: Value"), ((b!("\n"), 0), None))
            )),
            input,
            parser
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("Name: Value\n\r\n"),
                (b!("www.google.com"), ((b!("\r"), 0), None))
            )),
            input,
            parser_cr
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("\n"),
                (
                    b!("www.google.com\rName: Value"),
                    ((b!("\n\r"), Flags::DEFORMED_EOL), None)
                )
            )),
            input,
            parser_lfcr
        );

        let input = b"www.google.com\rName: Value\r\n\n";
        assert_value_bytes_result_eq!(
            Ok((
                b!("\n"),
                (b!("www.google.com\rName: Value"), ((b!("\r\n"), 0), None))
            )),
            input,
            parser,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("Name: Value\r\n\n"),
                (b!("www.google.com"), ((b!("\r"), 0), None))
            )),
            input,
            parser_cr
        );

        assert_value_bytes_result_eq!(
            Ok((b!("next"), (b!(""), ((b!("\r\n"), 0), None)))),
            b"\r\nnext",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Ok((b!("name2"), (b!("value"), ((b!("\r\n"), 0), None)))),
            b"value\r\nname2",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("more"),
                (b!("value"), ((b!("\n"), Flags::FOLDING), Some(b!(" "))))
            )),
            b"value\n more",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("more"),
                (b!("value"), ((b!("\r\n"), Flags::FOLDING), Some(b!("\t "))))
            )),
            b"value\r\n\t more",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"value\n\rmore";
        assert_value_bytes_result_eq!(
            Ok((
                b!("more"),
                (
                    b!("value"),
                    ((b!("\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r")))
                )
            )),
            input,
            parser,
            parser_cr
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("more"),
                (b!("value"), ((b!("\n\r"), Flags::DEFORMED_EOL), None))
            )),
            input,
            parser_lfcr
        );

        assert_value_bytes_result_eq!(
            Ok((
                b!("more"),
                (
                    b!("value"),
                    ((b!("\r\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r")))
                )
            )),
            b"value\r\n\rmore",
            parser,
            parser_cr,
            parser_lfcr
        );
    }

    #[test]
    fn Value() {
        let parser = Parser::default();
        let parser_cr = Parser::new(Eol::CR, true);
        let parser_lfcr = Parser::new(Eol::LFCR, true);
        let input = b"value\rnext:";
        assert_value_result_eq!(Err(Incomplete(Needed::Size(1))), input, parser, parser_lfcr);
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: 0
                }
            )),
            input,
            parser_cr
        );

        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n more\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n ",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n more",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n more\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\n more\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            b"\r\n value    \r\nnext:",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            b"\r\n value\r\nnext:",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: 0
                }
            )),
            b"value\r\nnext:",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"".to_vec(),
                    flags: Flags::VALUE_EMPTY
                }
            )),
            b"\r\nnext:",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            b"value\r\n more\r\n\r\n",
            parser,
            parser_cr,
            parser_lfcr
        );
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            b"value\r\n more\r\n\tand more\r\nnext:",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"value\n more\n\r\tand more\r\n\r\n";
        assert_value_result_eq!(
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING_SPECIAL_CASE
                }
            )),
            input,
            parser,
            parser_cr
        );
        assert_value_result_eq!(
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING | Flags::DEFORMED_EOL
                }
            )),
            input,
            parser_lfcr
        );

        let input = b"value\n more\n\r\tand more\n\r\r\n";
        assert_value_result_eq!(
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::DEFORMED_EOL | Flags::FOLDING
                }
            )),
            input,
            parser_lfcr
        );
        assert_value_result_eq!(
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::DEFORMED_EOL | Flags::FOLDING_SPECIAL_CASE
                }
            )),
            input,
            parser,
            parser_cr
        );

        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            b"value\n\t\tmore\r\n  and\r\n more\r\nnext:",
            parser,
            parser_cr,
            parser_lfcr
        );

        let input = b"value\n\r\t\tmore\r\n  and\r\n more\r\nnext:";
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING_SPECIAL_CASE
                }
            )),
            input,
            parser,
            parser_cr
        );
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::DEFORMED_EOL | Flags::FOLDING
                }
            )),
            input,
            parser_lfcr
        );
    }
}
