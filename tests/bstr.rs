#![allow(non_snake_case)]
use htp::bstr::*;
use htp::bstr_builder::*;
use libc;
use std::ffi::CString;

// import common testing utilities
mod common;

#[test]
fn Bstr_Alloc() {
    unsafe {
        let p1: *mut bstr_t;
        p1 = bstr_alloc(10);
        assert_eq!(10, bstr_size(p1));
        assert_eq!(0, bstr_len(p1));
        bstr_free(p1);
    }
}

#[test]
fn Bstr_ExpandLocal() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;

        p1 = bstr_alloc(10);
        p2 = bstr_expand(p1, 100);
        assert!(!p2.is_null());
        assert_eq!(100, bstr_size(p2));
        assert_eq!(0, bstr_len(p2));

        bstr_free(p2);
    }
}

#[test]
fn Bstr_ExpandSmaller() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;

        p1 = bstr_alloc(100);
        p2 = bstr_expand(p1, 10);
        assert!(p2.is_null());

        bstr_free(p1);
    }
}

/*
// For the time being, expansion is not allowed
// when data is externally stored. This feature
// is currently only used when wrapping existing
// memory areas.
#[test]
fn Bstr_ExpandPtr() {
unsafe{
    let b: *mut bstr_t;
    b = (bstr*) libc::malloc(std::mem::size_of::<bstr_t>());
    assert!(!b.is_null());
    (*b).ptr = (unsigned char*) libc::malloc(10);
    (*b).len = 0;
    (*b).size = 10;
    assert!(!bstr_ptr(b).is_null());

    let p2: *mut bstr_t;
    p2 = bstr_expand(b, 100);
    EXPECT_TRUE(p2 != NULL);
    assert_eq!(100, bstr_size(p2));
    assert_eq!(0, bstr_len(p2));

    free((*p2).ptr);
    bstr_free(p2);
}}
*/

#[test]
fn Bstr_DupC() {
    unsafe {
        let p1: *mut bstr_t;
        p1 = bstr_dup_c(cstr!("arfarf"));

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
fn Bstr_DupStr() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_str("s0123456789abcdefghijklmnopqrstuvwxyz");
        p2 = bstr_dup(p1);

        assert_eq!(bstr_len(p1), bstr_len(p2));
        assert_eq!(
            0,
            libc::memcmp(
                bstr_ptr(p1) as *const core::ffi::c_void,
                bstr_ptr(p2) as *const core::ffi::c_void,
                bstr_len(p1)
            )
        );

        bstr_free(p1);
        bstr_free(p2);
    }
}

#[test]
fn Bstr_DupBin() {
    unsafe {
        let src: *mut bstr_t = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );
        let dst: *mut bstr_t;
        dst = bstr_dup(src);

        assert_eq!(bstr_len(src), bstr_len(dst));
        assert_eq!(
            0,
            libc::memcmp(
                bstr_ptr(src) as *const core::ffi::c_void,
                bstr_ptr(dst) as *const core::ffi::c_void,
                bstr_len(src)
            )
        );

        bstr_free(src);
        bstr_free(dst);
    }
}

#[test]
fn Bstr_DupEx() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_str("0123456789abcdefghijkl");
        p2 = bstr_dup_ex(p1, 4, 10);

        assert_eq!(10, bstr_size(p2));
        assert_eq!(10, bstr_len(p2));
        assert_eq!(
            0,
            libc::memcmp(
                cstr!("456789abcd") as *const core::ffi::c_void,
                bstr_ptr(p2) as *const core::ffi::c_void,
                10
            )
        );

        bstr_free(p1);
        bstr_free(p2);
    }
}

#[test]
fn Bstr_DupMem() {
    unsafe {
        let dst: *mut bstr_t;
        dst = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            18,
        );
        assert_eq!(
            0,
            libc::memcmp(
                b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
                bstr_ptr(dst) as *const core::ffi::c_void,
                18
            )
        );

        bstr_free(dst);
    }
}

#[test]
fn Bstr_DupLower() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_str("0123456789ABCDEFGhIJKL");
        p2 = bstr_dup_lower(p1);

        assert_eq!(
            0,
            libc::memcmp(
                cstr!("0123456789abcdefghijkl") as *const core::ffi::c_void,
                bstr_ptr(p2) as *const core::ffi::c_void,
                22
            )
        );

        bstr_free(p1);
        bstr_free(p2);
    }
}

