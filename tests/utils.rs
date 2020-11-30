#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
use libc;
use std::net::{IpAddr, Ipv4Addr};

use htp::{
    bstr::Bstr,
    c_api::{htp_connp_create, htp_connp_destroy_all},
    config::{create, Config, HtpUrlEncodingHandling},
    connection_parser::ConnectionParser,
    list::List,
    request::HtpMethod,
    table::Table,
    transaction::Transaction,
    urlencoded::{urlenp_finalize, urlenp_parse_complete, urlenp_parse_partial, Parser},
    util::*,
};
use nom::{
    error::ErrorKind::TakeUntil,
    Err::{Error, Incomplete},
    Needed,
};

// import common testing utilities
mod common;

// Util tests
#[test]
fn Separator() {
    assert_eq!(false, is_separator('a' as u8));
    assert_eq!(false, is_separator('^' as u8));
    assert_eq!(false, is_separator('-' as u8));
    assert_eq!(false, is_separator('_' as u8));
    assert_eq!(false, is_separator('&' as u8));
    assert_eq!(true, is_separator('(' as u8));
    assert_eq!(true, is_separator('\\' as u8));
    assert_eq!(true, is_separator('/' as u8));
    assert_eq!(true, is_separator('=' as u8));
    assert_eq!(true, is_separator('\t' as u8));
}

#[test]
fn Token() {
    assert_eq!(true, is_token('a' as u8));
    assert_eq!(true, is_token('&' as u8));
    assert_eq!(true, is_token('+' as u8));
    assert_eq!(false, is_token('\t' as u8));
    assert_eq!(false, is_token('\n' as u8));
}

fn unsize<T>(x: &[T]) -> &[T] {
    x
}

#[test]
fn Chomp() {
    assert_eq!(chomp(b"test\r\n"), b"test");
    assert_eq!(chomp(b"test\r\n\n"), b"test");
    assert_eq!(chomp(b"test\r\n\r\n"), b"test");
    assert_eq!(chomp(b"te\nst"), b"te\nst");
    assert_eq!(chomp(b"foo\n"), b"foo");
    assert_eq!(chomp(b"arfarf"), b"arfarf");
    assert_eq!(chomp(b""), b"");
}

#[test]
fn Space() {
    assert_eq!(false, is_space(0x61)); // a
    assert_eq!(true, is_space(0x20)); // space
    assert_eq!(true, is_space(0x0c)); // Form feed
    assert_eq!(true, is_space(0x0a)); // newline
    assert_eq!(true, is_space(0x0d)); // carriage return
    assert_eq!(true, is_space(0x09)); // tab
    assert_eq!(true, is_space(0x0b)); // Vertical tab
}

#[test]
fn Method() {
    let method = b"GET";
    assert_eq!(HtpMethod::GET, convert_to_method(method));
}

#[test]
fn IsLineEmpty() {
    let data = b"arfarf";
    assert_eq!(false, is_line_empty(data));
    assert_eq!(true, is_line_empty(b"\x0d\x0a"));
    assert_eq!(true, is_line_empty(b"\x0d"));
    assert_eq!(true, is_line_empty(b"\x0a"));
    assert_eq!(false, is_line_empty(b"\x0a\x0d"));
    assert_eq!(false, is_line_empty(b"\x0dabc"));
}

#[test]
fn IsLineWhitespace() {
    let data = b"arfarf";
    assert_eq!(false, is_line_whitespace(data));
    assert_eq!(true, is_line_whitespace(b"\x0d\x0a"));
    assert_eq!(true, is_line_whitespace(b"\x0d"));
    assert_eq!(false, is_line_whitespace(b"\x0dabc"));
}

#[test]
fn IsLineFolded() {
    assert_eq!(true, is_line_folded(b"\tline"));
    assert_eq!(true, is_line_folded(b" line"));
    assert_eq!(false, is_line_folded(b"line "));
}

#[test]
fn ValidateHostname_1() {
    assert!(validate_hostname(b"www.example.com"));
}

#[test]
fn ValidateHostname_2() {
    assert!(!validate_hostname(b".www.example.com"));
}

#[test]
fn ValidateHostname_3() {
    assert!(!validate_hostname(b"www..example.com"));
}

