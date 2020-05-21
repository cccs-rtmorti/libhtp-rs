use crate::{htp_connection_parser, htp_request_generic, Status};

/// Extract one request header. A header can span multiple lines, in
/// which case they will be folded into one before parsing is attempted.
///
/// Returns HTP_OK or HTP_ERROR
pub unsafe extern "C" fn htp_process_request_header_apache_2_2(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut u8,
    mut len: usize,
) -> Status {
    return htp_request_generic::htp_process_request_header_generic(connp, data, len);
}

/// Parse request line as Apache 2.2 does.
///
/// Returns HTP_OK or HTP_ERROR
pub unsafe extern "C" fn htp_parse_request_line_apache_2_2(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    return htp_request_generic::htp_parse_request_line_generic_ex(connp, 1);
}
