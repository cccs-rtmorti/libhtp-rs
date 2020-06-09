use crate::{bstr, htp_connection_parser, htp_table, Status};

extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub const _ISspace: i32 = 8192;

/// Parses a single v0 request cookie and places the results into tx->request_cookies.
///
/// Returns HTP_OK on success, HTP_ERROR on error.
pub unsafe fn htp_parse_single_cookie_v0(
    connp: *mut htp_connection_parser::htp_connp_t,
    data: *mut u8,
    len: usize,
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

    let name = bstr::bstr_t::from(std::slice::from_raw_parts(data, pos));
    let mut value: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
    if pos == len {
        // The cookie is empty.
        value = bstr::bstr_alloc(0);
    } else {
        // The cookie is not empty.
        value = bstr::bstr_dup_mem(
            data.offset(pos as isize).offset(1) as *const core::ffi::c_void,
            len.wrapping_sub(pos).wrapping_sub(1),
        )
    }
    if value.is_null() {
        return Status::ERROR;
    }
    (*(*(*connp).in_tx).request_cookies).add(name, value);
    Status::OK
}

/// Parses the Cookie request header in v0 format.
///
/// Returns HTP_OK on success, HTP_ERROR on error
pub unsafe fn htp_parse_cookies_v0(mut connp: *mut htp_connection_parser::htp_connp_t) -> Status {
    let cookie_header_opt = (*(*(*connp).in_tx).request_headers).get_nocase_nozero("cookie");
    if cookie_header_opt.is_none() {
        return Status::OK;
    }
    let cookie_header = cookie_header_opt.unwrap().1;
    // Create a new table to store cookies.
    (*(*connp).in_tx).request_cookies = htp_table::htp_table_alloc(4);
    let data: *mut u8 = bstr::bstr_ptr((*cookie_header).value);
    let len: usize = bstr::bstr_len((*cookie_header).value);
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
        let start: usize = pos;
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
    Status::OK
}