#[test]
fn Bstr_ChrRchr() {
    unsafe {
        let p1: *mut bstr_t = bstr_dup_str("0123456789abcdefghijklmnopqrstuvwxyz");
        assert_eq!(13, bstr_chr(p1, b'd' as i32));
        assert_eq!(-1, bstr_chr(p1, b'?' as i32));
        assert_eq!(13, bstr_chr(p1, b'd' as i32));
        assert_eq!(-1, bstr_chr(p1, b'?' as i32));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_Cmp() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        let p3: *mut bstr_t;
        let p4: *mut bstr_t;
        p1 = bstr_dup_str("arfarf");
        p2 = bstr_dup_str("arfarf");
        p3 = bstr_dup_str("arfArf");
        p4 = bstr_dup_str("arfarf2");

        assert_eq!(0, bstr_cmp(p1, p1));
        assert_eq!(0, bstr_cmp(p1, p2));
        assert_eq!(0, bstr_cmp(p2, p1));
        assert_eq!(1, bstr_cmp(p1, p3));
        assert_eq!(-1, bstr_cmp(p3, p1));
        assert_eq!(-1, bstr_cmp(p1, p4));
        assert_eq!(1, bstr_cmp(p4, p1));

        bstr_free(p1);
        bstr_free(p2);
        bstr_free(p3);
        bstr_free(p4);
    }
}

#[test]
fn Bstr_CmpNocase() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        let p3: *mut bstr_t;
        p1 = bstr_dup_str("arfarf");
        p2 = bstr_dup_str("arfarf");
        p3 = bstr_dup_str("arfArf");

        assert_eq!(0, bstr_cmp_nocase(p1, p1));
        assert_eq!(0, bstr_cmp_nocase(p1, p2));
        assert_eq!(0, bstr_cmp_nocase(p2, p1));
        assert_eq!(0, bstr_cmp_nocase(p1, p3));
        assert_eq!(0, bstr_cmp_nocase(p3, p1));

        bstr_free(p1);
        bstr_free(p2);
        bstr_free(p3);
    }
}

