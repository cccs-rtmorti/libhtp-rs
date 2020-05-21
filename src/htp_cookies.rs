use crate::{bstr, htp_connection_parser, htp_table, htp_transaction, Status};

extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub const _ISspace: i32 = 8192;

/// Parses a single v0 request cookie and places the results into tx->request_cookies.
///
/// Returns HTP_OK on success, HTP_ERROR on error.
pub unsafe fn htp_parse_single_cookie_v0(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut u8,
    mut len: usize,
) -> Status {
    if len == 0 {
        return Status::OK;
    }
    let mut pos: usize = 0;
    // Look for '='.
    while pos < len && *data.offset(pos as isize) != '=' as u8 {
        pos = pos.wrapping_add(1)
    } // Ignore a nameless cookie.
    if pos == 0 {
        return Status::OK;
    }
    let mut name: *mut bstr::bstr_t = bstr::bstr_dup_mem(data as *const core::ffi::c_void, pos);
    if name.is_null() {
        return Status::ERROR;
    }
    let mut value: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
    if pos == len {
        // The cookie is empty.
        value = bstr::bstr_dup_c(b"\x00" as *const u8 as *const i8)
    } else {
        // The cookie is not empty.
        value = bstr::bstr_dup_mem(
            data.offset(pos as isize).offset(1) as *const core::ffi::c_void,
            len.wrapping_sub(pos).wrapping_sub(1),
        )
    }
    if value.is_null() {
        bstr::bstr_free(name);
        return Status::ERROR;
    }
    htp_table::htp_table_addn(
        (*(*connp).in_tx).request_cookies,
        name,
        value as *const core::ffi::c_void,
    );
    return Status::OK;
}

/// Parses the Cookie request header in v0 format.
///
/// Returns HTP_OK on success, HTP_ERROR on error
pub unsafe fn htp_parse_cookies_v0(mut connp: *mut htp_connection_parser::htp_connp_t) -> Status {
    let mut cookie_header: *mut htp_transaction::htp_header_t = htp_table::htp_table_get_c(
        (*(*connp).in_tx).request_headers,
        b"cookie\x00" as *const u8 as *const i8,
    )
        as *mut htp_transaction::htp_header_t;
    if cookie_header.is_null() {
        return Status::OK;
    }
    // Create a new table to store cookies.
    (*(*connp).in_tx).request_cookies = htp_table::htp_table_create(4);
    if (*(*connp).in_tx).request_cookies.is_null() {
        return Status::ERROR;
    }
    let mut data: *mut u8 = if (*(*cookie_header).value).realptr.is_null() {
        ((*cookie_header).value as *mut u8).offset(::std::mem::size_of::<bstr::bstr_t>() as isize)
    } else {
        (*(*cookie_header).value).realptr
    };
    let mut len: usize = (*(*cookie_header).value).len;
    let mut pos: usize = 0;
    while pos < len {
        // Ignore whitespace at the beginning.
        while pos < len
            && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as isize) as i32 & _ISspace
                != 0
        {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            return Status::OK;
        }
        let mut start: usize = pos;
        // Find the end of the cookie.
        while pos < len && *data.offset(pos as isize) != ';' as u8 {
            pos = pos.wrapping_add(1)
        }
        if htp_parse_single_cookie_v0(connp, data.offset(start as isize), pos.wrapping_sub(start))
            != Status::OK
        {
            return Status::ERROR;
        }
        // Go over the semicolon.
        if pos < len {
            pos = pos.wrapping_add(1)
        }
    }
    return Status::OK;
}
