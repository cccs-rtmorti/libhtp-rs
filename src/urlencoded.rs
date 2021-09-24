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

    /// Finalizes parsing, forcing the parser to convert any outstanding
    /// data into parameters. This method should be invoked at the end
    /// of a parsing operation that used urlenp_parse_partial().
    pub fn finalize(&mut self) {
        self.complete = true;
        self.parse_partial(b"")
    }

    /// Parses the provided data chunk under the assumption
    /// that it contains all the data that will be parsed. When this
    /// method is used for parsing the finalization method should not
    /// be invoked.
    pub fn parse_complete(&mut self, data: &[u8]) {
        self.parse_partial(data);
        self.finalize()
    }

    /// Parses the provided data chunk, searching for argument seperators and '=' to locate names and values,
    /// keeping state to allow streaming parsing, i.e., the parsing where only partial information is available
    /// at any one time. The method urlenp_finalize() must be invoked at the end to finalize parsing.
    pub fn parse_partial(&mut self, data: &[u8]) {
        self.field.add(data);
        let input = self.field.clone();
        let mut input = input.as_slice();
        if input.is_empty() {
            if self.complete && self.params.size() == 0 && self.saw_data {
                self.params.add(Bstr::new(), Bstr::new());
            }
            return;
        }
        let mut remaining: &[u8] = b"";
        let sep = self.argument_separator;
        self.saw_data = true;
        if !self.complete {
            let data: Vec<&[u8]> = input.rsplitn(2, |c| *c == sep).collect();
            if data.len() == 2 {
                input = data[1];
                remaining = data[0];
            } else {
                return;
            }
        }
        input.split(|c| *c == sep).for_each(|segment| {
            if let Ok((value, name)) = name_value(segment) {
                let mut name = Bstr::from(name);
                let mut value = Bstr::from(value);
                if self.decode_url_encoding {
                    if let Ok((_, (consumed, flags, expected_status))) =
                        urldecode_ex(name.as_slice(), &self.cfg)
                    {
                        self.flags.set(flags);
                        self.response_status_expected_number = expected_status;
                        name.clear();
                        name.add(consumed);
                    }
                    if let Ok((_, (consumed, flags, expected_status))) =
                        urldecode_ex(value.as_slice(), &self.cfg)
                    {
                        self.flags.set(flags);
                        self.response_status_expected_number = expected_status;
                        value.clear();
                        value.add(consumed);
                    }
                }
                self.params.add(name, value);
            }
        });
        self.field.clear();
        self.field.add(remaining);
    }
}

/// Extracts names and values from the url parameters
///
/// Returns a name value pair, separated by an '='
fn name_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((peek(take(1usize)), take_till(|c| c == b'='), opt(char('=')))),
        |(_, name, _)| name,
    )(input)
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

// Tests
#[test]
fn Empty() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"");

    assert_eq!(0, urlenp.params.size());
}

#[test]
fn EmptyKey1() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey2() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"=&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey3() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"=1&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq_slice("1"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey4() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"&=");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKey5() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"&&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn EmptyKeyAndValue() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"=");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn OnePairEmptyValue() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"p=");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn OnePairEmptyKey() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"=p");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq_slice("p"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn OnePair() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"p=1");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice("1"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn TwoPairs() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"p=1&q=2");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice("1"));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq_slice("2"));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn KeyNoValue1() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"p");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn KeyNoValue2() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"p&");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn KeyNoValue3() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"p&q");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq_slice(""));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn KeyNoValue4() {
    let mut urlenp = Parser::default();
    urlenp.parse_complete(b"p&q=2");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq_slice("2"));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn Partial1() {
    let mut urlenp = Parser::default();
    urlenp.parse_partial(b"p");
    urlenp.finalize();

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial2() {
    let mut urlenp = Parser::default();
    urlenp.parse_partial(b"p");
    urlenp.parse_partial(b"x");
    urlenp.finalize();

    assert!(urlenp.params.get_nocase("px").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial3() {
    let mut urlenp = Parser::default();
    urlenp.parse_partial(b"p");
    urlenp.parse_partial(b"x&");
    urlenp.finalize();

    assert!(urlenp.params.get_nocase("px").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial4() {
    let mut urlenp = Parser::default();
    urlenp.parse_partial(b"p");
    urlenp.parse_partial(b"=");
    urlenp.finalize();

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial5() {
    let mut urlenp = Parser::default();
    urlenp.parse_partial(b"p");
    urlenp.parse_partial(b"");
    urlenp.parse_partial(b"");
    urlenp.parse_partial(b"");
    urlenp.finalize();

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq_slice(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn Partial6() {
    let mut urlenp = Parser::default();
    urlenp.parse_partial(b"px");
    urlenp.parse_partial(b"n");
    urlenp.parse_partial(b"");
    urlenp.parse_partial(b"=");
    urlenp.parse_partial(b"1");
    urlenp.parse_partial(b"2");
    urlenp.parse_partial(b"&");
    urlenp.parse_partial(b"qz");
    urlenp.parse_partial(b"n");
    urlenp.parse_partial(b"");
    urlenp.parse_partial(b"=");
    urlenp.parse_partial(b"2");
    urlenp.parse_partial(b"3");
    urlenp.parse_partial(b"&");
    urlenp.finalize();

    assert!(urlenp.params.get_nocase("pxn").unwrap().1.eq_slice("12"));
    assert!(urlenp.params.get_nocase("qzn").unwrap().1.eq_slice("23"));
    assert_eq!(2, urlenp.params.size());
}