#[test]
fn Bstr_CmpC() {
    unsafe {
        let p1: *mut bstr_t;
        p1 = bstr_dup_str("arfarf");
        assert_eq!(0, bstr_cmp_c(p1, cstr!("arfarf")));
        assert_eq!(-1, bstr_cmp_c(p1, cstr!("arfarf2")));
        assert_eq!(1, bstr_cmp_c(p1, cstr!("arf")));
        assert_eq!(-1, bstr_cmp_c(p1, cstr!("not equal")));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_CmpStr() {
    unsafe {
        let p1: *mut bstr_t;
        p1 = bstr_dup_str("arfarf");
        assert_eq!(0, bstr_cmp_str(p1, "arfarf"));
        assert_eq!(-1, bstr_cmp_str(p1, "arfarf2"));
        assert_eq!(1, bstr_cmp_str(p1, "arf"));
        assert_eq!(-1, bstr_cmp_str(p1, "not equal"));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_CmpStrNocase() {
    unsafe {
        let p1: *mut bstr_t;
        p1 = bstr_dup_str("arfarf");
        assert_eq!(0, bstr_cmp_str_nocase(p1, "arfarf"));
        assert_eq!(0, bstr_cmp_str_nocase(p1, "arfARF"));
        assert_eq!(1, bstr_cmp_str_nocase(p1, "ArF"));
        assert_eq!(-1, bstr_cmp_str_nocase(p1, "Not equal"));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_ToLowercase() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_str("aRf3ArF");
        p2 = bstr_to_lowercase(p1);

        assert_eq!(p1, p2);
        assert_eq!(1, bstr_cmp_str(p1, "aRf3ArF"));
        assert_eq!(0, bstr_cmp_str(p1, "arf3arf"));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_AddMem() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_str("testtest");
        p2 = bstr_add_mem(p1, cstr!("12345678") as *const core::ffi::c_void, 4);

        assert_eq!(0, bstr_cmp_str(p2, "testtest1234"));

        bstr_free(p2);
    }
}

#[test]
fn Bstr_AddNoex() {
    unsafe {
        let mut p1: *mut bstr_t;
        let p2: *mut bstr_t;
        let p3: *mut bstr_t;
        p1 = bstr_alloc(10);
        p1 = bstr_add_c_noex(p1, cstr!("12345"));
        p2 = bstr_dup_str("abcdef");
        p3 = bstr_add_noex(p1, p2);

        assert_eq!(p1, p3);
        assert_eq!(0, bstr_cmp_str(p3, "12345abcde"));
        bstr_free(p1);
        bstr_free(p2);
    }
}

#[test]
fn Bstr_AddCNoex() {
    unsafe {
        let mut p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_alloc(10);
        p1 = bstr_add_c_noex(p1, cstr!("12345"));
        p2 = bstr_add_c_noex(p1, cstr!("abcdefghijk"));

        assert_eq!(p1, p2);
        assert_eq!(0, bstr_cmp_str(p2, "12345abcde"));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_CharAtEnd() {
    unsafe {
        let str: *mut bstr_t = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );
        assert_eq!(b'T' as i32, bstr_char_at_end(str, 0));
        assert_eq!(0, bstr_char_at_end(str, 7));
        assert_eq!(-1, bstr_char_at_end(str, bstr_len(str)));

        bstr_free(str);
    }
}

#[test]
fn Bstr_Chop() {
    unsafe {
        let p1: *mut bstr_t = bstr_dup_str("abcdef");
        let p2: *mut bstr_t = bstr_alloc(10);
        bstr_chop(p1);
        assert_eq!(0, bstr_cmp_str(p1, "abcde"));

        bstr_chop(p2);
        assert_eq!(0, bstr_len(p2));

        bstr_free(p1);
        bstr_free(p2);
    }
}

#[test]
fn Bstr_AdjustLen() {
    unsafe {
        let p1: *mut bstr_t = bstr_dup_str("abcdef");

        bstr_adjust_len(p1, 3);
        assert_eq!(3, bstr_len(p1));
        assert_eq!(0, bstr_cmp_str(p1, "abc"));

        bstr_free(p1);
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

#[test]
fn Bstr_DupToC() {
    unsafe {
        let c: *mut i8;
        let str: *mut bstr_t = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );

        c = bstr_util_strdup_to_c(str);
        assert_eq!(0, libc::strcmp(cstr!("ABCDEFGHIJKL\\0NOPQRST"), c));

        libc::free(c as *mut core::ffi::c_void);
        bstr_free(str);
    }
}

#[test]
fn Bstr_UtilMemTrim() {
    unsafe {
        let d = CString::new(" \r\t0123456789\x0c\x0b  ").unwrap();
        let mut data: *mut i8 = d.as_ptr() as *mut i8;
        let data_ptr: *mut *mut i8 = &mut data;
        let mut len: usize = libc::strlen(data);

        bstr_util_mem_trim(data_ptr as *mut *mut u8, &mut len);

        let src = std::slice::from_raw_parts(data as *const u8, len);
        let b = bstr_t::from("0123456789");
        assert!(b.eq(src));
    }
}

#[test]
fn Bstr_CreateDestroy() {
    unsafe {
        let bb: *mut bstr_builder_t = bstr_builder_create();
        assert_eq!(0, bstr_builder_size(bb));
        bstr_builder_destroy(bb);
    }
}

#[test]
fn Bstr_Append() {
    unsafe {
        let bb: *mut bstr_builder_t = bstr_builder_create();

        assert_eq!(0, bstr_builder_size(bb));

        bstr_builder_append_mem(bb, cstr!("!@#$%^&*()") as *const core::ffi::c_void, 4);

        assert_eq!(1, bstr_builder_size(bb));

        let result: *mut bstr_t = bstr_builder_to_str(bb);
        assert_eq!(4, bstr_len(result));

        assert_eq!(
            0,
            libc::memcmp(
                cstr!("!@#$") as *const core::ffi::c_void,
                bstr_ptr(result) as *const core::ffi::c_void,
                4
            )
        );
        bstr_free(result);

        bstr_builder_clear(bb);
        assert_eq!(0, bstr_builder_size(bb));

        bstr_builder_destroy(bb);
    }
}
