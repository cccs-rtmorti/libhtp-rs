use crate::{bstr, htp_table, htp_transaction, htp_util};
use nom::{
    bytes::complete::{take, take_till},
    character::complete::char,
    combinator::{map, opt, peek},
    sequence::tuple,
    IResult,
};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

/// This is the main URLENCODED parser structure. It is used to store
/// parser configuration, temporary parsing data, as well as the parameters.
#[derive(Clone)]
pub struct htp_urlenp_t {
    /// The transaction this parser belongs to.
    pub tx: *mut htp_transaction::htp_tx_t,
    /// The character used to separate parameters. Defaults to & and should
    /// not be changed without good reason.
    pub argument_separator: u8,
    /// Whether to perform URL-decoding on parameters.
    pub decode_url_encoding: bool,
    /// This table contains the list of parameters, indexed by name.
    pub params: htp_table::htp_table_t<bstr::bstr_t>,
    // Private fields; these are used during the parsing process only
    complete: bool,
    saw_data: bool,
    field: bstr::bstr_t,
}

/// Creates a new URLENCODED parser.
///
/// Returns New parser, or NULL on memory allocation failure.
pub unsafe fn htp_urlenp_create(tx: *mut htp_transaction::htp_tx_t) -> *mut htp_urlenp_t {
    let urlenp: *mut htp_urlenp_t =
        calloc(1, ::std::mem::size_of::<htp_urlenp_t>()) as *mut htp_urlenp_t;
    if urlenp.is_null() {
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp).tx = tx;
    (*urlenp).params = htp_table::htp_table_t::with_capacity(32);
    (*urlenp).field = bstr::bstr_t::with_capacity(64);
    (*urlenp).argument_separator = '&' as u8;
    (*urlenp).decode_url_encoding = true;
    urlenp
}

/// Destroys an existing URLENCODED parser.
pub unsafe fn htp_urlenp_destroy(urlenp: *mut htp_urlenp_t) {
    if urlenp.is_null() {
        return;
    }
    (*urlenp).field.clear();
    (*urlenp).params.elements.clear();
    free(urlenp as *mut core::ffi::c_void);
}

/// Finalizes parsing, forcing the parser to convert any outstanding
/// data into parameters. This method should be invoked at the end
/// of a parsing operation that used htp_urlenp_parse_partial().
pub fn htp_urlenp_finalize(urlenp: &mut htp_urlenp_t) {
    urlenp.complete = true;
    htp_urlenp_parse_partial(urlenp, b"")
}

/// Parses the provided data chunk under the assumption
/// that it contains all the data that will be parsed. When this
/// method is used for parsing the finalization method should not
/// be invoked.
pub fn htp_urlenp_parse_complete(urlenp: &mut htp_urlenp_t, data: &[u8]) {
    htp_urlenp_parse_partial(urlenp, data);
    htp_urlenp_finalize(urlenp)
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
/// at any one time. The method htp_urlenp_finalize() must be invoked at the end to finalize parsing.
pub fn htp_urlenp_parse_partial(urlenp: &mut htp_urlenp_t, data: &[u8]) {
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
                    let _result =
                        htp_util::htp_tx_urldecode_params_inplace(&mut *urlenp.tx, &mut name);
                    let _result =
                        htp_util::htp_tx_urldecode_params_inplace(&mut *urlenp.tx, &mut value);
                }
            }
            urlenp.params.add(name, value);
        }
    });
    urlenp.field.clear();
    urlenp.field.add(remaining);
}