#[test]
fn ValidateHostname_4() {
    assert!(!validate_hostname(b"www.example.com.."));
}

#[test]
fn ValidateHostname_5() {
    assert!(!validate_hostname(b"www example com"));
}

#[test]
fn ValidateHostname_6() {
    assert!(!validate_hostname(b""));
}

#[test]
fn ValidateHostname_7() {
    // Label over 63 characters.
    assert!(!validate_hostname(
        b"www.exampleexampleexampleexampleexampleexampleexampleexampleexampleexample.com"
    ));
}

#[test]
fn ValidateHostname_8() {
    assert!(validate_hostname(b"www.ExAmplE-1984.com"));
}

#[test]
fn ValidateHostname_9() {
    assert!(validate_hostname(b"[:::]"));
}

#[test]
fn ValidateHostname_10() {
    assert!(!validate_hostname(b"[:::"));
}

#[test]
fn ValidateHostname_11() {
    assert!(!validate_hostname(b"[:::/path[0]"));
}

#[test]
fn ValidateHostname_12() {
    assert!(!validate_hostname(b"[:::#garbage]"));
}

#[test]
fn ValidateHostname_13() {
    assert!(!validate_hostname(b"[:::?]"));
}

struct DecodingTest {
    connp: *mut ConnectionParser,
    tx: *mut Transaction,
}

impl DecodingTest {
    fn new() -> Self {
        let mut ret = Self {
            connp: std::ptr::null_mut(),
            tx: std::ptr::null_mut(),
        };
        unsafe {
            ret.connp = htp_connp_create(&mut Config::default());
            (*ret.connp).open(
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(32768),
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(80),
                None,
            );
            let tx_id = (*ret.connp).create_tx().unwrap();
            ret.tx = (*ret.connp).conn.tx_mut_ptr(tx_id);
        }
        ret
    }
}

