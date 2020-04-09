use crate::{htp_connection_parser, htp_request_generic, Status};
use ::libc;

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type htp_time_t = libc::timeval;

/* *
 * Extract one request header. A header can span multiple lines, in
 * which case they will be folded into one before parsing is attempted.
 *
 * @param[in] connp
 * @param[in] data
 * @param[in] len
 * @return HTP_OK or HTP_ERROR
 */
#[no_mangle]
pub unsafe extern "C" fn htp_process_request_header_apache_2_2(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> Status {
    return htp_request_generic::htp_process_request_header_generic(connp, data, len);
}
/* *
 * Parse request line as Apache 2.2 does.
 *
 * @param[in] connp
 * @return HTP_OK or HTP_ERROR
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_request_line_apache_2_2(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    return htp_request_generic::htp_parse_request_line_generic_ex(connp, 1 as libc::c_int);
}
