#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
use libc;
use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::io::Write;

use htp::bstr::*;
use htp::htp_base64::*;
use htp::htp_config;
use htp::htp_connection_parser::*;
use htp::htp_list::*;
use htp::htp_request::*;
use htp::htp_table::*;
use htp::htp_transaction::*;
use htp::htp_urlencoded::*;
use htp::htp_util::*;
use htp::Status;

macro_rules! cstr {
    ( $x:expr ) => {{
        CString::new($x).unwrap().as_ptr()
    }};
}

// UTF8 tests
#[test]
fn Single() {
    unsafe {
        assert_eq!(62, htp_base64_decode_single('+' as i8));
        assert_eq!(63, htp_base64_decode_single('/' as i8));
        assert_eq!(-1, htp_base64_decode_single(',' as i8));
        assert_eq!(-1, htp_base64_decode_single(0));
        assert_eq!(-1, htp_base64_decode_single('~' as i8));
        assert_eq!(26, htp_base64_decode_single('a' as i8));
        assert_eq!(0, htp_base64_decode_single('A' as i8));
    }
}

#[test]
fn Decode() {
    unsafe {
        let input = CString::new("dGhpcyBpcyBhIHRlc3QuLg==").unwrap();
        let out: *mut bstr_t = htp_base64_decode_mem(
            input.as_ptr() as *const core::ffi::c_void,
            libc::strlen(input.as_ptr()),
        );
        assert_eq!(0, bstr_cmp_str(out, "this is a test.."));
        bstr_free(out);
    }
}
// Util tests
#[test]
fn Separator() {
    assert_eq!(false, htp_is_separator('a' as u8));
    assert_eq!(false, htp_is_separator('^' as u8));
    assert_eq!(false, htp_is_separator('-' as u8));
    assert_eq!(false, htp_is_separator('_' as u8));
    assert_eq!(false, htp_is_separator('&' as u8));
    assert_eq!(true, htp_is_separator('(' as u8));
    assert_eq!(true, htp_is_separator('\\' as u8));
    assert_eq!(true, htp_is_separator('/' as u8));
    assert_eq!(true, htp_is_separator('=' as u8));
    assert_eq!(true, htp_is_separator('\t' as u8));
}

#[test]
fn Token() {
    assert_eq!(true, htp_is_token('a' as u8));
    assert_eq!(true, htp_is_token('&' as u8));
    assert_eq!(true, htp_is_token('+' as u8));
    assert_eq!(false, htp_is_token('\t' as u8));
    assert_eq!(false, htp_is_token('\n' as u8));
}

fn unsize<T>(x: &[T]) -> &[T] {
    x
}

#[test]
fn Chomp() {
    unsafe {
        let data: [u8; 100] = [0; 100];
        let mut len: usize;
        let mut result: i32;

        libc::strcpy(data.as_ptr() as *mut i8, cstr!("test\r\n"));
        len = libc::strlen(data.as_ptr() as *mut i8);
        result = htp_chomp(data.as_ptr() as *mut u8, &mut len);
        assert_eq!(2, result);
        assert_eq!(4, len);

        libc::strcpy(data.as_ptr() as *mut i8, cstr!("test\r\n\n"));
        len = libc::strlen(data.as_ptr() as *mut i8);
        result = htp_chomp(data.as_ptr() as *mut u8, &mut len);
        assert_eq!(2, result);
        assert_eq!(4, len);

        libc::strcpy(data.as_ptr() as *mut i8, cstr!("test\r\n\r\n"));
        len = libc::strlen(data.as_ptr() as *mut i8);
        result = htp_chomp(data.as_ptr() as *mut u8, &mut len);
        assert_eq!(2, result);
        assert_eq!(4, len);

        libc::strcpy(data.as_ptr() as *mut i8, cstr!("te\nst"));
        len = libc::strlen(data.as_ptr() as *mut i8);
        result = htp_chomp(data.as_ptr() as *mut u8, &mut len);
        assert_eq!(0, result);
        assert_eq!(5, len);

        libc::strcpy(data.as_ptr() as *mut i8, cstr!("foo\n"));
        len = libc::strlen(data.as_ptr() as *mut i8);
        result = htp_chomp(data.as_ptr() as *mut u8, &mut len);
        assert_eq!(1, result);
        assert_eq!(3, len);

        libc::strcpy(data.as_ptr() as *mut i8, cstr!("arfarf"));
        len = libc::strlen(data.as_ptr() as *mut i8);
        result = htp_chomp(data.as_ptr() as *mut u8, &mut len);
        assert_eq!(0, result);
        assert_eq!(6, len);

        libc::strcpy(data.as_ptr() as *mut i8, cstr!(""));
        len = libc::strlen(data.as_ptr() as *mut i8);
        result = htp_chomp(data.as_ptr() as *mut u8, &mut len);
        assert_eq!(0, result);
        assert_eq!(0, len);
    }
}

#[test]
fn Space() {
    assert_eq!(false, htp_is_space(0x61)); // a
    assert_eq!(true, htp_is_space(0x20)); // space
    assert_eq!(true, htp_is_space(0x0c)); // Form feed
    assert_eq!(true, htp_is_space(0x0a)); // newline
    assert_eq!(true, htp_is_space(0x0d)); // carriage return
    assert_eq!(true, htp_is_space(0x09)); // tab
    assert_eq!(true, htp_is_space(0x0b)); // Vertical tab
}

#[test]
fn Method() {
    unsafe {
        let method: *mut bstr_t = bstr_dup_str("GET");

        assert_eq!(
            htp_method_t::HTP_M_GET as i32,
            htp_convert_method_to_number(method)
        );

        bstr_free(method);
    }
}

