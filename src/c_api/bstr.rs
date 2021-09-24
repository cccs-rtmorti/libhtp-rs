use crate::bstr::Bstr;
use core::cmp::Ordering;
use std::{boxed::Box, ffi::CStr};

/// Allocate a zero-length bstring, reserving space for at least size bytes.
#[no_mangle]
pub extern "C" fn bstr_alloc(len: libc::size_t) -> *mut Bstr {
    let b = Bstr::with_capacity(len);
    let boxed = Box::new(b);
    Box::into_raw(boxed)
}

/// Deallocate the supplied bstring instance. Allows NULL on input.
/// # Safety
/// This function is unsafe because improper use may lead to memory problems. For example, a double-free may occur if the function is called twice on the same raw pointer.
#[no_mangle]
pub unsafe extern "C" fn bstr_free(b: *mut Bstr) {
    if !b.is_null() {
        // b will be dropped when this box goes out of scope
        Box::from_raw(b);
    }
}

/// Return the length of the string
/// # Safety
/// x must be properly intialized: not NULL, dangling, or misaligned
#[no_mangle]
pub unsafe extern "C" fn bstr_len(x: *const Bstr) -> libc::size_t {
    (*x).len()
}

/// Return a pointer to the bstr payload
/// # Safety
/// x must be properly intialized: not NULL, dangling, or misaligned
#[no_mangle]
pub unsafe extern "C" fn bstr_ptr(x: *const Bstr) -> *mut libc::c_uchar {
    (*x).as_ptr() as *mut u8
}

/// Return the capacity of the string
/// # Safety
/// x must be properly intialized: not NULL, dangling, or misaligned
#[no_mangle]
pub unsafe extern "C" fn bstr_size(x: *const Bstr) -> libc::size_t {
    (*x).capacity()
}

