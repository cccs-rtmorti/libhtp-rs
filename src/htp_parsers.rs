use crate::htp_transaction::Protocol;
use crate::{
    bstr, htp_base64, htp_connection_parser, htp_table, htp_transaction, htp_util, Status,
};
use ::libc;

extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type htp_time_t = libc::timeval;

/// Determines protocol number from a textual representation (i.e., "HTTP/1.1"). This
/// function will only understand a properly formatted protocol information. It does
/// not try to be flexible.
///
/// Returns Protocol version or PROTOCOL_UNKNOWN.
pub unsafe extern "C" fn htp_parse_protocol(mut protocol: *mut bstr::bstr_t) -> Protocol {
    if protocol.is_null() {
        return Protocol::INVALID;
    }
    // TODO This function uses a very strict approach to parsing, whereas
    //      browsers will typically be more flexible, allowing whitespace
    //      before and after the forward slash, as well as allowing leading
    //      zeroes in the numbers. We should be able to parse such malformed
    //      content correctly (but emit a warning).
    if (*protocol).len == 8 as libc::c_int as libc::c_ulong {
        let mut ptr: *mut libc::c_uchar = if (*protocol).realptr.is_null() {
            (protocol as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
        } else {
            (*protocol).realptr
        };
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == 'H' as i32
            && *ptr.offset(1 as libc::c_int as isize) as libc::c_int == 'T' as i32
            && *ptr.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
            && *ptr.offset(3 as libc::c_int as isize) as libc::c_int == 'P' as i32
            && *ptr.offset(4 as libc::c_int as isize) as libc::c_int == '/' as i32
            && *ptr.offset(6 as libc::c_int as isize) as libc::c_int == '.' as i32
        {
            // Check the version numbers
            if *ptr.offset(5 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                if *ptr.offset(7 as libc::c_int as isize) as libc::c_int == '9' as i32 {
                    return Protocol::V0_9;
                }
            } else if *ptr.offset(5 as libc::c_int as isize) as libc::c_int == '1' as i32 {
                if *ptr.offset(7 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                    return Protocol::V1_0;
                } else {
                    if *ptr.offset(7 as libc::c_int as isize) as libc::c_int == '1' as i32 {
                        return Protocol::V1_1;
                    }
                }
            }
        }
    }
    return Protocol::INVALID;
}

/// Determines the numerical value of a response status given as a string.
///
/// Returns Status code on success, or HTP_STATUS_INVALID on error.
pub unsafe extern "C" fn htp_parse_status(mut status: *mut bstr::bstr) -> libc::c_int {
    let mut r: int64_t = htp_util::htp_parse_positive_integer_whitespace(
        if (*status).realptr.is_null() {
            (status as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
        } else {
            (*status).realptr
        },
        (*status).len,
        10 as libc::c_int,
    );
    if r >= 100 as libc::c_int as libc::c_long && r <= 999 as libc::c_int as libc::c_long {
        return r as libc::c_int;
    } else {
        return -(1 as libc::c_int);
    };
}

/// Parses Digest Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization_digest(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut auth_header: *mut htp_transaction::htp_header_t,
) -> Status {
    // Extract the username
    let mut i: libc::c_int = bstr::bstr_index_of_c(
        (*auth_header).value,
        b"username=\x00" as *const u8 as *const libc::c_char,
    );
    if i == -(1 as libc::c_int) {
        return Status::DECLINED;
    }
    let mut data: *mut libc::c_uchar = if (*(*auth_header).value).realptr.is_null() {
        ((*auth_header).value as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*(*auth_header).value).realptr
    };
    let mut len: size_t = (*(*auth_header).value).len;
    let mut pos: size_t = (i + 9 as libc::c_int) as size_t;
    // Ignore whitespace
    while pos < len
        && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return Status::DECLINED;
    }
    if *data.offset(pos as isize) as libc::c_int != '\"' as i32 {
        return Status::DECLINED;
    }
    return htp_util::htp_extract_quoted_string_as_bstr(
        data.offset(pos as isize),
        len.wrapping_sub(pos),
        &mut (*(*connp).in_tx).request_auth_username,
        0 as *mut size_t,
    );
}

/// Parses Basic Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization_basic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut auth_header: *mut htp_transaction::htp_header_t,
) -> Status {
    let mut data: *mut libc::c_uchar = if (*(*auth_header).value).realptr.is_null() {
        ((*auth_header).value as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*(*auth_header).value).realptr
    };
    let mut len: size_t = (*(*auth_header).value).len;
    let mut pos: size_t = 5 as libc::c_int as size_t;
    // Ignore whitespace
    while pos < len
        && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return Status::DECLINED;
    }
    // Decode base64-encoded data
    let mut decoded: *mut bstr::bstr = htp_base64::htp_base64_decode_mem(
        data.offset(pos as isize) as *const libc::c_void,
        len.wrapping_sub(pos),
    );
    if decoded.is_null() {
        return Status::ERROR;
    }
    // Now extract the username and password
    let mut i: libc::c_int =
        bstr::bstr_index_of_c(decoded, b":\x00" as *const u8 as *const libc::c_char);
    if i == -(1 as libc::c_int) {
        bstr::bstr_free(decoded);
        return Status::DECLINED;
    }
    (*(*connp).in_tx).request_auth_username =
        bstr::bstr_dup_ex(decoded, 0 as libc::c_int as size_t, i as size_t);
    if (*(*connp).in_tx).request_auth_username.is_null() {
        bstr::bstr_free(decoded);
        return Status::ERROR;
    }
    (*(*connp).in_tx).request_auth_password = bstr::bstr_dup_ex(
        decoded,
        (i + 1 as libc::c_int) as size_t,
        (*decoded)
            .len
            .wrapping_sub(i as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if (*(*connp).in_tx).request_auth_password.is_null() {
        bstr::bstr_free(decoded);
        bstr::bstr_free((*(*connp).in_tx).request_auth_username);
        return Status::ERROR;
    }
    bstr::bstr_free(decoded);
    return Status::OK;
}

/// Parses Authorization request header.
pub unsafe extern "C" fn htp_parse_authorization(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    let mut auth_header: *mut htp_transaction::htp_header_t = htp_table::htp_table_get_c(
        (*(*connp).in_tx).request_headers,
        b"authorization\x00" as *const u8 as *const libc::c_char,
    )
        as *mut htp_transaction::htp_header_t;
    if auth_header.is_null() {
        (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_NONE;
        return Status::OK;
    }
    // TODO Need a flag to raise when failing to parse authentication headers.
    if bstr::bstr_begins_with_c_nocase(
        (*auth_header).value,
        b"basic\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        // Basic authentication
        (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_BASIC;
        return htp_parse_authorization_basic(connp, auth_header);
    } else {
        if bstr::bstr_begins_with_c_nocase(
            (*auth_header).value,
            b"digest\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            // Digest authentication
            (*(*connp).in_tx).request_auth_type = htp_transaction::htp_auth_type_t::HTP_AUTH_DIGEST;
            return htp_parse_authorization_digest(connp, auth_header);
        } else {
            // Unrecognized authentication method
            (*(*connp).in_tx).request_auth_type =
                htp_transaction::htp_auth_type_t::HTP_AUTH_UNRECOGNIZED
        }
    }
    return Status::OK;
}
