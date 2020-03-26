/* Adapted from the libb64 project (http://sourceforge.net/projects/libb64), which is in public domain. */
use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
}

pub type size_t = libc::c_ulong;
pub type bstr = crate::src::bstr::bstr_t;
pub type htp_base64_decodestep = libc::c_uint;
pub const step_d: htp_base64_decodestep = 3;
pub const step_c: htp_base64_decodestep = 2;
pub const step_b: htp_base64_decodestep = 1;
pub const step_a: htp_base64_decodestep = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_base64_decoder {
    pub step: htp_base64_decodestep,
    pub plainchar: libc::c_char,
}

/* *
 * Decode single base64-encoded character.
 *
 * @param[in] value_in
 * @return decoded character
 */
#[no_mangle]
pub unsafe extern "C" fn htp_base64_decode_single(mut value_in: libc::c_schar) -> libc::c_int {
    static mut decoding: [libc::c_schar; 80] = [
        62 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        63 as libc::c_int as libc::c_schar,
        52 as libc::c_int as libc::c_schar,
        53 as libc::c_int as libc::c_schar,
        54 as libc::c_int as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        56 as libc::c_int as libc::c_schar,
        57 as libc::c_int as libc::c_schar,
        58 as libc::c_int as libc::c_schar,
        59 as libc::c_int as libc::c_schar,
        60 as libc::c_int as libc::c_schar,
        61 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        14 as libc::c_int as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
        16 as libc::c_int as libc::c_schar,
        17 as libc::c_int as libc::c_schar,
        18 as libc::c_int as libc::c_schar,
        19 as libc::c_int as libc::c_schar,
        20 as libc::c_int as libc::c_schar,
        21 as libc::c_int as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        24 as libc::c_int as libc::c_schar,
        25 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        28 as libc::c_int as libc::c_schar,
        29 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        32 as libc::c_int as libc::c_schar,
        33 as libc::c_int as libc::c_schar,
        34 as libc::c_int as libc::c_schar,
        35 as libc::c_int as libc::c_schar,
        36 as libc::c_int as libc::c_schar,
        37 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        40 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        43 as libc::c_int as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
        45 as libc::c_int as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        47 as libc::c_int as libc::c_schar,
        48 as libc::c_int as libc::c_schar,
        49 as libc::c_int as libc::c_schar,
        50 as libc::c_int as libc::c_schar,
        51 as libc::c_int as libc::c_schar,
    ];
    static mut decoding_size: libc::c_schar =
        ::std::mem::size_of::<[libc::c_schar; 80]>() as libc::c_ulong as libc::c_schar;
    value_in = (value_in as libc::c_int - 43 as libc::c_int) as libc::c_schar;
    if (value_in as libc::c_int) < 0 as libc::c_int
        || value_in as libc::c_int > decoding_size as libc::c_int - 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return decoding[value_in as libc::c_int as usize] as libc::c_int;
}

/* *
 * Initialize base64 decoder.
 *
 * @param[in] decoder
 */
#[no_mangle]
pub unsafe extern "C" fn htp_base64_decoder_init(mut decoder: *mut htp_base64_decoder) {
    (*decoder).step = step_a;
    (*decoder).plainchar = 0 as libc::c_int as libc::c_char;
}

/* *
 * Feed the supplied memory range to the decoder.
 *
 * @param[in] decoder
 * @param[in] _code_in
 * @param[in] length_in
 * @param[in] _plaintext_out
 * @param[in] length_out
 * @return how many bytes were placed into plaintext output
 */