/// Case-sensitive comparison of a bstring and a NUL-terminated string.
/// returns -1 if b is less than c
///          0 if b is equal to c
///          1 if b is greater than c
/// # Safety
/// b and c must be properly intialized: not NULL, dangling, or misaligned.
/// c must point to memory that contains a valid nul terminator byte at the end of the string
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_c(b: *const Bstr, c: *const libc::c_char) -> libc::c_int {
    let cs = CStr::from_ptr(c);
    match (*b).cmp_slice(cs.to_bytes()) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Create a new bstring by copying the provided NUL-terminated string
/// # Safety
/// cstr must be properly intialized: not NULL, dangling, or misaligned.
/// cstr must point to memory that contains a valid nul terminator byte at the end of the string
#[no_mangle]
pub unsafe extern "C" fn bstr_dup_c(cstr: *const libc::c_char) -> *mut Bstr {
    let cs = CStr::from_ptr(cstr).to_bytes();
    let new = bstr_alloc(cs.len());
    (*new).add(cs);
    new
}

/// Create a new NUL-terminated string out of the provided bstring. If NUL bytes
/// are contained in the bstring, each will be replaced with "\0" (two characters).
/// The caller is responsible to keep track of the allocated memory area and free
/// it once it is no longer needed.
/// returns The newly created NUL-terminated string, or NULL in case of memory
///         allocation failure.
/// # Safety
/// b must be properly intialized and not dangling nor misaligned.
#[no_mangle]
pub unsafe extern "C" fn bstr_util_strdup_to_c(b: *const Bstr) -> *mut libc::c_char {
    if b.is_null() {
        return std::ptr::null_mut();
    }
    let src = std::slice::from_raw_parts(bstr_ptr(b), bstr_len(b));

    // Since the memory returned here is just a char* and the caller will
    // free() it we have to use malloc() here.
    // So we allocate enough space for doubled NULL bytes plus the trailing NULL.
    let mut null_count = 1;
    for byte in src {
        if *byte == 0 {
            null_count += 1;
        }
    }
    let newlen = bstr_len(b) + null_count;
    let mem = libc::malloc(newlen) as *mut i8;
    if mem.is_null() {
        return std::ptr::null_mut();
    }
    let dst: &mut [i8] = std::slice::from_raw_parts_mut(mem, newlen);
    let mut dst_idx = 0;
    for byte in src {
        if *byte == 0 {
            dst[dst_idx] = '\\' as i8;
            dst_idx += 1;
            dst[dst_idx] = '0' as i8;
        } else {
            dst[dst_idx] = *byte as i8;
        }
        dst_idx += 1;
    }
    dst[dst_idx] = 0;

    mem
}

/// Convert contents of a memory region to a positive integer.
///
/// base: The desired number base.
/// lastlen: Points to the first unused byte in the region
///
/// returns If the conversion was successful, this function returns the
/// number. When the conversion fails, -1 will be returned when not
/// one valid digit was found, and -2 will be returned if an overflow
/// occurred.
/// # Safety
/// 'data' must be properly intialized: not null, dangling, or misaligned.
/// 'data' must be valid for read of length 'len'
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_to_pint(
    data: *const libc::c_void,
    len: libc::size_t,
    base: libc::c_int,
    lastlen: *mut libc::size_t,
) -> libc::c_long {
    // sanity check radix is in the convertable range
    // and will fit inside a u8
    if !(2..=36).contains(&base) {
        return -1;
    }

    // initialize out param
    *lastlen = 0;
    let mut rval: i64 = 0;

    // Make an open range [first, last) for the range of digits
    // and range of characters appropriate for this base
    let upper = base as u8;
    let search = if base <= 10 {
        ((b'0', b'0' + upper), (255, 0), (255, 0))
    } else {
        (
            (b'0', b'9'),
            (b'a', b'a' + upper - 10),
            (b'A', b'A' + upper - 10),
        )
    };

    let src = std::slice::from_raw_parts(data as *const u8, len);
    for b in src {
        match if (search.0).0 <= *b && *b < (search.0).1 {
            Some(*b - (search.0).0)
        } else if (search.1).0 <= *b && *b < (search.1).1 {
            Some(10 + *b - (search.1).0)
        } else if (search.2).0 <= *b && *b < (search.2).1 {
            Some(10 + *b - (search.2).0)
        } else {
            None
        } {
            None => return if *lastlen == 0 { -1 } else { rval },
            Some(d) => {
                *lastlen += 1;
                match rval.checked_mul(base as i64) {
                    None => return -2,
                    Some(new) => match new.checked_add(d as i64) {
                        None => return -2,
                        Some(new) => rval = new,
                    },
                }
            }
        }
    }
    *lastlen += 1;
    rval
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;

    macro_rules! cstr {
        ( $x:expr ) => {{
            CString::new($x).unwrap().as_ptr()
        }};
    }

    #[test]
    fn Bstr_Alloc() {
        unsafe {
            let p1: *mut Bstr;
            p1 = bstr_alloc(10);
            assert_eq!(10, bstr_size(p1));
            assert_eq!(0, bstr_len(p1));
            bstr_free(p1);
        }
    }

    #[test]
    fn Bstr_DupC() {
        unsafe {
            let p1 = bstr_dup_c(cstr!("arfarf"));

            assert_eq!(6, bstr_size(p1));
            assert_eq!(6, bstr_len(p1));
            assert_eq!(
                0,
                libc::memcmp(
                    cstr!("arfarf") as *const core::ffi::c_void,
                    bstr_ptr(p1) as *const core::ffi::c_void,
                    6
                )
            );
            bstr_free(p1);
        }
    }

    #[test]
    fn Bstr_UtilDupToC() {
        unsafe {
            let s = Bstr::from(b"ABCDEFGHIJKL\x00NOPQRST" as &[u8]);
            let c = bstr_util_strdup_to_c(&s);
            let e = CString::new("ABCDEFGHIJKL\\0NOPQRST").unwrap();
            assert_eq!(0, libc::strcmp(e.as_ptr(), c));

            libc::free(c as *mut core::ffi::c_void);
        }
    }

    #[test]
    fn Bstr_CmpC() {
        unsafe {
            let p1 = Bstr::from("arfarf");
            assert_eq!(0, bstr_cmp_c(&p1, cstr!("arfarf")));
            assert_eq!(-1, bstr_cmp_c(&p1, cstr!("arfarf2")));
            assert_eq!(1, bstr_cmp_c(&p1, cstr!("arf")));
            assert_eq!(-1, bstr_cmp_c(&p1, cstr!("not equal")));
        }
    }

    #[test]
    fn Bstr_ToPint() {
        unsafe {
            let mut lastlen: usize = 0;

            assert_eq!(
                -1,
                bstr_util_mem_to_pint(
                    cstr!("abc") as *const core::ffi::c_void,
                    3,
                    10,
                    &mut lastlen
                )
            );
            assert_eq!(
                -2,
                bstr_util_mem_to_pint(
                    cstr!("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
                        as *const core::ffi::c_void,
                    40,
                    16,
                    &mut lastlen
                )
            );
            assert_eq!(
                0x7fff_ffff_ffff_ffff,
                bstr_util_mem_to_pint(
                    cstr!("7fffffffffffffff") as *const core::ffi::c_void,
                    16,
                    16,
                    &mut lastlen
                )
            );
            assert_eq!(
                -2,
                bstr_util_mem_to_pint(
                    cstr!("9223372036854775808") as *const core::ffi::c_void,
                    19,
                    10,
                    &mut lastlen
                )
            );
            assert_eq!(
                -2,
                bstr_util_mem_to_pint(
                    cstr!("555555555555555555555555555555") as *const core::ffi::c_void,
                    30,
                    10,
                    &mut lastlen
                )
            );
            assert_eq!(
                0xabc,
                bstr_util_mem_to_pint(
                    cstr!("abc") as *const core::ffi::c_void,
                    3,
                    16,
                    &mut lastlen
                )
            );
            assert_eq!(4, lastlen);
            assert_eq!(
                0xabc,
                bstr_util_mem_to_pint(
                    cstr!("ABC") as *const core::ffi::c_void,
                    3,
                    16,
                    &mut lastlen
                )
            );
            assert_eq!(
                131,
                bstr_util_mem_to_pint(
                    cstr!("abc") as *const core::ffi::c_void,
                    3,
                    12,
                    &mut lastlen
                )
            );
            assert_eq!(2, lastlen);
            assert_eq!(
                83474,
                bstr_util_mem_to_pint(
                    cstr!("83474abc") as *const core::ffi::c_void,
                    8,
                    10,
                    &mut lastlen
                )
            );
            assert_eq!(5, lastlen);
            assert_eq!(
                5,
                bstr_util_mem_to_pint(
                    cstr!("0101") as *const core::ffi::c_void,
                    4,
                    2,
                    &mut lastlen
                )
            );
            assert_eq!(5, lastlen);
            assert_eq!(
                5,
                bstr_util_mem_to_pint(
                    cstr!("0101") as *const core::ffi::c_void,
                    4,
                    2,
                    &mut lastlen
                )
            );
            assert_eq!(5, lastlen);
        }
    }
}