#[test]
fn IsLineEmpty() {
    let data = b"arfarf";
    assert_eq!(false, htp_is_line_empty(data));
    assert_eq!(true, htp_is_line_empty(b"\x0d\x0a"));
    assert_eq!(true, htp_is_line_empty(b"\x0d"));
    assert_eq!(true, htp_is_line_empty(b"\x0a"));
    assert_eq!(false, htp_is_line_empty(b"\x0a\x0d"));
    assert_eq!(false, htp_is_line_empty(b"\x0dabc"));
}

#[test]
fn IsLineWhitespace() {
    let data = b"arfarf";
    assert_eq!(false, htp_is_line_whitespace(data));
    assert_eq!(true, htp_is_line_whitespace(b"\x0d\x0a"));
    assert_eq!(true, htp_is_line_whitespace(b"\x0d"));
    assert_eq!(false, htp_is_line_whitespace(b"\x0dabc"));}

#[test]
fn ParsePositiveIntegerWhitespace() {
    unsafe {
        assert_eq!(
            123,
            htp_parse_positive_integer_whitespace(cstr!("123   ") as *const u8, 6, 10)
        );
        assert_eq!(
            123,
            htp_parse_positive_integer_whitespace(cstr!("   123") as *const u8, 6, 10)
        );
        assert_eq!(
            123,
            htp_parse_positive_integer_whitespace(cstr!("   123   ") as *const u8, 9, 10)
        );
        assert_eq!(
            -1,
            htp_parse_positive_integer_whitespace(cstr!("a123") as *const u8, 4, 10)
        );
        assert_eq!(
            -1001,
            htp_parse_positive_integer_whitespace(cstr!("   \t") as *const u8, 4, 10)
        );
        assert_eq!(
            -1002,
            htp_parse_positive_integer_whitespace(cstr!("123b ") as *const u8, 5, 10)
        );

        assert_eq!(
            -1,
            htp_parse_positive_integer_whitespace(cstr!("   a123   ") as *const u8, 9, 10)
        );
        assert_eq!(
            -1002,
            htp_parse_positive_integer_whitespace(cstr!("   123b   ") as *const u8, 9, 10)
        );

        assert_eq!(
            0x123,
            htp_parse_positive_integer_whitespace(cstr!("   123   ") as *const u8, 9, 16)
        );
    }
}

#[test]
fn ParseContentLength() {
    unsafe {
        let data: *mut bstr_t = bstr_dup_str("134");

        assert_eq!(134, htp_parse_content_length(data, std::ptr::null_mut()));

        bstr_free(data);
    }
}

#[test]
fn ParseChunkedLength() {
    unsafe {
        let mut_data = CString::new("12a5").unwrap();
        assert_eq!(
            0x12a5,
            htp_parse_chunked_length(mut_data.as_ptr() as *mut u8, 4)
        );
    }
}

#[test]
fn IsLineFolded() {
    assert_eq!(true, htp_connp_is_line_folded(b"\tline"));
    assert_eq!(true, htp_connp_is_line_folded(b" line"));
    assert_eq!(false, htp_connp_is_line_folded(b"line "));
}

fn free_htp_uri_t(urip: *mut *mut htp_uri_t) {
    unsafe {
        let uri = *urip;

        if uri == std::ptr::null_mut() {
            return;
        }
        bstr_free((*uri).scheme);
        bstr_free((*uri).username);
        bstr_free((*uri).password);
        bstr_free((*uri).hostname);
        bstr_free((*uri).port);
        bstr_free((*uri).path);
        bstr_free((*uri).query);
        bstr_free((*uri).fragment);

        libc::free(uri as *mut core::ffi::c_void);
        *urip = std::ptr::null_mut();
    }
}

#[repr(C)]
#[derive(Clone)]
struct uri_expected {
    scheme: *const i8,
    username: *const i8,
    password: *const i8,
    hostname: *const i8,
    port: *const i8,
    path: *const i8,
    query: *const i8,
    fragment: *const i8,
}

#[repr(C)]
#[derive(Clone)]
struct uri_test {
    uri: *const i8,
    expected: uri_expected,
}

fn bstr_equal_c(b: *const bstr_t, c: *const i8) -> bool {
    unsafe {
        if (c == std::ptr::null()) || (b == std::ptr::null()) {
            (c == std::ptr::null()) && (b == std::ptr::null())
        } else {
            0 == bstr_cmp_c(b, c)
        }
    }
}

fn append_message<W: Write>(
    o: &mut W,
    label: *const i8,
    expected: *const i8,
    actual: *const bstr_t,
) -> Result<(), std::io::Error> {
    unsafe {
        o.write_fmt(format_args!(
            "{} missmatch: ",
            CStr::from_ptr(label).to_str().unwrap()
        ))?;
        if expected != std::ptr::null() {
            o.write_fmt(format_args!(
                "'{}'",
                CStr::from_ptr(expected).to_str().unwrap()
            ))?;
        } else {
            o.write(b"<NULL>")?;
        }
        o.write(b" != ")?;
        if actual != std::ptr::null() {
            o.write(b"'")?;
            o.write(std::slice::from_raw_parts(
                bstr_ptr(actual),
                bstr_len(actual),
            ))?;
            o.write(b"'")?;
        } else {
            o.write(b"<NULL>")?;
        }
        o.write(b"\n")?;
        Ok(())
    }
}

