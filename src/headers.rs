use crate::util::{is_token, take_until_null, FlagOperations};
use nom::{
    branch::alt,
    bytes::complete::tag as complete_tag,
    bytes::streaming::{tag, take_till, take_till1, take_while, take_while1},
    character::{
        complete::space1 as complete_space1,
        is_space,
        streaming::{space0, space1},
    },
    combinator::{map, not, opt, peek},
    sequence::tuple,
    Err::Incomplete,
    IResult, Needed,
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
    pub const DEFORMED_SEPARATOR: u64 = (0x0800 | Self::NAME_NON_TOKEN_CHARS);
    pub const FOLDING_EMPTY: u64 = (0x1000 | Self::DEFORMED_EOL);
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

/// Enumerates possible parser types
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Side {
    /// Request Parser: null terminates
    Request,
    /// Response Parser: accepts CR as a line ending
    Response,
}

pub struct Parser {
    side: Side,
}

impl Parser {
    pub fn new(side: Side) -> Self {
        Self { side }
    }
    /// Returns true if c is a line feed character
    fn is_eol(&self) -> impl Fn(u8) -> bool + '_ {
        move |c| c == b'\n' || (self.side == Side::Response && c == b'\r')
    }

    /// Parse one complete end of line character or character set
    fn complete_eol_regular(&self) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> + '_ {
        move |input| {
            if self.side == Side::Response {
                alt((
                    complete_tag("\r\n"),
                    complete_tag("\n\r"),
                    complete_tag("\n"),
                    complete_tag("\r"),
                ))(input)
            } else {
                alt((complete_tag("\r\n"), complete_tag("\n")))(input)
            }
        }
    }

    /// Parse one complete deformed end of line character set
    fn complete_eol_deformed(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            if self.side == Side::Response {
                alt((
                    map(
                        tuple((
                            complete_tag("\n\r\r\n"),
                            peek(alt((complete_tag("\n"), complete_tag("\r\n")))),
                        )),
                        |(eol, _)| (eol, Flags::DEFORMED_EOL),
                    ),
                    // Treat EOL + empty folding + EOL as just EOL
                    self.folding_empty(),
                    map(
                        tuple((
                            complete_tag("\r\n\r"),
                            take_while1(|c| c == b'\r' || c == b' ' || c == b'\t'),
                            opt(complete_tag("\n")),
                            not(alt((complete_tag("\n"), complete_tag("\r\n")))),
                        )),
                        |(eol1, eol2, eol3, _): (&[u8], &[u8], Option<&[u8]>, _)| {
                            (
                                &input[..(eol1.len() + eol2.len() + eol3.unwrap_or(b"").len())],
                                Flags::DEFORMED_EOL,
                            )
                        },
                    ),
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
                    |(eol, _)| (eol, Flags::DEFORMED_EOL),
                )(input)
            }
        }
    }

    /// Parse one complete end of line character or character set
    fn complete_eol(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            alt((
                self.complete_eol_deformed(),
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

    /// Parse empty header folding as a single EOL (eol + whitespace + eol = eol)
    fn folding_empty(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            map(
                tuple((
                    self.complete_eol_regular(),
                    complete_space1,
                    self.complete_eol_regular(),
                )),
                |(eol1, spaces, eol2): (&[u8], &[u8], &[u8])| {
                    (
                        &input[..eol1.len() + spaces.len() + eol2.len()],
                        Flags::FOLDING_EMPTY,
                    )
                },
            )(input)
        }
    }
    /// Parse header folding bytes (eol + whitespace or eol + special cases)
    fn folding(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], &[u8], u64)> + '_ {
        move |input| {
            if self.side == Side::Response {
                map(
                    tuple((
                        not(self.folding_empty()),
                        map(self.complete_eol_regular(), |eol| (eol, 0)),
                        folding_lws,
                    )),
                    |(_, (eol, flags), (folding_lws, other_flags))| {
                        (eol, folding_lws, flags | other_flags)
                    },
                )(input)
            } else {
                map(
                    tuple((self.complete_eol(), folding_lws)),
                    |((eol, flags), (folding_lws, other_flags))| {
                        (eol, folding_lws, flags | other_flags)
                    },
                )(input)
            }
        }
    }

    /// Special case check for end of headers with space or tab seperating the EOLs
    fn terminator_special_case(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            //Treat the empty folding as a single EOL when it is followed by another eol.
            if let Ok((remaining, ((eol, flags), _))) =
                tuple((self.folding_empty(), peek(self.complete_eol_regular())))(input)
            {
                Ok((remaining, (eol, Flags::TERMINATOR_SPECIAL_CASE | flags)))
            } else {
                map(
                    tuple((
                        self.complete_eol_regular(),
                        space1,
                        peek(tuple((
                            self.complete_eol_regular(),
                            not(tuple((token_chars, separator_regular))),
                        ))),
                    )),
                    |(eol, space, _)| {
                        (
                            &input[..eol.len() + space.len()],
                            Flags::TERMINATOR_SPECIAL_CASE,
                        )
                    },
                )(input)
            }
        }
    }

    /// Parse folding bytes or a value terminator (eol or null)
    fn folding_or_terminator(
        &self,
    ) -> impl Fn(&[u8]) -> IResult<&[u8], ((&[u8], u64), Option<&[u8]>)> + '_ {
        move |input| {
            if self.side == Side::Response {
                alt((
                    map(self.terminator_special_case(), |result| (result, None)),
                    map(self.folding(), |(end, fold, flags)| {
                        ((end, flags), Some(fold))
                    }),
                    map(self.null_or_eol(), |end| (end, None)),
                ))(input)
            } else {
                alt((
                    map(self.folding(), |(end, fold, flags)| {
                        ((end, flags), Some(fold))
                    }),
                    map(self.null_or_eol(), |end| (end, None)),
                ))(input)
            }
        }
    }

    /// Removes trailing unwanted characaters from input.
    /// If null terminates is set to true, it will remove all characters before the null character
    fn remove_trailing(&self, input: &mut Vec<u8>, flags: &mut u64) {
        if self.side == Side::Request {
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
                    if self.side == Side::Response {
                        // Peek ahead for ambiguous name with lws vs. value with folding
                        match tuple((token_chars, separator_regular))(i) {
                            Ok(_) => {
                                flags.unset(Flags::FOLDING_SPECIAL_CASE);
                                if value.is_empty() {
                                    flags.set(Flags::VALUE_EMPTY);
                                } else {
                                    self.remove_trailing(&mut value, &mut flags);
                                }
                                return Ok((rest, Value { value, flags }));
                            }
                            Err(Incomplete(_)) => {
                                return Err(Incomplete(Needed::Size(1)));
                            }
                            _ => {}
                        }
                    }
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

    /// Parse one header name, incluing the : and trailing whitespace
    fn name(&self) -> impl Fn(&[u8]) -> IResult<&[u8], Name> + '_ {
        move |input| {
            //We first attempt to parse a token name before we attempt a non token name
            map(
                alt((self.token_name(), self.non_token_name())),
                |(name, flags)| Name {
                    name: name.into(),
                    flags,
                },
            )(input)
        }
    }

    /// Parse name containing non token characters with either regular separator or deformed separator
    fn non_token_name(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            map(
                tuple((
                    space0,
                    alt((
                        tuple((
                            take_till(|c| c == b':' || self.is_terminator(c) || c == b'\r'),
                            peek(self.separator()),
                        )),
                        tuple((
                            take_till(|c| c == b':' || self.is_terminator(c)),
                            peek(self.separator()),
                        )),
                    )),
                )),
                |(leading_spaces, (mut name, _)): (&[u8], (&[u8], _))| {
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
    }

    /// Check if the byte is LF or, if is a request parser, null
    ///
    ///
    fn is_terminator(&self, c: u8) -> bool {
        c == b'\n' || (self.side == Side::Request && c == b'\0')
    }

    /// Handles accepted deformed separators
    fn separator_deformed(&self) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> + '_ {
        move |input| {
            map(
                tuple((
                    not(tuple((self.complete_eol(), self.complete_eol()))),
                    alt((
                        map(
                            tuple((
                                take_while1(is_special_whitespace),
                                complete_tag(":"),
                                space0,
                                not(tuple((self.complete_eol(), self.complete_eol()))),
                                take_while(is_special_whitespace),
                            )),
                            |(_, tagged, _, _, _)| tagged,
                        ),
                        map(
                            tuple((
                                take_while(is_special_whitespace),
                                complete_tag(":"),
                                space0,
                                not(tuple((self.complete_eol(), self.complete_eol()))),
                                take_while1(is_special_whitespace),
                            )),
                            |(_, tagged, _, _, _)| tagged,
                        ),
                    )),
                )),
                |(_, sep)| sep,
            )(input)
        }
    }

    /// Parse a separator between header name and value
    fn separator(&self) -> impl Fn(&[u8]) -> IResult<&[u8], u64> + '_ {
        move |input| {
            if self.side == Side::Response {
                alt((
                    map(self.separator_deformed(), |_| Flags::DEFORMED_SEPARATOR),
                    map(separator_regular, |_| 0),
                ))(input)
            } else {
                map(separator_regular, |_| 0)(input)
            }
        }
    }

    /// Parse name containing only token characters
    fn token_name(&self) -> impl Fn(&[u8]) -> IResult<&[u8], (&[u8], u64)> + '_ {
        move |input| {
            // The name should consist only of token characters (i.e., no spaces, seperators, control characters, etc)
            map(
                tuple((token_chars, peek(self.separator()))),
                |((leading_spaces, name, trailing_spaces), _): ((&[u8], &[u8], &[u8]), _)| {
                    let mut flags = 0;
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
    }

    /// Parse data before an eol with no colon as an empty name with the data as the value
    fn header_sans_colon(&self) -> impl Fn(&[u8]) -> IResult<&[u8], Header> + '_ {
        move |input| {
            let (mut remaining, (_, mut value)) = tuple((
                not(tag("\r\n")),
                take_till1(|c| c == b':' || self.is_terminator(c)),
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

    /// Parse a header name separator value
    fn header_with_colon(&self) -> impl Fn(&[u8]) -> IResult<&[u8], Header> + '_ {
        move |input| {
            map(
                tuple((self.name(), self.separator(), self.value())),
                |(mut name, flag, mut value)| {
                    name.flags |= flag;
                    value.flags |= flag;
                    Header { name, value }
                },
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

/// Parse a regular separator (colon followed by optional spaces) between header name and value
fn separator_regular(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    tuple((complete_tag(":"), space0))(input)
}

/// Parse token characters with leading and trailing whitespace
fn token_chars(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8], &[u8])> {
    tuple((space0, take_while(is_token), space0))(input)
}

/// Check if the input is a space, HT, VT, CR, LF, or FF
fn is_special_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == b'\x0b' || c == b'\x0c'
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
            assert_eq!($r, parser.headers()($i), "Failed to assert headers result on Eol::{:#?} parser", parser.side);
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
            assert_eq!($r, parser.header()($i), "Failed to assert header result on Eol::{:#?} parser", parser.side);
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
            assert_eq!($r, parser.eol()($i), "Failed to assert eol result on Eol::{:#?} parser", parser.side);
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
            assert_eq!($r, parser.complete_eol()($i), "Failed to assert complete_eol result on Eol::{:#?} parser", parser.side);
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
            assert_eq!($r, parser.null_or_eol()($i), "Failed to assert null_or_eol result on Eol::{:#?} parser", parser.side);
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
            assert_eq!($r, parser.folding()($i), "Failed to assert folding result on Eol::{:#?} parser", parser.side);
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
            assert_eq!($r, parser.folding_or_terminator()($i), "Failed to assert folding_or_terminator result on Eol::{:#?} parser", parser.side);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_folding_or_terminator_result_eq! { $r, $i, $p }
            assert_folding_or_terminator_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_separator_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.separator()($i), "Failed to assert separator result on Eol::{:#?} parser", parser.side);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_separator_result_eq! { $r, $i, $p }
            assert_separator_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_non_token_name_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.non_token_name()($i), "Failed to assert non_token_name result on Eol::{:#?} parser", parser.side);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_non_token_name_result_eq! { $r, $i, $p }
            assert_non_token_name_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_token_name_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.token_name()($i), "Failed to assert token_name result on Eol::{:#?} parser", parser.side);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_token_name_result_eq! { $r, $i, $p }
            assert_token_name_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_name_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.name()($i), "Failed to assert name result on Eol::{:#?} parser", parser.side);
        }};

        // Decompose multiple parsers recursively
        ($r:expr, $i:expr, $p:expr, $($es:expr),+) => {{
            assert_name_result_eq! { $r, $i, $p }
            assert_name_result_eq! { $r, $i, $($es),+ }
        }};
    }

    macro_rules! assert_value_result_eq {
        // The pattern for a single parser evaluation
        ($r:expr, $i:expr, $p:expr) => {{
	        let parser: &Parser = &$p;
            assert_eq!($r, parser.value()($i), "Failed to assert value result on Eol::{:#?} parser", parser.side);
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
            assert_eq!($r, parser.value_bytes()($i), "Failed to assert value_bytes result on Eol::{:#?} parser", parser.side);
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
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);

        let input = b"k1:v1\r\nk2:v2 before\0v2 after\r\n\r\n";
        let res_result = Ok((
            b!(""),
            (
                vec![
                    header!(b"k1", 0, b"v1", 0),
                    header!(b"k2", 0, b"v2 before\0v2 after", 0),
                ],
                true,
            ),
        ));
        let req_result = Ok((
            b!("\r\n"),
            (
                vec![
                    header!(b"k1", 0, b"v1", 0),
                    header!(b"k2", 0, b"v2 before", Flags::NULL_TERMINATED),
                ],
                true,
            ),
        ));
        assert_headers_result_eq!(res_result, input, res_parser);
        assert_headers_result_eq!(req_result, input, req_parser);
    }

    #[test]
    fn Headers() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);

        let input = b"k1:v1\r\n:v2\r\n v2+\r\nk3: v3\r\nk4 v4\r\nk\r5:v\r5\n\rmore\r\n\r\n";
        let common = vec![
            header!(b"k1", 0, b"v1", 0),
            header!(b"", Flags::NAME_EMPTY, b"v2 v2+", Flags::FOLDING),
            header!(b"k3", 0, b"v3", 0),
            header!(b"", Flags::MISSING_COLON, b"k4 v4", Flags::MISSING_COLON),
        ];
        let req_result = Ok((
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
        let res_result = Ok((
            b!(""),
            (
                [
                    common.as_slice(),
                    vec![
                        header!(b"k\r5", Flags::NAME_NON_TOKEN_CHARS, b"v", 0),
                        header!(b"", Flags::MISSING_COLON, b"5", Flags::MISSING_COLON),
                        header!(b"", Flags::MISSING_COLON, b"more", Flags::MISSING_COLON),
                    ]
                    .as_slice(),
                ]
                .concat(),
                true,
            ),
        ));
        assert_headers_result_eq!(req_result, input, req_parser);
        assert_headers_result_eq!(res_result, input, res_parser);

        let input = b"k1:v1\r\nk2:v2\r";
        let result = Ok((b!("k2:v2\r"), (vec![header!(b"k1", 0, b"v1", 0)], false)));
        assert_headers_result_eq!(result, input, req_parser, res_parser);

        let input = b"k1:v1\nk2:v2\0v2\r\nk3:v3\r";
        assert_headers_result_eq!(
            Ok((
                b!("k3:v3\r"),
                (
                    vec![
                        header!(b"k1", 0, b"v1", 0),
                        header!(b"k2", 0, b"v2", Flags::NULL_TERMINATED),
                    ],
                    true,
                ),
            )),
            input,
            req_parser
        );
        assert_headers_result_eq!(
            Ok((
                b!("k3:v3\r"),
                (
                    vec![header!(b"k1", 0, b"v1", 0), header!(b"k2", 0, b"v2\0v2", 0),],
                    false,
                ),
            )),
            input,
            res_parser
        );

        let result = Ok((
            b!(""),
            (
                vec![
                    header!(b"Name1", 0, b"Value1", 0),
                    header!(b"Name2", 0, b"Value2", 0),
                    header!(b"Name3", 0, b"Val ue3", Flags::FOLDING),
                    header!(b"Name4", 0, b"Value4 Value4.1 Value4.2", Flags::FOLDING),
                ],
                true,
            ),
        ));
        // Test only \n terminators (should be same result as above)
        let i = b"Name1: Value1\n\
                  Name2:Value2\n\
                  Name3: Val\n ue3\n\
                  Name4: Value4\n Value4.1\n Value4.2\n\
                  \n";
        assert_headers_result_eq!(result, i, req_parser, res_parser);

        // Test only \r\n terminators (should be same result as above)
        let i = b"Name1: Value1\r\n\
                  Name2:Value2\r\n\
                  Name3: Val\r\n ue3\r\n\
                  Name4: Value4\r\n Value4.1\r\n Value4.2\r\n\
                  \r\n";
        assert_headers_result_eq!(result, i, req_parser, res_parser);

        // Test a mix of \r\n and \n terminators (should be same result as above)
        let i = b"Name1: Value1\r\n\
                  Name2:Value2\n\
                  Name3: Val\r\n ue3\r\n\
                  Name4: Value4\r\n Value4.1\n Value4.2\r\n\
                  \n";
        assert_headers_result_eq!(result, i, req_parser, res_parser);

        // Test only \r terminators (should be same result as above)
        let i = b"Name1: Value1\r\
                  Name2:Value2\r\
                  Name3: Val\r\n ue3\r\
                  Name4: Value4\r\n Value4.1\r\n Value4.2\r\
                  \r\n";
        assert_headers_result_eq!(result, i, res_parser);

        // Test a mix of \r\n, \r, and \n terminators (should be same result as above)
        let i = b"Name1: Value1\r\
                  Name2:Value2\r\
                  Name3: Val\r\n ue3\r\n\
                  Name4: Value4\r\n Value4.1\n Value4.2\r\n\
                  \n";
        assert_headers_result_eq!(result, i, res_parser);

        // Test a mix of \r\n, \n\r, \n terminators
        let i = b"Name1: Value1\r\n\
                  Name2:Value2\n\
                  Name3: Val\n\r ue3\n\r\
                  Name4: Value4\r\n Value4.1\n Value4.2\r\n\
                  \n";
        assert_headers_result_eq!(result, i, res_parser);

        // Test only \n\r terminators
        let i = b"Name1: Value1\n\r\
                  Name2:Value2\n\r\
                  Name3: Val\n\r ue3\n\r\
                  Name4: Value4\n\r Value4.1\n\r Value4.2\n\r\
                  \n\r";
        assert_headers_result_eq!(result, i, res_parser);
    }

    #[test]
    fn HeaderSansColon() {
        let parser = Parser::new(Side::Request);
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
        let parser = Parser::new(Side::Request);
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
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert_header_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"K: V",
            req_parser,
            res_parser
        );
        assert_header_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"K: V\r\n",
            req_parser,
            res_parser
        );

        let input = b"Host:www.google.com\rName: Value\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"Host", 0, b"www.google.com\rName: Value", 0),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("Name: Value\r\n\r\n"),
                header!(b"Host", 0, b"www.google.com", 0),
            )),
            input,
            res_parser
        );

        let input = b"K1 V1\r\n";
        assert_header_result_eq!(
            Ok((
                b!(""),
                header!(b"", Flags::MISSING_COLON, b"K1 V1", Flags::MISSING_COLON),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(Err(Incomplete(Needed::Size(1))), b"K1 V1\r\n", res_parser);
        assert_header_result_eq!(
            Ok((
                b!("K2:V2\n\r\n"),
                header!(b"", Flags::MISSING_COLON, b"K1 V1", Flags::MISSING_COLON),
            )),
            b"K1 V1\r\nK2:V2\n\r\n",
            req_parser,
            res_parser
        );
        assert_header_result_eq!(
            Ok((b!("K2:V2\n\r\n"), header!(b"K1", 0, b"V1", 0),)),
            b"K1:V1\nK2:V2\n\r\n",
            req_parser,
            res_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"", Flags::NAME_EMPTY, b"", Flags::VALUE_EMPTY),
            )),
            b":\r\n\r\n",
            req_parser,
            res_parser
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"", Flags::VALUE_EMPTY),)),
            b"K:\r\n\r\n",
            req_parser,
            res_parser
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"", Flags::NAME_EMPTY, b"V", 0),)),
            b":V\r\n\r\n",
            req_parser,
            res_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"folded V", Flags::FOLDING_SPECIAL_CASE),
            )),
            b"K:folded\r\n\rV\r\n\r\n",
            req_parser,
            res_parser
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V", 0),)),
            b"K: V\r\n\r\n",
            req_parser,
            res_parser
        );

        let input = b"K: V before\0 V after\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"V before", Flags::NULL_TERMINATED),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V before\0 V after", 0),)),
            input,
            res_parser
        );

        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V a l u e", Flags::FOLDING),)),
            b"K: V\n a\r\n l\n u\r\n\te\r\n\r\n",
            req_parser,
            res_parser
        );

        let input = b"K: V\r a\r\n l\n u\r\n\te\r\n\r\n";
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V\r a l u e", Flags::FOLDING),)),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((b!("\r\n"), header!(b"K", 0, b"V a l u e", Flags::FOLDING),)),
            input,
            res_parser
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
            req_parser
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
            res_parser
        );

        let input = b"K:deformed folded\n\r V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"deformed folded V", Flags::FOLDING_SPECIAL_CASE),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"deformed folded V", Flags::FOLDING),
            )),
            input,
            res_parser
        );

        let input = b"K:deformed folded\n\r\r V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(b"K", 0, b"deformed folded V", Flags::FOLDING_SPECIAL_CASE),
            )),
            input,
            res_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\r V\r\n\r\n"),
                header!(b"K", 0, b"deformed folded", 0),
            )),
            input,
            req_parser
        );

        let input = b"K\r \r :\r V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K\r \r",
                    Flags::NAME_NON_TOKEN_CHARS | Flags::NAME_TRAILING_WHITESPACE,
                    b"\r V",
                    0
                ),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    Flags::DEFORMED_SEPARATOR,
                    b"V",
                    Flags::DEFORMED_SEPARATOR
                ),
            )),
            input,
            res_parser
        );

        let input = b"K\n\r \r\n :\r\n V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r \r\n :\r\n V\r\n\r\n"),
                header!(b"", Flags::MISSING_COLON, b"K", Flags::MISSING_COLON),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    Flags::DEFORMED_SEPARATOR,
                    b"V",
                    Flags::DEFORMED_SEPARATOR
                ),
            )),
            input,
            res_parser
        );

        let input = b"K\r\n \r\n :\r\n V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!(" \r\n :\r\n V\r\n\r\n"),
                header!(b"", Flags::MISSING_COLON, b"K", Flags::MISSING_COLON),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    Flags::DEFORMED_SEPARATOR,
                    b"V",
                    Flags::DEFORMED_SEPARATOR
                ),
            )),
            input,
            res_parser
        );
        let input = b"K:\r\n\0Value\r\n V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\0Value\r\n V\r\n\r\n"),
                header!(b"K", 0, b"", Flags::VALUE_EMPTY),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    Flags::DEFORMED_SEPARATOR,
                    b"\0Value V",
                    Flags::DEFORMED_SEPARATOR | Flags::FOLDING
                ),
            )),
            input,
            res_parser
        );

        let input = b"K\r\n:Value\r\n V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!(":Value\r\n V\r\n\r\n"),
                header!(b"", Flags::MISSING_COLON, b"K", Flags::MISSING_COLON),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    Flags::DEFORMED_SEPARATOR,
                    b"Value V",
                    Flags::DEFORMED_SEPARATOR | Flags::FOLDING
                ),
            )),
            input,
            res_parser
        );

        let input = b"K\x0c:Value\r\n V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K\x0c",
                    Flags::NAME_NON_TOKEN_CHARS,
                    b"Value V",
                    Flags::FOLDING
                ),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    Flags::DEFORMED_SEPARATOR,
                    b"Value V",
                    Flags::DEFORMED_SEPARATOR | Flags::FOLDING
                ),
            )),
            input,
            res_parser
        );

        let input = b"K\r :Value\r\n V\r\n\r\n";
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K\r",
                    Flags::NAME_TRAILING_WHITESPACE | Flags::NAME_NON_TOKEN_CHARS,
                    b"Value V",
                    Flags::FOLDING
                ),
            )),
            input,
            req_parser
        );
        assert_header_result_eq!(
            Ok((
                b!("\r\n"),
                header!(
                    b"K",
                    Flags::DEFORMED_SEPARATOR,
                    b"Value V",
                    Flags::DEFORMED_SEPARATOR | Flags::FOLDING
                ),
            )),
            input,
            res_parser
        );
    }

    #[test]
    fn Separator() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        let input = b" : ";
        assert_separator_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);
        assert_separator_result_eq!(Err(Incomplete(Needed::Size(1))), input, res_parser);
        let input = b" ";
        assert_separator_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);
        assert_separator_result_eq!(Err(Incomplete(Needed::Size(1))), input, res_parser);
        assert_separator_result_eq!(Ok((b!("value"), 0)), b":value", req_parser, res_parser);
        assert_separator_result_eq!(Ok((b!("value"), 0)), b": value", req_parser, res_parser);
        assert_separator_result_eq!(Ok((b!("value"), 0)), b":\t value", req_parser, res_parser);
        assert_separator_result_eq!(
            Ok((b!("value"), Flags::DEFORMED_SEPARATOR)),
            b"\r\n \n:\t\r\n value",
            res_parser
        );
        assert_separator_result_eq!(
            Ok((b!("value"), Flags::DEFORMED_SEPARATOR)),
            b"\x0c:\t value",
            res_parser
        );
        assert_separator_result_eq!(
            Ok((b!("value"), Flags::DEFORMED_SEPARATOR)),
            b"\r: value",
            res_parser
        );
    }

    #[test]
    fn TokenChars() {
        assert_eq!(Err(Incomplete(Needed::Size(1))), token_chars(b"name"));
        assert_eq!(
            Ok((b!(":"), (b!(""), b!("name"), b!("")))),
            token_chars(b"name:")
        );
        assert_eq!(
            Ok((b!(":"), (b!(""), b!("name"), b!(" ")))),
            token_chars(b"name :")
        );
        assert_eq!(
            Ok((b!(":"), (b!(" "), b!("name"), b!(" ")))),
            token_chars(b" name :")
        );
    }

    #[test]
    fn TokenName() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert_token_name_result_eq!(
            Ok((b!(": world"), (b!("Hello"), 0))),
            b"Hello: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Ok((b!(": world"), (b!("Hello"), Flags::NAME_LEADING_WHITESPACE))),
            b" Hello: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Ok((
                b!(": world"),
                (b!("Hello"), Flags::NAME_TRAILING_WHITESPACE)
            )),
            b"Hello : world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE | Flags::NAME_TRAILING_WHITESPACE
                )
            )),
            b" Hello : world",
            req_parser,
            res_parser
        );
        let input = b" Hello \r\n \n:\n world";
        assert_token_name_result_eq!(
            Ok((
                b!("\r\n \n:\n world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE | Flags::NAME_TRAILING_WHITESPACE
                )
            )),
            input,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\r\n \n:\n world".as_ref(), Tag))),
            input,
            req_parser
        );

        let input = b" Hello \n\r \n:\n world";
        assert_token_name_result_eq!(
            Ok((
                b!("\n\r \n:\n world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE | Flags::NAME_TRAILING_WHITESPACE
                )
            )),
            input,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\n\r \n:\n world".as_ref(), Tag))),
            input,
            req_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"Invalid: world".as_ref(), Tag))),
            b"Hello Invalid: world",
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b";Invalid: world".as_ref(), Tag))),
            b"Hello;Invalid: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"Hello",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\rInvalid: world".as_ref(), Tag))),
            b"Hello\rInvalid: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\nInvalid: world".as_ref(), Tag))),
            b"Hello\nInvalid: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\0Invalid: world".as_ref(), Tag))),
            b"Hello\0Invalid: world",
            req_parser,
            res_parser
        );
    }

    #[test]
    fn NonTokenName() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert_non_token_name_result_eq!(
            Ok((b!(": world"), (b!("Hello"), Flags::NAME_NON_TOKEN_CHARS))),
            b"Hello: world",
            req_parser,
            res_parser
        );
        assert_non_token_name_result_eq!(
            Ok((
                b!("\r\n \n: \r\nworld"),
                (b!("Hello"), Flags::NAME_NON_TOKEN_CHARS)
            )),
            b"Hello\r\n \n: \r\nworld",
            res_parser
        );
        assert_non_token_name_result_eq!(
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE | Flags::NAME_NON_TOKEN_CHARS
                )
            )),
            b" Hello: world",
            req_parser,
            res_parser
        );
        assert_non_token_name_result_eq!(
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_TRAILING_WHITESPACE | Flags::NAME_NON_TOKEN_CHARS
                )
            )),
            b"Hello : world",
            req_parser,
            res_parser
        );
        assert_non_token_name_result_eq!(
            Ok((
                b!(": world"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE
                        | Flags::NAME_TRAILING_WHITESPACE
                        | Flags::NAME_NON_TOKEN_CHARS
                )
            )),
            b" Hello : world",
            req_parser,
            res_parser
        );
        assert_non_token_name_result_eq!(
            Ok((
                b!("\r\n \r\n: \r\nworld"),
                (
                    b!("Hello"),
                    Flags::NAME_LEADING_WHITESPACE
                        | Flags::NAME_TRAILING_WHITESPACE
                        | Flags::NAME_NON_TOKEN_CHARS
                )
            )),
            b" Hello \r\n \r\n: \r\nworld",
            res_parser
        );
        assert_non_token_name_result_eq!(
            Ok((
                b!(": world"),
                (b!("Hello Invalid"), Flags::NAME_NON_TOKEN_CHARS)
            )),
            b"Hello Invalid: world",
            req_parser,
            res_parser
        );
        assert_non_token_name_result_eq!(
            Ok((
                b!(": world"),
                (b!("Hello;Invalid"), Flags::NAME_NON_TOKEN_CHARS)
            )),
            b"Hello;Invalid: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\rInvalid: world".as_ref(), Tag))),
            b"Hello\rInvalid: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\nInvalid: world".as_ref(), Tag))),
            b"Hello\nInvalid: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Error((b"\0Invalid: world".as_ref(), Tag))),
            b"Hello\0Invalid: world",
            req_parser,
            res_parser
        );
        assert_token_name_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"Hello",
            req_parser,
            res_parser
        );
    }

    #[test]
    fn Name() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello".to_vec(),
                    flags: 0
                }
            )),
            b"Hello: world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"".to_vec(),
                    flags: Flags::NAME_EMPTY
                }
            )),
            b": world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(":www.google.com\rName: Value"),
                Name {
                    name: b"Host".to_vec(),
                    flags: 0
                }
            )),
            b"Host:www.google.com\rName: Value",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello".to_vec(),
                    flags: Flags::NAME_TRAILING_WHITESPACE
                }
            )),
            b"Hello : world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello".to_vec(),
                    flags: Flags::NAME_LEADING_WHITESPACE | Flags::NAME_TRAILING_WHITESPACE
                }
            )),
            b" Hello : world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello;invalid".to_vec(),
                    flags: Flags::NAME_NON_TOKEN_CHARS
                }
            )),
            b"Hello;invalid: world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello invalid".to_vec(),
                    flags: Flags::NAME_NON_TOKEN_CHARS
                }
            )),
            b"Hello invalid: world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"Hello Invalid".to_vec(),
                    flags: Flags::NAME_LEADING_WHITESPACE
                        | Flags::NAME_TRAILING_WHITESPACE
                        | Flags::NAME_NON_TOKEN_CHARS
                }
            )),
            b" Hello Invalid : world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!(": world"),
                Name {
                    name: b"".to_vec(),
                    flags: Flags::NAME_EMPTY
                }
            )),
            b"  : world",
            req_parser,
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!("\r\n \r\n:\r\n world"),
                Name {
                    name: b"".to_vec(),
                    flags: Flags::NAME_EMPTY
                }
            )),
            b"  \r\n \r\n:\r\n world",
            res_parser
        );
        assert_name_result_eq!(
            Ok((
                b!("\r\n \n: \nworld"),
                Name {
                    name: b"Hello".to_vec(),
                    flags: Flags::NAME_LEADING_WHITESPACE | Flags::NAME_TRAILING_WHITESPACE
                }
            )),
            b" Hello \r\n \n: \nworld",
            res_parser
        );
        assert_name_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"Hello",
            req_parser,
            res_parser
        );
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
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert_eol_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            req_parser,
            res_parser
        );
        assert_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            req_parser,
            res_parser
        );
        assert_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\n",
            req_parser,
            res_parser
        );
        assert_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            req_parser,
            res_parser
        );
        let input = b"\ra";
        assert_eol_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);
        assert_eol_result_eq!(Ok((b!("a"), (b!("\r"), 0))), input, res_parser);

        let input = b"\r\r";
        assert_eol_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);
        assert_eol_result_eq!(Err(Incomplete(Needed::Size(1))), input, res_parser);

        assert_eol_result_eq!(Err(Incomplete(Needed::Size(1))), b"\n\r", req_parser);

        let input = b"\n\ra";
        assert_eol_result_eq!(Err(Error((b!("\ra"), Not))), input, req_parser);
        assert_eol_result_eq!(Ok((b!("a"), (b!("\n\r"), 0))), input, res_parser);
        let input = b"\n\r\n";
        assert_eol_result_eq!(Ok((b!("\r\n"), (b!("\n"), 0))), input, req_parser);
        assert_eol_result_eq!(Ok((b!("\n"), (b!("\n\r"), 0))), input, res_parser);
        let input = b"\n\r\n\r";
        assert_eol_result_eq!(Ok((b!("\r\n\r"), (b!("\n"), 0))), input, req_parser);
        assert_eol_result_eq!(Ok((b!("\n\r"), (b!("\n\r"), 0))), input, res_parser);
        assert_eol_result_eq!(
            Ok((b!("a"), (b!("\n"), 0))),
            (b"\na"),
            req_parser,
            res_parser
        );

        let input = b"\n\r\r\na";
        assert_eol_result_eq!(
            Ok((b!("\r\na"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            req_parser
        );
        assert_eol_result_eq!(Ok((b!("\r\na"), (b!("\n\r"), 0))), input, res_parser);

        assert_eol_result_eq!(
            Ok((b!("\r\na"), (b!("\r\n"), 0))),
            b"\r\n\r\na",
            req_parser,
            res_parser
        );

        assert_complete_eol_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            req_parser,
            res_parser
        );
        assert_complete_eol_result_eq!(
            Ok((b!(""), (b!("\r\n"), 0))),
            b"\r\n",
            req_parser,
            res_parser
        );
        assert_complete_eol_result_eq!(Ok((b!(""), (b!("\n"), 0))), b"\n", req_parser, res_parser);

        let input = b"\n\r\r\n";
        assert_complete_eol_result_eq!(
            Ok((b!("\r\n"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            req_parser
        );
        assert_complete_eol_result_eq!(Ok((b!("\r\n"), (b!("\n\r"), 0))), input, res_parser);

        assert_complete_eol_result_eq!(
            Ok((b!("\r\n"), (b!("\r\n"), 0))),
            b"\r\n\r\n",
            req_parser,
            res_parser
        );
    }

    #[test]
    fn NullOrEol() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert_null_or_eol_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            req_parser,
            res_parser
        );
        assert_null_or_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            req_parser,
            res_parser
        );
        assert_null_or_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\n",
            req_parser,
            res_parser
        );

        let input = b"\r";
        assert_null_or_eol_result_eq!(Err(Incomplete(Needed::Size(1))), input, res_parser);
        assert_null_or_eol_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);

        let input = b"\ra";
        assert_null_or_eol_result_eq!(Ok((b!("a"), (b!("\r"), 0))), input, res_parser);
        assert_null_or_eol_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);

        assert_null_or_eol_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            req_parser,
            res_parser
        );
        let input = b"\r\r";
        assert_null_or_eol_result_eq!(Err(Incomplete(Needed::Size(1))), input, res_parser);
        assert_null_or_eol_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);

        assert_null_or_eol_result_eq!(
            Ok((b!("a"), (b!("\0"), Flags::NULL_TERMINATED))),
            b"\0a",
            req_parser,
            res_parser
        );
        assert_null_or_eol_result_eq!(Ok((b!("a"), (b!("\n"), 0))), b"\na", req_parser, res_parser);
        let input = b"\n\r\r\na";
        assert_null_or_eol_result_eq!(
            Ok((b!("\r\na"), (b!("\n\r"), Flags::DEFORMED_EOL))),
            input,
            req_parser
        );
        assert_null_or_eol_result_eq!(Ok((b!("\r\na"), (b!("\n\r"), 0))), input, res_parser);
        assert_null_or_eol_result_eq!(
            Ok((b!("\r\n"), (b!("\r\n"), 0))),
            b"\r\n\r\n",
            req_parser,
            res_parser
        );
        let input = b"\n\r\n";
        assert_null_or_eol_result_eq!(Ok((b!("\r\n"), (b!("\n"), 0))), input, req_parser);
        assert_null_or_eol_result_eq!(Ok((b!("\n"), (b!("\n\r"), 0))), input, res_parser);
        let input = b"\n\r\n\r";
        assert_null_or_eol_result_eq!(Ok((b!("\r\n\r"), (b!("\n"), 0))), input, req_parser);
        assert_null_or_eol_result_eq!(Ok((b!("\n\r"), (b!("\n\r"), 0))), input, res_parser);
    }

    #[test]
    fn IsTerminator() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert!(req_parser.is_terminator(b'\n'));
        assert!(req_parser.is_terminator(b'\0'));
        assert!(!req_parser.is_terminator(b'\t'));
        assert!(!req_parser.is_terminator(b' '));
        assert!(!req_parser.is_terminator(b'\r'));

        assert!(res_parser.is_terminator(b'\n'));
        assert!(!res_parser.is_terminator(b'\0'));
        assert!(!res_parser.is_terminator(b'\t'));
        assert!(!res_parser.is_terminator(b' '));
        assert!(!res_parser.is_terminator(b'\r'));
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
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        assert_folding_result_eq!(
            Err(Error((b"test".as_ref(), Tag))),
            b"test",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n \t",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Err(Error((b"\n".as_ref(), Not))),
            b"\r\n\r\n",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\r",
            req_parser,
            res_parser
        );

        let input = b"\r\n\r\r";
        assert_folding_result_eq!(Err(Error((b"\r".as_ref(), Not))), input, req_parser);
        assert_folding_result_eq!(Err(Error((b"\r".as_ref(), Not))), input, req_parser);

        assert_folding_result_eq!(
            Ok((
                b!("next"),
                (b!("\r\n"), b!("\r"), Flags::FOLDING_SPECIAL_CASE)
            )),
            b"\r\n\rnext",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Ok((
                b!("next"),
                (b!("\r\n"), b!("\r\t "), Flags::FOLDING_SPECIAL_CASE)
            )),
            b"\r\n\r\t next",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Ok((b!("next"), (b!("\r\n"), b!(" "), Flags::FOLDING))),
            b"\r\n next",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Ok((b!("next"), (b!("\r\n"), b!("\t"), Flags::FOLDING))),
            b"\r\n\tnext",
            req_parser,
            res_parser
        );
        assert_folding_result_eq!(
            Ok((b!("next"), (b!("\r\n"), b!("\t "), Flags::FOLDING))),
            b"\r\n\t next",
            req_parser,
            res_parser
        );

        let input = b"\r\n\t\t\r\n";
        assert_folding_result_eq!(
            Ok((b!("\r\n"), (b!("\r\n"), b!("\t\t"), Flags::FOLDING))),
            input,
            req_parser
        );
        assert_folding_result_eq!(Err(Error((input.as_ref(), Not))), input, res_parser);

        let input = b"\r\n\t \t\r";
        assert_folding_result_eq!(
            Ok((b!("\r"), (b!("\r\n"), b!("\t \t"), Flags::FOLDING))),
            input,
            req_parser
        );
        assert_folding_result_eq!(Err(Error((input.as_ref(), Not))), input, res_parser);

        assert_folding_result_eq!(
            Ok((b!("\n"), (b!("\r\n"), b!("     "), Flags::FOLDING))),
            b"\r\n     \n",
            req_parser
        );
        assert_folding_result_eq!(
            Ok((
                b!("\n"),
                (b!("\n"), b!("\r     "), Flags::FOLDING_SPECIAL_CASE)
            )),
            b"\n\r     \n",
            req_parser
        );

        let input = b"\r    hello \n";
        assert_folding_result_eq!(
            Ok((b!("hello \n"), (b!("\r"), b!("    "), Flags::FOLDING))),
            input,
            res_parser
        );
        assert_folding_result_eq!(Err(Error((input.as_ref(), Tag))), input, req_parser);
    }

    #[test]
    fn FoldingOrTerminator() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        // All of these fail because they are incomplete.
        // We need more bytes before we can get the full fold
        // or decide there is no fold.
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n",
            req_parser,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\t",
            req_parser,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n ",
            req_parser,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\r\n\r",
            req_parser,
            res_parser
        );

        let input = b"\r\r";
        assert_folding_or_terminator_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            input,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r\n"), Flags::FOLDING), Some(b!("\t"))))),
            b"\r\n\ta",
            req_parser,
            res_parser
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
            req_parser,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r\n"), Flags::FOLDING), Some(b!(" "))))),
            b"\r\n a",
            req_parser,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r\n"), 0), None))),
            b"\r\na",
            req_parser,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\na"), ((b!("\n"), 0), None))),
            b"\n\na",
            req_parser,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\na"), ((b!("\r\n"), 0), None))),
            b"\r\n\r\na",
            req_parser,
            res_parser
        );
        let input = b"\n\r\r\na";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\na"), ((b!("\n\r"), Flags::DEFORMED_EOL), None))),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\na"), ((b!("\n\r"), 0), None))),
            input,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\0"), Flags::NULL_TERMINATED), None))),
            b"\0a",
            req_parser,
            res_parser
        );

        let input = b"\r a";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a"), ((b!("\r"), Flags::FOLDING), Some(b!(" "))))),
            input,
            res_parser
        );
        assert_folding_or_terminator_result_eq!(
            Err(Error((input.as_ref(), Tag))),
            input,
            req_parser
        );
        let input = b"\n\r \na:b";
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\na:b"),
                ((b!("\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r ")))
            )),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a:b"), ((b!("\n\r \n"), Flags::FOLDING_EMPTY), None))),
            input,
            res_parser
        );

        let input = b"\n\r \na:b";
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\na:b"),
                ((b!("\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r ")))
            )),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a:b"), ((b!("\n\r \n"), Flags::FOLDING_EMPTY), None))),
            input,
            res_parser
        );

        let input = b"\n \na:b";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\na:b"), ((b!("\n"), Flags::FOLDING), Some(b!(" "))))),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a:b"), ((b!("\n \n"), Flags::FOLDING_EMPTY), None))),
            input,
            res_parser
        );

        let input = b"\r\n \na:b";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\na:b"), ((b!("\r\n"), Flags::FOLDING), Some(b!(" "))))),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a:b"), ((b!("\r\n \n"), Flags::FOLDING_EMPTY), None))),
            input,
            res_parser
        );

        let input = b"\r\n \r\na:b";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\na:b"), ((b!("\r\n"), Flags::FOLDING), Some(b!(" "))))),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((b!("a:b"), ((b!("\r\n \r\n"), Flags::FOLDING_EMPTY), None))),
            input,
            res_parser
        );

        let input = b"\n \r\na\n";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\na\n"), ((b!("\n"), Flags::FOLDING), Some(b!(" "))))),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\r\na\n"),
                ((b!("\n "), Flags::TERMINATOR_SPECIAL_CASE), None)
            )),
            input,
            res_parser
        );

        let input = b"\n \r\n\n";
        assert_folding_or_terminator_result_eq!(
            Ok((b!("\r\n\n"), ((b!("\n"), Flags::FOLDING), Some(b!(" "))))),
            input,
            req_parser
        );
        assert_folding_or_terminator_result_eq!(
            Ok((
                b!("\n"),
                (
                    (
                        b!("\n \r\n"),
                        Flags::FOLDING_EMPTY | Flags::TERMINATOR_SPECIAL_CASE
                    ),
                    None
                )
            )),
            input,
            res_parser
        );
    }

    #[test]
    fn ValueBytes() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        // Expect fail because we need to see EOL
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b" ",
            req_parser,
            res_parser
        );
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value",
            req_parser,
            res_parser
        );
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"\tvalue",
            req_parser,
            res_parser
        );
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b" value",
            req_parser,
            res_parser
        );

        let input = b"value\rname2";
        assert_value_bytes_result_eq!(Err(Incomplete(Needed::Size(1))), input, req_parser);
        assert_value_bytes_result_eq!(
            Ok((b!("name2"), (b!("value"), ((b!("\r"), 0), None)))),
            input,
            res_parser
        );
        // Expect fail because we need to see past EOL to check for folding
        assert_value_bytes_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n",
            req_parser,
            res_parser
        );

        let input = b"www.google.com\rName: Value\r\n\r\n";
        assert_value_bytes_result_eq!(
            Ok((
                b!("\r\n"),
                (b!("www.google.com\rName: Value"), ((b!("\r\n"), 0), None))
            )),
            input,
            req_parser
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("Name: Value\r\n\r\n"),
                (b!("www.google.com"), ((b!("\r"), 0), None))
            )),
            input,
            res_parser
        );

        let input = b"www.google.com\rName: Value\n\r\n";
        assert_value_bytes_result_eq!(
            Ok((
                b!("\r\n"),
                (b!("www.google.com\rName: Value"), ((b!("\n"), 0), None))
            )),
            input,
            req_parser
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("Name: Value\n\r\n"),
                (b!("www.google.com"), ((b!("\r"), 0), None))
            )),
            input,
            res_parser
        );

        let input = b"www.google.com\rName: Value\r\n\n";
        assert_value_bytes_result_eq!(
            Ok((
                b!("\n"),
                (b!("www.google.com\rName: Value"), ((b!("\r\n"), 0), None))
            )),
            input,
            req_parser
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("Name: Value\r\n\n"),
                (b!("www.google.com"), ((b!("\r"), 0), None))
            )),
            input,
            res_parser
        );

        assert_value_bytes_result_eq!(
            Ok((b!("next"), (b!(""), ((b!("\r\n"), 0), None)))),
            b"\r\nnext",
            req_parser,
            res_parser
        );
        assert_value_bytes_result_eq!(
            Ok((b!("name2"), (b!("value"), ((b!("\r\n"), 0), None)))),
            b"value\r\nname2",
            req_parser,
            res_parser
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("more"),
                (b!("value"), ((b!("\n"), Flags::FOLDING), Some(b!(" "))))
            )),
            b"value\n more",
            req_parser,
            res_parser
        );
        assert_value_bytes_result_eq!(
            Ok((
                b!("more"),
                (b!("value"), ((b!("\r\n"), Flags::FOLDING), Some(b!("\t "))))
            )),
            b"value\r\n\t more",
            req_parser,
            res_parser
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
            req_parser
        );
        assert_value_bytes_result_eq!(
            Ok((b!("more"), (b!("value"), ((b!("\n\r"), 0), None)))),
            input,
            res_parser
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
            req_parser,
            res_parser
        );
    }

    #[test]
    fn Value() {
        let req_parser = Parser::new(Side::Request);
        let res_parser = Parser::new(Side::Response);
        let input = b"value\rnext:";
        assert_value_result_eq!(Err(Incomplete(Needed::Size(1))), input, req_parser);
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: 0
                }
            )),
            input,
            res_parser
        );

        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n more\r\n",
            req_parser,
            res_parser
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n ",
            req_parser,
            res_parser
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n more",
            req_parser,
            res_parser
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\r\n more\n",
            req_parser,
            res_parser
        );
        assert_value_result_eq!(
            Err(Incomplete(Needed::Size(1))),
            b"value\n more\r\n",
            req_parser,
            res_parser
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
            req_parser,
            res_parser
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
            req_parser,
            res_parser
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
            req_parser,
            res_parser
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
            req_parser,
            res_parser
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
            req_parser,
            res_parser
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
            req_parser,
            res_parser
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
            req_parser
        );
        assert_value_result_eq!(
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            input,
            res_parser
        );

        let input = b"value\n more\n\r\tand more\n\r\r\n";
        assert_value_result_eq!(
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            input,
            res_parser
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
            req_parser
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
            req_parser,
            res_parser
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
            req_parser
        );
        assert_value_result_eq!(
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING
                }
            )),
            input,
            res_parser
        );

        let input = b"value1\n\r next: value2\r\n  and\r\n more\r\nnext3:";
        assert_value_result_eq!(
            Ok((
                b!("next3:"),
                Value {
                    value: b"value1 next: value2 and more".to_vec(),
                    flags: Flags::FOLDING_SPECIAL_CASE
                }
            )),
            input,
            req_parser
        );
        assert_value_result_eq!(
            Ok((
                b!("next: value2\r\n  and\r\n more\r\nnext3:"),
                Value {
                    value: b"value1".to_vec(),
                    flags: 0
                }
            )),
            input,
            res_parser
        );
    }
}