#[no_mangle]
pub unsafe extern "C" fn htp_base64_decode(
    mut decoder: *mut htp_base64_decoder,
    mut _code_in: *const libc::c_void,
    mut length_in: libc::c_int,
    mut _plaintext_out: *mut libc::c_void,
    mut length_out: libc::c_int,
) -> libc::c_int {
    let mut code_in: *const libc::c_uchar = _code_in as *const libc::c_uchar;
    let mut plaintext_out: *mut libc::c_uchar = _plaintext_out as *mut libc::c_uchar;
    let mut codechar: *const libc::c_uchar = code_in;
    let mut plainchar: *mut libc::c_uchar = plaintext_out;
    let mut fragment: libc::c_schar = 0;
    if length_out <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    *plainchar = (*decoder).plainchar as libc::c_uchar;
    's_252: {
        let mut current_block_49: u64;
        match (*decoder).step as libc::c_uint {
            0 => {
                current_block_49 = 13382527123874078044;
            }
            1 => {
                current_block_49 = 12054049053955448585;
            }
            2 => {
                current_block_49 = 11653228657089272161;
            }
            3 => {
                current_block_49 = 10238358027875099957;
            }
            _ => {
                break 's_252;
            }
        }
        loop {
            match current_block_49 {
                13382527123874078044 => {
                    loop {
                        if codechar == code_in.offset(length_in as isize) {
                            (*decoder).step = step_a;
                            (*decoder).plainchar = *plainchar as libc::c_char;
                            return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long
                                as libc::c_int;
                        }
                        let fresh0 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh0 as libc::c_schar)
                            as libc::c_char as libc::c_schar;
                        if !((fragment as libc::c_int) < 0 as libc::c_int) {
                            break;
                        }
                    }
                    *plainchar = ((fragment as libc::c_int & 0x3f as libc::c_int)
                        << 2 as libc::c_int) as libc::c_uchar;
                    current_block_49 = 12054049053955448585;
                    /* fall through */
                }
                11653228657089272161 =>
                /* fall through */
                {
                    loop {
                        if codechar == code_in.offset(length_in as isize) {
                            (*decoder).step = step_c;
                            (*decoder).plainchar = *plainchar as libc::c_char;
                            return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long
                                as libc::c_int;
                        }
                        let fresh3 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh3 as libc::c_schar)
                            as libc::c_char as libc::c_schar;
                        if !((fragment as libc::c_int) < 0 as libc::c_int) {
                            break;
                        }
                    }
                    let fresh4 = plainchar;
                    plainchar = plainchar.offset(1);
                    *fresh4 = (*fresh4 as libc::c_int
                        | (fragment as libc::c_int & 0x3c as libc::c_int) >> 2 as libc::c_int)
                        as libc::c_uchar;
                    *plainchar = ((fragment as libc::c_int & 0x3 as libc::c_int)
                        << 6 as libc::c_int) as libc::c_uchar;
                    length_out -= 1;
                    if length_out == 0 as libc::c_int {
                        return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long
                            as libc::c_int;
                    }
                    current_block_49 = 10238358027875099957;
                }
                12054049053955448585 =>
                /* fall through */
                {
                    loop {
                        if codechar == code_in.offset(length_in as isize) {
                            (*decoder).step = step_b;
                            (*decoder).plainchar = *plainchar as libc::c_char;
                            return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long
                                as libc::c_int;
                        }
                        let fresh1 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh1 as libc::c_schar)
                            as libc::c_char as libc::c_schar;
                        if !((fragment as libc::c_int) < 0 as libc::c_int) {
                            break;
                        }
                    }
                    let fresh2 = plainchar;
                    plainchar = plainchar.offset(1);
                    *fresh2 = (*fresh2 as libc::c_int
                        | (fragment as libc::c_int & 0x30 as libc::c_int) >> 4 as libc::c_int)
                        as libc::c_uchar;
                    *plainchar = ((fragment as libc::c_int & 0xf as libc::c_int)
                        << 4 as libc::c_int) as libc::c_uchar;
                    length_out -= 1;
                    if length_out == 0 as libc::c_int {
                        return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long
                            as libc::c_int;
                    }
                    current_block_49 = 11653228657089272161;
                }
                _ =>
                /* fall through */
                {
                    loop {
                        if codechar == code_in.offset(length_in as isize) {
                            (*decoder).step = step_d;
                            (*decoder).plainchar = *plainchar as libc::c_char;
                            return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long
                                as libc::c_int;
                        }
                        let fresh5 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh5 as libc::c_schar)
                            as libc::c_char as libc::c_schar;
                        if !((fragment as libc::c_int) < 0 as libc::c_int) {
                            break;
                        }
                    }
                    let fresh6 = plainchar;
                    plainchar = plainchar.offset(1);
                    *fresh6 = (*fresh6 as libc::c_int
                        | fragment as libc::c_int & 0x3f as libc::c_int)
                        as libc::c_uchar;
                    length_out -= 1;
                    if length_out == 0 as libc::c_int {
                        return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long
                            as libc::c_int;
                    }
                    current_block_49 = 13382527123874078044;
                }
            }
        }
    }
    /* control should not reach here */
    return plainchar.wrapping_offset_from(plaintext_out) as libc::c_long as libc::c_int;
}

/* *
 * Base64-decode input, given as bstring.
 *
 * @param[in] input
 * @return new base64-decoded bstring
 */
#[no_mangle]
pub unsafe extern "C" fn htp_base64_decode_bstr(mut input: *mut bstr) -> *mut bstr {
    return htp_base64_decode_mem(
        if (*input).realptr.is_null() {
            (input as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*input).realptr
        } as *const libc::c_void,
        (*input).len,
    );
}

/* *
 * Base64-decode input, given as memory range.
 *
 * @param[in] data
 * @param[in] len
 * @return new base64-decoded bstring
 */
#[no_mangle]
pub unsafe extern "C" fn htp_base64_decode_mem(
    mut data: *const libc::c_void,
    mut len: size_t,
) -> *mut bstr {
    let mut decoder: htp_base64_decoder = htp_base64_decoder {
        step: step_a,
        plainchar: 0,
    };
    let mut r: *mut bstr = 0 as *mut bstr;
    htp_base64_decoder_init(&mut decoder);
    let mut tmpstr: *mut libc::c_uchar = malloc(len) as *mut libc::c_uchar;
    if tmpstr.is_null() {
        return 0 as *mut bstr;
    }
    let mut resulting_len: libc::c_int = htp_base64_decode(
        &mut decoder,
        data,
        len as libc::c_int,
        tmpstr as *mut libc::c_void,
        len as libc::c_int,
    );
    if resulting_len > 0 as libc::c_int {
        r = bstr_dup_mem(tmpstr as *const libc::c_void, resulting_len as size_t)
    }
    free(tmpstr as *mut libc::c_void);
    return r;
}