fn UriIsExpected(expected: uri_expected, actual: *const htp_uri_t) -> Result<(), std::io::Error> {
    unsafe {
        let mut msg: Vec<u8> = vec![];
        let mut equal: bool = true;

        if !bstr_equal_c((*actual).scheme, expected.scheme) {
            equal = false;
            append_message(&mut msg, cstr!("scheme"), expected.scheme, (*actual).scheme)?;
        }

        if !bstr_equal_c((*actual).username, expected.username) {
            equal = false;
            append_message(
                &mut msg,
                cstr!("username"),
                expected.username,
                (*actual).username,
            )?;
        }

        if !bstr_equal_c((*actual).password, expected.password) {
            equal = false;
            append_message(
                &mut msg,
                cstr!("password"),
                expected.password,
                (*actual).password,
            )?;
        }

        if !bstr_equal_c((*actual).hostname, expected.hostname) {
            equal = false;
            append_message(
                &mut msg,
                cstr!("hostname"),
                expected.hostname,
                (*actual).hostname,
            )?;
        }

        if !bstr_equal_c((*actual).port, expected.port) {
            equal = false;
            append_message(&mut msg, cstr!("port"), expected.port, (*actual).port)?;
        }

        if !bstr_equal_c((*actual).path, expected.path) {
            equal = false;
            append_message(&mut msg, cstr!("path"), expected.path, (*actual).path)?;
        }

        if !bstr_equal_c((*actual).query, expected.query) {
            equal = false;
            append_message(&mut msg, cstr!("query"), expected.query, (*actual).query)?;
        }

        if !bstr_equal_c((*actual).fragment, expected.fragment) {
            equal = false;
            append_message(
                &mut msg,
                cstr!("fragment"),
                expected.fragment,
                (*actual).fragment,
            )?;
        }

        if equal {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                std::str::from_utf8(&msg).unwrap(),
            ))
        }
    }
}

struct UriTest {
    uri_tests: Vec<uri_test>,
}

impl UriTest {
    fn new() -> Self {
        Self {
            uri_tests: {
                [
                    uri_test {
                        uri: CString::new(
                            "http://user:pass@www.example.com:1234/path1/path2?a=b&c=d#frag",
                        )
                        .unwrap()
                        .into_raw(),
                        expected: uri_expected {
                            scheme: CString::new("http").unwrap().into_raw(),
                            username: CString::new("user").unwrap().into_raw(),
                            password: CString::new("pass").unwrap().into_raw(),
                            hostname: CString::new("www.example.com").unwrap().into_raw(),
                            port: CString::new("1234").unwrap().into_raw(),
                            path: CString::new("/path1/path2").unwrap().into_raw(),
                            query: CString::new("a=b&c=d").unwrap().into_raw(),
                            fragment: CString::new("frag").unwrap().into_raw(),
                        },
                    },
                    uri_test {
                        uri: CString::new("http://host.com/path").unwrap().into_raw(),
                        expected: uri_expected {
                            scheme: CString::new("http").unwrap().into_raw(),
                            username: std::ptr::null(),
                            password: std::ptr::null(),
                            hostname: CString::new("host.com").unwrap().into_raw(),
                            port: std::ptr::null(),
                            path: CString::new("/path").unwrap().into_raw(),
                            query: std::ptr::null(),
                            fragment: std::ptr::null(),
                        },
                    },
                    uri_test {
                        uri: CString::new("http://").unwrap().into_raw(),
                        expected: uri_expected {
                            scheme: CString::new("http").unwrap().into_raw(),
                            username: std::ptr::null(),
                            password: std::ptr::null(),
                            hostname: std::ptr::null(),
                            port: std::ptr::null(),
                            path: CString::new("//").unwrap().into_raw(),
                            query: std::ptr::null(),
                            fragment: std::ptr::null(),
                        },
                    },
                    uri_test {
                        uri: CString::new("/path").unwrap().into_raw(),
                        expected: uri_expected {
                            scheme: std::ptr::null(),
                            username: std::ptr::null(),
                            password: std::ptr::null(),
                            hostname: std::ptr::null(),
                            port: std::ptr::null(),
                            path: CString::new("/path").unwrap().into_raw(),
                            query: std::ptr::null(),
                            fragment: std::ptr::null(),
                        },
                    },
                    uri_test {
                        uri: CString::new("://").unwrap().into_raw(),
                        expected: uri_expected {
                            scheme: CString::new("").unwrap().into_raw(),
                            username: std::ptr::null(),
                            password: std::ptr::null(),
                            hostname: std::ptr::null(),
                            port: std::ptr::null(),
                            path: CString::new("//").unwrap().into_raw(),
                            query: std::ptr::null(),
                            fragment: std::ptr::null(),
                        },
                    },
                    uri_test {
                        uri: CString::new("").unwrap().into_raw(),
                        expected: uri_expected {
                            scheme: std::ptr::null(),
                            username: std::ptr::null(),
                            password: std::ptr::null(),
                            hostname: std::ptr::null(),
                            port: std::ptr::null(),
                            path: std::ptr::null(),
                            query: std::ptr::null(),
                            fragment: std::ptr::null(),
                        },
                    },
                    uri_test {
                        uri: CString::new("http://user@host.com").unwrap().into_raw(),
                        expected: uri_expected {
                            scheme: CString::new("http").unwrap().into_raw(),
                            username: CString::new("user").unwrap().into_raw(),
                            password: std::ptr::null(),
                            hostname: CString::new("host.com").unwrap().into_raw(),
                            port: std::ptr::null(),
                            path: CString::new("").unwrap().into_raw(),
                            query: std::ptr::null(),
                            fragment: std::ptr::null(),
                        },
                    },
                    uri_test {
                        uri: std::ptr::null(),
                        expected: uri_expected {
                            scheme: std::ptr::null(),
                            username: std::ptr::null(),
                            password: std::ptr::null(),
                            hostname: std::ptr::null(),
                            port: std::ptr::null(),
                            path: std::ptr::null(),
                            query: std::ptr::null(),
                            fragment: std::ptr::null(),
                        },
                    },
                ]
                .to_vec()
            },
        }
    }
}

