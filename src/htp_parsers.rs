use crate::error::Result;
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
/// Returns Status code as a u16 on success or None on failure
pub fn htp_parse_status(status: &[u8]) -> Option<u16> {
    if let Ok((trailing_data, (leading_data, status_code))) = htp_util::ascii_digits()(status) {
        if trailing_data.len() > 0 || leading_data.len() > 0 {
            //There are invalid characters in the status code
            return None;
        }
        if let Ok(status_code) = std::str::from_utf8(status_code) {
            if let Ok(status_code) = u16::from_str_radix(status_code, 10) {
                if status_code >= 100 && status_code <= 999 {
                    return Some(status_code);
                }
            }
        }
    }
    None
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
) -> Result<()> {
    let in_tx = (*connp).in_tx_mut().ok_or(Status::ERROR)?;
    let data = &(*auth_header).value;

    if data.len() <= 5 {
        return Err(Status::DECLINED);
    };

    // Skip 'Basic<lws>'
    let value_start = if let Some(pos) = data[5..].iter().position(|&c| !c.is_ascii_whitespace()) {
        pos + 5
    } else {
        return Err(Status::DECLINED);
    };

    // Decode base64-encoded data
    let decoded = if let Ok(decoded) = base64::decode(&data[value_start..]) {
        decoded
    } else {
        return Err(Status::DECLINED);
    };

    // Extract username and password
    let i = if let Some(i) = decoded.iter().position(|&c| c == ':' as u8) {
        i
    } else {
        return Err(Status::DECLINED);
    };

    let (username, password) = decoded.split_at(i);
    in_tx.request_auth_username = bstr::bstr_dup_str(username);
    in_tx.request_auth_password = bstr::bstr_dup_str(&password[1..]);

    Ok(())
}

/// Parses Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization(
    connp: *mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    let in_tx = (*connp).in_tx_mut().ok_or(Status::ERROR)?;
    let auth_header =
        if let Some((_, auth_header)) = in_tx.request_headers.get_nocase_nozero("authorization") {
            auth_header
        } else {
            in_tx.request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_NONE;
            return Ok(());
        };
    // TODO Need a flag to raise when failing to parse authentication headers.
    if auth_header.value.starts_with_nocase("basic") {
        // Basic authentication
        in_tx.request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_BASIC;
        return htp_parse_authorization_basic(connp, auth_header);
    } else if auth_header.value.starts_with_nocase("digest") {
        // Digest authentication
        in_tx.request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_DIGEST;
        if let Ok((_, auth_username)) =
            htp_parse_authorization_digest((*(*auth_header).value).as_slice())
        {
            if in_tx.request_auth_username.is_null() {
                in_tx.request_auth_username = bstr::bstr_dup_str(auth_username);
                if in_tx.request_auth_username.is_null() {
                    return Err(Status::ERROR);
                }
            } else {
                (*in_tx.request_auth_username).clear();
                (*in_tx.request_auth_username).add(auth_username);
                return Ok(());
            }
        }
        return Err(Status::DECLINED);
    } else {
        // Unrecognized authentication method
        in_tx.request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_UNRECOGNIZED
    }
    Ok(())
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

#[test]
fn Status() {
    let status = bstr::bstr_t::from("   200    ");
    assert_eq!(Some(200u16), htp_parse_status(&status));

    let status = bstr::bstr_t::from("  \t 404    ");
    assert_eq!(Some(404u16), htp_parse_status(&status));

    let status = bstr::bstr_t::from("123");
    assert_eq!(Some(123u16), htp_parse_status(&status));

    let status = bstr::bstr_t::from("99");
    assert!(htp_parse_status(&status).is_none());

    let status = bstr::bstr_t::from("1000");
    assert!(htp_parse_status(&status).is_none());

    let status = bstr::bstr_t::from("200 OK");
    assert!(htp_parse_status(&status).is_none());

    let status = bstr::bstr_t::from("NOT 200");
    assert!(htp_parse_status(&status).is_none());
}
