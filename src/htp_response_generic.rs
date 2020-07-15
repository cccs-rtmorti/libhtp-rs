use crate::htp_transaction::Protocol;
use crate::htp_util::Flags;
use crate::{bstr, htp_connection_parser, htp_parsers, htp_transaction, htp_util, Status};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

/// Generic response line parser.
pub unsafe extern "C" fn htp_parse_response_line_generic(
    connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    let mut tx: *mut htp_transaction::htp_tx_t = (*connp).out_tx;
    let data: *const u8 = bstr::bstr_ptr((*tx).response_line);
    let len: usize = bstr::bstr_len((*tx).response_line);
    let mut pos: usize = 0;
    (*tx).response_protocol = 0 as *mut bstr::bstr_t;
    (*tx).response_protocol_number = Protocol::INVALID;
    (*tx).response_status = 0 as *mut bstr::bstr_t;
    (*tx).response_status_number = -1;
    (*tx).response_message = 0 as *mut bstr::bstr_t;
    // Ignore whitespace at the beginning of the line.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize)) {
        pos = pos.wrapping_add(1)
    }
    let mut start: usize = pos;
    // Find the end of the protocol string.
    while pos < len && !htp_util::htp_is_space(*data.offset(pos as isize)) {
        pos = pos.wrapping_add(1)
    }
    if pos.wrapping_sub(start) == 0 {
        return Status::OK;
    }
    (*tx).response_protocol = bstr::bstr_dup_mem(
        data.offset(start as isize) as *const core::ffi::c_void,
        pos.wrapping_sub(start),
    );
    if (*tx).response_protocol.is_null() {
        return Status::ERROR;
    }
    (*tx).response_protocol_number = htp_parsers::htp_parse_protocol((*tx).response_protocol);
    // Ignore whitespace after the response protocol.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize)) {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return Status::OK;
    }
    start = pos;
    // Find the next whitespace character.
    while pos < len && !htp_util::htp_is_space(*data.offset(pos as isize)) {
        pos = pos.wrapping_add(1)
    }
    if pos.wrapping_sub(start) == 0 {
        return Status::OK;
    }
    (*tx).response_status = bstr::bstr_dup_mem(
        data.offset(start as isize) as *const core::ffi::c_void,
        pos.wrapping_sub(start),
    );
    if (*tx).response_status.is_null() {
        return Status::ERROR;
    }
    (*tx).response_status_number = htp_parsers::htp_parse_status((*tx).response_status);
    // Ignore whitespace that follows the status code.
    while pos < len && (*data.offset(pos as isize)).is_ascii_whitespace() {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return Status::OK;
    }
    // Assume the message stretches until the end of the line.
    (*tx).response_message = bstr::bstr_dup_mem(
        data.offset(pos as isize) as *const core::ffi::c_void,
        len.wrapping_sub(pos),
    );
    if (*tx).response_message.is_null() {
        return Status::ERROR;
    }
    Status::OK
}

