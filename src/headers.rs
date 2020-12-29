use crate::util::is_token;
use bitflags;
use nom::{
    branch::alt,
    bytes::complete::tag as complete_tag,
    bytes::streaming::{tag, take_till, take_till1, take_while},
    character::is_space,
    character::streaming::{space0, space1},
    combinator::{map, not, peek},
    sequence::tuple,
    IResult,
};

bitflags::bitflags! {
    pub struct Flags: u64 {
        const FOLDING = 0x0001;
        const FOLDING_SPECIAL_CASE = (0x0002 | Self::FOLDING.bits);
        const NAME_EMPTY = 0x0004;
        const VALUE_EMPTY = 0x0008;
        const NAME_NON_TOKEN_CHARS = 0x0010;
        const NAME_TRAILING_WHITESPACE = 0x0020;
        const NAME_LEADING_WHITESPACE = 0x0040;
        const NULL_TERMINATED = 0x0080;
        const MISSING_COLON = (0x0100 | Self::NAME_EMPTY.bits);
        const DEFORMED_EOL = 0x0200;
    }
}

#[derive(Debug, PartialEq)]
pub struct Name {
    pub name: Vec<u8>,
    pub flags: Flags,
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub value: Vec<u8>,
    pub flags: Flags,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub name: Name,
    pub value: Value,
}

/// Parse name containing non token characters
fn non_token_name(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
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
                    flags |= Flags::NAME_LEADING_WHITESPACE
                }
                while let Some(end) = name.last() {
                    if is_space(*end) {
                        flags |= Flags::NAME_TRAILING_WHITESPACE;
                        name = &name[..name.len() - 1];
                    } else {
                        break;
                    }
                }
            } else {
                flags |= Flags::NAME_EMPTY
            }
            (name, flags)
        },
    )(input)
}

