use crate::{bstr, bstr_builder, htp_connection_parser, htp_table, Status};
use ::libc;

extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type htp_time_t = libc::timeval;

/* *
 * Transcode all parameters supplied in the table.
 *
 * @param[in] connp
 * @param[in] params
 * @param[in] destroy_old
 */
#[no_mangle]
pub unsafe extern "C" fn htp_transcode_params(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut params: *mut *mut htp_table::htp_table_t,
    mut destroy_old: libc::c_int,
) -> Status {
    let mut input_params: *mut htp_table::htp_table_t = *params;
    // No transcoding unless necessary
    if (*(*connp).cfg).internal_encoding.is_null() || (*(*connp).cfg).request_encoding.is_null() {
        return Status::OK;
    }
    // Create a new table that will hold transcoded parameters
    let mut output_params: *mut htp_table::htp_table_t =
        htp_table::htp_table_create(htp_table::htp_table_size(input_params));
    if output_params.is_null() {
        return Status::ERROR;
    }
    // Initialize iconv
    let mut cd: iconv_t = iconv_open(
        (*(*connp).cfg).internal_encoding,
        (*(*connp).cfg).request_encoding,
    );
    if cd == -(1 as libc::c_int) as iconv_t {
        htp_table::htp_table_destroy(output_params);
        return Status::ERROR;
    }
    // Convert the parameters, one by one
    let mut name: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
    let mut value: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = htp_table::htp_table_size(input_params) as libc::c_int;
    while i < n {
        value =
            htp_table::htp_table_get_index(input_params, i as size_t, &mut name) as *mut bstr::bstr;
        let mut new_name: *mut bstr::bstr = 0 as *mut bstr::bstr;
        let mut new_value: *mut bstr::bstr = 0 as *mut bstr::bstr;
        // Convert name
        htp_transcode_bstr(cd, name, &mut new_name);
        if new_name.is_null() {
            iconv_close(cd);
            let mut b: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
            let mut j: libc::c_int = 0 as libc::c_int;
            let mut k: libc::c_int = htp_table::htp_table_size(output_params) as libc::c_int;
            while j < k {
                b = htp_table::htp_table_get_index(
                    output_params,
                    j as size_t,
                    0 as *mut *mut bstr::bstr,
                ) as *mut bstr::bstr;
                bstr::bstr_free(b);
                j += 1
            }
            htp_table::htp_table_destroy(output_params);
            return Status::ERROR;
        }
        // Convert value
        htp_transcode_bstr(cd, value, &mut new_value);
        if new_value.is_null() {
            bstr::bstr_free(new_name);
            iconv_close(cd);
            let mut b_0: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            let mut k_0: libc::c_int = htp_table::htp_table_size(output_params) as libc::c_int;
            while j_0 < k_0 {
                b_0 = htp_table::htp_table_get_index(
                    output_params,
                    j_0 as size_t,
                    0 as *mut *mut bstr::bstr,
                ) as *mut bstr::bstr;
                bstr::bstr_free(b_0);
                j_0 += 1
            }
            htp_table::htp_table_destroy(output_params);
            return Status::ERROR;
        }
        // Add to new table
        htp_table::htp_table_addn(output_params, new_name, new_value as *const libc::c_void);
        i += 1
    }
    // Replace the old parameter table
    *params = output_params;
    // Destroy the old parameter table if necessary
    if destroy_old != 0 {
        let mut b_1: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        let mut n_0: libc::c_int = htp_table::htp_table_size(input_params) as libc::c_int;
        while i_0 < n_0 {
            b_1 = htp_table::htp_table_get_index(
                input_params,
                i_0 as size_t,
                0 as *mut *mut bstr::bstr,
            ) as *mut bstr::bstr;
            bstr::bstr_free(b_1);
            i_0 += 1
        }
        htp_table::htp_table_destroy(input_params);
    }
    iconv_close(cd);
    return Status::OK;
}

/* *
 * Transcode one bstr.
 *
 * @param[in] cd
 * @param[in] input
 * @param[in] output
 */
#[no_mangle]
pub unsafe extern "C" fn htp_transcode_bstr(
    mut cd: iconv_t,
    mut input: *mut bstr::bstr_t,
    mut output: *mut *mut bstr::bstr_t,
) -> Status {
    // Reset conversion state for every new string
    iconv(
        cd,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
        0 as *mut *mut libc::c_char,
        0 as *mut size_t,
    );
    let mut bb: *mut bstr_builder::bstr_builder_t = 0 as *mut bstr_builder::bstr_builder_t;
    let buflen: size_t = 10 as libc::c_int as size_t;
    let mut buf: *mut libc::c_uchar = malloc(buflen) as *mut libc::c_uchar;
    if buf.is_null() {
        return Status::ERROR;
    }
    let mut inbuf: *const libc::c_char = if (*input).realptr.is_null() {
        (input as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr::bstr_t>() as libc::c_ulong as isize)
    } else {
        (*input).realptr
    } as *const libc::c_char;
    let mut inleft: size_t = (*input).len;
    let mut outbuf: *mut libc::c_char = buf as *mut libc::c_char;
    let mut outleft: size_t = buflen;
    let mut loop_0: libc::c_int = 1 as libc::c_int;
    while loop_0 != 0 {
        loop_0 = 0 as libc::c_int;
        if iconv(
            cd,
            &mut inbuf as *mut *const libc::c_char as *mut *mut libc::c_char,
            &mut inleft,
            &mut outbuf as *mut *mut libc::c_char,
            &mut outleft,
        ) == -(1 as libc::c_int) as size_t
        {
            if *__errno_location() == 7 as libc::c_int {
                // Create bstr::bstr_t builder on-demand
                if bb.is_null() {
                    bb = bstr_builder::bstr_builder_create();
                    if bb.is_null() {
                        free(buf as *mut libc::c_void);
                        return Status::ERROR;
                    }
                }
                // The output buffer is full
                bstr_builder::bstr_builder_append_mem(
                    bb,
                    buf as *const libc::c_void,
                    buflen.wrapping_sub(outleft),
                );
                outbuf = buf as *mut libc::c_char;
                outleft = buflen;
                // Continue in the loop, as there's more work to do
                loop_0 = 1 as libc::c_int
            } else {
                // Error
                if !bb.is_null() {
                    bstr_builder::bstr_builder_destroy(bb);
                }
                free(buf as *mut libc::c_void);
                return Status::ERROR;
            }
        }
    }
    if !bb.is_null() {
        bstr_builder::bstr_builder_append_mem(
            bb,
            buf as *const libc::c_void,
            buflen.wrapping_sub(outleft),
        );
        *output = bstr_builder::bstr_builder_to_str(bb);
        bstr_builder::bstr_builder_destroy(bb);
        if (*output).is_null() {
            free(buf as *mut libc::c_void);
            return Status::ERROR;
        }
    } else {
        *output = bstr::bstr_dup_mem(buf as *const libc::c_void, buflen.wrapping_sub(outleft));
        if (*output).is_null() {
            free(buf as *mut libc::c_void);
            return Status::ERROR;
        }
    }
    free(buf as *mut libc::c_void);
    return Status::OK;
}
