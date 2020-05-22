// Adapted from the libb64 project (http://sourceforge.net/projects/libb64), which is in public domain.

use crate::bstr;

extern "C" {
    #[no_mangle]
    fn malloc(_: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

pub type htp_base64_decodestep = u32;
pub const step_d: htp_base64_decodestep = 3;
pub const step_c: htp_base64_decodestep = 2;
pub const step_b: htp_base64_decodestep = 1;
pub const step_a: htp_base64_decodestep = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_base64_decoder {
    pub step: htp_base64_decodestep,
    pub plainchar: i8,
}

/// Decode single base64-encoded character.
///
/// Returns decoded character
pub unsafe fn htp_base64_decode_single(mut value_in: i8) -> i32 {
    static decoding: [i8; 80] = [
        62, -1, -1, -1, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -2, -1, -1, -1, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42,
        43, 44, 45, 46, 47, 48, 49, 50, 51,
    ];
    static decoding_size: i8 = ::std::mem::size_of::<[i8; 80]>() as i8;
    value_in = value_in - 43;
    if value_in < 0 || value_in > decoding_size - 1 {
        return -1;
    }
    return decoding[value_in as usize] as i32;
}

/// Initialize base64 decoder.
pub unsafe fn htp_base64_decoder_init(mut decoder: *mut htp_base64_decoder) {
    (*decoder).step = step_a;
    (*decoder).plainchar = 0;
}

/// Feed the supplied memory range to the decoder.
///
/// Returns how many bytes were placed into plaintext output
pub unsafe fn htp_base64_decode(
    mut decoder: *mut htp_base64_decoder,
    _code_in: *const core::ffi::c_void,
    length_in: i32,
    mut _plaintext_out: *mut core::ffi::c_void,
    mut length_out: i32,
) -> i32 {
    let code_in: *const u8 = _code_in as *const u8;
    let plaintext_out: *mut u8 = _plaintext_out as *mut u8;
    let mut codechar: *const u8 = code_in;
    let mut plainchar: *mut u8 = plaintext_out;
    let mut fragment: i8 = 0;
    if length_out <= 0 {
        return 0;
    }
    *plainchar = (*decoder).plainchar as u8;
    's_252: {
        let mut current_block_49: u64;
        match (*decoder).step {
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
                            (*decoder).plainchar = *plainchar as i8;
                            return plainchar.wrapping_offset_from(plaintext_out) as i32;
                        }
                        let fresh0 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh0 as i8) as i8;
                        if !((fragment) < 0) {
                            break;
                        }
                    }
                    *plainchar = ((fragment & 0x3f) << 2) as u8;
                    current_block_49 = 12054049053955448585;
                    // fall through
                }
                11653228657089272161 =>
                // fall through
                {
                    loop {
                        if codechar == code_in.offset(length_in as isize) {
                            (*decoder).step = step_c;
                            (*decoder).plainchar = *plainchar as i8;
                            return plainchar.wrapping_offset_from(plaintext_out) as i32;
                        }
                        let fresh3 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh3 as i8) as i8;
                        if !((fragment) < 0) {
                            break;
                        }
                    }
                    let fresh4 = plainchar;
                    plainchar = plainchar.offset(1);
                    *fresh4 = *fresh4 | ((fragment & 0x3c) >> 2) as u8;
                    *plainchar = ((fragment & 0x3) << 6) as u8;
                    length_out -= 1;
                    if length_out == 0 {
                        return plainchar.wrapping_offset_from(plaintext_out) as i32;
                    }
                    current_block_49 = 10238358027875099957;
                }
                12054049053955448585 =>
                // fall through
                {
                    loop {
                        if codechar == code_in.offset(length_in as isize) {
                            (*decoder).step = step_b;
                            (*decoder).plainchar = *plainchar as i8;
                            return plainchar.wrapping_offset_from(plaintext_out) as i32;
                        }
                        let fresh1 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh1 as i8) as i8;
                        if !((fragment) < 0) {
                            break;
                        }
                    }
                    let fresh2 = plainchar;
                    plainchar = plainchar.offset(1);
                    *fresh2 = *fresh2 | ((fragment & 0x30) >> 4) as u8;
                    *plainchar = ((fragment & 0xf) << 4) as u8;
                    length_out -= 1;
                    if length_out == 0 {
                        return plainchar.wrapping_offset_from(plaintext_out) as i32;
                    }
                    current_block_49 = 11653228657089272161;
                }
                _ =>
                // fall through
                {
                    loop {
                        if codechar == code_in.offset(length_in as isize) {
                            (*decoder).step = step_d;
                            (*decoder).plainchar = *plainchar as i8;
                            return plainchar.wrapping_offset_from(plaintext_out) as i32;
                        }
                        let fresh5 = codechar;
                        codechar = codechar.offset(1);
                        fragment = htp_base64_decode_single(*fresh5 as i8) as i8;
                        if !((fragment) < 0) {
                            break;
                        }
                    }
                    let fresh6 = plainchar;
                    plainchar = plainchar.offset(1);
                    *fresh6 = *fresh6 | (fragment & 0x3f) as u8;
                    length_out -= 1;
                    if length_out == 0 {
                        return plainchar.wrapping_offset_from(plaintext_out) as i32;
                    }
                    current_block_49 = 13382527123874078044;
                }
            }
        }
    }
    // control should not reach here
    return plainchar.wrapping_offset_from(plaintext_out) as i32;
}

/// Base64-decode input, given as memory range.
///
/// Returns new base64-decoded bstring
pub unsafe fn htp_base64_decode_mem(
    data: *const core::ffi::c_void,
    len: usize,
) -> *mut bstr::bstr_t {
    let mut decoder: htp_base64_decoder = htp_base64_decoder {
        step: step_a,
        plainchar: 0,
    };
    let mut r: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
    htp_base64_decoder_init(&mut decoder);
    let tmpstr: *mut u8 = malloc(len) as *mut u8;
    if tmpstr.is_null() {
        return 0 as *mut bstr::bstr_t;
    }
    let resulting_len: i32 = htp_base64_decode(
        &mut decoder,
        data,
        len as i32,
        tmpstr as *mut core::ffi::c_void,
        len as i32,
    );
    if resulting_len > 0 {
        r = bstr::bstr_dup_mem(tmpstr as *const core::ffi::c_void, resulting_len as usize)
    }
    free(tmpstr as *mut core::ffi::c_void);
    return r;
}