#[test]
fn HtpParseUri() {
    unsafe {
        let harness = UriTest::new();
        let mut input: *mut bstr_t;
        let mut uri: *mut htp_uri_t = std::ptr::null_mut();

        input = bstr_dup_str("");
        assert_eq!(Status::OK, htp_parse_uri(input, &mut uri));
        bstr_free(input);
        free_htp_uri_t(&mut uri);

        let tests = harness.uri_tests;
        for test in tests {
            if test.uri != std::ptr::null() {
                //println!("test.uri: {:?}", *(test.uri));
                input = bstr_dup_c(test.uri);
                assert_eq!(Status::OK, htp_parse_uri(input, &mut uri));
                if let Err(x) = UriIsExpected(test.expected, uri) {
                    println!("{}", x);
                    println!(
                        "Failed URI = {}",
                        CStr::from_ptr(test.uri).to_str().unwrap()
                    );
                    assert!(false);
                }

                bstr_free(input);
                free_htp_uri_t(&mut uri);
            }
        }
    }
}

#[test]
fn ParseHostPort_1() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert_eq!(bstr_cmp(i, host), 0);
        assert_eq!(-1, port);
        assert_eq!(0, flag);

        bstr_free(host);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_2() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str(" www.example.com ");
        let e: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(0, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_3() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str(" www.example.com:8001 ");
        let e: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(8001, port);
        assert_eq!(0, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_4() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str(" www.example.com :  8001 ");
        let e: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(8001, port);
        assert_eq!(0, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_5() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com.");
        let e: *mut bstr_t = bstr_dup_str("www.example.com.");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(0, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_6() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com.:8001");
        let e: *mut bstr_t = bstr_dup_str("www.example.com.");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(8001, port);
        assert_eq!(0, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_7() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com:");
        let e: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(1, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_8() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com:ff");
        let e: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(1, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_9() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com:0");
        let e: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(1, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_10() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com:65536");
        let e: *mut bstr_t = bstr_dup_str("www.example.com");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut flag = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut flag)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(1, flag);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_11() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("[::1]:8080");
        let e: *mut bstr_t = bstr_dup_str("[::1]");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut invalid = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut invalid)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(8080, port);
        assert_eq!(0, invalid);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_12() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("[::1]:");
        let e: *mut bstr_t = bstr_dup_str("[::1]");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut invalid = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut invalid)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(1, invalid);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_13() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("[::1]x");
        let e: *mut bstr_t = bstr_dup_str("[::1]");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut invalid = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut invalid)
        );

        assert!(!host.is_null());
        assert_eq!(bstr_cmp(e, host), 0);
        assert_eq!(-1, port);
        assert_eq!(1, invalid);

        bstr_free(host);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseHostPort_14() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("[::1");
        let mut host: *mut bstr_t = std::ptr::null_mut();
        let mut port = 0;
        let mut invalid = 0;

        assert_eq!(
            Status::OK,
            htp_parse_hostport(i, &mut host, std::ptr::null_mut(), &mut port, &mut invalid)
        );

        assert!(host.is_null());
        assert_eq!(-1, port);
        assert_eq!(1, invalid);

        bstr_free(host);
        bstr_free(i);
    }
}

#[test]
fn ParseContentType_1() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("multipart/form-data");
        let e: *mut bstr_t = bstr_dup_str("multipart/form-data");
        let mut ct: *mut bstr_t = std::ptr::null_mut();

        assert_eq!(Status::OK, htp_parse_ct_header(i, &mut ct));

        assert!(!ct.is_null());
        assert_eq!(bstr_cmp(e, ct), 0);

        bstr_free(ct);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseContentType_2() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("multipart/form-data;boundary=X");
        let e: *mut bstr_t = bstr_dup_str("multipart/form-data");
        let mut ct: *mut bstr_t = std::ptr::null_mut();

        assert_eq!(Status::OK, htp_parse_ct_header(i, &mut ct));

        assert!(!ct.is_null());
        assert_eq!(bstr_cmp(e, ct), 0);

        bstr_free(ct);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseContentType_3() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("multipart/form-data boundary=X");
        let e: *mut bstr_t = bstr_dup_str("multipart/form-data");
        let mut ct: *mut bstr_t = std::ptr::null_mut();

        assert_eq!(Status::OK, htp_parse_ct_header(i, &mut ct));

        assert!(!ct.is_null());
        assert_eq!(bstr_cmp(e, ct), 0);

        bstr_free(ct);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseContentType_4() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("multipart/form-data,boundary=X");
        let e: *mut bstr_t = bstr_dup_str("multipart/form-data");
        let mut ct: *mut bstr_t = std::ptr::null_mut();

        assert_eq!(Status::OK, htp_parse_ct_header(i, &mut ct));

        assert!(!ct.is_null());
        assert_eq!(bstr_cmp(e, ct), 0);

        bstr_free(ct);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseContentType_5() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("multipart/FoRm-data");
        let e: *mut bstr_t = bstr_dup_str("multipart/form-data");
        let mut ct: *mut bstr_t = std::ptr::null_mut();

        assert_eq!(Status::OK, htp_parse_ct_header(i, &mut ct));

        assert!(!ct.is_null());
        assert_eq!(bstr_cmp(e, ct), 0);

        bstr_free(ct);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ParseContentType_6() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("multipart/form-data\t boundary=X");
        let e: *mut bstr_t = bstr_dup_str("multipart/form-data\t");
        let mut ct: *mut bstr_t = std::ptr::null_mut();

        assert_eq!(Status::OK, htp_parse_ct_header(i, &mut ct));

        assert!(!ct.is_null());
        assert_eq!(bstr_cmp(e, ct), 0);

        bstr_free(ct);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_1() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com");
        assert_eq!(1, htp_validate_hostname(i));
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_2() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str(".www.example.com");
        assert_eq!(0, htp_validate_hostname(i));
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_3() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www..example.com");
        assert_eq!(0, htp_validate_hostname(i));
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_4() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.example.com..");
        assert_eq!(0, htp_validate_hostname(i));
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_5() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www example com");
        assert_eq!(0, htp_validate_hostname(i));
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_6() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("");
        assert_eq!(0, htp_validate_hostname(i));
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_7() {
    unsafe {
        // Label over 63 characters.
        let i: *mut bstr_t = bstr_dup_str(
            "www.exampleexampleexampleexampleexampleexampleexampleexampleexampleexample.com",
        );
        assert_eq!(0, htp_validate_hostname(i));
        bstr_free(i);
    }
}

#[test]
fn ValidateHostname_8() {
    unsafe {
        let i: *mut bstr_t = bstr_dup_str("www.ExAmplE-1984.com");
        assert_eq!(1, htp_validate_hostname(i));
        bstr_free(i);
    }
}

struct DecodingTest {
    connp: *mut htp_connp_t,
    cfg: *mut htp_config::htp_cfg_t,
    tx: *mut htp_tx_t,
}

impl DecodingTest {
    fn new() -> Self {
        let mut ret = Self {
            connp: std::ptr::null_mut(),
            cfg: std::ptr::null_mut(),
            tx: std::ptr::null_mut(),
        };
        unsafe {
            ret.cfg = htp_config::create();
            ret.connp = htp_connp_create(ret.cfg);
            htp_connp_open(
                ret.connp,
                cstr!("127.0.0.1"),
                32768,
                cstr!("127.0.0.1"),
                80,
                std::ptr::null_mut(),
            );
            ret.tx = htp_connp_tx_create(ret.connp);
        }
        ret
    }
}

impl Drop for DecodingTest {
    fn drop(&mut self) {
        unsafe {
            htp_connp_destroy_all(self.connp);
            (*self.cfg).destroy();
        }
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace1_Identity() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/dest");
        let e: *mut bstr_t = bstr_dup_str("/dest");
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace2_Urlencoded() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%64est");
        let e: *mut bstr_t = bstr_dup_str("/dest");
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace3_UrlencodedInvalidPreserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%xxest");
        let e: *mut bstr_t = bstr_dup_str("/%xxest");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace4_UrlencodedInvalidRemove() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%xxest");
        let e: *mut bstr_t = bstr_dup_str("/xxest");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace5_UrlencodedInvalidDecode() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%}9est");
        let e: *mut bstr_t = bstr_dup_str("/iest");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace6_UrlencodedInvalidNotEnoughBytes() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%a");
        let e: *mut bstr_t = bstr_dup_str("/%a");
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace7_UrlencodedInvalidNotEnoughBytes() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%");
        let e: *mut bstr_t = bstr_dup_str("/%");
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace8_Uencoded() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u0064");
        let e: *mut bstr_t = bstr_dup_str("/d");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace9_UencodedDoNotDecode() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u0064");
        let e: *mut bstr_t = bstr_dup_str("/%u0064");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 0);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace10_UencodedInvalidNotEnoughBytes() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u006");
        let e: *mut bstr_t = bstr_dup_str("/%u006");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace11_UencodedInvalidPreserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u006");
        let e: *mut bstr_t = bstr_dup_str("/%u006");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace12_UencodedInvalidRemove() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%uXXXX");
        let e: *mut bstr_t = bstr_dup_str("/uXXXX");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace13_UencodedInvalidDecode() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u00}9");
        let e: *mut bstr_t = bstr_dup_str("/i");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace14_UencodedInvalidPreserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u00");
        let e: *mut bstr_t = bstr_dup_str("/%u00");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace15_UencodedInvalidPreserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u0");
        let e: *mut bstr_t = bstr_dup_str("/%u0");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace16_UencodedInvalidPreserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u");
        let e: *mut bstr_t = bstr_dup_str("/%u");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace17_UrlencodedNul() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%00");
        let e: *mut bstr_t = bstr_dup_mem("/\0".as_ptr() as *const core::ffi::c_void, 2);
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace18_UrlencodedNulTerminates() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%00ABC");
        let e: *mut bstr_t = bstr_dup_str("/");
        (*test.cfg)
            .set_nul_encoded_terminates(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace19_RawNulTerminates() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_mem("/\0ABC".as_ptr() as *const core::ffi::c_void, 5);
        let e: *mut bstr_t = bstr_dup_str("/");
        (*test.cfg).set_nul_raw_terminates(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTes_DecodeUrlencodedInplace20_UencodedBestFit() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u0107");
        let e: *mut bstr_t = bstr_dup_str("/c");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_tx_urldecode_params_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace1_UrlencodedInvalidNotEnoughBytes() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%a");
        let e: *mut bstr_t = bstr_dup_str("/%a");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace2_UencodedInvalidNotEnoughBytes() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%uX");
        let e: *mut bstr_t = bstr_dup_str("/%uX");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace3_UencodedValid() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u0107");
        let e: *mut bstr_t = bstr_dup_str("/c");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace4_UencodedInvalidNotHexDigits_Remove() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%uXXXX");
        let e: *mut bstr_t = bstr_dup_str("/uXXXX");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace5_UencodedInvalidNotHexDigits_Preserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%uXXXX");
        let e: *mut bstr_t = bstr_dup_str("/%uXXXX");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace6_UencodedInvalidNotHexDigits_Process() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u00}9");
        let e: *mut bstr_t = bstr_dup_str("/i");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace7_UencodedNul() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%u0000");
        let e: *mut bstr_t = bstr_dup_mem("/\0".as_ptr() as *const core::ffi::c_void, 2);
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_ENCODED_NUL));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace8_UencodedNotEnough_Remove() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%uXXX");
        let e: *mut bstr_t = bstr_dup_str("/uXXX");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace9_UencodedNotEnough_Preserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%uXXX");
        let e: *mut bstr_t = bstr_dup_str("/%uXXX");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace10_UrlencodedNul() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%00123");
        let e: *mut bstr_t = bstr_dup_mem("/\x00123".as_ptr() as *const core::ffi::c_void, 5);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_ENCODED_NUL));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace11_UrlencodedNul_Terminates() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%00123");
        let e: *mut bstr_t = bstr_dup_mem("/".as_ptr() as *const core::ffi::c_void, 1);
        (*test.cfg)
            .set_nul_encoded_terminates(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_ENCODED_NUL));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace12_EncodedSlash() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/one%2ftwo");
        let e: *mut bstr_t = bstr_dup_str("/one%2ftwo");
        (*test.cfg)
            .set_path_separators_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 0);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_ENCODED_SEPARATOR));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace13_EncodedSlash_Decode() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/one%2ftwo");
        let e: *mut bstr_t = bstr_dup_str("/one/two");
        (*test.cfg)
            .set_path_separators_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_ENCODED_SEPARATOR));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace14_Urlencoded_Invalid_Preserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%HH");
        let e: *mut bstr_t = bstr_dup_str("/%HH");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace15_Urlencoded_Invalid_Remove() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%HH");
        let e: *mut bstr_t = bstr_dup_str("/HH");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace16_Urlencoded_Invalid_Process() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%}9");
        let e: *mut bstr_t = bstr_dup_str("/i");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace17_Urlencoded_NotEnough_Remove() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%H");
        let e: *mut bstr_t = bstr_dup_str("/H");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace18_Urlencoded_NotEnough_Preserve() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%H");
        let e: *mut bstr_t = bstr_dup_str("/%H");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace19_Urlencoded_NotEnough_Process() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/%H");
        let e: *mut bstr_t = bstr_dup_str("/%H");
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        assert!((*test.tx).flags.contains(Flags::HTP_PATH_INVALID_ENCODING));
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace20_RawNul1() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_mem("/\x00123".as_ptr() as *const core::ffi::c_void, 5);
        let e: *mut bstr_t = bstr_dup_str("/");
        (*test.cfg).set_nul_raw_terminates(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace21_RawNul1() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_mem("/\x00123".as_ptr() as *const core::ffi::c_void, 5);
        let e: *mut bstr_t = bstr_dup_mem("/\x00123".as_ptr() as *const core::ffi::c_void, 5);
        (*test.cfg).set_nul_raw_terminates(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 0);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace22_ConvertBackslash1() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/one\\two");
        let e: *mut bstr_t = bstr_dup_str("/one/two");
        (*test.cfg)
            .set_backslash_convert_slashes(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 1);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_DecodePathInplace23_ConvertBackslash2() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str("/one\\two");
        let e: *mut bstr_t = bstr_dup_str("/one\\two");
        (*test.cfg)
            .set_backslash_convert_slashes(htp_config::htp_decoder_ctx_t::HTP_DECODER_DEFAULTS, 0);
        htp_decode_path_inplace(test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

#[test]
fn DecodingTest_InvalidUtf8() {
    unsafe {
        let test = DecodingTest::new();
        let i: *mut bstr_t = bstr_dup_str(b"\xf1.");
        let e: *mut bstr_t = bstr_dup_str("?.");
        (*test.cfg)
            .set_utf8_convert_bestfit(htp_config::htp_decoder_ctx_t::HTP_DECODER_URL_PATH, 1);
        htp_utf8_decode_path_inplace(&mut *test.cfg, test.tx, i);
        assert_eq!(bstr_cmp(i, e), 0);
        bstr_free(e);
        bstr_free(i);
    }
}

struct UrlEncodedParserTest {
    connp: *mut htp_connp_t,
    cfg: *mut htp_config::htp_cfg_t,
    tx: *mut htp_tx_t,
    urlenp: *mut htp_urlenp_t,
}

impl UrlEncodedParserTest {
    fn new() -> Self {
        let mut ret = Self {
            connp: std::ptr::null_mut(),
            cfg: std::ptr::null_mut(),
            tx: std::ptr::null_mut(),
            urlenp: std::ptr::null_mut(),
        };
        unsafe {
            ret.cfg = htp_config::create();
            ret.connp = htp_connp_create(ret.cfg);
            htp_connp_open(
                ret.connp,
                cstr!("127.0.0.1"),
                32768,
                cstr!("127.0.0.1"),
                80,
                std::ptr::null_mut(),
            );
            ret.tx = htp_connp_tx_create(ret.connp);
            ret.urlenp = htp_urlenp_create(ret.tx);
            ret
        }
    }
}

impl Drop for UrlEncodedParserTest {
    fn drop(&mut self) {
        unsafe {
            htp_urlenp_destroy(self.urlenp);
            htp_connp_destroy_all(self.connp);
            (*self.cfg).destroy();
        }
    }
}

// Start of Url Parser tests.
#[test]
fn UrlencodedParser_Empty() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "".as_ptr() as *const core::ffi::c_void, 0);

        assert_eq!(0, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_EmptyKey1() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "&".as_ptr() as *const core::ffi::c_void, 1);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_EmptyKey2() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "=&".as_ptr() as *const core::ffi::c_void, 2);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_EmptyKey3() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "=1&".as_ptr() as *const core::ffi::c_void, 3);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("").unwrap().1.cmp("1")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_EmptyKeyAndValue() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "=".as_ptr() as *const core::ffi::c_void, 1);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_OnePairEmptyValue() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "p=".as_ptr() as *const core::ffi::c_void, 2);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_OnePair() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "p=1".as_ptr() as *const core::ffi::c_void, 3);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("1")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_TwoPairs() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(
            test.urlenp,
            "p=1&q=2".as_ptr() as *const core::ffi::c_void,
            7,
        );

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("1")
        );
        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("q").unwrap().1.cmp("2")
        );
        assert_eq!(2, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_KeyNoValue1() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "p".as_ptr() as *const core::ffi::c_void, 1);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_KeyNoValue2() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "p&".as_ptr() as *mut core::ffi::c_void, 2);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_KeyNoValue3() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "p&q".as_ptr() as *mut core::ffi::c_void, 3);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("q").unwrap().1.cmp("")
        );
        assert_eq!(2, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_KeyNoValue4() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_complete(test.urlenp, "p&q=2".as_ptr() as *mut core::ffi::c_void, 5);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("q").unwrap().1.cmp("2")
        );
        assert_eq!(2, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_Partial1() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_partial(test.urlenp, "p".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_finalize(test.urlenp);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_Partial2() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_partial(test.urlenp, "p".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "x".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_finalize(test.urlenp);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("px").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_Partial3() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_partial(test.urlenp, "p".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "x&".as_ptr() as *mut core::ffi::c_void, 2);
        htp_urlenp_finalize(test.urlenp);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("px").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_Partial4() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_partial(test.urlenp, "p".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "=".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_finalize(test.urlenp);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_Partial5() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_partial(test.urlenp, "p".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "".as_ptr() as *mut core::ffi::c_void, 0);
        htp_urlenp_parse_partial(test.urlenp, "".as_ptr() as *mut core::ffi::c_void, 0);
        htp_urlenp_parse_partial(test.urlenp, "".as_ptr() as *mut core::ffi::c_void, 0);
        htp_urlenp_finalize(test.urlenp);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("p").unwrap().1.cmp("")
        );
        assert_eq!(1, (*test.urlenp).params.size());
    }
}

#[test]
fn UrlencodedParser_Partial6i() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        htp_urlenp_parse_partial(test.urlenp, "px".as_ptr() as *mut core::ffi::c_void, 2);
        htp_urlenp_parse_partial(test.urlenp, "n".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "".as_ptr() as *mut core::ffi::c_void, 0);
        htp_urlenp_parse_partial(test.urlenp, "=".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "1".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "2".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "&".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "qz".as_ptr() as *mut core::ffi::c_void, 2);
        htp_urlenp_parse_partial(test.urlenp, "n".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "".as_ptr() as *mut core::ffi::c_void, 0);
        htp_urlenp_parse_partial(test.urlenp, "=".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "2".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "3".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_parse_partial(test.urlenp, "&".as_ptr() as *mut core::ffi::c_void, 1);
        htp_urlenp_finalize(test.urlenp);

        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("pxn").unwrap().1.cmp("12")
        );
        assert_eq!(
            Ordering::Equal,
            (*test.urlenp).params.get_nocase("qzn").unwrap().1.cmp("23")
        );
        assert_eq!(2, (*test.urlenp).params.size());
    }
}