impl Drop for DecodingTest {
    fn drop(&mut self) {
        unsafe {
            htp_connp_destroy_all(self.connp);
        }
    }
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace1_Identity() {
    let mut i = Bstr::from("/dest");
    let e = Bstr::from("/dest");
    unsafe {
        let test = DecodingTest::new();
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace2_Urlencoded() {
    let mut i = Bstr::from("/%64est");
    let e = Bstr::from("/dest");
    unsafe {
        let test = DecodingTest::new();
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace3_UrlencodedInvalidPreserve() {
    let mut i = Bstr::from("/%xxest");
    let e = Bstr::from("/%xxest");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace4_UrlencodedInvalidRemove() {
    let mut i = Bstr::from("/%xxest");
    let e = Bstr::from("/xxest");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace5_UrlencodedInvalidDecode() {
    let mut i = Bstr::from("/%}9est");
    let e = Bstr::from("/iest");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace6_UrlencodedInvalidNotEnoughBytes() {
    let mut i = Bstr::from("/%a");
    let e = Bstr::from("/%a");
    unsafe {
        let test = DecodingTest::new();
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace7_UrlencodedInvalidNotEnoughBytes() {
    let mut i = Bstr::from("/%");
    let e = Bstr::from("/%");
    unsafe {
        let test = DecodingTest::new();
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace8_Uencoded() {
    let mut i = Bstr::from("/%u0064");
    let e = Bstr::from("/d");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace9_UencodedDoNotDecode() {
    let mut i = Bstr::from("/%u0064");
    let e = Bstr::from("/%u0064");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(false);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace10_UencodedInvalidNotEnoughBytes() {
    let mut i = Bstr::from("/%u006");
    let e = Bstr::from("/%u006");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace11_UencodedInvalidPreserve() {
    let mut i = Bstr::from("/%u006");
    let e = Bstr::from("/%u006");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace12_UencodedInvalidRemove() {
    let mut i = Bstr::from("/%uXXXX");
    let e = Bstr::from("/uXXXX");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace13_UencodedInvalidDecode() {
    let mut i = Bstr::from("/%u00}9");
    let e = Bstr::from("/i");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace14_UencodedInvalidPreserve() {
    let mut i = Bstr::from("/%u00");
    let e = Bstr::from("/%u00");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace15_UencodedInvalidPreserve() {
    let mut i = Bstr::from("/%u0");
    let e = Bstr::from("/%u0");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace16_UencodedInvalidPreserve() {
    let mut i = Bstr::from("/%u");
    let e = Bstr::from("/%u");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace17_UrlencodedNul() {
    let mut i = Bstr::from("/%00");
    let e = Bstr::from("/\0");
    unsafe {
        let test = DecodingTest::new();
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace18_UrlencodedNulTerminates() {
    let mut i = Bstr::from("/%00ABC");
    let e = Bstr::from("/");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_nul_encoded_terminates(true);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace19_RawNulTerminates() {
    let mut i = Bstr::from("/\0ABC");
    let e = Bstr::from("/");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_nul_raw_terminates(true);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTes_DecodeUrlencodedInplace20_UencodedBestFit() {
    let mut i = Bstr::from("/%u0107");
    let e = Bstr::from("/c");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i).unwrap();
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodeUrlencodedInplace21_UencodedCaseInsensitive() {
    let mut i_lower = Bstr::from("/%u0064");
    let mut i_upper = Bstr::from("/%U0064");
    let e = Bstr::from("/d");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        tx_urldecode_params_inplace(&mut *test.tx, &mut i_lower).unwrap();
        tx_urldecode_params_inplace(&mut *test.tx, &mut i_upper).unwrap();
    }
    assert_eq!(i_upper, e);
    assert_eq!(i_lower, e);
}

#[test]
fn DecodingTest_DecodePathInplace1_UrlencodedInvalidNotEnoughBytes() {
    let mut i = Bstr::from("/%a");
    let e = Bstr::from("/%a");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace2_UencodedInvalidNotEnoughBytes() {
    let mut i = Bstr::from("/%uX");
    let e = Bstr::from("/%uX");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace3_UencodedValid() {
    let mut i = Bstr::from("/%u0107");
    let e = Bstr::from("/c");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace4_UencodedInvalidNotHexDigits_Remove() {
    let mut i = Bstr::from("/%uXXXX");
    let e = Bstr::from("/uXXXX");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace5_UencodedInvalidNotHexDigits_Preserve() {
    let mut i = Bstr::from("/%uXXXX");
    let e = Bstr::from("/%uXXXX");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace6_UencodedInvalidNotHexDigits_Process() {
    let mut i = Bstr::from("/%u00}9");
    let e = Bstr::from("/i");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace7_UencodedNul() {
    let mut i = Bstr::from("/%u0000");
    let e = Bstr::from("/\0");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_ENCODED_NUL));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace8_UencodedNotEnough_Remove() {
    let mut i = Bstr::from("/%uXXX");
    let e = Bstr::from("/uXXX");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace9_UencodedNotEnough_Preserve() {
    let mut i = Bstr::from("/%uXXX");
    let e = Bstr::from("/%uXXX");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace10_UrlencodedNul() {
    let mut i = Bstr::from("/%00123");
    let e = Bstr::from("/\x00123");
    unsafe {
        let test = DecodingTest::new();
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_ENCODED_NUL));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace11_UrlencodedNul_Terminates() {
    let mut i = Bstr::from("/%00123");
    let e = Bstr::from("/");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_nul_encoded_terminates(true);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_ENCODED_NUL));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace12_EncodedSlash() {
    let mut i = Bstr::from("/one%2ftwo");
    let e = Bstr::from("/one%2ftwo");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_path_separators_decode(false);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_ENCODED_SEPARATOR));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace13_EncodedSlash_Decode() {
    let mut i = Bstr::from("/one%2ftwo");
    let e = Bstr::from("/one/two");
    unsafe {
        let test = DecodingTest::new();

        (*test.connp).cfg.set_path_separators_decode(true);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_ENCODED_SEPARATOR));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace14_Urlencoded_Invalid_Preserve() {
    let mut i = Bstr::from("/%HH");
    let e = Bstr::from("/%HH");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace15_Urlencoded_Invalid_Remove() {
    let mut i = Bstr::from("/%HH");
    let e = Bstr::from("/HH");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace16_Urlencoded_Invalid_Process() {
    let mut i = Bstr::from("/%}9");
    let e = Bstr::from("/i");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace17_Urlencoded_NotEnough_Remove() {
    let mut i = Bstr::from("/%H");
    let e = Bstr::from("/H");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace18_Urlencoded_NotEnough_Preserve() {
    let mut i = Bstr::from("/%H");
    let e = Bstr::from("/%H");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace19_Urlencoded_NotEnough_Process() {
    let mut i = Bstr::from("/%H");
    let e = Bstr::from("/%H");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
        assert!((*test.tx).flags.contains(Flags::PATH_INVALID_ENCODING));
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace20_RawNul1() {
    let mut i = Bstr::from("/\x00123");
    let e = Bstr::from("/");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_nul_raw_terminates(true);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace21_RawNul1() {
    let mut i = Bstr::from("/\x00123");
    let e = Bstr::from("/\x00123");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_nul_raw_terminates(false);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace22_ConvertBackslash1() {
    let mut i = Bstr::from("/one\\two");
    let e = Bstr::from("/one/two");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_backslash_convert_slashes(true);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace23_ConvertBackslash2() {
    let mut i = Bstr::from("/one\\two");
    let e = Bstr::from("/one\\two");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_backslash_convert_slashes(false);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace24_CompressSeparators() {
    let mut i = Bstr::from("/one//two");
    let e = Bstr::from("/one/two");
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_path_separators_compress(true);
        decode_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
    }
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_InvalidUtf8() {
    let mut i = Bstr::from(b"\xf1.\xf1\xef\xbd\x9dabcd".to_vec());
    unsafe {
        let test = DecodingTest::new();
        (*test.connp).cfg.set_utf8_convert_bestfit(true);
        utf8_decode_and_validate_uri_path_inplace(
            &(*test.connp).cfg.decoder_cfg,
            &mut (*test.tx).flags,
            &mut (*test.tx).response_status_expected_number,
            &mut i,
        );
    }
    assert!(i.eq("?.?}abcd"));
}

struct UrlEncodedParserTest {
    connp: *mut ConnectionParser,
    cfg: *mut Config,
    tx: *mut Transaction,
}

impl UrlEncodedParserTest {
    fn new() -> Self {
        let mut ret = Self {
            connp: std::ptr::null_mut(),
            cfg: std::ptr::null_mut(),
            tx: std::ptr::null_mut(),
        };
        unsafe {
            ret.cfg = create();
            ret.connp = htp_connp_create(ret.cfg);
            (*ret.connp).open(
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(32768),
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(80),
                None,
            );
            let tx_id = (*ret.connp).create_tx().unwrap();
            ret.tx = (*ret.connp).conn.tx_mut_ptr(tx_id);
            ret
        }
    }
}

impl Drop for UrlEncodedParserTest {
    fn drop(&mut self) {
        unsafe {
            htp_connp_destroy_all(self.connp);
            (*self.cfg).destroy();
        }
    }
}

// Start of Url Parser tests.
#[test]
fn UrlencodedParser_Empty() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"");

    assert_eq!(0, urlenp.params.size());
}

#[test]
fn UrlencodedParser_EmptyKey1() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_EmptyKey2() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"=&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_EmptyKey3() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"=1&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq("1"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_EmptyKey4() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"&=");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_EmptyKey5() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"&&");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_EmptyKeyAndValue() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"=");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_OnePairEmptyValue() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"p=");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_OnePairEmptyKey() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"=p");

    assert!(urlenp.params.get_nocase("").unwrap().1.eq("p"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_OnePair() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"p=1");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq("1"));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_TwoPairs() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"p=1&q=2");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq("1"));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq("2"));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn UrlencodedParser_KeyNoValue1() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"p");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_KeyNoValue2() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"p&");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_KeyNoValue3() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"p&q");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq(""));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn UrlencodedParser_KeyNoValue4() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_complete(&mut urlenp, b"p&q=2");

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert!(urlenp.params.get_nocase("q").unwrap().1.eq("2"));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn UrlencodedParser_Partial1() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_Partial2() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"x");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("px").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_Partial3() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"x&");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("px").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_Partial4() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"=");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_Partial5() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_partial(&mut urlenp, b"p");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("p").unwrap().1.eq(""));
    assert_eq!(1, urlenp.params.size());
}

#[test]
fn UrlencodedParser_Partial6() {
    let test = UrlEncodedParserTest::new();
    let mut urlenp = Parser::new(test.tx);
    urlenp_parse_partial(&mut urlenp, b"px");
    urlenp_parse_partial(&mut urlenp, b"n");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"=");
    urlenp_parse_partial(&mut urlenp, b"1");
    urlenp_parse_partial(&mut urlenp, b"2");
    urlenp_parse_partial(&mut urlenp, b"&");
    urlenp_parse_partial(&mut urlenp, b"qz");
    urlenp_parse_partial(&mut urlenp, b"n");
    urlenp_parse_partial(&mut urlenp, b"");
    urlenp_parse_partial(&mut urlenp, b"=");
    urlenp_parse_partial(&mut urlenp, b"2");
    urlenp_parse_partial(&mut urlenp, b"3");
    urlenp_parse_partial(&mut urlenp, b"&");
    urlenp_finalize(&mut urlenp);

    assert!(urlenp.params.get_nocase("pxn").unwrap().1.eq("12"));
    assert!(urlenp.params.get_nocase("qzn").unwrap().1.eq("23"));
    assert_eq!(2, urlenp.params.size());
}

#[test]
fn List_Misc() {
    unsafe {
        let mut l = List::with_capacity(16);

        l.push("1".as_ptr() as *mut core::ffi::c_void);
        l.push("2".as_ptr() as *mut core::ffi::c_void);
        l.push("3".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(3, l.len());

        let p: *mut i8 = l.pop().unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        assert_eq!(2, l.len());

        let p = l.pop().unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("2".as_ptr() as *mut i8, p));

        let p = l.pop().unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("1".as_ptr() as *mut i8, p));

        let p = l.pop();
        assert!(p.is_none());

        drop(&l);
    }
}

#[test]
fn List_Misc2() {
    unsafe {
        let mut l = List::with_capacity(2);

        l.push("1".as_ptr() as *mut core::ffi::c_void);
        l.push("2".as_ptr() as *mut core::ffi::c_void);
        l.push("3".as_ptr() as *mut core::ffi::c_void);

        let p: *mut i8 = *l.get(2).unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        assert_eq!(3, l.len());

        let _ = l.replace(2, "4".as_ptr() as *mut core::ffi::c_void);

        let p = l.pop().unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("4".as_ptr() as *mut i8, p));

        drop(&l);
    }
}

#[test]
fn List_Expand1() {
    unsafe {
        let mut l = List::with_capacity(2);

        l.push("1".as_ptr() as *mut core::ffi::c_void);
        l.push("2".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(2, l.len());

        l.push("3".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(3, l.len());

        let p: *mut i8 = *l.get(0).unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("1".as_ptr() as *mut i8, p));

        let p = *l.get(1).unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("2".as_ptr() as *mut i8, p));

        let p = *l.get(2).unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        drop(&l);
    }
}

#[test]
fn List_Expand2() {
    unsafe {
        let mut l = List::with_capacity(2);

        l.push("1".as_ptr() as *mut core::ffi::c_void);
        l.push("2".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(2, l.len());

        l.push("3".as_ptr() as *mut core::ffi::c_void);
        l.push("4".as_ptr() as *mut core::ffi::c_void);

        assert_eq!(4, l.len());

        let p: *mut i8 = *l.get(0).unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("1".as_ptr() as *mut i8, p));

        let p = *l.get(1).unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("2".as_ptr() as *mut i8, p));

        let p = *l.get(2).unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("3".as_ptr() as *mut i8, p));

        let p = l.pop().unwrap() as *mut i8;
        assert!(!p.is_null());
        assert_eq!(0, libc::strcmp("4".as_ptr() as *mut i8, p));

        drop(&l);
    }
}

#[test]
fn Table_Misc() {
    let mut t: Table<&str> = Table::with_capacity(2);

    let mut pkey = Bstr::with_capacity(1);
    pkey.add("p");

    let mut qkey = Bstr::with_capacity(1);
    qkey.add("q");

    t.add(pkey, "1");
    t.add(qkey, "2");

    assert!(t.get_nocase("z").is_none());
    assert_eq!("1", t.get_nocase("p").unwrap().1);
}

#[test]
fn UrlencodedParser_UrlDecode1() {
    unsafe {
        let test = UrlEncodedParserTest::new();
        let mut flags: Flags = Flags::empty();

        let mut s = Bstr::from("/one/tw%u006f/three/%u123");
        let mut e = Bstr::from("/one/two/three/%u123");
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        urldecode_inplace(&(*test.connp).cfg.decoder_cfg, &mut s, &mut flags).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%uXXXX");
        e = Bstr::from("/one/two/three/%uXXXX");
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
        urldecode_inplace(&(*test.connp).cfg.decoder_cfg, &mut s, &mut flags).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%u123");
        e = Bstr::from("/one/two/three/u123");
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        urldecode_inplace(&(*test.connp).cfg.decoder_cfg, &mut s, &mut flags).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%3");
        e = Bstr::from("/one/two/three/3");
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
        urldecode_inplace(&(*test.connp).cfg.decoder_cfg, &mut s, &mut flags).unwrap();
        assert_eq!(e, s);

        s = Bstr::from("/one/tw%u006f/three/%3");
        e = Bstr::from("/one/two/three/%3");
        (*test.connp).cfg.set_u_encoding_decode(true);
        (*test.connp)
            .cfg
            .set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
        urldecode_inplace(&(*test.connp).cfg.decoder_cfg, &mut s, &mut flags).unwrap();
        assert_eq!(e, s);
    }
}

#[test]
fn TakeUntilNull() {
    assert_eq!(
        Ok(("\0   ".as_bytes(), "hello_world  ".as_bytes())),
        take_until_null(b"hello_world  \0   ")
    );
    assert_eq!(
        Ok(("\0\0\0\0".as_bytes(), "hello".as_bytes())),
        take_until_null(b"hello\0\0\0\0")
    );
    assert_eq!(Ok(("\0".as_bytes(), "".as_bytes())), take_until_null(b"\0"));
}

#[test]
fn TakeIsSpaceTrailing() {
    assert_eq!(
        Ok(("w0rd".as_bytes(), "   ".as_bytes())),
        take_is_space_trailing(b"w0rd   ")
    );
    assert_eq!(
        Ok(("word".as_bytes(), "   \t".as_bytes())),
        take_is_space_trailing(b"word   \t")
    );
    assert_eq!(
        Ok(("w0rd".as_bytes(), "".as_bytes())),
        take_is_space_trailing(b"w0rd")
    );
    assert_eq!(
        Ok(("\t  w0rd".as_bytes(), "   ".as_bytes())),
        take_is_space_trailing(b"\t  w0rd   ")
    );
    assert_eq!(
        Ok(("".as_bytes(), "     ".as_bytes())),
        take_is_space_trailing(b"     ")
    );
}

fn TakeIsSpace() {
    assert_eq!(
        Ok(("hello".as_bytes(), "   ".as_bytes())),
        take_is_space(b"   hello")
    );
    assert_eq!(
        Ok(("hell o".as_bytes(), "   \t".as_bytes())),
        take_is_space(b"   \thell o")
    );
    assert_eq!(
        Ok(("hell o".as_bytes(), "".as_bytes())),
        take_is_space(b"hell o")
    );
    assert_eq!(
        Ok(("hell o".as_bytes(), "\r\x0b".as_bytes())),
        take_is_space(b"\r\x0bhell o")
    );
    assert_eq!(
        Ok(("hell \to".as_bytes(), "\r\x0b  \t".as_bytes())),
        take_is_space(b"\r\x0b  \thell \to")
    )
}

#[test]
fn TreatResponseLineAsBody() {
    assert_eq!(false, treat_response_line_as_body(b"   http 1.1"));
    assert_eq!(false, treat_response_line_as_body(b"http"));
    assert_eq!(false, treat_response_line_as_body(b"HTTP"));
    assert_eq!(false, treat_response_line_as_body(b"    HTTP"));
    assert_eq!(true, treat_response_line_as_body(b"test"));
    assert_eq!(true, treat_response_line_as_body(b"     test"));
    assert_eq!(true, treat_response_line_as_body(b""));
    assert_eq!(true, treat_response_line_as_body(b"kfgjl  hTtp "));
}

#[test]
fn RemoveLWS() {
    assert_eq!(
        Ok(("hello".as_bytes(), "   ".as_bytes())),
        take_is_space(b"   hello")
    );
    assert_eq!(
        Ok(("hell o".as_bytes(), "   \t".as_bytes())),
        take_is_space(b"   \thell o")
    );
    assert_eq!(
        Ok(("hell o".as_bytes(), "".as_bytes())),
        take_is_space(b"hell o")
    );
}

#[test]
fn SplitByColon() {
    assert_eq!(
        Ok(("Content-Length".as_bytes(), "230".as_bytes())),
        split_by_colon(b"Content-Length: 230")
    );
    assert_eq!(
        Ok(("".as_bytes(), "No header name".as_bytes())),
        split_by_colon(b":No header name")
    );
    assert_eq!(
        Ok(("Header@Name".as_bytes(), "Not Token".as_bytes())),
        split_by_colon(b"Header@Name: Not Token")
    );
    assert_eq!(
        Err(Error(("No colon".as_bytes(), TakeUntil))),
        split_by_colon(b"No colon")
    );
}

#[test]
fn IsWordToken() {
    assert_eq!(true, is_word_token(b"allalpha"));
    assert_eq!(true, is_word_token(b"alpha567numeric1234"));
    assert_eq!(false, is_word_token(b"alpha{}"));
    assert_eq!(false, is_word_token(b"\n"));
    assert_eq!(true, is_word_token(b"234543"));
    assert_eq!(false, is_word_token(b"abcdeg\t"));
    assert_eq!(true, is_word_token(b"content-length"));
}

#[test]
fn TakeNotEol() {
    assert_eq!(
        Ok(("\n".as_bytes(), "header:value\r".as_bytes())),
        take_not_eol(b"header:value\r\n")
    );
    assert_eq!(
        Err(Incomplete(Needed::Size(1))),
        take_not_eol(b"header:value")
    );
}

#[test]
fn TakeTillLF() {
    assert_eq!(
        Ok(("hijk".as_bytes(), "abcdefg\n".as_bytes())),
        take_till_lf(b"abcdefg\nhijk")
    );
    assert_eq!(Err(Incomplete(Needed::Size(1))), take_till_lf(b"abcdefg"));
}

#[test]
fn SepByLineEndings() {
    let sep = res_sep_by_line_endings(b"Content-Type: test/html\r\n");
    let res = vec![&b"Content-Type: test/html"[..], &b"\r\n"[..]];
    assert_eq!(sep, Ok((&b""[..], res)));
    let sep = res_sep_by_line_endings(b"Content-Type: test/html\r\nContent-Length: 6\r\n\r\n");
    let res = vec![
        &b"Content-Type: test/html"[..],
        &b"\r\n"[..],
        &b"Content-Length: 6"[..],
        &b"\r\n\r\n"[..],
    ];
    assert_eq!(sep, Ok((&b""[..], res)));

    let sep = res_sep_by_line_endings(b"Content-Type: test/html\nContent-Length: 6\n\n");
    let res = vec![
        &b"Content-Type: test/html"[..],
        &b"\n"[..],
        &b"Content-Length: 6"[..],
        &b"\n\n"[..],
    ];
    assert_eq!(sep, Ok((&b""[..], res)));

    let sep = res_sep_by_line_endings(b"Content-Type: test/html\rContent-Length: 6\r\r");
    let res = vec![
        &b"Content-Type: test/html"[..],
        &b"\r"[..],
        &b"Content-Length: 6"[..],
        &b"\r\r"[..],
    ];
    assert_eq!(sep, Ok((&b""[..], res)));

    let sep = res_sep_by_line_endings(b"Content-Type: test/html\r\nContent-Length: 6\n\r\r\n\r\n");
    let res = vec![
        &b"Content-Type: test/html"[..],
        &b"\r\n"[..],
        &b"Content-Length: 6"[..],
        &b"\n\r\r\n\r\n"[..],
    ];
    assert_eq!(sep, Ok((&b""[..], res)));

    // Incomplete line
    let sep = res_sep_by_line_endings(b"Content-Type: test/html\r\nContent-Le");
    let res = vec![
        &b"Content-Type: test/html"[..],
        &b"\r\n"[..],
        &b"Content-Le"[..],
    ];
    assert_eq!(sep, Ok((&b""[..], res)));
}
