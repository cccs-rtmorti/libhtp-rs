use crate::bstr::{bstr_len, bstr_ptr};
use crate::htp_transaction::Protocol;
use crate::{bstr, htp_connection_parser, htp_transaction, htp_util, Status};

use nom::{
    bytes::complete::{tag, take_until, take_while},
    sequence::tuple,
    IResult,
};

/// Determines protocol number from a textual representation (i.e., "HTTP/1.1"). This
/// function will only understand a properly formatted protocol information. It does
/// not try to be flexible.
///
/// Returns Protocol version or PROTOCOL_UNKNOWN.
pub unsafe extern "C" fn htp_parse_protocol(protocol: *const bstr::bstr_t) -> Protocol {
    if protocol.is_null() {
        return Protocol::INVALID;
    }
    // TODO This function uses a very strict approach to parsing, whereas
    //      browsers will typically be more flexible, allowing whitespace
    //      before and after the forward slash, as well as allowing leading
    //      zeroes in the numbers. We should be able to parse such malformed
    //      content correctly (but emit a warning).
    if bstr_len(protocol) == 8 {
        let ptr: *mut u8 = bstr_ptr(protocol);
        if *ptr.offset(0) == 'H' as u8
            && *ptr.offset(1) == 'T' as u8
            && *ptr.offset(2) == 'T' as u8
            && *ptr.offset(3) == 'P' as u8
            && *ptr.offset(4) == '/' as u8
            && *ptr.offset(6) == '.' as u8
        {
            // Check the version numbers
            if *ptr.offset(5) == '0' as u8 {
                if *ptr.offset(7) == '9' as u8 {
                    return Protocol::V0_9;
                }
            } else if *ptr.offset(5) == '1' as u8 {
                if *ptr.offset(7) == '0' as u8 {
                    return Protocol::V1_0;
                } else if *ptr.offset(7) == '1' as u8 {
                    return Protocol::V1_1;
                }
            }
        }
    }
    Protocol::INVALID
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
    let data = &*(*auth_header).value;

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
    let auth_header_opt = (*(*(*connp).in_tx).request_headers).get_nocase_nozero("authorization");
    if auth_header_opt.is_none() {
        (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_NONE;
        return Status::OK;
    }
    let auth_header = auth_header_opt.unwrap().1;
    // TODO Need a flag to raise when failing to parse authentication headers.
    if (*(*auth_header).value).starts_with_nocase("basic") {
        // Basic authentication
        (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_BASIC;
        return htp_parse_authorization_basic(connp, auth_header);
    } else if (*(*auth_header).value).starts_with_nocase("digest") {
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
