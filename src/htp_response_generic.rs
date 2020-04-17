use crate::htp_transaction::Protocol;
use crate::htp_util::Flags;
use crate::{
    bstr, htp_connection_parser, htp_parsers, htp_table, htp_transaction, htp_util, Status,
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

pub type htp_time_t = libc::timeval;

/* *
 * Generic response line parser.
 *
 * @param[in] connp
 * @return HTP status
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_response_line_generic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
) -> Status {
    let mut tx: *mut htp_transaction::htp_tx_t = (*connp).out_tx;
    let mut data: *mut libc::c_uchar = if (*(*tx).response_line).realptr.is_null() {
        ((*tx).response_line as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*(*tx).response_line).realptr
    };
    let mut len: size_t = (*(*tx).response_line).len;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    (*tx).response_protocol = 0 as *mut bstr::bstr_t;
    (*tx).response_protocol_number = Protocol::INVALID as libc::c_int;
    (*tx).response_status = 0 as *mut bstr::bstr_t;
    (*tx).response_status_number = -(1 as libc::c_int);
    (*tx).response_message = 0 as *mut bstr::bstr_t;
    // Ignore whitespace at the beginning of the line.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    let mut start: size_t = pos;
    // Find the end of the protocol string.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0 {
        pos = pos.wrapping_add(1)
    }
    if pos.wrapping_sub(start) == 0 as libc::c_int as libc::c_ulong {
        return Status::OK;
    }
    (*tx).response_protocol = bstr::bstr_dup_mem(
        data.offset(start as isize) as *const libc::c_void,
        pos.wrapping_sub(start),
    );
    if (*tx).response_protocol.is_null() {
        return Status::ERROR;
    }
    (*tx).response_protocol_number =
        htp_parsers::htp_parse_protocol((*tx).response_protocol) as libc::c_int;
    // Ignore whitespace after the response protocol.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return Status::OK;
    }
    start = pos;
    // Find the next whitespace character.
    while pos < len && htp_util::htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0 {
        pos = pos.wrapping_add(1)
    }
    if pos.wrapping_sub(start) == 0 as libc::c_int as libc::c_ulong {
        return Status::OK;
    }
    (*tx).response_status = bstr::bstr_dup_mem(
        data.offset(start as isize) as *const libc::c_void,
        pos.wrapping_sub(start),
    );
    if (*tx).response_status.is_null() {
        return Status::ERROR;
    }
    (*tx).response_status_number = htp_parsers::htp_parse_status((*tx).response_status);
    // Ignore whitespace that follows the status code.
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
    // Assume the message stretches until the end of the line.
    (*tx).response_message = bstr::bstr_dup_mem(
        data.offset(pos as isize) as *const libc::c_void,
        len.wrapping_sub(pos),
    );
    if (*tx).response_message.is_null() {
        return Status::ERROR;
    }
    return Status::OK;
}

/* *
 * Generic response header parser.
 *
 * @param[in] connp
 * @param[in] h
 * @param[in] data
 * @param[in] len
 * @return HTP status
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_response_header_generic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut h: *mut htp_transaction::htp_header_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> Status {
    let mut name_start: size_t = 0;
    let mut name_end: size_t = 0;
    let mut value_start: size_t = 0;
    let mut value_end: size_t = 0;
    let mut prev: size_t = 0;
    htp_util::htp_chomp(data, &mut len);
    name_start = 0 as libc::c_int as size_t;
    // Look for the first colon.
    let mut colon_pos: size_t = 0 as libc::c_int as size_t;
    while colon_pos < len && *data.offset(colon_pos as isize) as libc::c_int != ':' as i32 {
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
            htp_util::htp_log(
                connp,
                b"htp_response_generic.c\x00" as *const u8 as *const libc::c_char,
                147 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Response field invalid: missing colon.\x00" as *const u8 as *const libc::c_char,
            );
        }
        // Reset the position. We're going to treat this invalid header
        // as a header with an empty name. That will increase the probability
        // that the content will be inspected.
        colon_pos = 0 as libc::c_int as size_t;
        // suppress scan-build warning
        name_end = 0 as libc::c_int as size_t;
        value_start = 0 as libc::c_int as size_t
    } else {
        // Header line with a colon.
        if colon_pos == 0 as libc::c_int as libc::c_ulong {
            // Empty header name.
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            if !(*(*connp).out_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                // Only once per transaction.
                (*(*connp).out_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_util::htp_log(
                    connp,
                    b"htp_response_generic.c\x00" as *const u8 as *const libc::c_char,
                    168 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"Response field invalid: empty name.\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        name_end = colon_pos;
        // Ignore unprintable after field-name.
        prev = name_end;
        while prev > name_start
            && *data.offset(prev.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                <= 0x20 as libc::c_int
        {
            prev = prev.wrapping_sub(1);
            name_end = name_end.wrapping_sub(1);
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            if !(*(*connp).out_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                // Only once per transaction.
                (*(*connp).out_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_util::htp_log(
                    connp,
                    b"htp_response_generic.c\x00" as *const u8 as *const libc::c_char,
                    185 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"Response field invalid: LWS after name.\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        value_start = colon_pos.wrapping_add(1 as libc::c_int as libc::c_ulong)
    }
    // Header value.
    // Ignore LWS before field-content.
    while value_start < len
        && htp_util::htp_is_lws(*data.offset(value_start as isize) as libc::c_int) != 0
    {
        value_start = value_start.wrapping_add(1)
    }
    // Look for the end of field-content.
    value_end = len;
    // Check that the header name is a token.
    let mut i: size_t = name_start;
    while i < name_end {
        if htp_util::htp_is_token(*data.offset(i as isize) as libc::c_int) == 0 {
            (*h).flags |= Flags::HTP_FIELD_INVALID;
            if !(*(*connp).out_tx).flags.contains(Flags::HTP_FIELD_INVALID) {
                (*(*connp).out_tx).flags |= Flags::HTP_FIELD_INVALID;
                htp_util::htp_log(
                    connp,
                    b"htp_response_generic.c\x00" as *const u8 as *const libc::c_char,
                    210 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"Response header name is not a token.\x00" as *const u8 as *const libc::c_char,
                );
            }
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    i = value_start;
    while i < value_end {
        if *data.offset(i as isize) as libc::c_int == 0 as libc::c_int {
            htp_util::htp_log(
                connp,
                b"htp_response_generic.c\x00" as *const u8 as *const libc::c_char,
                220 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Response header value contains null.\x00" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    // Now extract the name and the value.
    (*h).name = bstr::bstr_dup_mem(
        data.offset(name_start as isize) as *const libc::c_void,
        name_end.wrapping_sub(name_start),
    );
    (*h).value = bstr::bstr_dup_mem(
        data.offset(value_start as isize) as *const libc::c_void,
        value_end.wrapping_sub(value_start),
    );
    if (*h).name.is_null() || (*h).value.is_null() {
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        return Status::ERROR;
    }
    return Status::OK;
}

/* *
 * Generic response header line(s) processor, which assembles folded lines
 * into a single buffer before invoking the parsing function.
 *
 * @param[in] connp
 * @param[in] data
 * @param[in] len
 * @return HTP status
 */
#[no_mangle]
pub unsafe extern "C" fn htp_process_response_header_generic(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> Status {
    // Create a new header structure.
    let mut h: *mut htp_transaction::htp_header_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_transaction::htp_header_t>() as libc::c_ulong,
    ) as *mut htp_transaction::htp_header_t;
    if h.is_null() {
        return Status::ERROR;
    }
    if htp_parse_response_header_generic(connp, h, data, len) != Status::OK {
        free(h as *mut libc::c_void);
        return Status::ERROR;
    }
    // Do we already have a header with the same name?
    let mut h_existing: *mut htp_transaction::htp_header_t =
        htp_table::htp_table_get((*(*connp).out_tx).response_headers, (*h).name)
            as *mut htp_transaction::htp_header_t;
    if !h_existing.is_null() {
        // Keep track of repeated same-name headers.
        if !(*h_existing).flags.contains(Flags::HTP_FIELD_REPEATED) {
            // This is the second occurence for this header.
            htp_util::htp_log(
                connp,
                b"htp_response_generic.c\x00" as *const u8 as *const libc::c_char,
                267 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                0 as libc::c_int,
                b"Repetition for header\x00" as *const u8 as *const libc::c_char,
            );
        } else if ((*(*connp).out_tx).res_header_repetitions as libc::c_int) < 64 as libc::c_int {
            (*(*connp).out_tx).res_header_repetitions =
                (*(*connp).out_tx).res_header_repetitions.wrapping_add(1)
        } else {
            bstr::bstr_free((*h).name);
            bstr::bstr_free((*h).value);
            free(h as *mut libc::c_void);
            return Status::OK;
        }
        (*h_existing).flags |= Flags::HTP_FIELD_REPEATED;
        // For simplicity reasons, we count the repetitions of all headers
        // Having multiple C-L headers is against the RFC but many
        // browsers ignore the subsequent headers if the values are the same.
        if bstr::bstr_cmp_c_nocase(
            (*h).name,
            b"Content-Length\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            // Don't use string comparison here because we want to
            // ignore small formatting differences.
            let mut existing_cl: int64_t = 0;
            let mut new_cl: int64_t = 0;
            existing_cl = htp_util::htp_parse_content_length(
                (*h_existing).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            new_cl = htp_util::htp_parse_content_length(
                (*h).value,
                0 as *mut htp_connection_parser::htp_connp_t,
            );
            if existing_cl == -(1 as libc::c_int) as libc::c_long
                || new_cl == -(1 as libc::c_int) as libc::c_long
                || existing_cl != new_cl
            {
                // Ambiguous response C-L value.
                htp_util::htp_log(
                    connp,
                    b"htp_response_generic.c\x00" as *const u8 as *const libc::c_char,
                    293 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"Ambiguous response C-L value\x00" as *const u8 as *const libc::c_char,
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
                return Status::ERROR;
            }
            (*h_existing).value = new_value;
            bstr::bstr_add_mem_noex(
                (*h_existing).value,
                b", \x00" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
            bstr::bstr_add_noex((*h_existing).value, (*h).value);
        }
        // The new header structure is no longer needed.
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        free(h as *mut libc::c_void);
    } else if htp_table::htp_table_add(
        (*(*connp).out_tx).response_headers,
        (*h).name,
        h as *const libc::c_void,
    ) != Status::OK
    {
        bstr::bstr_free((*h).name);
        bstr::bstr_free((*h).value);
        free(h as *mut libc::c_void);
        return Status::ERROR;
    }
    return Status::OK;
}
