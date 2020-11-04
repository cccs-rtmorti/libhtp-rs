use crate::error::Result;
use crate::transaction::HtpProtocol;
use crate::{bstr, connection_parser, table, transaction, util, HtpStatus};
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
        util::take_ascii_whitespace(),
        tag_no_case("HTTP"),
        util::take_ascii_whitespace(),
        tag("/"),
        take_while(|c: u8| c.is_ascii_whitespace() || c == '0' as u8),
        alt((tag(".9"), tag("1.0"), tag("1.1"))),
        util::take_ascii_whitespace(),
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
/// Returns HtpProtocol version or invalid.
pub fn parse_protocol<'a>(
    input: &'a [u8],
    connp: &connection_parser::ConnectionParser,
) -> HtpProtocol {
    if let Ok((remaining, (version, contains_trailing))) = protocol_version(input) {
        if remaining.len() > 0 {
            return HtpProtocol::INVALID;
        }
        if contains_trailing {
            htp_warn!(
                    connp,
                    HtpLogCode::PROTOCOL_CONTAINS_EXTRA_DATA,
                    "HtpProtocol version contains leading and/or trailing whitespace and/or leading zeros"
                )
        }
        match version {
            b".9" => HtpProtocol::V0_9,
            b"1.0" => HtpProtocol::V1_0,
            b"1.1" => HtpProtocol::V1_1,
            _ => HtpProtocol::INVALID,
        }
    } else {
        HtpProtocol::INVALID
    }
}

