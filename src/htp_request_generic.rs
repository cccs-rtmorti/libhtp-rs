use crate::htp_util::Flags;
use crate::{
    bstr, htp_config, htp_connection_parser, htp_parsers, htp_request, htp_table, htp_transaction,
    htp_util,
};
use ::libc;

extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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

pub type htp_status_t = libc::c_int;

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
pub unsafe extern "C" fn htp_process_request_header_generic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> htp_status_t {
    // Create a new header structure.
    let mut h: *mut htp_transaction::htp_header_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_transaction::htp_header_t>() as libc::c_ulong,
    ) as *mut htp_transaction::htp_header_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    // Now try to parse the header.
    if htp_parse_request_header_generic(connp, h, data, len) != 1 as libc::c_int {
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    // Do we already have a header with the same name?
    let mut h_existing: *mut htp_transaction::htp_header_t =
        htp_table::htp_table_get((*(*connp).in_tx).request_headers, (*h).name)
            as *mut htp_transaction::htp_header_t;
    if !h_existing.is_null() {
        // TODO Do we want to have a list of the headers that are
        //      allowed to be combined in this way?
        if !(*h_existing).flags.contains(Flags::HTP_FIELD_REPEATED) {
            // This is the second occurence for this header.
            htp_util::htp_log(
                connp,
                b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Repetition for header\x00" as *const u8 as *const libc::c_char,
            );
        } else if ((*(*connp).in_tx).req_header_repetitions as libc::c_int) < 64 as libc::c_int {
            (*(*connp).in_tx).req_header_repetitions =
                (*(*connp).in_tx).req_header_repetitions.wrapping_add(1)
        } else {
            bstr::bstr_free((*h).name);
            bstr::bstr_free((*h).value);
            free(h as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        // For simplicity reasons, we count the repetitions of all headers
        // Keep track of repeated same-name headers.
        (*h_existing).flags |= Flags::HTP_FIELD_REPEATED;
        // Having multiple C-L headers is against the RFC but
        // servers may ignore the subsequent headers if the values are the same.
        if bstr::bstr_cmp_c_nocase(
            (*h).name,
            b"Content-Length\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            // Don't use string comparison here because we want to
            // ignore small formatting differences.
            let mut existing_cl: int64_t = htp_util::htp_parse_content_length(
                (*h_existing).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            let mut new_cl: int64_t = htp_util::htp_parse_content_length(
                (*h).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            // Ambiguous response C-L value.
            if existing_cl == -(1 as libc::c_int) as libc::c_long
                || new_cl == -(1 as libc::c_int) as libc::c_long
                || existing_cl != new_cl
            {
                htp_util::htp_log(
                    connp,
                    b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                    100 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"Ambiguous request C-L value\x00" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            // Add to the existing header.
            let mut new_value: *mut bstr::bstr = bstr::bstr_expand(
                (*h_existing).value,
                (*(*h_existing).value)
                    .len
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add((*(*h).value).len),
            );
            if new_value.is_null() {
                bstr::bstr_free((*h).name);
                bstr::bstr_free((*h).value);
                free(h as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h_existing).value = new_value;
            bstr::bstr_add_mem_noex(
                (*h_existing).value,
                b", \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
            bstr::bstr_add_noex((*h_existing).value, (*h).value);
        }
        // The new header structure is no longer needed.
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        free(h as *mut libc::c_void);
    } else if htp_table::htp_table_add(
        (*(*connp).in_tx).request_headers,
        (*h).name,
        h as *const libc::c_void,
    ) != 1 as libc::c_int
    {
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        free(h as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}

/* *
 * Generic request header parser.
 *
 * @param[in] connp
 * @param[in] h
 * @param[in] data
 * @param[in] len
 * @return HTP_OK or HTP_ERROR
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_request_header_generic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut h: *mut htp_transaction::htp_header_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> htp_status_t {
    let mut name_start: size_t = 0;
    let mut name_end: size_t = 0;
    let mut value_start: size_t = 0;
    let mut value_end: size_t = 0;
    htp_util::htp_chomp(data, &mut len);
    name_start = 0 as libc::c_int as size_t;
    // Look for the colon.
    let mut colon_pos: size_t = 0 as libc::c_int as size_t;
    while colon_pos < len
        && *data.offset(colon_pos as isize) as libc::c_int != '\u{0}' as i32
        && *data.offset(colon_pos as isize) as libc::c_int != ':' as i32
    {
        colon_pos = colon_pos.wrapping_add(1)
    }
    if colon_pos == len || *data.offset(colon_pos as isize) as libc::c_int == '\u{0}' as i32 {
        // Missing colon.
        (*h).flags |= Flags::HTP_FIELD_UNPARSEABLE;
        // Log only once per transaction.
        if !(*(*connp).in_tx)
            .flags
            .contains(Flags::HTP_FIELD_UNPARSEABLE)
        {
            (*(*connp).in_tx).flags |= Flags::HTP_FIELD_UNPARSEABLE;
            htp_util::htp_log(
                connp,
                b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                163 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Request field invalid: colon missing\x00" as *const u8 as *const libc::c_char,
            );
        }
        // We handle this case as a header with an empty name, with the value equal
        // to the entire input string.
        // TODO Apache will respond to this problem with a 400.
        // Now extract the name and the value
        (*h).name = bstr::bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
        if (*h).name.is_null() {
            return -(1 as libc::c_int);
        }
        (*h).value = bstr::bstr_dup_mem(data as *const libc::c_void, len);
        if (*h).value.is_null() {
            bstr::bstr_free((*h).name);
            return -(1 as libc::c_int);
        }
        return 1 as libc::c_int;
    }
    if colon_pos == 0 as libc::c_int as libc::c_ulong {
        // Empty header name.
        (*h).flags |= Flags::HTP_FIELD_INVALID;
        // Log only once per transaction.
        if !(*(*connp).in_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
            (*(*connp).in_tx).flags |= Flags::HTP_FIELD_INVALID;
            htp_util::htp_log(
                connp,
                b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                192 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Request field invalid: empty name\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    name_end = colon_pos;
    // Ignore LWS after field-name.
    let mut prev: size_t = name_end;
    while prev > name_start
        && htp_util::htp_is_lws(
            *data.offset(prev.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int,
        ) != 0
    {
        // LWS after header name.
        prev = prev.wrapping_sub(1);
        name_end = name_end.wrapping_sub(1);
        (*h).flags |= Flags::HTP_FIELD_INVALID;
        // Log only once per transaction.
        if !(*(*connp).in_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
            (*(*connp).in_tx).flags |= Flags::HTP_FIELD_INVALID;
            htp_util::htp_log(
                connp,
                b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                211 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Request field invalid: LWS after name\x00" as *const u8 as *const libc::c_char,
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
    while value_start < len
        && htp_util::htp_is_lws(*data.offset(value_start as isize) as libc::c_int) != 0
    {
        value_start = value_start.wrapping_add(1)
    }
    // Look for the end of field-content.
    value_end = value_start;
    while value_end < len && *data.offset(value_end as isize) as libc::c_int != '\u{0}' as i32 {
        value_end = value_end.wrapping_add(1)
    }
    // Ignore LWS after field-content.
    prev = value_end.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while prev > value_start
        && htp_util::htp_is_lws(*data.offset(prev as isize) as libc::c_int) != 0
    {
        prev = prev.wrapping_sub(1);
        value_end = value_end.wrapping_sub(1)
    }
    // Check that the header name is a token.
    let mut i: size_t = name_start;
    while i < name_end {
        if htp_util::htp_is_token(*data.offset(i as isize) as libc::c_int) == 0 {
            // Incorrectly formed header name.
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            // Log only once per transaction.
            if !(*(*connp).in_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                (*(*connp).in_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_util::htp_log(
                    connp,
                    b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                    251 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"Request header name is not a token\x00" as *const u8 as *const libc::c_char,
                );
            }
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    // Now extract the name and the value
    (*h).name = bstr::bstr_dup_mem(
        data.offset(name_start as isize) as *const libc::c_void,
        name_end.wrapping_sub(name_start),
    );
    if (*h).name.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).value = bstr::bstr_dup_mem(
        data.offset(value_start as isize) as *const libc::c_void,
        value_end.wrapping_sub(value_start),
    );
    if (*h).value.is_null() {
        bstr::bstr_free((*h).name);
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/* *
 * Generic request line parser.
 *
 * @param[in] connp
 * @return HTP_OK or HTP_ERROR
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_request_line_generic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> htp_status_t {
    return htp_parse_request_line_generic_ex(connp, 0 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn htp_parse_request_line_generic_ex(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut nul_terminates: libc::c_int,
) -> htp_status_t {
    let mut tx: *mut htp_transaction::htp_tx_t = (*connp).in_tx;
    let mut data: *mut libc::c_uchar = if (*(*tx).request_line).realptr.is_null() {
        ((*tx).request_line as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*(*tx).request_line).realptr
    };
    let mut len: size_t = (*(*tx).request_line).len;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut mstart: size_t = 0 as libc::c_int as size_t;
    let mut start: size_t = 0;
    let mut bad_delim: size_t = 0;
    if nul_terminates != 0 {
        // The line ends with the first NUL byte.
        let mut newlen: size_t = 0 as libc::c_int as size_t;
        while pos < len && *data.offset(pos as isize) as libc::c_int != '\u{0}' as i32 {
            pos = pos.wrapping_add(1);
            newlen = newlen.wrapping_add(1)
        }
        // Start again, with the new length.
        len = newlen;
        pos = 0 as libc::c_int as size_t
    }
    // skip past leading whitespace. IIS allows this
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    if pos != 0 {
        htp_util::htp_log(
            connp,
            b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
            309 as libc::c_int,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"Request line: leading whitespace\x00" as *const u8 as *const libc::c_char,
        );
        mstart = pos;
        if (*(*connp).cfg).requestline_leading_whitespace_unwanted
            != htp_config::htp_unwanted_t::HTP_UNWANTED_IGNORE
        {
            // reset mstart so that we copy the whitespace into the method
            mstart = 0 as libc::c_int as size_t;
            // set expected response code to this anomaly
            (*tx).response_status_expected_number =
                (*(*connp).cfg).requestline_leading_whitespace_unwanted as libc::c_int
        }
    }
    // The request method starts at the beginning of the
    // line and ends with the first whitespace character.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0 {
        pos = pos.wrapping_add(1)
    }
    // No, we don't care if the method is empty.
    (*tx).request_method = bstr::bstr_dup_mem(
        data.offset(mstart as isize) as *const libc::c_void,
        pos.wrapping_sub(mstart),
    );
    if (*tx).request_method.is_null() {
        return -(1 as libc::c_int);
    }
    (*tx).request_method_number =
        htp_util::htp_convert_method_to_number((*tx).request_method) as libc::c_uint;
    bad_delim = 0 as libc::c_int as size_t;
    // Ignore whitespace after request method. The RFC allows
    // for only one SP, but then suggests any number of SP and HT
    // should be permitted. Apache uses isspace(), which is even
    // more permitting, so that's what we use here.
    while pos < len
        && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        if bad_delim == 0 && *data.offset(pos as isize) as libc::c_int != 0x20 as libc::c_int {
            bad_delim = bad_delim.wrapping_add(1)
        }
        pos = pos.wrapping_add(1)
    }
    // Too much performance overhead for fuzzing
    if bad_delim != 0 {
        htp_util::htp_log(
            connp,
            b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"Request line: non-compliant delimiter between Method and URI\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // Is there anything after the request method?
    if pos == len {
        // No, this looks like a HTTP/0.9 request.
        (*tx).is_protocol_0_9 = 1 as libc::c_int;
        (*tx).request_protocol_number = 9 as libc::c_int;
        if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_UNKNOWN as libc::c_uint {
            htp_util::htp_log(
                connp,
                b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                360 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Request line: unknown method only\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    start = pos;
    bad_delim = 0 as libc::c_int as size_t;
    // The URI ends with the first whitespace.
    while pos < len && *data.offset(pos as isize) as libc::c_int != 0x20 as libc::c_int {
        if bad_delim == 0 && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0
        {
            bad_delim = bad_delim.wrapping_add(1)
        }
        pos = pos.wrapping_add(1)
    }
    /* if we've seen some 'bad' delimiters, we retry with those */
    if bad_delim != 0 && pos == len {
        // special case: even though RFC's allow only SP (0x20), many
        // implementations allow other delimiters, like tab or other
        // characters that isspace() accepts.
        pos = start;
        while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0 {
            pos = pos.wrapping_add(1)
        }
    }
    // Too much performance overhead for fuzzing
    if bad_delim != 0 {
        // warn regardless if we've seen non-compliant chars
        htp_util::htp_log(
            connp,
            b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"Request line: URI contains non-compliant delimiter\x00" as *const u8
                as *const libc::c_char,
        );
    }
    (*tx).request_uri = bstr::bstr_dup_mem(
        data.offset(start as isize) as *const libc::c_void,
        pos.wrapping_sub(start),
    );
    if (*tx).request_uri.is_null() {
        return -(1 as libc::c_int);
    }
    // Ignore whitespace after URI.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    // Is there protocol information available?
    if pos == len {
        // No, this looks like a HTTP/0.9 request.
        (*tx).is_protocol_0_9 = 1 as libc::c_int;
        (*tx).request_protocol_number = 9 as libc::c_int;
        if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_UNKNOWN as libc::c_uint {
            htp_util::htp_log(
                connp,
                b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
                408 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Request line: unknown method and no protocol\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    // The protocol information continues until the end of the line.
    (*tx).request_protocol = bstr::bstr_dup_mem(
        data.offset(pos as isize) as *const libc::c_void,
        len.wrapping_sub(pos),
    );
    if (*tx).request_protocol.is_null() {
        return -(1 as libc::c_int);
    }
    (*tx).request_protocol_number = htp_parsers::htp_parse_protocol((*tx).request_protocol);
    if (*tx).request_method_number == htp_request::htp_method_t::HTP_M_UNKNOWN as libc::c_uint
        && (*tx).request_protocol_number == -(2 as libc::c_int)
    {
        htp_util::htp_log(
            connp,
            b"htp_request_generic.c\x00" as *const u8 as *const libc::c_char,
            419 as libc::c_int,
            htp_util::htp_log_level_t::HTP_LOG_WARNING,
            0 as libc::c_int,
            b"Request line: unknown method and invalid protocol\x00" as *const u8
                as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