#[test]
fn List_Misc() {
    unsafe {
        let l: *mut htp_list_array_t = htp_list_array_create(16);

        htp_list_array_push(l, "1".as_ptr() as *mut core::ffi::c_void);
        htp_list_array_push(l, "2".as_ptr() as *mut core::ffi::c_void);
        htp_list_array_push(l, "3".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(3, htp_list_array_size(l));

        let mut p: *mut i8 = htp_list_array_pop(l) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        assert_eq!(2, htp_list_array_size(l));

        p = htp_list_array_pop(l) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("2".as_ptr() as *mut i8, p));

        p = htp_list_array_pop(l) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("1".as_ptr() as *mut i8, p));

        p = htp_list_array_pop(l) as *mut i8;
        assert!(p.is_null());

        htp_list_array_destroy(l);
    }
}

#[test]
fn List_Misc2() {
    unsafe {
        let l: *mut htp_list_array_t = htp_list_array_create(2);

        htp_list_array_push(l, "1".as_ptr() as *mut core::ffi::c_void);
        htp_list_array_push(l, "2".as_ptr() as *mut core::ffi::c_void);
        htp_list_array_push(l, "3".as_ptr() as *mut core::ffi::c_void);

        let mut p: *mut i8 = htp_list_array_get(l, 2) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        assert_eq!(3, htp_list_array_size(l));

        htp_list_array_replace(l, 2, "4".as_ptr() as *mut core::ffi::c_void);

        p = htp_list_array_pop(l) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("4".as_ptr() as *mut i8, p));

        htp_list_array_destroy(l);
    }
}

