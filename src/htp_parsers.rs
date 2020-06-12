use crate::bstr::{bstr_len, bstr_ptr};
use crate::htp_transaction::Protocol;
use crate::{bstr, htp_base64, htp_connection_parser, htp_transaction, htp_util, Status};

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
pub unsafe extern "C" fn htp_parse_authorization_digest(
    connp: *mut htp_connection_parser::htp_connp_t,
    auth_header: *const htp_transaction::htp_header_t,
) -> Status {
    // Extract the username
    let i: i32 = bstr::bstr_index_of((*auth_header).value, "username=");
    if i == -1 {
        return Status::DECLINED;
    }
    let data: *mut u8 = bstr_ptr((*auth_header).value);
    let len: usize = bstr_len((*auth_header).value);
    let mut pos: usize = (i + 9) as usize;
    // Ignore whitespace
    while pos < len && (*data.offset(pos as isize)).is_ascii_whitespace() {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return Status::DECLINED;
    }
    if *data.offset(pos as isize) != '\"' as u8 {
        return Status::DECLINED;
    }
    htp_util::htp_extract_quoted_string_as_bstr(
        data.offset(pos as isize),
        len.wrapping_sub(pos),
        &mut (*(*connp).in_tx).request_auth_username,
        0 as *mut usize,
    )
}

/// Parses Basic Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization_basic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    auth_header: *const htp_transaction::htp_header_t,
) -> Status {
    let data: *mut u8 = bstr_ptr((*auth_header).value);
    let len: usize = bstr_len((*auth_header).value);
    let mut pos: usize = 5;
    // Ignore whitespace
    while pos < len && (*data.offset(pos as isize)).is_ascii_whitespace() {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return Status::DECLINED;
    }
    // Decode base64-encoded data
    let decoded: *mut bstr::bstr_t = htp_base64::htp_base64_decode_mem(
        data.offset(pos as isize) as *const core::ffi::c_void,
        len.wrapping_sub(pos),
    );
    if decoded.is_null() {
        return Status::ERROR;
    }
    // Now extract the username and password
    let i: i32 = bstr::bstr_index_of(decoded, ":");
    if i == -1 {
        bstr::bstr_free(decoded);
        return Status::DECLINED;
    }
    (*(*connp).in_tx).request_auth_username = bstr::bstr_dup_ex(decoded, 0, i as usize);
    if (*(*connp).in_tx).request_auth_username.is_null() {
        bstr::bstr_free(decoded);
        return Status::ERROR;
    }
    (*(*connp).in_tx).request_auth_password = bstr::bstr_dup_ex(
        decoded,
        (i + 1) as usize,
        bstr_len(decoded).wrapping_sub(i as usize).wrapping_sub(1),
    );
    if (*(*connp).in_tx).request_auth_password.is_null() {
        bstr::bstr_free(decoded);
        bstr::bstr_free((*(*connp).in_tx).request_auth_username);
        return Status::ERROR;
    }
    bstr::bstr_free(decoded);
    Status::OK
}

/// Parses Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization(
    mut connp: *mut htp_connection_parser::htp_connp_t,
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
        return htp_parse_authorization_digest(connp, auth_header);
    } else {
        // Unrecognized authentication method
        (*(*connp).in_tx).request_auth_type =
            htp_transaction::htp_auth_type_t::HTP_AUTH_UNRECOGNIZED
    }
    Status::OK
}
