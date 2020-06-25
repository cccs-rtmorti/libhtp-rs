use crate::htp_transaction::Protocol;
use crate::htp_util::Flags;
use crate::{
    bstr, htp_config, htp_connection_parser, htp_parsers, htp_request, htp_transaction, htp_util,
    Status,
};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

/// Extract one request header. A header can span multiple lines, in
/// which case they will be folded into one before parsing is attempted.
///
/// Returns HTP_OK or HTP_ERROR
pub unsafe extern "C" fn htp_process_request_header_generic(
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
    // Now try to parse the header.
    if htp_parse_request_header_generic(connp, h, data, len) != Status::OK {
        free(h as *mut core::ffi::c_void);
        return Status::ERROR;
    }
    // Do we already have a header with the same name?
    let h_existing_opt = (*(*(*connp).in_tx).request_headers).get_nocase((*(*h).name).as_slice());
    if h_existing_opt.is_some() {
        let mut h_existing = h_existing_opt.unwrap().1;
        // TODO Do we want to have a list of the headers that are
        //      allowed to be combined in this way?
        if !(*h_existing).flags.contains(Flags::HTP_FIELD_REPEATED) {
            // This is the second occurence for this header.
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_HEADER_REPETITION,
                "Repetition for header"
            );
        } else if ((*(*connp).in_tx).req_header_repetitions) < 64 {
            (*(*connp).in_tx).req_header_repetitions =
                (*(*connp).in_tx).req_header_repetitions.wrapping_add(1)
        } else {
            bstr::bstr_free((*h).name);
            bstr::bstr_free((*h).value);
            free(h as *mut core::ffi::c_void);
            return Status::OK;
        }
        // For simplicity reasons, we count the repetitions of all headers
        // Keep track of repeated same-name headers.
        (*h_existing).flags |= Flags::HTP_FIELD_REPEATED;
        // Having multiple C-L headers is against the RFC but
        // servers may ignore the subsequent headers if the values are the same.
        if bstr::bstr_cmp_str_nocase((*h).name, "Content-Length") == 0 {
            // Don't use string comparison here because we want to
            // ignore small formatting differences.
            let existing_cl: i64 = htp_util::htp_parse_content_length(
                (*h_existing).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            let new_cl: i64 = htp_util::htp_parse_content_length(
                (*h).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            // Ambiguous response C-L value.
            if existing_cl == -1 || new_cl == -1 || existing_cl != new_cl {
                htp_log!(
                    connp,
                    htp_log_level_t::HTP_LOG_WARNING,
                    htp_log_code::DUPLICATE_CONTENT_LENGTH_FIELD_IN_REQUEST,
                    "Ambiguous request C-L value"
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
        (*(*(*connp).in_tx).request_headers).add((*(*h).name).clone(), h);
    }
    Status::OK
}

/// Generic request header parser.
///
/// Returns HTP_OK or HTP_ERROR
pub unsafe extern "C" fn htp_parse_request_header_generic(
    connp: *mut htp_connection_parser::htp_connp_t,
    mut h: *mut htp_transaction::htp_header_t,
    data: *mut u8,
    mut len: usize,
) -> Status {
    let mut name_start: usize = 0;
    let mut name_end: usize = 0;
    let mut value_start: usize = 0;
    let mut value_end: usize = 0;
    htp_util::htp_chomp(data, &mut len);
    name_start = 0;
    // Look for the colon.
    let mut colon_pos: usize = 0;
    while colon_pos < len
        && *data.offset(colon_pos as isize) != '\u{0}' as u8
        && *data.offset(colon_pos as isize) != ':' as u8
    {
        colon_pos = colon_pos.wrapping_add(1)
    }
    if colon_pos == len || *data.offset(colon_pos as isize) == '\u{0}' as u8 {
        // Missing colon.
        (*h).flags |= Flags::HTP_FIELD_UNPARSEABLE;
        // Log only once per transaction.
        if !(*(*connp).in_tx)
            .flags
            .contains(Flags::HTP_FIELD_UNPARSEABLE)
        {
            (*(*connp).in_tx).flags |= Flags::HTP_FIELD_UNPARSEABLE;
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_FIELD_MISSING_COLON,
                "Request field invalid: colon missing"
            );
        }
        // We handle this case as a header with an empty name, with the value equal
        // to the entire input string.
        // TODO Apache will respond to this problem with a 400.
        // Now extract the name and the value
        (*h).name = bstr::bstr_alloc(0);
        if (*h).name.is_null() {
            return Status::ERROR;
        }
        (*h).value = bstr::bstr_dup_mem(data as *const core::ffi::c_void, len);
        if (*h).value.is_null() {
            bstr::bstr_free((*h).name);
            return Status::ERROR;
        }
        return Status::OK;
    }
    if colon_pos == 0 {
        // Empty header name.
        (*h).flags |= Flags::HTP_FIELD_INVALID;
        // Log only once per transaction.
        if !(*(*connp).in_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
            (*(*connp).in_tx).flags |= Flags::HTP_FIELD_INVALID;
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_INVALID_EMPTY_NAME,
                "Request field invalid: empty name"
            );
        }
    }
    name_end = colon_pos;
    // Ignore LWS after field-name.
    let mut prev: usize = name_end;
    while prev > name_start && htp_util::htp_is_lws(*data.offset(prev.wrapping_sub(1) as isize)) {
        // LWS after header name.
        prev = prev.wrapping_sub(1);
        name_end = name_end.wrapping_sub(1);
        (*h).flags |= Flags::HTP_FIELD_INVALID;
        // Log only once per transaction.
        if !(*(*connp).in_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
            (*(*connp).in_tx).flags |= Flags::HTP_FIELD_INVALID;
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_INVALID_LWS_AFTER_NAME,
                "Request field invalid: LWS after name"
            );
        }
    }
    // Header value.
    value_start = colon_pos;
    // Go over the colon.
    if value_start < len {
        value_start = value_start.wrapping_add(1)
    }
    // Ignore LWS before field-content.
    while value_start < len && htp_util::htp_is_lws(*data.offset(value_start as isize)) {
        value_start = value_start.wrapping_add(1)
    }
    // Look for the end of field-content.
    value_end = value_start;
    while value_end < len && *data.offset(value_end as isize) != '\u{0}' as u8 {
        value_end = value_end.wrapping_add(1)
    }
    // Ignore LWS after field-content.
    prev = value_end.wrapping_sub(1);
    while prev > value_start && htp_util::htp_is_lws(*data.offset(prev as isize)) {
        prev = prev.wrapping_sub(1);
        value_end = value_end.wrapping_sub(1)
    }
    // Check that the header name is a token.
    let mut i: usize = name_start;
    while i < name_end {
        if !htp_util::htp_is_token(*data.offset(i as isize)) {
            // Incorrectly formed header name.
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            // Log only once per transaction.
            if !(*(*connp).in_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                (*(*connp).in_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_log!(
                    connp,
                    htp_log_level_t::HTP_LOG_WARNING,
                    htp_log_code::REQUEST_HEADER_INVALID,
                    "Request header name is not a token"
                );
            }
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    // Now extract the name and the value
    (*h).name = bstr::bstr_dup_mem(
        data.offset(name_start as isize) as *const core::ffi::c_void,
        name_end.wrapping_sub(name_start),
    );
    if (*h).name.is_null() {
        return Status::ERROR;
    }
    (*h).value = bstr::bstr_dup_mem(
        data.offset(value_start as isize) as *const core::ffi::c_void,
        value_end.wrapping_sub(value_start),
    );
    if (*h).value.is_null() {
        bstr::bstr_free((*h).name);
        return Status::ERROR;
    }
    Status::OK
}

/// Generic request line parser.
///
/// Returns HTP_OK or HTP_ERROR
pub unsafe extern "C" fn htp_parse_request_line_generic(
    connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    htp_parse_request_line_generic_ex(connp, 0)
}

pub unsafe extern "C" fn htp_parse_request_line_generic_ex(
    connp: *mut htp_connection_parser::htp_connp_t,
    nul_terminates: i32,
) -> Status {
    let mut tx: *mut htp_transaction::htp_tx_t = (*connp).in_tx;
    let data: *mut u8 = bstr::bstr_ptr((*tx).request_line);
    let mut len: usize = bstr::bstr_len((*tx).request_line);
    let mut pos: usize = 0;
    let mut mstart: usize = 0;
    let mut start: usize = 0;
    let mut bad_delim: usize = 0;
    if nul_terminates != 0 {
        // The line ends with the first NUL byte.
        let mut newlen: usize = 0;
        while pos < len && *data.offset(pos as isize) != '\u{0}' as u8 {
            pos = pos.wrapping_add(1);
            newlen = newlen.wrapping_add(1)
        }
        // Start again, with the new length.
        len = newlen;
        pos = 0
    }
    // skip past leading whitespace. IIS allows this
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize)) {
        pos = pos.wrapping_add(1)
    }
    if pos != 0 {
        htp_log!(
            connp,
            htp_log_level_t::HTP_LOG_WARNING,
            htp_log_code::REQUEST_LINE_LEADING_WHITESPACE,
            "Request line: leading whitespace"
        );
        mstart = pos;
        if (*(*connp).cfg).requestline_leading_whitespace_unwanted
            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
        {
            // reset mstart so that we copy the whitespace into the method
            mstart = 0;
            // set expected response code to this anomaly
            (*tx).response_status_expected_number =
                (*(*connp).cfg).requestline_leading_whitespace_unwanted as i32
        }
    }
    // The request method starts at the beginning of the
    // line and ends with the first whitespace character.
    while pos < len && !htp_util::htp_is_space(*data.offset(pos as isize)) {
        pos = pos.wrapping_add(1)
    }
    // No, we don't care if the method is empty.
    (*tx).request_method = bstr::bstr_dup_mem(
        data.offset(mstart as isize) as *const core::ffi::c_void,
        pos.wrapping_sub(mstart),
    );
    if (*tx).request_method.is_null() {
        return Status::ERROR;
    }
    (*tx).request_method_number =
        htp_util::htp_convert_method_to_number((*tx).request_method) as u32;
    bad_delim = 0;
    // Ignore whitespace after request method. The RFC allows
    // for only one SP, but then suggests any number of SP and HT
    // should be permitted. Apache uses isspace(), which is even
    // more permitting, so that's what we use here.
    while pos < len && (*data.offset(pos as isize)).is_ascii_whitespace() {
        if bad_delim == 0 && *data.offset(pos as isize) != 0x20 {
            bad_delim = bad_delim.wrapping_add(1)
        }
        pos = pos.wrapping_add(1)
    }
    // Too much performance overhead for fuzzing
    if bad_delim != 0 {
        htp_log!(
            connp,
            htp_log_level_t::HTP_LOG_WARNING,
            htp_log_code::METHOD_DELIM_NON_COMPLIANT,
            "Request line: non-compliant delimiter between Method and URI"
        );
    }
    // Is there anything after the request method?
    if pos == len {
        // No, this looks like a HTTP/0.9 request.
        (*tx).is_protocol_0_9 = 1;
        (*tx).request_protocol_number = Protocol::V0_9 as i32;
        if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_UNKNOWN as u32 {
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_LINE_UNKNOWN_METHOD,
                "Request line: unknown method only"
            );
        }
        return Status::OK;
    }
    start = pos;
    bad_delim = 0;
    // The URI ends with the first whitespace.
    while pos < len && *data.offset(pos as isize) != 0x20 {
        if bad_delim == 0 && htp_util::htp_is_space(*data.offset(pos as isize)) {
            bad_delim = bad_delim.wrapping_add(1)
        }
        pos = pos.wrapping_add(1)
    }
    // if we've seen some 'bad' delimiters, we retry with those
    if bad_delim != 0 && pos == len {
        // special case: even though RFC's allow only SP (0x20), many
        // implementations allow other delimiters, like tab or other
        // characters that isspace() accepts.
        pos = start;
        while pos < len && !htp_util::htp_is_space(*data.offset(pos as isize)) {
            pos = pos.wrapping_add(1)
        }
    }
    // Too much performance overhead for fuzzing
    if bad_delim != 0 {
        // warn regardless if we've seen non-compliant chars
        htp_log!(
            connp,
            htp_log_level_t::HTP_LOG_WARNING,
            htp_log_code::URI_DELIM_NON_COMPLIANT,
            "Request line: URI contains non-compliant delimiter"
        );
    }
    (*tx).request_uri = bstr::bstr_dup_mem(
        data.offset(start as isize) as *const core::ffi::c_void,
        pos.wrapping_sub(start),
    );
    if (*tx).request_uri.is_null() {
        return Status::ERROR;
    }
    // Ignore whitespace after URI.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize)) {
        pos = pos.wrapping_add(1)
    }
    // Is there protocol information available?
    if pos == len {
        // No, this looks like a HTTP/0.9 request.
        (*tx).is_protocol_0_9 = 1;
        (*tx).request_protocol_number = Protocol::V0_9 as i32;
        if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_UNKNOWN as u32 {
            htp_log!(
                connp,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_LINE_UNKNOWN_METHOD_NO_PROTOCOL,
                "Request line: unknown method and no protocol"
            );
        }
        return Status::OK;
    }
    // The protocol information continues until the end of the line.
    (*tx).request_protocol = bstr::bstr_dup_mem(
        data.offset(pos as isize) as *const core::ffi::c_void,
        len.wrapping_sub(pos),
    );
    if (*tx).request_protocol.is_null() {
        return Status::ERROR;
    }
    (*tx).request_protocol_number = htp_parsers::htp_parse_protocol((*tx).request_protocol) as i32;
    if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_UNKNOWN as u32
        && (*tx).request_protocol_number == Protocol::INVALID as i32
    {
        htp_log!(
            connp,
            htp_log_level_t::HTP_LOG_WARNING,
            htp_log_code::REQUEST_LINE_UNKNOWN_METHOD_INVALID_PROTOCOL,
            "Request line: unknown method and invalid protocol"
        );
    }
    Status::OK
}
