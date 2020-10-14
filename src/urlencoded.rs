use crate::{bstr, table, transaction, util};
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
pub struct htp_urlenp_t {
    /// The transaction this parser belongs to.
    pub tx: *mut transaction::htp_tx_t,
    /// The character used to separate parameters. Defaults to & and should
    /// not be changed without good reason.
    pub argument_separator: u8,
    /// Whether to perform URL-decoding on parameters. Defaults to true.
    pub decode_url_encoding: bool,
    /// This table contains the list of parameters, indexed by name.
    pub params: table::htp_table_t<bstr::bstr_t>,
    // Private fields; these are used during the parsing process only
    complete: bool,
    saw_data: bool,
    field: bstr::bstr_t,
}

impl htp_urlenp_t {
    pub fn new(tx: *mut transaction::htp_tx_t) -> Self {
        Self {
            tx: tx,
            argument_separator: '&' as u8,
            decode_url_encoding: true,
            params: table::htp_table_t::with_capacity(32),
            complete: false,
            saw_data: false,
            field: bstr::bstr_t::with_capacity(64),
        }
    }
}

/// Finalizes parsing, forcing the parser to convert any outstanding
/// data into parameters. This method should be invoked at the end
/// of a parsing operation that used urlenp_parse_partial().
pub fn urlenp_finalize(urlenp: &mut htp_urlenp_t) {
    urlenp.complete = true;
    urlenp_parse_partial(urlenp, b"")
}

/// Parses the provided data chunk under the assumption
/// that it contains all the data that will be parsed. When this
/// method is used for parsing the finalization method should not
/// be invoked.
pub fn urlenp_parse_complete(urlenp: &mut htp_urlenp_t, data: &[u8]) {
    urlenp_parse_partial(urlenp, data);
    urlenp_finalize(urlenp)
}

/// Extracts names and values from the url parameters
///
/// Returns a name value pair, separated by an '='
fn urlen_name_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map(
        tuple((
            peek(take(1usize)),
            take_till(|c: u8| c == '=' as u8),
            opt(char('=')),
        )),
        |(_, name, _)| name,
    )(input)
}
/// Parses the provided data chunk, searching for argument seperators and '=' to locate names and values,
/// keeping state to allow streaming parsing, i.e., the parsing where only partial information is available
/// at any one time. The method urlenp_finalize() must be invoked at the end to finalize parsing.
pub fn urlenp_parse_partial(urlenp: &mut htp_urlenp_t, data: &[u8]) {
    urlenp.field.add(data);
    let input = urlenp.field.clone();
    let mut input = input.as_slice();
    if input.is_empty() {
        if urlenp.complete && urlenp.params.size() == 0 && urlenp.saw_data {
            urlenp.params.add(bstr::bstr_t::new(), bstr::bstr_t::new());
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
            let mut name = bstr::bstr_t::from(name);
            let mut value = bstr::bstr_t::from(value);
            if (*urlenp).decode_url_encoding {
                unsafe {
                    //Don't currently care about this result
                    let _result = util::tx_urldecode_params_inplace(&mut *urlenp.tx, &mut name);
                    let _result = util::tx_urldecode_params_inplace(&mut *urlenp.tx, &mut value);
                }
            }
            urlenp.params.add(name, value);
        }
    });
    urlenp.field.clear();
    urlenp.field.add(remaining);
}
