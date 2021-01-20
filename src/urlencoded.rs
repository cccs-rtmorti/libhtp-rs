use crate::{
    bstr::Bstr, table::Table, transaction::Transaction, util::tx_urldecode_params_inplace,
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
    /// The transaction this parser belongs to.
    pub tx: *mut Transaction,
    /// The character used to separate parameters. Defaults to & and should
    /// not be changed without good reason.
    pub argument_separator: u8,
    /// Whether to perform URL-decoding on parameters. Defaults to true.
    pub decode_url_encoding: bool,
    /// This table contains the list of parameters, indexed by name.
    pub params: Table<Bstr>,
    // Private fields; these are used during the parsing process only
    complete: bool,
    saw_data: bool,
    field: Bstr,
}

impl Parser {
    /// Construct new Parser with default values.
    pub fn new(tx: *mut Transaction) -> Self {
        Self {
            tx,
            argument_separator: b'&',
            decode_url_encoding: true,
            params: Table::with_capacity(32),
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
                unsafe {
                    //Don't currently care about this result
                    let _result = tx_urldecode_params_inplace(&mut *urlenp.tx, &mut name);
                    let _result = tx_urldecode_params_inplace(&mut *urlenp.tx, &mut value);
                }
            }
            urlenp.params.add(name, value);
        }
    });
    urlenp.field.clear();
    urlenp.field.add(remaining);
}
