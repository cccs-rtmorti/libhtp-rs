use crate::htp_transaction::Protocol;
use crate::htp_util::Flags;
use crate::{bstr, htp_connection_parser, htp_parsers, htp_transaction, htp_util, Status};
use std::cmp::Ordering;

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
    let tx = if let Some(out_tx) = (*connp).out_tx_mut() {
        out_tx
    } else {
        return Status::ERROR;
    };
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
    (*tx).response_protocol_number =
        htp_parsers::htp_parse_protocol(&*(*tx).response_protocol, &mut *connp);
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
    if let Some(status_code) = htp_parsers::htp_parse_status(&*(*tx).response_status) {
        (*tx).response_status_number = status_code as i32;
    } else {
        (*tx).response_status_number = -1;
    }
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
    data: *mut u8,
    mut len: usize,
) -> Result<htp_transaction::htp_header_t, Status> {
    let out_tx = if let Some(out_tx) = (*connp).out_tx_mut() {
        out_tx
    } else {
        return Err(Status::ERROR);
    };
    let mut name_start: usize = 0;
    let mut name_end: usize = 0;
    let mut value_start: usize = 0;
    let mut value_end: usize = 0;
    let mut prev: usize = 0;
    let mut flags = Flags::empty();
    let s = std::slice::from_raw_parts(data as *const u8, len);
    let s = htp_util::htp_chomp(&s);
    len = s.len();
    name_start = 0;
    // Look for the first colon.
    let mut colon_pos: usize = 0;
    while colon_pos < len && *data.offset(colon_pos as isize) != ':' as u8 {
        colon_pos = colon_pos.wrapping_add(1)
    }
    if colon_pos == len {
        // Header line with a missing colon.
        flags |= Flags::HTP_FIELD_UNPARSEABLE;
        flags |= Flags::HTP_FIELD_INVALID;
        if !out_tx.flags.contains(Flags::HTP_FIELD_UNPARSEABLE) {
            // Only once per transaction.
            out_tx.flags |= Flags::HTP_FIELD_UNPARSEABLE;
            out_tx.flags |= Flags::HTP_FIELD_INVALID;
            htp_warn!(
                connp,
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
            flags |= Flags::HTP_FIELD_INVALID;
            if !out_tx.flags.contains(Flags::HTP_FIELD_INVALID) {
                // Only once per transaction.
                out_tx.flags |= Flags::HTP_FIELD_INVALID;
                htp_warn!(
                    connp,
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
            flags |= Flags::HTP_FIELD_INVALID;
            if !out_tx.flags.contains(Flags::HTP_FIELD_INVALID) {
                // Only once per transaction.
                out_tx.flags |= Flags::HTP_FIELD_INVALID;
                htp_warn!(
                    connp,
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
            flags |= Flags::HTP_FIELD_INVALID;
            if !out_tx.flags.contains(Flags::HTP_FIELD_INVALID) {
                out_tx.flags |= Flags::HTP_FIELD_INVALID;
                htp_warn!(
                    connp,
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
            htp_warn!(
                connp,
                htp_log_code::REQUEST_HEADER_INVALID,
                "Response header value contains null."
            );
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    // Now extract the name and the value.
    let name = std::slice::from_raw_parts(
        data.offset(name_start as isize),
        name_end.wrapping_sub(name_start),
    );
    let value = std::slice::from_raw_parts(
        data.offset(value_start as isize),
        value_end.wrapping_sub(value_start),
    );
    Ok(htp_transaction::htp_header_t::new_with_flags(
        name.into(),
        value.into(),
        flags,
    ))
}

/// Generic response header line(s) processor, which assembles folded lines
/// into a single buffer before invoking the parsing function.
pub unsafe extern "C" fn htp_process_response_header_generic(
    connp: *mut htp_connection_parser::htp_connp_t,
    data: *mut u8,
    len: usize,
) -> Status {
    let out_tx = if let Some(out_tx) = (*connp).out_tx_mut() {
        out_tx
    } else {
        return Status::ERROR;
    };
    let header = if let Ok(header) = htp_parse_response_header_generic(connp, data, len) {
        header
    } else {
        return Status::ERROR;
    };
    // Do we already have a header with the same name?
    if let Some((_, h_existing)) = out_tx
        .response_headers
        .get_nocase_mut(header.name.as_slice())
    {
        // Keep track of repeated same-name headers.
        if !h_existing.flags.contains(Flags::HTP_FIELD_REPEATED) {
            // This is the second occurence for this header.
            htp_warn!(
                connp,
                htp_log_code::RESPONSE_HEADER_REPETITION,
                "Repetition for header"
            );
        } else if (out_tx.res_header_repetitions) < 64 {
            out_tx.res_header_repetitions = out_tx.res_header_repetitions.wrapping_add(1)
        } else {
            return Status::OK;
        }
        h_existing.flags |= Flags::HTP_FIELD_REPEATED;
        // For simplicity reasons, we count the repetitions of all headers
        // Having multiple C-L headers is against the RFC but many
        // browsers ignore the subsequent headers if the values are the same.
        if header.name.cmp_nocase("Content-Length") == Ordering::Equal {
            // Don't use string comparison here because we want to
            // ignore small formatting differences.
            let existing_cl = htp_util::htp_parse_content_length(&h_existing.value, None);
            let new_cl = htp_util::htp_parse_content_length(&(header.value), None);
            if existing_cl.is_none() || new_cl.is_none() || existing_cl != new_cl {
                // Ambiguous response C-L value.
                htp_warn!(
                    connp,
                    htp_log_code::DUPLICATE_CONTENT_LENGTH_FIELD_IN_RESPONSE,
                    "Ambiguous response C-L value"
                );
            }
        } else {
            // Add to the existing header.
            h_existing.value.extend_from_slice(b", ");
            h_existing.value.extend_from_slice(header.value.as_slice());
        }
    } else {
        out_tx.response_headers.add(header.name.clone(), header);
    }
    Status::OK
}
