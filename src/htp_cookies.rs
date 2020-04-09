use crate::{bstr, htp_connection_parser, htp_table, htp_transaction, Status};
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

/* *
 * Parses a single v0 request cookie and places the results into tx->request_cookies.
 *
 * @param[in] connp
 * @param[in] data
 * @param[in] len
 * @return HTP_OK on success, HTP_ERROR on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_single_cookie_v0(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> Status {
    if len == 0 as libc::c_int as libc::c_ulong {
        return Status::OK;
    }
    let mut pos: size_t = 0 as libc::c_int as size_t;
    // Look for '='.
    while pos < len && *data.offset(pos as isize) as libc::c_int != '=' as i32 {
        pos = pos.wrapping_add(1)
    } // Ignore a nameless cookie.
    if pos == 0 as libc::c_int as libc::c_ulong {
        return Status::OK;
    }
    let mut name: *mut bstr::bstr = bstr::bstr_dup_mem(data as *const libc::c_void, pos);
    if name.is_null() {
        return Status::ERROR;
    }
    let mut value: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
    if pos == len {
        // The cookie is empty.
        value = bstr::bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        // The cookie is not empty.
        value = bstr::bstr_dup_mem(
            data.offset(pos as isize).offset(1 as libc::c_int as isize) as *const libc::c_void,
            len.wrapping_sub(pos)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
    }
    if value.is_null() {
        bstr::bstr_free(name);
        return Status::ERROR;
    }
    htp_table::htp_table_addn(
        (*(*connp).in_tx).request_cookies,
        name,
        value as *const libc::c_void,
    );
    return Status::OK;
}

/* *
 * Parses the Cookie request header in v0 format.
 *
 * @param[in] connp
 * @return HTP_OK on success, HTP_ERROR on error
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_cookies_v0(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    let mut cookie_header: *mut htp_transaction::htp_header_t = htp_table::htp_table_get_c(
        (*(*connp).in_tx).request_headers,
        b"cookie\x00" as *const u8 as *const libc::c_char,
    )
        as *mut htp_transaction::htp_header_t;
    if cookie_header.is_null() {
        return Status::OK;
    }
    // Create a new table to store cookies.
    (*(*connp).in_tx).request_cookies = htp_table::htp_table_create(4 as libc::c_int as size_t);
    if (*(*connp).in_tx).request_cookies.is_null() {
        return Status::ERROR;
    }
    let mut data: *mut libc::c_uchar = if (*(*cookie_header).value).realptr.is_null() {
        ((*cookie_header).value as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*(*cookie_header).value).realptr
    };
    let mut len: size_t = (*(*cookie_header).value).len;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len {
        // Ignore whitespace at the beginning.
        while pos < len
            && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            return Status::OK;
        }
        let mut start: size_t = pos;
        // Find the end of the cookie.
        while pos < len && *data.offset(pos as isize) as libc::c_int != ';' as i32 {
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