/// Parse name containing only token characters
fn token_name(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
    // The name should consist only of token characters (i.e., no spaces, seperators, control characters, etc)
    map(
        tuple((space0, take_while(is_token), space0, peek(tag(":")))),
        |(leading_spaces, name, trailing_spaces, _): (&[u8], &[u8], &[u8], _)| {
            let mut flags = Flags::empty();
            if !name.is_empty() {
                if !leading_spaces.is_empty() {
                    flags |= Flags::NAME_LEADING_WHITESPACE
                }
                if !trailing_spaces.is_empty() {
                    flags |= Flags::NAME_TRAILING_WHITESPACE
                }
            } else {
                flags |= Flags::NAME_EMPTY
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

/// Parse one complete end of line character or character set
fn complete_eol_regular(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((complete_tag(b"\r\n"), complete_tag(b"\n")))(input)
}

/// Parse one complete deformed end of line character set
fn complete_eol_deformed(input: &[u8]) -> IResult<&[u8], &[u8]> {
    complete_tag(b"\n\r\r\n")(input)
}

/// Parse one complete end of line character or character set
fn complete_eol(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
    alt((
        map(complete_eol_deformed, |eol| (eol, Flags::DEFORMED_EOL)),
        map(complete_eol_regular, |eol| (eol, Flags::empty())),
    ))(input)
}

/// Parse one header end of line, and guarantee that it is not folding
fn eol(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
    map(tuple((complete_eol, peek(not(folding_lws)))), |(end, _)| {
        end
    })(input)
}

/// Check if the byte is LF or null
fn is_terminator(c: u8) -> bool {
    c == b'\n' || c == b'\0'
}

/// Parse one null character and return it and the NULL_TERMINATED flag
fn null(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
    map(complete_tag("\0"), |null| (null, Flags::NULL_TERMINATED))(input)
}

/// Parse one null byte or one end of line, and guarantee that it is not folding
fn null_or_eol(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
    alt((null, eol))(input)
}

/// Parse one null byte or complete end of line
fn complete_null_or_eol(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
    alt((null, complete_eol))(input)
}

/// Handles any special cases that are exceptions to the spec
///
/// Currently handles the use of a single CR as folding LWS
fn folding_lws_special(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((tag("\r"), peek(not(alt((tag("\r"), tag("\n"))))), space0)),
        |(fold, _, spaces): (&[u8], _, &[u8])| &input[..fold.len() + spaces.len()],
    )(input)
}

/// Extracts any folding lws (whitespace or any special cases)
fn folding_lws(input: &[u8]) -> IResult<&[u8], (&[u8], Flags)> {
    alt((
        map(space1, |fold| (fold, Flags::FOLDING)),
        map(folding_lws_special, |fold| {
            (fold, Flags::FOLDING_SPECIAL_CASE)
        }),
    ))(input)
}

/// Parse header folding bytes (eol + whitespace or eol + special cases)
fn folding(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8], Flags)> {
    map(
        tuple((complete_eol, folding_lws)),
        |((eol, flags), (folding_lws, other_flags))| (eol, folding_lws, flags | other_flags),
    )(input)
}

/// Parse folding bytes or a value terminator (eol or null)
fn folding_or_terminator(input: &[u8]) -> IResult<&[u8], ((&[u8], Flags), Option<&[u8]>)> {
    if let Ok((rest, (end, fold, flags))) = folding(input) {
        Ok((rest, ((end, flags), Some(fold))))
    } else {
        map(null_or_eol, |end| (end, None))(input)
    }
}

/// Parse a header value.
/// Returns the bytes and the value terminator; null, eol or folding
/// eg. (bytes, (eol_bytes, Option<fold_bytes>))
fn value_bytes(input: &[u8]) -> IResult<&[u8], (&[u8], ((&[u8], Flags), Option<&[u8]>))> {
    map(
        tuple((take_till(is_terminator), folding_or_terminator)),
        |(mut value, ((mut eol, flags), fold))| {
            if value.last() == Some(&b'\r') {
                value = &value[..value.len() - 1];
                eol = &input[value.len()..value.len() + eol.len() + 1];
            }
            (value, ((eol, flags), fold))
        },
    )(input)
}

/// Parse a complete header value, including any folded headers
fn value(input: &[u8]) -> IResult<&[u8], Value> {
    let (rest, (val_bytes, ((_eol, mut flags), fold))) = value_bytes(input)?;
    let mut value = val_bytes.to_vec();
    if fold.is_some() {
        let mut i = rest;
        loop {
            match value_bytes(i) {
                Ok((rest, (val_bytes, ((_eol, other_flags), fold)))) => {
                    i = rest;
                    flags |= other_flags;
                    //If the value is empty, the value started with a fold and we don't want to push back a space
                    if !value.is_empty() {
                        value.push(b' ');
                    }
                    value.extend(val_bytes);
                    if fold.is_none() {
                        remove_trailing_lws(&mut value);
                        return Ok((rest, Value { value, flags }));
                    }
                }
                Err(e) => return Err(e),
            }
        }
    } else {
        if value.is_empty() {
            flags |= Flags::VALUE_EMPTY;
        } else {
            remove_trailing_lws(&mut value);
        }
        Ok((rest, Value { value, flags }))
    }
}

/// Removes trailing lws from input
fn remove_trailing_lws(input: &mut Vec<u8>) {
    while let Some(end) = input.last() {
        if is_space(*end) {
            input.pop();
        } else {
            break;
        }
    }
}
/// Parse a separator (colon + space) between header name and value
fn separator(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    tuple((tag(b":"), space0))(input)
}

/// Parse data before an eol with no colon as an empty name with the data as the value
fn header_sans_colon(input: &[u8]) -> IResult<&[u8], Header> {
    map(
        tuple((
            peek(not(tag("\r\n"))),
            take_till1(|c| c == b':' || is_terminator(c)),
            complete_null_or_eol,
        )),
        |(_, mut value, (_, flags))| {
            if value.last() == Some(&b'\r') {
                value = &value[..value.len() - 1];
            }
            Header {
                name: Name {
                    name: Vec::new(),
                    flags: Flags::MISSING_COLON | flags,
                },
                value: Value {
                    value: value.into(),
                    flags: Flags::MISSING_COLON | flags,
                },
            }
        },
    )(input)
}

/// Parse a header name: value
fn header_with_colon(input: &[u8]) -> IResult<&[u8], Header> {
    map(tuple((name, separator, value)), |(name, _, value)| Header {
        name,
        value,
    })(input)
}

/// Parses a header name and value with, or without a colon separator
fn header(input: &[u8]) -> IResult<&[u8], Header> {
    alt((header_with_colon, header_sans_colon))(input)
}

/// Parse multiple headers and indicate if end of headers or null was found
pub fn headers(input: &[u8]) -> IResult<&[u8], (Vec<Header>, bool)> {
    let (rest, head) = header(input)?;
    let is_null_terminated = head.value.flags.contains(Flags::NULL_TERMINATED);
    let mut out = Vec::with_capacity(16);
    out.push(head);
    if is_null_terminated {
        return Ok((rest, (out, true)));
    }
    if let Ok((rest, _eoh)) = complete_eol(rest) {
        return Ok((rest, (out, true)));
    }
    let mut i = rest;
    loop {
        match header(i) {
            Ok((rest, head)) => {
                i = rest;

                let is_null_terminated = head.value.flags.contains(Flags::NULL_TERMINATED);
                out.push(head);
                if is_null_terminated {
                    return Ok((rest, (out, true)));
                }
                if let Ok((rest, _eoh)) = complete_eol(rest) {
                    return Ok((rest, (out, true)));
                }
            }
            Err(nom::Err::Incomplete(_)) => {
                return Ok((rest, (out, false)));
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::headers::*;

    macro_rules! b {
        ($b: literal) => {
            $b.as_bytes()
        };
    }

    #[test]
    fn Headers() {
        assert_eq!(
            headers(b"k1:v1\r\n:v2\r\n v2+\r\nk3: v3\r\nk4 v4\r\nk\r5:v\r5\n\rmore\r\n\r\n"),
            Ok((
                b!(""),
                (
                    vec![
                        Header {
                            name: Name {
                                name: b"k1".to_vec(),
                                flags: Flags::empty()
                            },
                            value: Value {
                                value: b"v1".to_vec(),
                                flags: Flags::empty()
                            },
                        },
                        Header {
                            name: Name {
                                name: b"".to_vec(),
                                flags: Flags::NAME_EMPTY
                            },
                            value: Value {
                                value: b"v2 v2+".to_vec(),
                                flags: Flags::FOLDING
                            },
                        },
                        Header {
                            name: Name {
                                name: b"k3".to_vec(),
                                flags: Flags::empty()
                            },
                            value: Value {
                                value: b"v3".to_vec(),
                                flags: Flags::empty()
                            },
                        },
                        Header {
                            name: Name {
                                name: b"".to_vec(),
                                flags: Flags::NAME_EMPTY | Flags::MISSING_COLON
                            },
                            value: Value {
                                value: b"k4 v4".to_vec(),
                                flags: Flags::NAME_EMPTY | Flags::MISSING_COLON
                            },
                        },
                        Header {
                            name: Name {
                                name: b"k\r5".to_vec(),
                                flags: Flags::NAME_NON_TOKEN_CHARS
                            },
                            value: Value {
                                value: b"v\r5 more".to_vec(),
                                flags: Flags::FOLDING_SPECIAL_CASE
                            },
                        }
                    ],
                    true
                ),
            ))
        );
        assert_eq!(
            headers(b"k1:v1\r\nk2:v2\r"),
            Ok((
                b!("k2:v2\r"),
                (
                    vec![Header {
                        name: Name {
                            name: b"k1".to_vec(),
                            flags: Flags::empty()
                        },
                        value: Value {
                            value: b"v1".to_vec(),
                            flags: Flags::empty()
                        },
                    },],
                    false
                ),
            ))
        );
        assert_eq!(
            headers(b"k1:v1\nk2:v2\0v2\r\nk3:v3\r"),
            Ok((
                b!("v2\r\nk3:v3\r"),
                (
                    vec![
                        Header {
                            name: Name {
                                name: b"k1".to_vec(),
                                flags: Flags::empty()
                            },
                            value: Value {
                                value: b"v1".to_vec(),
                                flags: Flags::empty()
                            },
                        },
                        Header {
                            name: Name {
                                name: b"k2".to_vec(),
                                flags: Flags::empty()
                            },
                            value: Value {
                                value: b"v2".to_vec(),
                                flags: Flags::NULL_TERMINATED
                            },
                        },
                    ],
                    true
                ),
            ))
        );

        let result = Ok((
            b!(""),
            (
                vec![
                    Header {
                        name: Name {
                            name: b"Name1".to_vec(),
                            flags: Flags::empty(),
                        },
                        value: Value {
                            value: b"Value1".to_vec(),
                            flags: Flags::empty(),
                        },
                    },
                    Header {
                        name: Name {
                            name: b"Name2".to_vec(),
                            flags: Flags::empty(),
                        },
                        value: Value {
                            value: b"Value2".to_vec(),
                            flags: Flags::empty(),
                        },
                    },
                    Header {
                        name: Name {
                            name: b"Name3".to_vec(),
                            flags: Flags::empty(),
                        },
                        value: Value {
                            value: b"Val ue3".to_vec(),
                            flags: Flags::FOLDING,
                        },
                    },
                    Header {
                        name: Name {
                            name: b"Name4".to_vec(),
                            flags: Flags::empty(),
                        },
                        value: Value {
                            value: b"Value4 Value4.1 Value4.2".to_vec(),
                            flags: Flags::FOLDING,
                        },
                    },
                ],
                true,
            ),
        ));
        //Test only \n terminators (should be same result as above)
        assert_eq!(result, headers(b"Name1: Value1\nName2:Value2\nName3: Val\n ue3\nName4: Value4\n Value4.1\n Value4.2\n\n"));
        //Test only \r\n terminators (should be same result as above)
        assert_eq!(result, headers(b"Name1: Value1\r\nName2:Value2\r\nName3: Val\r\n ue3\r\nName4: Value4\r\n Value4.1\r\n Value4.2\r\n\r\n"));
        //Test a mix of \r\n and \n terminators (should be same result as above)
        assert_eq!(result, headers(b"Name1: Value1\r\nName2:Value2\nName3: Val\r\n ue3\r\nName4: Value4\r\n Value4.1\n Value4.2\r\n\n"));
    }

    #[test]
    fn HeaderSansColon() {
        assert!(header_sans_colon(b"K V").is_err());
        assert!(header_sans_colon(b"K:V\r\n").is_err());
        assert!(header_sans_colon(b"\r\n").is_err());
        assert_eq!(
            header_sans_colon(b"K V\0alue\r\n"),
            Ok((
                b!("alue\r\n"),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::MISSING_COLON | Flags::NULL_TERMINATED
                    },
                    value: Value {
                        value: b"K V".to_vec(),
                        flags: Flags::MISSING_COLON | Flags::NULL_TERMINATED
                    },
                },
            ))
        );
        assert_eq!(
            header_sans_colon(b"K V\ralue\r\n"),
            Ok((
                b!(""),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::MISSING_COLON
                    },
                    value: Value {
                        value: b"K V\ralue".to_vec(),
                        flags: Flags::MISSING_COLON
                    },
                },
            ))
        );
        let result = Ok((
            b!("k1:v1\r\n"),
            Header {
                name: Name {
                    name: b"".to_vec(),
                    flags: Flags::MISSING_COLON,
                },
                value: Value {
                    value: b"K V".to_vec(),
                    flags: Flags::MISSING_COLON,
                },
            },
        ));
        assert_eq!(result, header_sans_colon(b"K V\r\nk1:v1\r\n"));
        assert_eq!(result, header_sans_colon(b"K V\nk1:v1\r\n"));
    }

    #[test]
    fn HeaderWithColon() {
        assert!(header_with_colon(b"K: V").is_err());
        assert!(header_with_colon(b"K: V\r\n").is_err());
        assert!(header_with_colon(b"K V\r\n").is_err());
        assert!(header_with_colon(b"K V\r\nK:V\r\n").is_err());
        assert!(header_with_colon(b"K\0ey:Value\r\nK:V\r\n").is_err());
        assert_eq!(
            header_with_colon(b"K1:V1\nK2:V2\n\r\n"),
            Ok((
                b!("K2:V2\n\r\n"),
                Header {
                    name: Name {
                        name: b"K1".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V1".to_vec(),
                        flags: Flags::empty()
                    },
                }
            ))
        );
        assert_eq!(
            header_with_colon(b":\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::NAME_EMPTY
                    },
                    value: Value {
                        value: b"".to_vec(),
                        flags: Flags::VALUE_EMPTY
                    },
                }
            ))
        );
        assert_eq!(
            header_with_colon(b"K:\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"".to_vec(),
                        flags: Flags::VALUE_EMPTY
                    },
                }
            ))
        );
        assert_eq!(
            header_with_colon(b":V\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::NAME_EMPTY
                    },
                    value: Value {
                        value: b"V".to_vec(),
                        flags: Flags::empty()
                    },
                }
            ))
        );
        assert_eq!(
            header_with_colon(b"K:folded\r\n\rV\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"folded V".to_vec(),
                        flags: Flags::FOLDING_SPECIAL_CASE
                    },
                }
            ))
        );
        assert_eq!(
            header_with_colon(b"K: V\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V".to_vec(),
                        flags: Flags::empty()
                    },
                }
            ))
        );
        assert_eq!(
            header_with_colon(b"K: V before\0 V after\r\n\r\n"),
            Ok((
                b!(" V after\r\n\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V before".to_vec(),
                        flags: Flags::NULL_TERMINATED
                    },
                }
            ))
        );
        assert_eq!(
            header_with_colon(b"K: V\r\n a\r\n l\r\n u\r\n\te\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V a l u e".to_vec(),
                        flags: Flags::FOLDING
                    },
                }
            ))
        );
    }

    #[test]
    fn Header() {
        assert!(header(b"K: V").is_err());
        assert!(header(b"K: V\r\n").is_err());
        assert_eq!(
            header(b"Host:www.google.com\rName: Value\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"Host".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"www.google.com\rName: Value".to_vec(),
                        flags: Flags::empty()
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K1 V1\r\n"),
            Ok((
                b!(""),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::MISSING_COLON
                    },
                    value: Value {
                        value: b"K1 V1".to_vec(),
                        flags: Flags::MISSING_COLON
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K1 V1\r\nK2:V2\n\r\n"),
            Ok((
                b!("K2:V2\n\r\n"),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::MISSING_COLON
                    },
                    value: Value {
                        value: b"K1 V1".to_vec(),
                        flags: Flags::MISSING_COLON
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K1:V1\nK2:V2\n\r\n"),
            Ok((
                b!("K2:V2\n\r\n"),
                Header {
                    name: Name {
                        name: b"K1".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V1".to_vec(),
                        flags: Flags::empty()
                    },
                }
            ))
        );
        assert_eq!(
            header(b":\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::NAME_EMPTY
                    },
                    value: Value {
                        value: b"".to_vec(),
                        flags: Flags::VALUE_EMPTY
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K:\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"".to_vec(),
                        flags: Flags::VALUE_EMPTY
                    },
                }
            ))
        );
        assert_eq!(
            header(b":V\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"".to_vec(),
                        flags: Flags::NAME_EMPTY
                    },
                    value: Value {
                        value: b"V".to_vec(),
                        flags: Flags::empty()
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K:folded\r\n\rV\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"folded V".to_vec(),
                        flags: Flags::FOLDING_SPECIAL_CASE
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K: V\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V".to_vec(),
                        flags: Flags::empty()
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K: V before\0 V after\r\n\r\n"),
            Ok((
                b!(" V after\r\n\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V before".to_vec(),
                        flags: Flags::NULL_TERMINATED
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K: V\n a\r\n l\n u\r\n\te\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V a l u e".to_vec(),
                        flags: Flags::FOLDING
                    },
                }
            ))
        );
        assert_eq!(
            header(b"K: V\r a\r\n l\n u\r\n\te\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Header {
                    name: Name {
                        name: b"K".to_vec(),
                        flags: Flags::empty()
                    },
                    value: Value {
                        value: b"V\r a l u e".to_vec(),
                        flags: Flags::FOLDING
                    },
                }
            ))
        );
    }

    #[test]
    fn Separator() {
        assert!(separator(b" : ").is_err());
        assert!(separator(b" ").is_err());
        assert!(separator(b": value").is_ok());
        assert!(separator(b":value").is_ok());
        assert!(separator(b": value").is_ok());
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
            Ok((b!(": world"), (b!("Hello"), Flags::empty())))
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
                    flags: Flags::empty()
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
        assert!(eol(b"test").is_err());
        assert!(eol(b"\r\n").is_err());
        assert!(eol(b"\n").is_err());
        assert!(eol(b"\r\n ").is_err());
        assert!(eol(b"\r\n\t").is_err());
        assert!(eol(b"\r\n\t ").is_err());
        assert!(eol(b"\r\r").is_err());
        assert!(eol(b"\ra").is_err());
        assert_eq!(eol(b"\na"), Ok((b!("a"), (b!("\n"), Flags::empty()))));
        assert_eq!(
            eol(b"\n\r\r\na"),
            Ok((b!("a"), (b!("\n\r\r\n"), Flags::DEFORMED_EOL)))
        );
        assert_eq!(
            eol(b"\r\n\r\na"),
            Ok((b!("\r\na"), (b!("\r\n"), Flags::empty())))
        );

        assert!(complete_eol(b"test").is_err());
        assert!(complete_eol(b"\r\n").is_ok());
        assert!(complete_eol(b"\n").is_ok());
        assert_eq!(
            complete_eol(b"\r\n"),
            Ok((b!(""), (b!("\r\n"), Flags::empty())))
        );
        assert_eq!(
            complete_eol(b"\n"),
            Ok((b!(""), (b!("\n"), Flags::empty())))
        );
        assert_eq!(
            complete_eol(b"\n\r\r\n"),
            Ok((b!(""), (b!("\n\r\r\n"), Flags::DEFORMED_EOL)))
        );
        assert_eq!(
            complete_eol(b"\r\n\r\n"),
            Ok((b!("\r\n"), (b!("\r\n"), Flags::empty())))
        );
    }

    #[test]
    fn NullOrEol() {
        assert!(null_or_eol(b"test").is_err());
        assert!(null_or_eol(b"\r\n").is_err());
        assert!(null_or_eol(b"\n").is_err());
        assert!(null_or_eol(b"\r").is_err());
        assert!(null_or_eol(b"\r\n ").is_err());
        assert!(null_or_eol(b"\r\n\t").is_err());
        assert!(null_or_eol(b"\r\n\t ").is_err());
        assert!(null_or_eol(b"\ra").is_err());
        assert!(null_or_eol(b"\r\r").is_err());
        assert_eq!(
            null_or_eol(b"\0a"),
            Ok((b!("a"), (b!("\0"), Flags::NULL_TERMINATED)))
        );
        assert_eq!(
            null_or_eol(b"\na"),
            Ok((b!("a"), (b!("\n"), Flags::empty())))
        );
        assert_eq!(
            null_or_eol(b"\n\r\r\na"),
            Ok((b!("a"), (b!("\n\r\r\n"), Flags::DEFORMED_EOL)))
        );
        assert_eq!(
            null_or_eol(b"\r\n\r\na"),
            Ok((b!("\r\na"), (b!("\r\n"), Flags::empty())))
        );
        assert_eq!(
            null_or_eol(b"\r\n\r\n"),
            Ok((b!("\r\n"), (b!("\r\n"), Flags::empty())))
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
        assert!(folding(b"test").is_err());
        assert!(folding(b"\r\n").is_err());
        assert!(folding(b"\r\n ").is_err());
        assert!(folding(b"\r\n\t").is_err());
        assert!(folding(b"\r\n\t ").is_err());
        assert!(folding(b"\r\n \t").is_err());
        assert!(folding(b"\r\n\r\n").is_err());
        assert!(folding(b"\r\n\r").is_err());
        assert!(folding(b"\r\n\r\r").is_err());
        assert_eq!(
            folding(b"\r\n\rnext"),
            Ok((
                b!("next"),
                (b!("\r\n"), b!("\r"), Flags::FOLDING_SPECIAL_CASE)
            ))
        );
        assert_eq!(
            folding(b"\r\n\r\t next"),
            Ok((
                b!("next"),
                (b!("\r\n"), b!("\r\t "), Flags::FOLDING_SPECIAL_CASE)
            ))
        );
        assert_eq!(
            folding(b"\r\n next"),
            Ok((b!("next"), (b!("\r\n"), b!(" "), Flags::FOLDING)))
        );
        assert_eq!(
            folding(b"\r\n\tnext"),
            Ok((b!("next"), (b!("\r\n"), b!("\t"), Flags::FOLDING)))
        );
        assert_eq!(
            folding(b"\r\n\t next"),
            Ok((b!("next"), (b!("\r\n"), b!("\t "), Flags::FOLDING)))
        );
        assert_eq!(
            folding(b"\r\n\t\t\r\n"),
            Ok((b!("\r\n"), (b!("\r\n"), b!("\t\t"), Flags::FOLDING)))
        );
        assert_eq!(
            folding(b"\r\n\t \t\r"),
            Ok((b!("\r"), (b!("\r\n"), b!("\t \t"), Flags::FOLDING)))
        );
        assert_eq!(
            folding(b"\r\n     \n"),
            Ok((b!("\n"), (b!("\r\n"), b!("     "), Flags::FOLDING)))
        );
    }

    #[test]
    fn FoldingOrTerminator() {
        // All of these fail because they are incomplete.
        // We need more bytes before we can get the full fold
        // or decide there is no fold.
        assert!(folding_or_terminator(b"\r\n").is_err());
        assert!(folding_or_terminator(b"\r\n\t").is_err());
        assert!(folding_or_terminator(b"\r\n ").is_err());
        assert!(folding_or_terminator(b"\r\n\r").is_err());
        assert!(folding_or_terminator(b"\r\r").is_err());
        assert_eq!(
            folding_or_terminator(b"\r\n\ta"),
            Ok((b!("a"), ((b!("\r\n"), Flags::FOLDING), Some(b!("\t")))))
        );
        assert_eq!(
            folding_or_terminator(b"\r\n\ra"),
            Ok((
                b!("a"),
                (
                    (b!("\r\n"), Flags::FOLDING | Flags::FOLDING_SPECIAL_CASE),
                    Some(b!("\r"))
                )
            ))
        );
        assert_eq!(
            folding_or_terminator(b"\r\n a"),
            Ok((b!("a"), ((b!("\r\n"), Flags::FOLDING), Some(b!(" ")))))
        );
        assert_eq!(
            folding_or_terminator(b"\r\na"),
            Ok((b!("a"), ((b!("\r\n"), Flags::empty()), None)))
        );
        assert_eq!(
            folding_or_terminator(b"\n\na"),
            Ok((b!("\na"), ((b!("\n"), Flags::empty()), None)))
        );
        assert_eq!(
            folding_or_terminator(b"\r\n\r\na"),
            Ok((b!("\r\na"), ((b!("\r\n"), Flags::empty()), None)))
        );
        assert_eq!(
            folding_or_terminator(b"\n\r\r\na"),
            Ok((b!("a"), ((b!("\n\r\r\n"), Flags::DEFORMED_EOL), None)))
        );
        assert_eq!(
            folding_or_terminator(b"\0a"),
            Ok((b!("a"), ((b!("\0"), Flags::NULL_TERMINATED), None)))
        );
    }

    #[test]
    fn ValueBytes() {
        // Expect fail because we need to see EOL
        assert!(value_bytes(b" ").is_err());
        assert!(value_bytes(b"value").is_err());
        assert!(value_bytes(b"\tvalue").is_err());
        assert!(value_bytes(b" value").is_err());
        assert!(value_bytes(b"value\rname2").is_err());
        // Expect fail because we need to see past EOL to check for folding
        assert!(value_bytes(b"value\r\n").is_err());
        assert_eq!(
            value_bytes(b"www.google.com\rName: Value\r\n\r\n"),
            Ok((
                b!("\r\n"),
                (
                    b!("www.google.com\rName: Value"),
                    ((b!("\r\n"), Flags::empty()), None)
                )
            ))
        );
        assert_eq!(
            value_bytes(b"www.google.com\rName: Value\n\r\n"),
            Ok((
                b!("\r\n"),
                (
                    b!("www.google.com\rName: Value"),
                    ((b!("\n"), Flags::empty()), None)
                )
            ))
        );
        assert_eq!(
            value_bytes(b"www.google.com\rName: Value\r\n\n"),
            Ok((
                b!("\n"),
                (
                    b!("www.google.com\rName: Value"),
                    ((b!("\r\n"), Flags::empty()), None)
                )
            ))
        );
        assert_eq!(
            value_bytes(b"\r\nnext"),
            Ok((b!("next"), (b!(""), ((b!("\r\n"), Flags::empty()), None))))
        );
        assert_eq!(
            value_bytes(b"value\r\nname2"),
            Ok((
                b!("name2"),
                (b!("value"), ((b!("\r\n"), Flags::empty()), None))
            ))
        );
        assert_eq!(
            value_bytes(b"value\n more"),
            Ok((
                b!("more"),
                (b!("value"), ((b!("\n"), Flags::FOLDING), Some(b!(" "))))
            ))
        );
        assert_eq!(
            value_bytes(b"value\r\n\t more"),
            Ok((
                b!("more"),
                (b!("value"), ((b!("\r\n"), Flags::FOLDING), Some(b!("\t "))))
            ))
        );
        assert_eq!(
            value_bytes(b"value\n\rmore"),
            Ok((
                b!("more"),
                (
                    b!("value"),
                    ((b!("\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r")))
                )
            ))
        );
        assert_eq!(
            value_bytes(b"value\r\n\rmore"),
            Ok((
                b!("more"),
                (
                    b!("value"),
                    ((b!("\r\n"), Flags::FOLDING_SPECIAL_CASE), Some(b!("\r")))
                )
            ))
        );
    }

    #[test]
    fn Value() {
        assert!(value(b"value\rnext:").is_err());
        assert!(value(b"value\r\n more\r\n").is_err());
        assert!(value(b"value\r\n ").is_err());
        assert!(value(b"value\r\n more").is_err());
        assert!(value(b"value\r\n more\n").is_err());
        assert!(value(b"value\n more\r\n").is_err());
        assert_eq!(
            value(b"\r\n value    \r\nnext:"),
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: Flags::FOLDING
                }
            ))
        );
        assert_eq!(
            value(b"\r\n value\r\nnext:"),
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: Flags::FOLDING
                }
            ))
        );
        assert_eq!(
            value(b"value\r\nnext:"),
            Ok((
                b!("next:"),
                Value {
                    value: b"value".to_vec(),
                    flags: Flags::empty()
                }
            ))
        );
        assert_eq!(
            value(b"\r\nnext:"),
            Ok((
                b!("next:"),
                Value {
                    value: b"".to_vec(),
                    flags: Flags::VALUE_EMPTY
                }
            ))
        );
        assert_eq!(
            value(b"value\r\n more\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more".to_vec(),
                    flags: Flags::FOLDING
                }
            ))
        );
        assert_eq!(
            value(b"value\r\n more\r\n\tand more\r\nnext:"),
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING
                }
            ))
        );
        assert_eq!(
            value(b"value\n more\n\r\r\n\tand more\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING | Flags::DEFORMED_EOL
                }
            ))
        );
        assert_eq!(
            value(b"value\n more\n\r\r\n\rand more\r\n\r\n"),
            Ok((
                b!("\r\n"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING_SPECIAL_CASE | Flags::DEFORMED_EOL
                }
            ))
        );
        assert_eq!(
            value(b"value\n\t\tmore\r\n  and\r\n more\r\nnext:"),
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING
                }
            ))
        );

        assert_eq!(
            value(b"value\n\r\t\tmore\r\n  and\r\n more\r\nnext:"),
            Ok((
                b!("next:"),
                Value {
                    value: b"value more and more".to_vec(),
                    flags: Flags::FOLDING_SPECIAL_CASE
                }
            ))
        );
    }
}
