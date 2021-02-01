#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use htp::{
    bstr::Bstr,
    config::{Config, DecoderConfig, HtpUnwanted, HtpUrlEncodingHandling},
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

#[test]
fn DecodeUrlencodedEx1_Identity() {
    let i = Bstr::from("/dest");
    let e = "/dest".as_bytes();
    let cfg = DecoderConfig::default();
    assert_eq!(e, urldecode_ex(&i, &cfg).unwrap().1 .0);
}

#[test]
fn DecodeUrlencodedEx2_Urlencoded() {
    let i = Bstr::from("/%64est");
    let e = "/dest".as_bytes();
    let cfg = DecoderConfig::default();
    assert_eq!(e, urldecode_ex(&i, &cfg).unwrap().1 .0);
}

#[test]
fn DecodeUrlencodedEx3_UrlencodedInvalidPreserve() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let i = Bstr::from("/%xxest");
    let e = "/%xxest".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx4_UrlencodedInvalidRemove() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    let i = Bstr::from("/%xxest");
    let e = "/xxest".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx5_UrlencodedInvalidDecode() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    let i = Bstr::from("/%}9est");
    let e = "/iest".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx6_UrlencodedInvalidNotEnoughBytes() {
    let cfg = DecoderConfig::default();
    let i = Bstr::from("/%a");
    let e = "/%a".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx7_UrlencodedInvalidNotEnoughBytes() {
    let cfg = DecoderConfig::default();
    let i = Bstr::from("/%");
    let e = "/%".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx8_Uencoded() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    let i = Bstr::from("/%u0064");
    let e = "/d".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx9_UencodedDoNotDecode() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    cfg.set_u_encoding_decode(false);
    let i = Bstr::from("/%u0064");
    let e = "/%u0064".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx10_UencodedInvalidNotEnoughBytes() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    let i = Bstr::from("/%u006");
    let e = "/%u006".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx11_UencodedInvalidPreserve() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let i = Bstr::from("/%u006");
    let e = "/%u006".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx12_UencodedInvalidRemove() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    let i = Bstr::from("/%uXXXX");
    let e = "/uXXXX".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx13_UencodedInvalidDecode() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    let i = Bstr::from("/%u00}9");
    let e = "/i".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx14_UencodedInvalidPreserve() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let i = Bstr::from("/%u00");
    let e = "/%u00".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx15_UencodedInvalidPreserve() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let i = Bstr::from("/%u0");
    let e = "/%u0".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx16_UencodedInvalidPreserve() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let i = Bstr::from("/%u");
    let e = "/%u".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx17_UrlencodedNul() {
    let cfg = DecoderConfig::default();
    let i = Bstr::from("/%00");
    let e = "/\0".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx18_UrlencodedNulTerminates() {
    let mut cfg = Config::default();
    cfg.set_nul_encoded_terminates(true);
    let i = Bstr::from("/%00ABC");
    let e = "/".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx19_RawNulTerminates() {
    let mut cfg = Config::default();
    cfg.set_nul_raw_terminates(true);
    let i = Bstr::from("/\0ABC");
    let e = "/".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx20_UencodedBestFit() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    let i = Bstr::from("/%u0107");
    let e = "/c".as_bytes();
    assert_eq!(urldecode_ex(&i, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodeUrlencodedEx21_UencodedCaseInsensitive() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    let i_lower = Bstr::from("/%u0064");
    let i_upper = Bstr::from("/%U0064");
    let e = "/d".as_bytes();
    assert_eq!(urldecode_ex(&i_upper, &cfg.decoder_cfg).unwrap().1 .0, e);
    assert_eq!(urldecode_ex(&i_lower, &cfg.decoder_cfg).unwrap().1 .0, e);
}

#[test]
fn DecodingTest_DecodePathInplace1_UrlencodedInvalidNotEnoughBytes() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    let mut i = Bstr::from("/%a");
    let e = Bstr::from("/%a");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace2_UencodedInvalidNotEnoughBytes() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%uX");
    let e = Bstr::from("/%uX");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace3_UencodedValid() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%u0107");
    let e = Bstr::from("/c");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace4_UencodedInvalidNotHexDigits_Remove() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%uXXXX");
    let e = Bstr::from("/uXXXX");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace5_UencodedInvalidNotHexDigits_Preserve() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%uXXXX");
    let e = Bstr::from("/%uXXXX");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace6_UencodedInvalidNotHexDigits_Process() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%u00}9");
    let e = Bstr::from("/i");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace7_UencodedNul() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%u0000");
    let e = Bstr::from("/\0");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_ENCODED_NUL));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace8_UencodedNotEnough_Remove() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%uXXX");
    let e = Bstr::from("/uXXX");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace9_UencodedNotEnough_Preserve() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    cfg.set_u_encoding_decode(true);
    let mut i = Bstr::from("/%uXXX");
    let e = Bstr::from("/%uXXX");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace10_UrlencodedNul() {
    let mut i = Bstr::from("/%00123");
    let e = Bstr::from("/\x00123");
    let cfg = DecoderConfig::default();
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_ENCODED_NUL));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace11_UrlencodedNul_Terminates() {
    let mut cfg = Config::default();
    cfg.set_nul_encoded_terminates(true);
    let mut i = Bstr::from("/%00123");
    let e = Bstr::from("/");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_ENCODED_NUL));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace12_EncodedSlash() {
    let mut cfg = Config::default();
    cfg.set_path_separators_decode(false);
    let mut i = Bstr::from("/one%2ftwo");
    let e = Bstr::from("/one%2ftwo");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_ENCODED_SEPARATOR));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace13_EncodedSlash_Decode() {
    let mut cfg = Config::default();
    cfg.set_path_separators_decode(true);
    let mut i = Bstr::from("/one%2ftwo");
    let e = Bstr::from("/one/two");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_ENCODED_SEPARATOR));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace14_Urlencoded_Invalid_Preserve() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let mut i = Bstr::from("/%HH");
    let e = Bstr::from("/%HH");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace15_Urlencoded_Invalid_Remove() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    let mut i = Bstr::from("/%HH");
    let e = Bstr::from("/HH");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace16_Urlencoded_Invalid_Process() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    let mut i = Bstr::from("/%}9");
    let e = Bstr::from("/i");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace17_Urlencoded_NotEnough_Remove() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    let mut i = Bstr::from("/%H");
    let e = Bstr::from("/H");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace18_Urlencoded_NotEnough_Preserve() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let mut i = Bstr::from("/%H");
    let e = Bstr::from("/%H");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace19_Urlencoded_NotEnough_Process() {
    let mut cfg = Config::default();
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    let mut i = Bstr::from("/%H");
    let e = Bstr::from("/%H");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(flags.is_set(HtpFlags::PATH_INVALID_ENCODING));
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace20_RawNul1() {
    let mut cfg = Config::default();
    cfg.set_nul_raw_terminates(true);
    let mut i = Bstr::from("/\x00123");
    let e = Bstr::from("/");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace21_RawNul1() {
    let mut cfg = Config::default();
    cfg.set_nul_raw_terminates(false);
    let mut i = Bstr::from("/\x00123");
    let e = Bstr::from("/\x00123");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace22_ConvertBackslash1() {
    let mut cfg = Config::default();
    cfg.set_backslash_convert_slashes(true);
    let mut i = Bstr::from("/one\\two");
    let e = Bstr::from("/one/two");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace23_ConvertBackslash2() {
    let mut cfg = Config::default();
    cfg.set_backslash_convert_slashes(false);
    let mut i = Bstr::from("/one\\two");
    let e = Bstr::from("/one\\two");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_DecodePathInplace24_CompressSeparators() {
    let mut cfg = Config::default();
    cfg.set_path_separators_compress(true);
    let mut i = Bstr::from("/one//two");
    let e = Bstr::from("/one/two");
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    decode_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert_eq!(i, e);
}

#[test]
fn DecodingTest_InvalidUtf8() {
    let mut cfg = Config::default();
    cfg.set_utf8_convert_bestfit(true);
    let mut i = Bstr::from(b"\xf1.\xf1\xef\xbd\x9dabcd".to_vec());
    let mut flags = 0;
    let mut response_status_expected_number = HtpUnwanted::IGNORE;
    utf8_decode_and_validate_uri_path_inplace(
        &cfg.decoder_cfg,
        &mut flags,
        &mut response_status_expected_number,
        &mut i,
    );
    assert!(i.eq("?.?}abcd"));
}

#[test]
fn UrlencodedParser_UrlDecode1() {
    let mut cfg = Config::default();
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    let mut flags: u64 = 0;
    let mut s = Bstr::from("/one/tw%u006f/three/%u123");
    let mut e = Bstr::from("/one/two/three/%u123");

    urldecode_inplace(&cfg.decoder_cfg, &mut s, &mut flags).unwrap();
    assert_eq!(e, s);

    s = Bstr::from("/one/tw%u006f/three/%uXXXX");
    e = Bstr::from("/one/two/three/%uXXXX");
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PRESERVE_PERCENT);
    urldecode_inplace(&cfg.decoder_cfg, &mut s, &mut flags).unwrap();
    assert_eq!(e, s);

    s = Bstr::from("/one/tw%u006f/three/%u123");
    e = Bstr::from("/one/two/three/u123");
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    urldecode_inplace(&cfg.decoder_cfg, &mut s, &mut flags).unwrap();
    assert_eq!(e, s);

    s = Bstr::from("/one/tw%u006f/three/%3");
    e = Bstr::from("/one/two/three/3");
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::REMOVE_PERCENT);
    urldecode_inplace(&cfg.decoder_cfg, &mut s, &mut flags).unwrap();
    assert_eq!(e, s);

    s = Bstr::from("/one/tw%u006f/three/%3");
    e = Bstr::from("/one/two/three/%3");
    cfg.set_u_encoding_decode(true);
    cfg.set_url_encoding_invalid_handling(HtpUrlEncodingHandling::PROCESS_INVALID);
    urldecode_inplace(&cfg.decoder_cfg, &mut s, &mut flags).unwrap();
    assert_eq!(e, s);
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

#[test]
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
