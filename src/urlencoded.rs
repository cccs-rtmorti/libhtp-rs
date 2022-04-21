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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty("", &[])]
    #[case::empty_key_value("&", &[("", "")])]
    #[case::empty_key_value("=&", &[("", "")])]
    #[case::empty_key_value("&=", &[("", "")])]
    #[case::empty_key_value("&&", &[("", "")])]
    #[case::empty_key_value("=", &[("", "")])]
    #[case::empty_key("=1&", &[("", "1")])]
    #[case::empty_key("=p", &[("", "p")])]
    #[case::empty_value("p", &[("p", "")])]
    #[case::empty_value("p=", &[("p", "")])]
    #[case::empty_value("p&", &[("p", "")])]
    #[case::pair("p=1", &[("p", "1")])]
    #[case::two_pair("p=1&q=2", &[("p", "1"), ("q", "2")])]
    #[case::two_keys("p&q", &[("p", ""), ("q", "")])]
    #[case::two_keys_one_value("p&q=2", &[("p", ""), ("q", "2")])]
    fn test_parse_complete(#[case] input: &str, #[case] expected: &[(&str, &str)]) {
        let mut urlenp = Parser::default();
        urlenp.parse_complete(input.as_bytes());
        for (key, value) in expected {
            assert!(urlenp.params.get_nocase(key).unwrap().1.eq_slice(value));
        }
        assert_eq!(
            expected.len(),
            urlenp.params.size(),
            "Test case expected {} params. parse_complete resulted in {} params.",
            expected.len(),
            urlenp.params.size()
        );
    }

    #[rstest]
    #[case::empty_value(&["p"], &[("p", "")])]
    #[case::empty_value(&["p", "x"], &[("px", "")])]
    #[case::empty_value(&["p", "x&"], &[("px", "")])]
    #[case::empty_value(&["p", "="], &[("p", "")])]
    #[case::empty_value(&["p", "", "", ""], &[("p", "")])]
    #[case::two_pairs(
            &["px", "n", "", "=", "1", "2", "&", "qz", "n", "", "=", "2", "3", "&"],
            &[("pxn", "12"), ("qzn", "23")]
        )]
    fn test_parse_partial(#[case] input: &[&str], #[case] expected: &[(&str, &str)]) {
        let mut urlenp = Parser::default();
        for i in input {
            urlenp.parse_partial(i.as_bytes());
        }
        urlenp.finalize();
        for (key, value) in expected {
            assert!(urlenp.params.get_nocase(key).unwrap().1.eq_slice(value));
        }
        assert_eq!(
            expected.len(),
            urlenp.params.size(),
            "Test case expected {} params. parse_complete resulted in {} params.",
            expected.len(),
            urlenp.params.size()
        );
    }
}
