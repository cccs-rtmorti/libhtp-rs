#![allow(non_snake_case)]
use htp::bstr::*;
use htp::bstr_builder::*;
use libc;
use std::ffi::CString;

macro_rules! cstr {
    ( $x:expr ) => {{
        CString::new($x).unwrap().as_ptr()
    }};
}

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

#[test]
fn Bstr_ExpandPtr() {
    unsafe {
        let b: *mut bstr_t;
        b = libc::malloc(std::mem::size_of::<bstr_t>()) as *mut bstr_t;
        assert!(!b.is_null());
        (*b).realptr = libc::malloc(10) as *mut libc::c_uchar;
        (*b).len = 0;
        (*b).size = 10;
        assert!(!bstr_ptr(b).is_null());

        let p2: *mut bstr_t = bstr_expand(b, 100);
        assert!(p2.is_null());

        libc::free((*b).realptr as *mut core::ffi::c_void);
        bstr_free(b);
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
        p1 = bstr_dup_c(cstr!("s0123456789abcdefghijklmnopqrstuvwxyz"));
        p2 = bstr_dup(p1);

        assert_eq!(bstr_len(p1), bstr_len(p2));
        assert_eq!(
            0,
            libc::memcmp(
                bstr_ptr(p1) as *const core::ffi::c_void,
                bstr_ptr(p2) as *const core::ffi::c_void,
                bstr_len(p1) as usize
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
                bstr_len(src) as usize
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
        p1 = bstr_dup_c(cstr!("0123456789abcdefghijkl"));
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
        p1 = bstr_dup_c(cstr!("0123456789ABCDEFGhIJKL"));
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
        let p1: *mut bstr_t = bstr_dup_c(cstr!("0123456789abcdefghijklmnopqrstuvwxyz"));
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
        p1 = bstr_dup_c(cstr!("arfarf"));
        p2 = bstr_dup_c(cstr!("arfarf"));
        p3 = bstr_dup_c(cstr!("arfArf"));
        p4 = bstr_dup_c(cstr!("arfarf2"));

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
        p1 = bstr_dup_c(cstr!("arfarf"));
        p2 = bstr_dup_c(cstr!("arfarf"));
        p3 = bstr_dup_c(cstr!("arfArf"));

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
        p1 = bstr_dup_c(cstr!("arfarf"));
        assert_eq!(0, bstr_cmp_c(p1, cstr!("arfarf")));
        assert_eq!(-1, bstr_cmp_c(p1, cstr!("arfarf2")));
        assert_eq!(1, bstr_cmp_c(p1, cstr!("arf")));
        assert_eq!(-1, bstr_cmp_c(p1, cstr!("not equal")));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_CmpCNocase() {
    unsafe {
        let p1: *mut bstr_t;
        p1 = bstr_dup_c(cstr!("arfarf"));
        assert_eq!(0, bstr_cmp_c_nocase(p1, cstr!("arfarf")));
        assert_eq!(0, bstr_cmp_c_nocase(p1, cstr!("arfARF")));
        assert_eq!(1, bstr_cmp_c_nocase(p1, cstr!("ArF")));
        assert_eq!(-1, bstr_cmp_c_nocase(p1, cstr!("Not equal")));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_CmpEx() {
    unsafe {
        let s1 = CString::new("arfarf12345").unwrap();
        let s2 = CString::new("arfarF2345").unwrap();

        assert_eq!(
            0,
            bstr_util_cmp_mem(
                s1.as_ptr() as *const core::ffi::c_void,
                5,
                s2.as_ptr() as *const core::ffi::c_void,
                5
            )
        );
        assert_eq!(
            1,
            bstr_util_cmp_mem(
                s1.as_ptr() as *const core::ffi::c_void,
                6,
                s2.as_ptr() as *const core::ffi::c_void,
                6
            )
        );
        assert_eq!(
            1,
            bstr_util_cmp_mem(
                s1.as_ptr() as *const core::ffi::c_void,
                5,
                s2.as_ptr() as *const core::ffi::c_void,
                4
            )
        );
        assert_eq!(
            -1,
            bstr_util_cmp_mem(
                s2.as_ptr() as *const core::ffi::c_void,
                4,
                s1.as_ptr() as *const core::ffi::c_void,
                5
            )
        );
    }
}

#[test]
fn Bstr_CmpNocaseEx() {
    unsafe {
        let s1 = CString::new("arfarf12345").unwrap();
        let s2 = CString::new("arfarF2345").unwrap();

        assert_eq!(
            0,
            bstr_util_cmp_mem_nocase(
                s1.as_ptr() as *const core::ffi::c_void,
                6,
                s2.as_ptr() as *const core::ffi::c_void,
                6
            )
        );
        assert_eq!(
            1,
            bstr_util_cmp_mem_nocase(
                s1.as_ptr() as *const core::ffi::c_void,
                6,
                s2.as_ptr() as *const core::ffi::c_void,
                5
            )
        );
        assert_eq!(
            -1,
            bstr_util_cmp_mem_nocase(
                s2.as_ptr() as *const core::ffi::c_void,
                5,
                s1.as_ptr() as *const core::ffi::c_void,
                6
            )
        );
    }
}

#[test]
fn Bstr_CmpMem() {
    unsafe {
        let s: *mut bstr_t = bstr_dup_c(cstr!("arfArf"));
        assert_eq!(
            0,
            bstr_cmp_mem(s, cstr!("arfArf") as *const core::ffi::c_void, 6)
        );
        bstr_free(s);
    }
}

#[test]
fn Bstr_ToLowercase() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_c(cstr!("aRf3ArF"));
        p2 = bstr_to_lowercase(p1);

        assert_eq!(p1, p2);
        assert_eq!(1, bstr_cmp_c(p1, cstr!("aRf3ArF")));
        assert_eq!(0, bstr_cmp_c(p1, cstr!("arf3arf")));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_Add() {
    unsafe {
        let src1: *mut bstr_t;
        let src2: *mut bstr_t;
        let dest: *mut bstr_t;

        src1 = bstr_dup_c(cstr!("testtest"));
        src2 = bstr_dup_c(cstr!("0123456789abcdefghijklmnopqrstuvwxyz"));
        dest = bstr_add(src1, src2);

        assert_eq!(
            0,
            bstr_cmp_c(dest, cstr!("testtest0123456789abcdefghijklmnopqrstuvwxyz"))
        );

        // src1 is either invalid or the same as dest after bstr_add
        bstr_free(src2);
        bstr_free(dest);
    }
}

#[test]
fn Bstr_AddC() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_c(cstr!("testtest"));
        p2 = bstr_add_c(p1, cstr!("1234"));

        assert_eq!(0, bstr_cmp_c(p2, cstr!("testtest1234")));

        bstr_free(p2);
    }
}

#[test]
fn Bstr_AddMem() {
    unsafe {
        let p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_dup_c(cstr!("testtest"));
        p2 = bstr_add_mem(p1, cstr!("12345678") as *const core::ffi::c_void, 4);

        assert_eq!(0, bstr_cmp_c(p2, cstr!("testtest1234")));

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
        p1 = bstr_add_c(p1, cstr!("12345"));
        p2 = bstr_dup_c(cstr!("abcdef"));
        p3 = bstr_add_noex(p1, p2);

        assert_eq!(p1, p3);
        assert_eq!(0, bstr_cmp_c(p3, cstr!("12345abcde")));
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
        p1 = bstr_add_c(p1, cstr!("12345"));
        p2 = bstr_add_c_noex(p1, cstr!("abcdefghijk"));

        assert_eq!(p1, p2);
        assert_eq!(0, bstr_cmp_c(p2, cstr!("12345abcde")));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_AddMemNoex() {
    unsafe {
        let mut p1: *mut bstr_t;
        let p2: *mut bstr_t;
        p1 = bstr_alloc(10);
        p1 = bstr_add_c(p1, cstr!("12345"));
        p2 = bstr_add_mem_noex(p1, cstr!("abcdefghijklmnop") as *const core::ffi::c_void, 6);

        assert_eq!(p1, p2);
        assert_eq!(0, bstr_cmp_c(p2, cstr!("12345abcde")));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_IndexOf() {
    unsafe {
        let haystack: *mut bstr_t = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );
        let p1: *mut bstr_t = bstr_dup_c(cstr!("NOPQ"));
        let p2: *mut bstr_t = bstr_dup_c(cstr!("siej"));
        let p3: *mut bstr_t = bstr_dup_c(cstr!("TUVWXYZ"));
        let p4: *mut bstr_t = bstr_dup_c(cstr!("nopq"));
        assert_eq!(13, bstr_index_of(haystack, p1));
        assert_eq!(-1, bstr_index_of(haystack, p2));
        assert_eq!(-1, bstr_index_of(haystack, p3));

        assert_eq!(-1, bstr_index_of(haystack, p4));
        assert_eq!(13, bstr_index_of_nocase(haystack, p4));

        assert_eq!(16, bstr_index_of_c(haystack, cstr!("QRS")));
        assert_eq!(-1, bstr_index_of_c(haystack, cstr!("qrs")));
        assert_eq!(16, bstr_index_of_c_nocase(haystack, cstr!("qrs")));

        assert_eq!(
            16,
            bstr_index_of_mem(haystack, cstr!("QRSSDF") as *const core::ffi::c_void, 3)
        );
        assert_eq!(
            -1,
            bstr_index_of_mem(haystack, cstr!("qrssdf") as *const core::ffi::c_void, 3)
        );
        assert_eq!(
            16,
            bstr_index_of_mem_nocase(haystack, cstr!("qrssdf") as *const core::ffi::c_void, 3)
        );

        bstr_free(p1);
        bstr_free(p2);
        bstr_free(p3);
        bstr_free(p4);
        bstr_free(haystack);
    }
}

#[test]
fn Bstr_MemIndexOf() {
    unsafe {
        assert_eq!(
            0,
            bstr_util_mem_index_of_c(
                b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
                20,
                cstr!("ABC")
            )
        );
        assert_eq!(
            -1,
            bstr_util_mem_index_of_c(
                b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
                20,
                cstr!("ABD")
            )
        );
        assert_eq!(
            -1,
            bstr_util_mem_index_of_c(
                b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
                20,
                cstr!("CBA")
            )
        );
    }
}

#[test]
fn Bstr_BeginsWith() {
    unsafe {
        let haystack: *mut bstr_t = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );
        let p1: *mut bstr_t = bstr_dup_c(cstr!("ABCD"));
        let p2: *mut bstr_t = bstr_dup_c(cstr!("aBcD"));

        assert_eq!(1, bstr_begins_with(haystack, p1));
        assert_ne!(1, bstr_begins_with(haystack, p2));
        assert_eq!(1, bstr_begins_with_nocase(haystack, p2));

        assert_eq!(1, bstr_begins_with_c(haystack, cstr!("AB")));
        assert_ne!(1, bstr_begins_with_c(haystack, cstr!("ab")));
        assert_eq!(1, bstr_begins_with_c_nocase(haystack, cstr!("ab")));

        assert_eq!(
            1,
            bstr_begins_with_mem(haystack, cstr!("ABq") as *const core::ffi::c_void, 2)
        );
        assert_ne!(
            1,
            bstr_begins_with_mem(haystack, cstr!("abq") as *const core::ffi::c_void, 2)
        );
        assert_eq!(
            1,
            bstr_begins_with_mem_nocase(haystack, cstr!("abq") as *const core::ffi::c_void, 2)
        );

        bstr_free(p1);
        bstr_free(p2);
        bstr_free(haystack);
    }
}

#[test]
fn Bstr_BeginsWith2() {
    unsafe {
        let haystack: *mut bstr_t = bstr_dup_c(cstr!("ABC"));
        let p1: *mut bstr_t = bstr_dup_c(cstr!("ABCD"));
        let p2: *mut bstr_t = bstr_dup_c(cstr!("EDFG"));

        assert_eq!(
            0,
            bstr_begins_with_mem(
                haystack,
                bstr_ptr(p1) as *const core::ffi::c_void,
                bstr_len(p1)
            )
        );
        assert_eq!(
            0,
            bstr_begins_with_mem_nocase(
                haystack,
                bstr_ptr(p1) as *const core::ffi::c_void,
                bstr_len(p1)
            )
        );
        assert_eq!(
            0,
            bstr_begins_with_mem_nocase(
                haystack,
                bstr_ptr(p2) as *const core::ffi::c_void,
                bstr_len(p2)
            )
        );

        bstr_free(p1);
        bstr_free(p2);
        bstr_free(haystack);
    }
}

#[test]
fn Bstr_CharAt() {
    unsafe {
        let str: *mut bstr_t = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );
        assert_eq!(0, bstr_char_at(str, 12));
        assert_eq!(-1, bstr_char_at(str, 45));

        bstr_free(str);
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
        let p1: *mut bstr_t = bstr_dup_c(cstr!("abcdef"));
        let p2: *mut bstr_t = bstr_alloc(10);
        bstr_chop(p1);
        assert_eq!(0, bstr_cmp_c(p1, cstr!("abcde")));

        bstr_chop(p2);
        assert_eq!(0, bstr_len(p2));

        bstr_free(p1);
        bstr_free(p2);
    }
}

#[test]
fn Bstr_AdjustLen() {
    unsafe {
        let p1: *mut bstr_t = bstr_dup_c(cstr!("abcdef"));

        bstr_adjust_len(p1, 3);
        assert_eq!(3, bstr_len(p1));
        assert_eq!(0, bstr_cmp_c(p1, cstr!("abc")));

        bstr_free(p1);
    }
}

#[test]
fn Bstr_ToPint() {
    unsafe {
        let mut lastlen: u64 = 0;

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
        let mut c: *mut libc::c_char;
        let str: *mut bstr_t = bstr_dup_mem(
            b"ABCDEFGHIJKL\x00NOPQRSTUVWXYZ".as_ptr() as *const core::ffi::c_void,
            20,
        );

        c = bstr_util_memdup_to_c(b"1234\x006789".as_ptr() as *const core::ffi::c_void, 9);
        assert_eq!(0, libc::strcmp(cstr!("1234\\06789"), c));
        libc::free(c as *mut core::ffi::c_void);

        c = bstr_util_strdup_to_c(str);
        assert_eq!(0, libc::strcmp(cstr!("ABCDEFGHIJKL\\0NOPQRST"), c));

        libc::free(c as *mut core::ffi::c_void);
        bstr_free(str);
    }
}

#[test]
fn Bstr_RChr() {
    unsafe {
        let b: *mut bstr_t = bstr_dup_c(cstr!("---I---I---"));

        assert_eq!(bstr_rchr(b, b'I' as i32), 7);
        assert_eq!(bstr_rchr(b, b'M' as i32), -1);

        bstr_free(b);
    }
}

#[test]
fn Bstr_AdjustRealPtr() {
    unsafe {
        let b: *mut bstr_t = bstr_dup_c(cstr!("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        let c = CString::new("0123456789").unwrap();

        bstr_adjust_realptr(b, c.as_ptr() as *mut core::ffi::c_void);
        bstr_adjust_len(b, libc::strlen(c.as_ptr()) as u64);

        assert_eq!(bstr_ptr(b) as *const libc::c_char, c.as_ptr());

        bstr_free(b);
    }
}

#[test]
fn Bstr_UtilMemTrim() {
    unsafe {
        let d = CString::new(" \r\t0123456789\x0c\x0b  ").unwrap();
        let mut data: *mut libc::c_char = d.as_ptr() as *mut libc::c_char;
        let data_ptr: *mut *mut libc::c_char = &mut data;
        let mut len: u64 = libc::strlen(data) as u64;

        bstr_util_mem_trim(data_ptr as *mut *mut u8, &mut len);

        assert_eq!(
            0,
            bstr_util_cmp_mem(
                data as *const core::ffi::c_void,
                len,
                cstr!("0123456789") as *const core::ffi::c_void,
                10
            )
        );
    }
}

#[test]
fn Bstr_Wrap() {
    unsafe {
        let _s = CString::new("ABC").unwrap();
        let s: *mut bstr_t = bstr_wrap_c(_s.as_ptr());
        assert_eq!(
            0,
            bstr_cmp_mem(s, cstr!("ABC") as *const core::ffi::c_void, 3)
        );
        bstr_free(s);
    }
}

#[test]
fn Bstr_CreateDestroy() {
    unsafe {
        let bb: *mut bstr_builder_t = bstr_builder_create();
        assert_eq!(0, bstr_builder_size(bb));

        bstr_builder_append_c(bb, cstr!("ABC"));

        bstr_builder_destroy(bb);
    }
}

#[test]
fn Bstr_Append() {
    unsafe {
        let bb: *mut bstr_builder_t = bstr_builder_create();
        let str1: *mut bstr_t = bstr_dup_c(cstr!("0123456789"));
        let str2: *mut bstr_t = bstr_dup_c(cstr!("abcdefghijklmnopqrstuvwxyz"));

        assert_eq!(0, bstr_builder_size(bb));

        bstr_builder_appendn(bb, str1);
        bstr_builder_append_c(bb, cstr!("#"));
        bstr_builder_appendn(bb, str2);
        bstr_builder_append_c(bb, cstr!("#"));
        bstr_builder_append_mem(bb, cstr!("!@#$%^&*()") as *const core::ffi::c_void, 4);

        assert_eq!(5, bstr_builder_size(bb));

        let result: *mut bstr_t = bstr_builder_to_str(bb);
        assert_eq!(42, bstr_len(result));

        assert_eq!(
            0,
            libc::memcmp(
                cstr!("0123456789#abcdefghijklmnopqrstuvwxyz#!@#$") as *const core::ffi::c_void,
                bstr_ptr(result) as *const core::ffi::c_void,
                42
            )
        );
        bstr_free(result);

        bstr_builder_clear(bb);
        assert_eq!(0, bstr_builder_size(bb));

        bstr_builder_destroy(bb);
    }
}