#[test]
fn List_Expand1() {
    unsafe {
        let l: *mut htp_list_array_t = htp_list_array_create(2);

        htp_list_array_push(l, "1".as_ptr() as *mut core::ffi::c_void);
        htp_list_array_push(l, "2".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(2, htp_list_array_size(l));

        htp_list_array_push(l, "3".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(3, htp_list_array_size(l));

        let mut p: *mut i8 = htp_list_array_get(l, 0) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("1".as_ptr() as *mut i8, p));

        p = htp_list_array_get(l, 1) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("2".as_ptr() as *mut i8, p));

        p = htp_list_array_get(l, 2) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        htp_list_array_destroy(l);
    }
}

#[test]
fn List_Expand2() {
    unsafe {
        let l: *mut htp_list_array_t = htp_list_array_create(2);

        htp_list_array_push(l, "1".as_ptr() as *mut core::ffi::c_void);
        htp_list_array_push(l, "2".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(2, htp_list_array_size(l));

        htp_list_array_push(l, "3".as_ptr() as *mut core::ffi::c_void);
        htp_list_array_push(l, "4".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(4, htp_list_array_size(l));

        let mut p: *mut i8 = htp_list_array_get(l, 0) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("1".as_ptr() as *mut i8, p));

        p = htp_list_array_get(l, 1) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("2".as_ptr() as *mut i8, p));

        p = htp_list_array_get(l, 2) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        p = htp_list_array_pop(l) as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("4".as_ptr() as *mut i8, p));

        htp_list_array_destroy(l);
    }
}

