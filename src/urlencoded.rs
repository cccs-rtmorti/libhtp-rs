use crate::{
    bstr::Bstr,
    config::{DecoderConfig, HtpUnwanted},
    table::Table,
    util::{urldecode_ex, FlagOperations},
};
use nom::{
    bytes::complete::{take, take_till},
    character::complete::char,
    combinator::{map, opt, peek},
    sequence::tuple,
    IResult,
};

/// This is the main URLENCODED parser structure. It is used to store
/// parser configuration, temporary parsing data, as well as the parameters.
#[derive(Clone)]
pub struct Parser {
    /// The configuration structure associated with this parser
    pub cfg: DecoderConfig,
    /// The character used to separate parameters. Defaults to & and should
    /// not be changed without good reason.
    pub argument_separator: u8,
    /// Whether to perform URL-decoding on parameters. Defaults to true.
    pub decode_url_encoding: bool,
    /// This table contains the list of parameters, indexed by name.
    pub params: Table<Bstr>,
    /// Contains parsing flags
    pub flags: u64,
    /// This field is set if the parser thinks that the
    /// backend server will reject a request with a particular status code.
    pub response_status_expected_number: HtpUnwanted,
    // Private fields; these are used during the parsing process only
    complete: bool,
    saw_data: bool,
    field: Bstr,
}

impl Parser {
    /// Construct new Parser with provided decoder configuration
    pub fn new(cfg: DecoderConfig) -> Self {
        Self {
            cfg,
            argument_separator: b'&',
            decode_url_encoding: true,
            params: Table::with_capacity(32),
            flags: 0,
            response_status_expected_number: HtpUnwanted::IGNORE,
            complete: false,
            saw_data: false,
            field: Bstr::with_capacity(64),
        }
    }
}

impl Default for Parser {
    /// Construt new Parser with default values
    fn default() -> Self {
        Self {
            cfg: DecoderConfig::default(),
            argument_separator: b'&',
            decode_url_encoding: true,
            params: Table::with_capacity(32),
            flags: 0,
            response_status_expected_number: HtpUnwanted::IGNORE,
            complete: false,
            saw_data: false,
            field: Bstr::with_capacity(64),
        }
    }
}
/// Finalizes parsing, forcing the parser to convert any outstanding
/// data into parameters. This method should be invoked at the end
/// of a parsing operation that used urlenp_parse_partial().
pub fn urlenp_finalize(urlenp: &mut Parser) {
    urlenp.complete = true;
    urlenp_parse_partial(urlenp, b"")
}

/// Parses the provided data chunk under the assumption
/// that it contains all the data that will be parsed. When this
/// method is used for parsing the finalization method should not
/// be invoked.
pub fn urlenp_parse_complete(urlenp: &mut Parser, data: &[u8]) {
    urlenp_parse_partial(urlenp, data);
    urlenp_finalize(urlenp)
}

/// Extracts names and values from the url parameters
///
/// Returns a name value pair, separated by an '='
fn urlen_name_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((peek(take(1usize)), take_till(|c| c == b'='), opt(char('=')))),
        |(_, name, _)| name,
    )(input)
}

/// Parses the provided data chunk, searching for argument seperators and '=' to locate names and values,
/// keeping state to allow streaming parsing, i.e., the parsing where only partial information is available
/// at any one time. The method urlenp_finalize() must be invoked at the end to finalize parsing.
pub fn urlenp_parse_partial(urlenp: &mut Parser, data: &[u8]) {
    urlenp.field.add(data);
    let input = urlenp.field.clone();
    let mut input = input.as_slice();
    if input.is_empty() {
        if urlenp.complete && urlenp.params.size() == 0 && urlenp.saw_data {
            urlenp.params.add(Bstr::new(), Bstr::new());
        }
        return;
    }
    let mut remaining: &[u8] = b"";
    let sep = urlenp.argument_separator;
    urlenp.saw_data = true;
    if !urlenp.complete {
        let data: Vec<&[u8]> = input.rsplitn(2, |c| *c == sep).collect();
        if data.len() == 2 {
            input = data[1];
            remaining = data[0];
        } else {
            return;
        }
    }
    input.split(|c| *c == sep).for_each(|segment| {
        if let Ok((value, name)) = urlen_name_value(segment) {
            let mut name = Bstr::from(name);
            let mut value = Bstr::from(value);
            if (*urlenp).decode_url_encoding {
                if let Ok((_, (consumed, flags, expected_status))) =
                    urldecode_ex(name.as_slice(), &urlenp.cfg)
                {
                    urlenp.flags.set(flags);
                    urlenp.response_status_expected_number = expected_status;
                    name.clear();
                    name.add(consumed);
                }
                if let Ok((_, (consumed, flags, expected_status))) =
                    urldecode_ex(value.as_slice(), &urlenp.cfg)
                {
                    urlenp.flags.set(flags);
                    urlenp.response_status_expected_number = expected_status;
                    value.clear();
                    value.add(consumed);
                }
            }
            urlenp.params.add(name, value);
        }
    });
    urlenp.field.clear();
    urlenp.field.add(remaining);
}

// Tests
#[test]
fn Empty() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"");

    assert_eq!(0, urlenp.params.size());
}

#[test]
fn EmptyKey1() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey2() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"=&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey3() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"=1&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq("1"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey4() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"&=");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey5() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"&&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKeyAndValue() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"=");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn OnePairEmptyValue() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"p=");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn OnePairEmptyKey() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"=p");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq("p"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn OnePair() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"p=1");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq("1"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn TwoPairs() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"p=1&q=2");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq("1"));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq("2"));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn KeyNoValue1() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"p");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn KeyNoValue2() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"p&");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn KeyNoValue3() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"p&q");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq(""));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn KeyNoValue4() {
    let mut urlenp = Parser::default();
    urlenp_parse_complete(&mut urlenp, b"p&q=2");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq("2"));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn Partial1() {
    let mut urlenp = Parser::default();
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial2() {
    let mut urlenp = Parser::default();
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"x");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("px").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial3() {
    let mut urlenp = Parser::default();
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"x&");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("px").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial4() {
    let mut urlenp = Parser::default();
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"=");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial5() {
    let mut urlenp = Parser::default();
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial6() {
    let mut urlenp = Parser::default();
    urlenp_parse_partial(&mut urlenp, b"px");
    urlenp_parse_partial(&mut urlenp, b"n");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"=");
    urlenp_parse_partial(&mut urlenp, b"1");
    urlenp_parse_partial(&mut urlenp, b"2");
    urlenp_parse_partial(&mut urlenp, b"&");
    urlenp_parse_partial(&mut urlenp, b"qz");
    urlenp_parse_partial(&mut urlenp, b"n");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"=");
    urlenp_parse_partial(&mut urlenp, b"2");
    urlenp_parse_partial(&mut urlenp, b"3");
    urlenp_parse_partial(&mut urlenp, b"&");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("pxn").unwrap().1.eq("12"));
    assert!(urlenp.params.get_nocase("qzn").unwrap().1.eq("23"));
    assert_eq!(2, urlenp.params.size());
}
