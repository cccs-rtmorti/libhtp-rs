use crate::bstr::{bstr_len, bstr_ptr};
use crate::htp_transaction::Protocol;
use crate::{bstr, htp_connection_parser, htp_transaction, htp_util, Status};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until, take_while},
    sequence::tuple,
    IResult,
};

/// Extracts the version protocol from the input slice.
///
/// Returns (any unparsed trailing data, (version_number, flag indicating whether input contains trailing and/or leading whitespace and/or leading zeros))
pub fn protocol_version<'a>(input: &'a [u8]) -> IResult<&'a [u8], (&'a [u8], bool)> {
    let (remaining, (_, _, leading, _, trailing, version, _)) = tuple((
        htp_util::take_ascii_whitespace(),
        tag_no_case("HTTP"),
        htp_util::take_ascii_whitespace(),
        tag("/"),
        take_while(|c: u8| c.is_ascii_whitespace() || c == '0' as u8),
        alt((tag(".9"), tag("1.0"), tag("1.1"))),
        htp_util::take_ascii_whitespace(),
    ))(input)?;
    Ok((
        remaining,
        (version, leading.len() > 0 || trailing.len() > 0),
    ))
}

/// Determines protocol number from a textual representation (i.e., "HTTP/1.1"). This
/// function tries to be flexible, allowing whitespace before and after the forward slash,
/// as well as allowing leading zeros in the version number. If such leading/trailing
/// characters are discovered, however, a warning will be logged.
///
/// Returns Protocol version or invalid.
pub fn htp_parse_protocol<'a>(
    input: &'a [u8],
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Protocol {
    if let Ok((remaining, (version, contains_trailing))) = protocol_version(input) {
        if remaining.len() > 0 {
            return Protocol::INVALID;
        }
        if contains_trailing {
            unsafe {
                htp_warn!(
                    connp as *mut htp_connection_parser::htp_connp_t,
                    htp_log_code::PROTOCOL_CONTAINS_EXTRA_DATA,
                    "Protocol version contains leading and/or trailing whitespace and/or leading zeros"
                )
            };
        }
        match version {
            b".9" => Protocol::V0_9,
            b"1.0" => Protocol::V1_0,
            b"1.1" => Protocol::V1_1,
            _ => Protocol::INVALID,
        }
    } else {
        Protocol::INVALID
    }
}

/// Determines the numerical value of a response status given as a string.
///
/// Returns Status code on success, or HTP_STATUS_INVALID on error.
pub unsafe extern "C" fn htp_parse_status(status: *const bstr::bstr_t) -> i32 {
    let r: i64 =
        htp_util::htp_parse_positive_integer_whitespace(bstr_ptr(status), bstr_len(status), 10);
    if r >= 100 && r <= 999 {
        return r as i32;
    }
    -1
}

/// Parses Digest Authorization request header.
fn htp_parse_authorization_digest<'a>(auth_header_value: &'a [u8]) -> IResult<&'a [u8], Vec<u8>> {
    // Extract the username
    let (mut remaining_input, _) = tuple((
        take_until("username="),
        tag("username="),
        take_while(|c: u8| c.is_ascii_whitespace()), // allow leading whitespace
        tag("\""), // First character after LWS must be a double quote
    ))(auth_header_value)?;
    let mut result = Vec::new();
    // Unescape any escaped double quotes and find the closing quote
    loop {
        let (remaining, (auth_header, _)) = tuple((take_until("\""), tag("\"")))(remaining_input)?;
        remaining_input = remaining;
        result.extend_from_slice(auth_header);
        if result.last() == Some(&('\\' as u8)) {
            // Remove the escape and push back the double quote
            result.pop();
            result.push('\"' as u8);
        } else {
            // We found the closing double quote!
            break;
        }
    }
    Ok((remaining_input, result))
}

/// Parses Basic Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization_basic(
    connp: *mut htp_connection_parser::htp_connp_t,
    auth_header: *const htp_transaction::htp_header_t,
) -> Status {
    let data = &(*auth_header).value;

    if data.len() <= 5 {
        return Status::DECLINED;
    };

    // Skip 'Basic<lws>'
    let value_start = if let Some(pos) = data[5..].iter().position(|&c| !c.is_ascii_whitespace()) {
        pos + 5
    } else {
        return Status::DECLINED;
    };

    // Decode base64-encoded data
    let decoded = if let Ok(decoded) = base64::decode(&data[value_start..]) {
        decoded
    } else {
        return Status::DECLINED;
    };

    // Extract username and password
    let i = if let Some(i) = decoded.iter().position(|&c| c == ':' as u8) {
        i
    } else {
        return Status::DECLINED;
    };

    let (username, password) = decoded.split_at(i);
    (*(*connp).in_tx).request_auth_username = bstr::bstr_dup_str(username);
    (*(*connp).in_tx).request_auth_password = bstr::bstr_dup_str(&password[1..]);

    Status::OK
}

/// Parses Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization(
    connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    let auth_header = if let Some((_, auth_header)) = (*(*connp).in_tx)
        .request_headers
        .get_nocase_nozero("authorization")
    {
        auth_header
    } else {
        (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_NONE;
        return Status::OK;
    };
    // TODO Need a flag to raise when failing to parse authentication headers.
    if auth_header.value.starts_with_nocase("basic") {
        // Basic authentication
        (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_BASIC;
        return htp_parse_authorization_basic(connp, auth_header);
    } else if auth_header.value.starts_with_nocase("digest") {
        // Digest authentication
        (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_DIGEST;
        if let Ok((_, auth_username)) =
            htp_parse_authorization_digest((*(*auth_header).value).as_slice())
        {
            if (*(*connp).in_tx).request_auth_username.is_null() {
                (*(*connp).in_tx).request_auth_username = bstr::bstr_dup_str(auth_username);
                if (*(*connp).in_tx).request_auth_username.is_null() {
                    return Status::ERROR;
                }
            } else {
                (*(*(*connp).in_tx).request_auth_username).clear();
                (*(*(*connp).in_tx).request_auth_username).add(auth_username);
                return Status::OK;
            }
        }
        return Status::DECLINED;
    } else {
        // Unrecognized authentication method
        (*(*connp).in_tx).request_auth_type =
            htp_transaction::htp_auth_type_t::HTP_AUTH_UNRECOGNIZED
    }
    Status::OK
}

#[test]
fn AuthDigest() {
    assert_eq!(
        b"ivan\"r\"".to_vec(),
        htp_parse_authorization_digest(b"   username=   \"ivan\\\"r\\\"\"")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivan\"r\"".to_vec(),
        htp_parse_authorization_digest(b"username=\"ivan\\\"r\\\"\"")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivan\"r\"".to_vec(),
        htp_parse_authorization_digest(b"username=\"ivan\\\"r\\\"\"   ")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivanr".to_vec(),
        htp_parse_authorization_digest(b"username=\"ivanr\"   ")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivanr".to_vec(),
        htp_parse_authorization_digest(b"username=   \"ivanr\"   ")
            .unwrap()
            .1
    );
    assert!(htp_parse_authorization_digest(b"username=ivanr\"   ").is_err()); //Missing opening quote
    assert!(htp_parse_authorization_digest(b"username=\"ivanr   ").is_err()); //Missing closing quote
}