/// Determines the numerical value of a response status given as a string.
///
/// Returns HtpStatus code as a u16 on success or None on failure
pub fn parse_status(status: &[u8]) -> Option<u16> {
    if let Ok((trailing_data, (leading_data, status_code))) = util::ascii_digits()(status) {
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
fn parse_authorization_digest<'a>(auth_header_value: &'a [u8]) -> IResult<&'a [u8], Vec<u8>> {
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
pub fn parse_authorization_basic(
    in_tx: &mut transaction::Transaction,
    auth_header: &transaction::Header,
) -> Result<()> {
    let data = &auth_header.value;

    if data.len() <= 5 {
        return Err(HtpStatus::DECLINED);
    };

    // Skip 'Basic<lws>'
    let value_start = if let Some(pos) = data[5..].iter().position(|&c| !c.is_ascii_whitespace()) {
        pos + 5
    } else {
        return Err(HtpStatus::DECLINED);
    };

    // Decode base64-encoded data
    let decoded = if let Ok(decoded) = base64::decode(&data[value_start..]) {
        decoded
    } else {
        return Err(HtpStatus::DECLINED);
    };

    // Extract username and password
    let i = if let Some(i) = decoded.iter().position(|&c| c == ':' as u8) {
        i
    } else {
        return Err(HtpStatus::DECLINED);
    };

    let (username, password) = decoded.split_at(i);
    in_tx.request_auth_username = Some(bstr::Bstr::from(username));
    in_tx.request_auth_password = Some(bstr::Bstr::from(&password[1..]));

    Ok(())
}

/// Parses Authorization request header.
pub fn parse_authorization(in_tx: &mut transaction::Transaction) -> Result<()> {
    let auth_header =
        if let Some((_, auth_header)) = in_tx.request_headers.get_nocase_nozero("authorization") {
            auth_header.clone()
        } else {
            in_tx.request_auth_type = transaction::HtpAuthType::NONE;
            return Ok(());
        };
    // TODO Need a flag to raise when failing to parse authentication headers.
    if auth_header.value.starts_with_nocase("basic") {
        // Basic authentication
        in_tx.request_auth_type = transaction::HtpAuthType::BASIC;
        return parse_authorization_basic(in_tx, &auth_header);
    } else if auth_header.value.starts_with_nocase("digest") {
        // Digest authentication
        in_tx.request_auth_type = transaction::HtpAuthType::DIGEST;
        if let Ok((_, auth_username)) = parse_authorization_digest(auth_header.value.as_slice()) {
            if let Some(username) = &mut in_tx.request_auth_username {
                username.clear();
                username.add(auth_username);
                return Ok(());
            } else {
                in_tx.request_auth_username = Some(bstr::Bstr::from(auth_username));
            }
        }
        return Err(HtpStatus::DECLINED);
    } else {
        // Unrecognized authentication method
        in_tx.request_auth_type = transaction::HtpAuthType::UNRECOGNIZED
    }
    Ok(())
}

/// Parses a single v0 request cookie.
///
/// Returns the (name, value).
pub fn parse_single_cookie_v0(data: &[u8]) -> (&[u8], &[u8]) {
    let parts: Vec<&[u8]> = data.splitn(2, |&x| x == '=' as u8).collect();
    match parts.len() {
        1 => (data, b""),
        2 => (parts[0], parts[1]),
        _ => (b"", b""),
    }
}

/// Parses the Cookie request header in v0 format and places the results into tx->request_cookies.
///
/// Returns OK on success, ERROR on error
pub fn parse_cookies_v0(in_tx: &mut transaction::Transaction) -> Result<()> {
    if let Some((_, cookie_header)) = in_tx.request_headers.get_nocase_nozero_mut("cookie") {
        let data: &[u8] = cookie_header.value.as_ref();
        // Create a new table to store cookies.
        in_tx.request_cookies = table::Table::with_capacity(4);
        for cookie in data.split(|b| *b == ';' as u8) {
            if let Ok((cookie, _)) = util::take_ascii_whitespace()(cookie) {
                if cookie.is_empty() {
                    continue;
                }
                let (name, value) = parse_single_cookie_v0(cookie);
                if !name.is_empty() {
                    in_tx
                        .request_cookies
                        .add(bstr::Bstr::from(name), bstr::Bstr::from(value));
                }
            }
        }
    }
    Ok(())
}

#[test]
fn ParseSingleCookieV0() {
    assert_eq!(
        (b"yummy_cookie".as_ref(), b"choco".as_ref()),
        parse_single_cookie_v0(b"yummy_cookie=choco")
    );
    assert_eq!(
        (b"".as_ref(), b"choco".as_ref()),
        parse_single_cookie_v0(b"=choco")
    );
    assert_eq!(
        (b"yummy_cookie".as_ref(), b"".as_ref()),
        parse_single_cookie_v0(b"yummy_cookie=")
    );
    assert_eq!((b"".as_ref(), b"".as_ref()), parse_single_cookie_v0(b"="));
    assert_eq!((b"".as_ref(), b"".as_ref()), parse_single_cookie_v0(b""));
}

#[test]
fn AuthDigest() {
    assert_eq!(
        b"ivan\"r\"".to_vec(),
        parse_authorization_digest(b"   username=   \"ivan\\\"r\\\"\"")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivan\"r\"".to_vec(),
        parse_authorization_digest(b"username=\"ivan\\\"r\\\"\"")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivan\"r\"".to_vec(),
        parse_authorization_digest(b"username=\"ivan\\\"r\\\"\"   ")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivanr".to_vec(),
        parse_authorization_digest(b"username=\"ivanr\"   ")
            .unwrap()
            .1
    );
    assert_eq!(
        b"ivanr".to_vec(),
        parse_authorization_digest(b"username=   \"ivanr\"   ")
            .unwrap()
            .1
    );
    assert!(parse_authorization_digest(b"username=ivanr\"   ").is_err()); //Missing opening quote
    assert!(parse_authorization_digest(b"username=\"ivanr   ").is_err()); //Missing closing quote
}

#[test]
fn HtpStatus() {
    let status = bstr::Bstr::from("   200    ");
    assert_eq!(Some(200u16), parse_status(&status));

    let status = bstr::Bstr::from("  \t 404    ");
    assert_eq!(Some(404u16), parse_status(&status));

    let status = bstr::Bstr::from("123");
    assert_eq!(Some(123u16), parse_status(&status));

    let status = bstr::Bstr::from("99");
    assert!(parse_status(&status).is_none());

    let status = bstr::Bstr::from("1000");
    assert!(parse_status(&status).is_none());

    let status = bstr::Bstr::from("200 OK");
    assert!(parse_status(&status).is_none());

    let status = bstr::Bstr::from("NOT 200");
    assert!(parse_status(&status).is_none());
}