#[test]
fn Table_Misc() {
    let mut t: htp_table_t<&str> = htp_table_t::with_capacity(2);

    let mut pkey = bstr_t::with_capacity(1);
    pkey.add("p");

    let mut qkey = bstr_t::with_capacity(1);
    qkey.add("q");

    t.add(pkey, "1");
    t.add(qkey, "2");

    assert!(t.get_nocase("z").is_none());
    assert_eq!("1", t.get_nocase("p").unwrap().1);
}

#[test]
fn Util_ExtractQuotedString() {
    unsafe {
        let mut s: *mut bstr_t = bstr_alloc(0);
        let mut end_offset = 0;

        let rc: Status = htp_extract_quoted_string_as_bstr(
            cstr!("\"test\"") as *mut u8,
            6,
            &mut s,
            &mut end_offset,
        );
        assert_eq!(Status::OK, rc);
        assert!(!s.is_null());
        assert_eq!(0, bstr_cmp_str(s, "test"));
        assert_eq!(5, end_offset);
        bstr_free(s);

        let rc = htp_extract_quoted_string_as_bstr(
            cstr!("\"te\\\"st\"") as *mut u8,
            8,
            &mut s,
            &mut end_offset,
        );
        assert_eq!(Status::OK, rc);
        assert!(!s.is_null());
        assert_eq!(0, bstr_cmp_str(s, "te\"st"));
        assert_eq!(7, end_offset);
        bstr_free(s);
    }
}