/// Generic response header parser.
pub unsafe extern "C" fn htp_parse_response_header_generic(
    connp: *mut htp_connection_parser::htp_connp_t,
    mut h: *mut htp_transaction::htp_header_t,
    data: *mut u8,
    mut len: usize,
) -> Status {
    let mut name_start: usize = 0;
    let mut name_end: usize = 0;
    let mut value_start: usize = 0;
    let mut value_end: usize = 0;
    let mut prev: usize = 0;
    htp_util::htp_chomp(data, &mut len);
    name_start = 0;
    // Look for the first colon.
    let mut colon_pos: usize = 0;
    while colon_pos < len && *data.offset(colon_pos as isize) != ':' as u8 {
        colon_pos = colon_pos.wrapping_add(1)
    }
    if colon_pos == len {
        // Header line with a missing colon.
        (*h).flags |= Flags::HTP_FIELD_UNPARSEABLE;
        (*h).flags |= Flags::HTP_FIELD_INVALID;
        if !(*(*connp).out_tx)
            .flags
            .contains(Flags::HTP_FIELD_UNPARSEABLE)
        {
            // Only once per transaction.
            (*(*connp).out_tx).flags |= Flags::HTP_FIELD_UNPARSEABLE;
            (*(*connp).out_tx).flags |= Flags::HTP_FIELD_INVALID;
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::RESPONSE_FIELD_MISSING_COLON,
                "Response field invalid: missing colon."
            );
        }
        // Reset the position. We're going to treat this invalid header
        // as a header with an empty name. That will increase the probability
        // that the content will be inspected.
        colon_pos = 0;
        // suppress scan-build warning
        name_end = 0;
        value_start = 0
    } else {
        // Header line with a colon.
        if colon_pos == 0 {
            // Empty header name.
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            if !(*(*connp).out_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                // Only once per transaction.
                (*(*connp).out_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_log!(
                    connp,
                    htp_log_level_t::HTP_LOG_WARNING,
                    htp_log_code::RESPONSE_INVALID_EMPTY_NAME,
                    "Response field invalid: empty name."
                );
            }
        }
        name_end = colon_pos;
        // Ignore unprintable after field-name.
        prev = name_end;
        while prev > name_start && *data.offset(prev.wrapping_sub(1) as isize) <= 0x20 {
            prev = prev.wrapping_sub(1);
            name_end = name_end.wrapping_sub(1);
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            if !(*(*connp).out_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                // Only once per transaction.
                (*(*connp).out_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_log!(
                    connp,
                    htp_log_level_t::HTP_LOG_WARNING,
                    htp_log_code::RESPONSE_INVALID_LWS_AFTER_NAME,
                    "Response field invalid: LWS after name"
                );
            }
        }
        value_start = colon_pos.wrapping_add(1)
    }
    // Header value.
    // Ignore LWS before field-content.
    while value_start < len && htp_util::htp_is_lws(*data.offset(value_start as isize)) {
        value_start = value_start.wrapping_add(1)
    }
    // Look for the end of field-content.
    value_end = len;
    // Check that the header name is a token.
    let mut i: usize = name_start;
    while i < name_end {
        if !htp_util::htp_is_token(*data.offset(i as isize)) {
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            if !(*(*connp).out_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                (*(*connp).out_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_log!(
                    connp,
                    htp_log_level_t::HTP_LOG_WARNING,
                    htp_log_code::RESPONSE_HEADER_NAME_NOT_TOKEN,
                    "Response header name is not a token."
                );
            }
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    i = value_start;
    while i < value_end {
        if *data.offset(i as isize) == 0 {
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_HEADER_INVALID,
                "Response header value contains null."
            );
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    // Now extract the name and the value.
    (*h).name = bstr::bstr_dup_mem(
        data.offset(name_start as isize) as *const core::ffi::c_void,
        name_end.wrapping_sub(name_start),
    );
    (*h).value = bstr::bstr_dup_mem(
        data.offset(value_start as isize) as *const core::ffi::c_void,
        value_end.wrapping_sub(value_start),
    );
    if (*h).name.is_null() || (*h).value.is_null() {
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        return Status::ERROR;
    }
    Status::OK
}

/// Generic response header line(s) processor, which assembles folded lines
/// into a single buffer before invoking the parsing function.
pub unsafe extern "C" fn htp_process_response_header_generic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    data: *mut u8,
    len: usize,
) -> Status {
    // Create a new header structure.
    let h: *mut htp_transaction::htp_header_t =
        calloc(1, ::std::mem::size_of::<htp_transaction::htp_header_t>())
            as *mut htp_transaction::htp_header_t;
    if h.is_null() {
        return Status::ERROR;
    }
    if htp_parse_response_header_generic(connp, h, data, len) != Status::OK {
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    // Do we already have a header with the same name?
    let h_existing_opt = (*(*(*connp).out_tx).response_headers).get_nocase((*(*h).name).as_slice());
    if h_existing_opt.is_some() {
        let mut h_existing = h_existing_opt.unwrap().1;
        // Keep track of repeated same-name headers.
        if !(*h_existing).flags.contains(Flags::HTP_FIELD_REPEATED) {
            // This is the second occurence for this header.
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::RESPONSE_HEADER_REPETITION,
                "Repetition for header"
            );
        } else if ((*(*connp).out_tx).res_header_repetitions) < 64 {
            (*(*connp).out_tx).res_header_repetitions =
                (*(*connp).out_tx).res_header_repetitions.wrapping_add(1)
        } else {
            bstr::bstr_free((*h).name);
            bstr::bstr_free((*h).value);
            free(h as *mut core::ffi::c_void);
            return Status::OK;
        }
        (*h_existing).flags |= Flags::HTP_FIELD_REPEATED;
        // For simplicity reasons, we count the repetitions of all headers
        // Having multiple C-L headers is against the RFC but many
        // browsers ignore the subsequent headers if the values are the same.
        if bstr::bstr_cmp_str_nocase((*h).name, "Content-Length") == 0 {
            // Don't use string comparison here because we want to
            // ignore small formatting differences.
            let mut existing_cl: i64 = 0;
            let mut new_cl: i64 = 0;
            existing_cl = htp_util::htp_parse_content_length(
                (*h_existing).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            new_cl = htp_util::htp_parse_content_length(
                (*h).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            if existing_cl == -1 || new_cl == -1 || existing_cl != new_cl {
                // Ambiguous response C-L value.
                htp_log!(
                    connp,
                    htp_log_level_t::HTP_LOG_WARNING,
                    htp_log_code::DUPLICATE_CONTENT_LENGTH_FIELD_IN_RESPONSE,
                    "Ambiguous response C-L value"
                );
            }
        } else {
            // Add to the existing header.
            let new_value: *mut bstr::bstr_t = bstr::bstr_expand(
                (*h_existing).value,
                bstr::bstr_len((*h_existing).value)
                    .wrapping_add(2)
                    .wrapping_add(bstr::bstr_len((*h).value)),
            );
            if new_value.is_null() {
                bstr::bstr_free((*h).name);
                bstr::bstr_free((*h).value);
                free(h as *mut core::ffi::c_void);
                return Status::ERROR;
            }
            (*h_existing).value = new_value;
            (*(*h_existing).value).add_noex(", ");
            bstr::bstr_add_noex((*h_existing).value, (*h).value);
        }
        // The new header structure is no longer needed.
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        free(h as *mut core::ffi::c_void);
    } else {
        (*(*(*connp).out_tx).response_headers).add((*(*h).name).clone(), h);
    }
    Status::OK
}