#[test]
fn Util_NormalizeUriPath() {
    unsafe {
        let s: *mut bstr_t = bstr_dup_str("/a/b/c/./../../g");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, "/a/g"));
        bstr_free(s);

        let s = bstr_dup_str("mid/content=5/../6");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, "mid/6"));
        bstr_free(s);

        let s = bstr_dup_str("./one");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, "one"));
        bstr_free(s);

        let s = bstr_dup_str("../one");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, "one"));
        bstr_free(s);

        let s = bstr_dup_str(".");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, ""));
        bstr_free(s);

        let s = bstr_dup_str("..");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, ""));
        bstr_free(s);

        let s = bstr_dup_str("one/.");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, "one"));
        bstr_free(s);

        let s = bstr_dup_str("one/..");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, ""));
        bstr_free(s);

        let s = bstr_dup_str("one/../");
        htp_normalize_uri_path_inplace(s);
        assert_eq!(0, bstr_cmp_str(s, ""));
        bstr_free(s);
    }
}

#[test]
fn UrlencodedParser_UrlDecode1() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        let mut flags: Flags = Flags::empty();

        let s = bstr_dup_str("/one/tw%u006f/three/%u123");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_urldecode_inplace(
            test.cfg,
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            s,
            &mut flags,
        );
        assert_eq!(0, bstr_cmp_str(s, "/one/two/three/%u123"));
        bstr_free(s);

        let s = bstr_dup_str("/one/tw%u006f/three/%uXXXX");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PRESERVE_PERCENT,
        );
        htp_urldecode_inplace(
            test.cfg,
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            s,
            &mut flags,
        );
        assert_eq!(0, bstr_cmp_str(s, "/one/two/three/%uXXXX"));
        bstr_free(s);

        let s = bstr_dup_str("/one/tw%u006f/three/%u123");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_urldecode_inplace(
            test.cfg,
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            s,
            &mut flags,
        );
        assert_eq!(0, bstr_cmp_str(s, "/one/two/three/u123"));
        bstr_free(s);

        let s = bstr_dup_str("/one/tw%u006f/three/%3");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_REMOVE_PERCENT,
        );
        htp_urldecode_inplace(
            test.cfg,
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            s,
            &mut flags,
        );
        assert_eq!(0, bstr_cmp_str(s, "/one/two/three/3"));
        bstr_free(s);

        let s = bstr_dup_str("/one/tw%u006f/three/%3");
        (*test.cfg).set_u_encoding_decode(htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED, 1);
        (*test.cfg).set_url_encoding_invalid_handling(
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            htp_config::htp_url_encoding_handling_t::HTP_URL_DECODE_PROCESS_INVALID,
        );
        htp_urldecode_inplace(
            test.cfg,
            htp_config::htp_decoder_ctx_t::HTP_DECODER_URLENCODED,
            s,
            &mut flags,
        );
        assert_eq!(0, bstr_cmp_str(s, "/one/two/three/%3"));
        bstr_free(s);
    }
}
