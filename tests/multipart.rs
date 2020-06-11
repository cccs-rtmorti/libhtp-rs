#![allow(non_snake_case)]
use htp::bstr::*;
use htp::htp_config;
use htp::htp_config::htp_server_personality_t::*;
use htp::htp_connection_parser::*;
use htp::htp_list::*;
use htp::htp_multipart::*;
use htp::htp_request::*;
use htp::htp_table::*;
use htp::htp_transaction::*;
use htp::Status;
use libc::calloc;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;

macro_rules! cstr {
    ( $x:expr ) => {{
        CString::new($x).unwrap().as_ptr()
    }};
}

struct Test {
    connp: *mut htp_connp_t,
    cfg: *mut htp_config::htp_cfg_t,
    body: *mut htp_multipart_t,
    mpartp: *mut htp_mpartp_t,
    tx: *mut htp_tx_t,
}

impl Test {
    fn new() -> Self {
        unsafe {
            let cfg: *mut htp_config::htp_cfg_t = htp_config::create();
            assert!(!cfg.is_null());
            (*cfg).set_server_personality(HTP_SERVER_APACHE_2);
            (*cfg).register_multipart_parser();
            let connp = htp_connp_create(cfg);
            assert!(!connp.is_null());
            let body = std::ptr::null_mut();
            let mpartp = std::ptr::null_mut();
            let tx = std::ptr::null_mut();
            Test {
                connp,
                cfg,
                body,
                mpartp,
                tx,
            }
        }
    }

    fn parseRequest(&mut self, headers: &Vec<&str>, data: &Vec<&str>) {
        unsafe {
            // Open connection
            htp_connp_open(
                self.connp,
                cstr!("127.0.0.1"),
                32768,
                cstr!("127.0.0.1"),
                80,
                std::ptr::null_mut(),
            );

            // Send headers
            for header in headers {
                htp_connp_req_data(
                    self.connp,
                    std::ptr::null_mut(),
                    header.as_ptr() as *const core::ffi::c_void,
                    header.chars().count(),
                );
            }

            // Calculate body length.
            let mut bodyLen: usize = 0;
            for d in data {
                bodyLen += d.chars().count();
            }

            let contentStr = format!("Content-Length: {}\r\n", bodyLen);
            htp_connp_req_data(
                self.connp,
                std::ptr::null_mut(),
                contentStr.as_ptr() as *const core::ffi::c_void,
                contentStr.chars().count(),
            );

            htp_connp_req_data(
                self.connp,
                std::ptr::null_mut(),
                "\r\n".as_ptr() as *const core::ffi::c_void,
                2,
            );

            // Send data.
            for d in data {
                htp_connp_req_data(
                    self.connp,
                    std::ptr::null_mut(),
                    d.as_ptr() as *const core::ffi::c_void,
                    d.chars().count(),
                );
            }

            assert_eq!(1, htp_list_array_size((*(*self.connp).conn).transactions));

            self.tx = htp_list_array_get((*(*self.connp).conn).transactions, 0) as *mut htp_tx_t;

            assert!(!self.tx.is_null());
            assert!(!(*self.tx).request_mpartp.is_null());
            self.mpartp = (*self.tx).request_mpartp;
            self.body = htp_mpartp_get_multipart(self.mpartp);
            assert!(!self.body.is_null());
        }
    }

    fn parseRequestThenVerify(&mut self, headers: &Vec<&str>, data: &Vec<&str>) {
        self.parseRequest(headers, data);
        unsafe {
            assert!(!(*self.body).parts.is_null());
            assert_eq!(3, htp_list_array_size((*self.body).parts));

            assert!(!(*self.body)
                .flags
                .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));

            // Field 1
            let field1 = htp_list_array_get((*self.body).parts, 0) as *mut htp_multipart_part_t;
            assert!(!field1.is_null());
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*field1).type_0);
            assert!(!(*field1).name.is_null());
            assert_eq!(0, bstr_cmp_str((*field1).name, "field1"));
            assert!(!(*field1).value.is_null());
            assert_eq!(0, bstr_cmp_str((*field1).value, "ABCDEF"));

            // File 1
            let file1 = htp_list_array_get((*self.body).parts, 1) as *mut htp_multipart_part_t;
            assert!(!file1.is_null());
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*file1).type_0);
            assert!(!(*file1).name.is_null());
            assert_eq!(0, bstr_cmp_str((*file1).name, "file1"));
            assert!(!(*(*file1).file).filename.is_null());
            assert_eq!(0, bstr_cmp_str((*(*file1).file).filename, "file.bin"));

            // Field 2
            let field2 = htp_list_array_get((*self.body).parts, 2) as *mut htp_multipart_part_t;
            assert!(!field2.is_null());
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*field2).type_0);
            assert!(!(*field2).name.is_null());
            assert_eq!(0, bstr_cmp_str((*field2).name, "field2"));
            assert!(!(*field2).value.is_null());
            assert_eq!(0, bstr_cmp_str((*field2).value, "GHIJKL"));
        }
    }

    fn parseParts(&mut self, parts: &Vec<&str>) {
        unsafe {
            self.mpartp = htp_mpartp_create(
                self.cfg,
                bstr_dup_str("0123456789"),
                MultipartFlags::empty(),
            );
            assert!(!self.mpartp.is_null());
            for part in parts {
                htp_mpartp_parse(
                    self.mpartp,
                    part.as_ptr() as *const core::ffi::c_void,
                    part.chars().count(),
                );
            }

            htp_mpartp_finalize(self.mpartp);
            self.body = htp_mpartp_get_multipart(self.mpartp);
            assert!(!self.body.is_null());
        }
    }

    fn parsePartsThenVerify(&mut self, parts: &Vec<&str>) {
        self.parseParts(parts);

        unsafe {
            // Examine the result
            self.body = htp_mpartp_get_multipart(self.mpartp);
            assert!(!self.body.is_null());
            assert_eq!(2, htp_list_array_size((*self.body).parts));

            let mut part = htp_list_array_get((*self.body).parts, 0) as *mut htp_multipart_part_t;
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*part).type_0);
            assert!(!(*part).name.is_null());
            assert_eq!(0, bstr_cmp_str((*part).name, "field1"));
            assert!(!(*part).value.is_null());
            assert_eq!(0, bstr_cmp_str((*part).value, "ABCDEF"));

            part = htp_list_array_get((*self.body).parts, 1) as *mut htp_multipart_part_t;
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*part).type_0);
            assert!(!(*part).name.is_null());
            assert_eq!(0, bstr_cmp_str((*part).name, "field2"));
            assert!(!(*part).value.is_null());
            assert_eq!(0, bstr_cmp_str((*part).value, "GHIJKL"));
        }
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            if !self.mpartp.is_null() {
                htp_mpartp_destroy(self.mpartp);
            }

            htp_connp_destroy(self.connp);
            (*self.cfg).destroy();
        }
    }
}

#[test]
fn Test1() {
    let mut t = Test::new();
    unsafe {
        t.mpartp = htp_mpartp_create(
            t.cfg,
            bstr_dup_str("---------------------------41184676334"),
            MultipartFlags::empty(),
        );

        let parts = vec![
            "-----------------------------41184676334\r\n",
            "Content-Disposition: form-data;\n name=\"field1\"\r\n",
            "\r\n",
            "0123456789\r\n-",
            "-------------",
            "---------------41184676334\r\n",
            "Content-Disposition: form-data;\n name=\"field2\"\r\n",
            "\r\n",
            "0123456789\r\n-",
            "-------------",
            "--------------X\r\n",
            "-----------------------------41184676334\r\n",
            "Content-Disposition: form-data;\n",
            " ",
            "name=\"field3\"\r\n",
            "\r\n",
            "9876543210\r\n",
            "-----------------------------41184676334\r\n",
            "Content-Disposition: form-data; name=\"file1\"; filename=\"New Text Document.txt\"\r\nContent-Type: text/plain\r\n\r\n",
            "1FFFFFFFFFFFFFFFFFFFFFFFFFFF\r\n",
            "2FFFFFFFFFFFFFFFFFFFFFFFFFFE\r",
            "3FFFFFFFFFFFFFFFFFFFFFFFFFFF\r\n4FFFFFFFFFFFFFFFFFFFFFFFFF123456789",
            "\r\n",
            "-----------------------------41184676334\r\n",
            "Content-Disposition: form-data; name=\"file2\"; filename=\"New Text Document.txt\"\r\n",
            "Content-Type: text/plain\r\n",
            "\r\n",
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFZ",
            "\r\n-----------------------------41184676334--"
    ];

        for part in parts {
            htp_mpartp_parse(
                t.mpartp,
                part.as_ptr() as *const core::ffi::c_void,
                part.chars().count(),
            );
        }

        htp_mpartp_finalize(t.mpartp);

        // Examine the result
        t.body = htp_mpartp_get_multipart(t.mpartp);
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(5, htp_list_array_size((*t.body).parts));

        let mut part = htp_list_array_get((*t.body).parts, 0) as *mut htp_multipart_part_t;
        assert!(!(*part).name.is_null());
        assert_eq!(0, bstr_cmp_str((*part).name, "field1"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*part).type_0);
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "0123456789"));

        part = htp_list_array_get((*t.body).parts, 1) as *mut htp_multipart_part_t;
        assert!(!(*part).name.is_null());
        assert_eq!(0, bstr_cmp_str((*part).name, "field2"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*part).type_0);
        assert!(!(*part).value.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*part).value, "0123456789\r\n----------------------------X")
        );

        part = htp_list_array_get((*t.body).parts, 2) as *mut htp_multipart_part_t;
        assert!(!(*part).name.is_null());
        assert_eq!(0, bstr_cmp_str((*part).name, "field3"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*part).type_0);
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "9876543210"));

        part = htp_list_array_get((*t.body).parts, 3) as *mut htp_multipart_part_t;
        assert!(!(*part).name.is_null());
        assert_eq!(0, bstr_cmp_str((*part).name, "file1"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*part).type_0);

        part = htp_list_array_get((*t.body).parts, 4) as *mut htp_multipart_part_t;
        assert!(!(*part).name.is_null());
        assert_eq!(0, bstr_cmp_str((*part).name, "file2"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*part).type_0);

        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_INCOMPLETE));

        htp_mpartp_destroy(t.mpartp);
        t.mpartp = std::ptr::null_mut();
    }
}

#[test]
fn Test2() {
    let mut t = Test::new();
    unsafe {
        t.mpartp = htp_mpartp_create(t.cfg, bstr_dup_str("BBB"), MultipartFlags::empty());

        let parts = vec![
            "x0000x\n--BBB\n\nx1111x\n--\nx2222x\n--",
            "BBB\n\nx3333x\n--B",
            "B\n\nx4444x\n--BB\r",
            "\n--B",
            "B",
            "B\n\nx5555x\r",
            "\n--x6666x\r",
            "-",
            "-",
        ];

        for part in parts {
            htp_mpartp_parse(
                t.mpartp,
                part.as_ptr() as *const core::ffi::c_void,
                part.chars().count(),
            );
        }

        htp_mpartp_finalize(t.mpartp);

        t.body = htp_mpartp_get_multipart(t.mpartp);
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(4, htp_list_array_size((*t.body).parts));

        let mut part = htp_list_array_get((*t.body).parts, 0) as *mut htp_multipart_part_t;
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_PREAMBLE,
            (*part).type_0
        );
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "x0000x"));

        part = htp_list_array_get((*t.body).parts, 1) as *mut htp_multipart_part_t;
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_UNKNOWN, (*part).type_0);
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "x1111x\n--\nx2222x"));

        part = htp_list_array_get((*t.body).parts, 2) as *mut htp_multipart_part_t;
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_UNKNOWN, (*part).type_0);
        assert!(!(*part).value.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*part).value, "x3333x\n--BB\n\nx4444x\n--BB")
        );

        part = htp_list_array_get((*t.body).parts, 3) as *mut htp_multipart_part_t;
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_UNKNOWN, (*part).type_0);
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "x5555x\r\n--x6666x\r--"));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));

        htp_mpartp_destroy(t.mpartp);
        t.mpartp = std::ptr::null_mut();
    }
}

#[test]
fn Test3() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n",
        "--0",
        "1",
        "2",
        "4: Value\r\n",
        "\r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);
}

#[test]
fn BeginsWithoutLine() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);
}

#[test]
fn BeginsWithCrLf() {
    let mut t = Test::new();
    let parts = vec![
        "\r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);
}

#[test]
fn BeginsWithLf() {
    let mut t = Test::new();
    let parts = vec![
        "\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);
}

#[test]
fn CrLfLineEndings() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_LF_LINE));
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CRLF_LINE));
    }
}

#[test]
fn LfLineEndings() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\n\
         Content-Disposition: form-data; name=\"field1\"\n\
         \n\
         ABCDEF\
         \n--0123456789\n\
         Content-Disposition: form-data; name=\"field2\"\n\
         \n\
         GHIJKL\
         \n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_LF_LINE));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CRLF_LINE));
    }
}

#[test]
fn CrAndLfLineEndings1() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\n\
         Content-Disposition: form-data; name=\"field1\"\n\
         \n\
         ABCDEF\
         \r\n--0123456789\n\
         Content-Disposition: form-data; name=\"field2\"\n\
         \n\
         GHIJKL\
         \n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_LF_LINE));
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CRLF_LINE));
    }
}

#[test]
fn CrAndLfLineEndings2() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\n\
         \n\
         ABCDEF\
         \n--0123456789\n\
         Content-Disposition: form-data; name=\"field2\"\n\
         \n\
         GHIJKL\
         \n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_LF_LINE));
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CRLF_LINE));
    }
}

#[test]
fn CrAndLfLineEndings3() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_LF_LINE));
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CRLF_LINE));
    }
}

#[test]
fn CrAndLfLineEndings4() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_LF_LINE));
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CRLF_LINE));
    }
}

#[test]
fn BoundaryInstanceWithLwsAfter() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789 \r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_BBOUNDARY_LWS_AFTER));
    }
}

#[test]
fn BoundaryInstanceWithNonLwsAfter1() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789 X \r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_BBOUNDARY_NLWS_AFTER));
    }
}

#[test]
fn BoundaryInstanceWithNonLwsAfter2() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789-\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_BBOUNDARY_NLWS_AFTER));
    }
}

#[test]
fn BoundaryInstanceWithNonLwsAfter3() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789\r\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);

    assert!(!t.body.is_null());
    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_BBOUNDARY_NLWS_AFTER));
    }
}

#[test]
fn WithPreamble() {
    let mut t = Test::new();
    let parts = vec![
        "Preamble\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789 X \r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_PREAMBLE));

        let part = htp_list_array_get((*t.body).parts, 0) as *mut htp_multipart_part_t;
        assert!(!part.is_null());
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_PREAMBLE,
            (*part).type_0
        );
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "Preamble"));
    }
}

#[test]
fn WithEpilogue1() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--\r\n\
         Epilogue",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let part = htp_list_array_get((*t.body).parts, 2) as *mut htp_multipart_part_t;
        assert!(!part.is_null());
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*part).type_0
        );
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "Epilogue"));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_INCOMPLETE));
    }
}

#[test]
fn WithEpilogue2() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--\r\n\
         Epi\nlogue",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let part = htp_list_array_get((*t.body).parts, 2) as *mut htp_multipart_part_t;
        assert!(!part.is_null());
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*part).type_0
        );
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "Epi\nlogue"));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_INCOMPLETE));
    }
}

#[test]
fn WithEpilogue3() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--\r\n\
         Epi\r",
        "\n--logue",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let part = htp_list_array_get((*t.body).parts, 2) as *mut htp_multipart_part_t;
        assert!(!part.is_null());
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*part).type_0
        );
        assert!(!(*part).value.is_null());
        assert_eq!(0, bstr_cmp_str((*part).value, "Epi\r\n--logue"));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_INCOMPLETE));
    }
}

#[test]
fn WithEpilogue4() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--\r\n\
         Epilogue1\
         \r\n--0123456789--\r\n\
         Epilogue2",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(4, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let ep1 = htp_list_array_get((*t.body).parts, 2) as *mut htp_multipart_part_t;
        assert!(!ep1.is_null());
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_EPILOGUE, (*ep1).type_0);
        assert!(!(*ep1).value.is_null());
        assert_eq!(0, bstr_cmp_str((*ep1).value, "Epilogue1"));

        let ep2 = htp_list_array_get((*t.body).parts, 3) as *mut htp_multipart_part_t;
        assert!(!ep2.is_null());
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_EPILOGUE, (*ep2).type_0);
        assert!(!(*ep2).value.is_null());
        assert_eq!(0, bstr_cmp_str((*ep2).value, "Epilogue2"));

        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_INCOMPLETE));
    }
}

#[test]
fn HasLastBoundary() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(2, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_SEEN_LAST_BOUNDARY));
    }
}

#[test]
fn DoesNotHaveLastBoundary() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789",
    ];

    t.parsePartsThenVerify(&parts);

    unsafe {
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_SEEN_LAST_BOUNDARY));
    }
}

#[test]
fn PartAfterLastBoundary() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789--\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789",
    ];

    t.parsePartsThenVerify(&parts);

    unsafe {
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_SEEN_LAST_BOUNDARY));
    }
}

#[test]
fn UnknownPart() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789--",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(1, htp_list_array_size((*t.body).parts));

        let part = htp_list_array_get((*t.body).parts, 0) as *mut htp_multipart_part_t;
        assert!(!part.is_null());
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_UNKNOWN, (*part).type_0);
    }
}

#[test]
fn WithFile() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"; filename=\"test.bin\"\r\n\
         Content-Type: application/octet-stream \r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseParts(&parts);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(2, htp_list_array_size((*t.body).parts));

        let part = htp_list_array_get((*t.body).parts, 1) as *mut htp_multipart_part_t;
        assert!(!part.is_null());
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*part).type_0);
        assert!(!(*part).content_type.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*part).content_type, "application/octet-stream")
        );
        assert!(!(*part).file.is_null());
        assert_eq!(0, bstr_cmp_str((*(*part).file).filename, "test.bin"));
        assert_eq!(6, (*(*part).file).len);
    }
}

#[test]
fn WithFileExternallyStored() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"; filename=\"test.bin\"\r\n\
         Content-Type: application/octet-stream \r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    unsafe {
        (*t.cfg).extract_request_files = 1;
        (*t.cfg).tmpdir = "/tmp\0".as_ptr() as *mut i8;

        t.parseParts(&parts);

        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(2, htp_list_array_size((*t.body).parts));

        let part = htp_list_array_get((*t.body).parts, 1) as *mut htp_multipart_part_t;
        assert!(!part.is_null());
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*part).type_0);

        assert!(!(*part).content_type.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*part).content_type, "application/octet-stream")
        );
        assert!(!(*part).file.is_null());
        assert_eq!(0, bstr_cmp_str((*(*part).file).filename, "test.bin"));
        assert_eq!(6, (*(*part).file).len);

        assert!(!(*(*part).file).tmpname.is_null());
        let contents = fs::read_to_string(
            CStr::from_ptr((*(*part).file).tmpname as *mut i8)
                .to_str()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(6, contents.chars().count());
        assert_eq!(contents, "GHIJKL");
    }
}

#[test]
fn PartHeadersEmptyLineBug() {
    let mut t = Test::new();
    let parts = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r",
        "\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parsePartsThenVerify(&parts);
}

#[test]
fn CompleteRequest() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequestThenVerify(&headers, &data);

    assert!(!t.body.is_null());
    unsafe {
        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_FOLDING));
    }
}

#[test]
fn InvalidHeader1() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Colon missing.

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_INVALID));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidHeader2() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Whitespace after header name.

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition : form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_INVALID));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidHeader3() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Whitespace before header name.

    let data = vec![
        "--0123456789\r\n \
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_INVALID));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidHeader4() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Invalid header name; contains a space.

    let data = vec![
        "--0123456789\r\n\
         Content Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_INVALID));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidHeader5() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // No header name.

    let data = vec![
        "--0123456789\r\n\
         : form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_INVALID));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidHeader6() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // No header name.

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: \r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_INVALID));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn NullByte() {
    let mut t = Test::new();
    unsafe {
        t.mpartp = htp_mpartp_create(t.cfg, bstr_dup_str("0123456789"), MultipartFlags::empty());
    }

    // NUL byte in the part header.
    let i1 = "--0123456789\r\n\
              Content-Disposition: form-data; ";
    let i2 = "\0";
    let i3 = "name=\"field1\"\r\n\
              \r\n\
              ABCDEF\
              \r\n--0123456789\r\n\
              Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
              \r\n\
              FILEDATA\
              \r\n--0123456789\r\n\
              Content-Disposition: form-data; name=\"field2\"\r\n\
              \r\n\
              GHIJKL\
              \r\n--0123456789--";

    unsafe {
        htp_mpartp_parse(
            t.mpartp,
            i1.as_ptr() as *const core::ffi::c_void,
            i1.chars().count(),
        );
        htp_mpartp_parse(t.mpartp, i2.as_ptr() as *const core::ffi::c_void, 1);
        htp_mpartp_parse(
            t.mpartp,
            i3.as_ptr() as *const core::ffi::c_void,
            i3.chars().count(),
        );
        htp_mpartp_finalize(t.mpartp);

        t.body = htp_mpartp_get_multipart(t.mpartp);
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_NUL_BYTE));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_INVALID));
    }
}

#[test]
fn MultipleContentTypeHeadersEvasion() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data\r\n\
         Content-Type: boundary=0123456789\r\n",
    ];

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequestThenVerify(&headers, &data);
    unsafe {
        assert!(!(*t.tx).request_content_type.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*t.tx).request_content_type, "multipart/form-data")
        );
    }
}

#[test]
fn BoundaryNormal() {
    let inputs = vec![
        "multipart/form-data; boundary=----WebKitFormBoundaryT4AfwQCOgIxNVwlD",
        "multipart/form-data; boundary=---------------------------21071316483088",
        "multipart/form-data; boundary=---------------------------7dd13e11c0452",
        "multipart/form-data; boundary=----------2JL5oh7QWEDwyBllIRc7fh",
        "multipart/form-data; boundary=----WebKitFormBoundaryre6zL3b0BelnTY5S",
    ];

    let outputs = vec![
        "----WebKitFormBoundaryT4AfwQCOgIxNVwlD",
        "---------------------------21071316483088",
        "---------------------------7dd13e11c0452",
        "----------2JL5oh7QWEDwyBllIRc7fh",
        "----WebKitFormBoundaryre6zL3b0BelnTY5S",
    ];

    unsafe {
        for i in 0..inputs.len() {
            let input: *mut bstr_t;
            input = bstr_dup_str(inputs[i]);
            let mut boundary: *mut bstr_t = std::ptr::null_mut::<bstr_t>();
            let mut flags: MultipartFlags = MultipartFlags::empty();
            let rc: Status = htp_mpartp_find_boundary(input, &mut boundary, &mut flags);
            assert_eq!(Status::OK, rc);
            assert!(!boundary.is_null());
            assert_eq!(0, bstr_cmp_str(boundary, outputs[i]));
            assert_eq!(MultipartFlags::empty(), flags);
            bstr_free(boundary);
            bstr_free(input);
        }
    }
}

#[test]
fn BoundaryParsing() {
    let inputs = vec![
        "multipart/form-data; boundary=1 ",
        "multipart/form-data; boundary=1, boundary=2",
        "multipart/form-data; boundary=\"1\"",
        "multipart/form-data; boundary=\"1\" ",
        "multipart/form-data; boundary=\"1",
    ];

    let outputs = vec!["1", "1", "1", "1", "\"1"];

    unsafe {
        for i in 0..inputs.len() {
            let input: *mut bstr_t;
            input = bstr_dup_str(inputs[i]);
            let mut boundary: *mut bstr_t = std::ptr::null_mut::<bstr_t>();
            let mut flags: MultipartFlags = MultipartFlags::empty();
            let rc: Status = htp_mpartp_find_boundary(input, &mut boundary, &mut flags);
            assert_eq!(Status::OK, rc);
            assert!(!boundary.is_null());
            assert_eq!(0, bstr_cmp_str(boundary, outputs[i]));
            bstr_free(boundary);
            bstr_free(input);
        }
    }
}

#[test]
fn BoundaryInvalid() {
    let inputs = vec![
        "multipart/form-data boundary=1",
        "multipart/form-data ; boundary=1",
        "multipart/form-data, boundary=1",
        "multipart/form-data , boundary=1",
        "multipart/form-datax; boundary=1",
        "multipart/; boundary=1",
        "multipart; boundary=1",
        "application/octet-stream; boundary=1",
        "boundary=1",
        "multipart/form-data; boundary",
        "multipart/form-data; boundary=",
        "multipart/form-data; boundaryX=",
        "multipart/form-data; boundary=\"\"",
        "multipart/form-data; bounDary=1",
        "multipart/form-data; boundary=1; boundary=2",
        "multipart/form-data; boundary=1 2",
        "multipart/form-data boundary=01234567890123456789012345678901234567890123456789012345678901234567890123456789",

    ];

    unsafe {
        for i in inputs {
            let input: *mut bstr_t;
            input = bstr_dup_str(i);
            let mut boundary: *mut bstr_t = std::ptr::null_mut::<bstr_t>();
            let mut flags: MultipartFlags = MultipartFlags::empty();
            let rc: Status = htp_mpartp_find_boundary(input, &mut boundary, &mut flags);
            assert_ne!(Status::ERROR, rc);
            assert!(flags.contains(MultipartFlags::HTP_MULTIPART_HBOUNDARY_INVALID));
            bstr_free(boundary);
            bstr_free(input);
        }
    }
}

#[test]
fn BoundaryUnusual() {
    let inputs = vec![
        "multipart/form-data; boundary=1 ",
        "multipart/form-data; boundary =1",
        "multipart/form-data; boundary= 1",
        "multipart/form-data; boundary=\"1\"",
        "multipart/form-data; boundary=\" 1 \"",
        //"multipart/form-data; boundary=1-2",
        "multipart/form-data; boundary=\"1?2\"",
    ];

    unsafe {
        for i in inputs {
            let input: *mut bstr_t;
            input = bstr_dup_str(i);
            let mut boundary: *mut bstr_t = std::ptr::null_mut::<bstr_t>();
            let mut flags: MultipartFlags = MultipartFlags::empty();
            let rc: Status = htp_mpartp_find_boundary(input, &mut boundary, &mut flags);
            assert_eq!(Status::OK, rc);
            assert!(!boundary.is_null());
            assert!(flags.contains(MultipartFlags::HTP_MULTIPART_HBOUNDARY_UNUSUAL));
            bstr_free(boundary);
            bstr_free(input);
        }
    }
}

#[test]
fn CaseInsensitiveBoundaryMatching() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=grumpyWizards\r\n",
    ];

    // The second boundary is all-lowercase and shouldn't be matched on.
    let data = vec![
        "--grumpyWizards\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n-grumpywizards\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--grumpyWizards\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--grumpyWizards--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(2, htp_list_array_size((*t.body).parts));
    }
}

#[test]
fn FoldedContentDisposition() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\";\r\n \
         filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequestThenVerify(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_FOLDING));
    }
}

#[test]
fn FoldedContentDisposition2() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\";\r\n\
         \rfilename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequestThenVerify(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_FOLDING));
    }
}

#[test]
fn InvalidPartNoData() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // The first part terminates abruptly by the next boundary. This
    // actually works in PHP because its part header parser will
    // consume everything (even boundaries) until the next empty line.

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        let field1 = htp_list_array_get((*t.body).parts, 0) as *mut htp_multipart_part_t;
        assert!(!field1.is_null());
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_UNKNOWN,
            (*field1).type_0
        );

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_INCOMPLETE));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidPartNoContentDisposition() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // A part without a Content-Disposition header.

    let data = vec![
        "--0123456789\r\n\
         Content-Type: text/html\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_UNKNOWN));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidPartMultipleCD() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // When we encounter a part with more than one C-D header, we
    // don't know which one the backend will use. Thus, we raise
    // HTP_MULTIPART_PART_INVALID.

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         Content-Disposition: form-data; name=\"field3\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequestThenVerify(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_REPEATED));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidPartUnknownHeader() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Unknown C-D header "Unknown".

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"\r\n\
         Unknown: Header\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequestThenVerify(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_HEADER_UNKNOWN));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_PART_INVALID));
    }
}

#[test]
fn InvalidContentDispositionMultipleParams1() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Two "name" parameters in a C-D header.

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"; name=\"field3\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CD_PARAM_REPEATED));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_CD_INVALID));
    }
}

#[test]
fn InvalidContentDispositionMultipleParams2() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Two "filename" parameters in a C-D header.

    let data = vec![
        "--0123456789\r\n\
        Content-Disposition: form-data; name=\"field1\"\r\n\
        \r\n\
        ABCDEF\
        \r\n--0123456789\r\n\
        Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"; filename=\"file2.bin\"\r\n\
        \r\n\
        FILEDATA\
        \r\n--0123456789\r\n\
        Content-Disposition: form-data; name=\"field2\"\r\n\
        \r\n\
        GHIJKL\
        \r\n--0123456789--"
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CD_PARAM_REPEATED));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_CD_INVALID));
    }
}

#[test]
fn InvalidContentDispositionUnknownParam() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    // Unknown C-D parameter "test".

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\"; test=\"param\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CD_PARAM_UNKNOWN));
        assert!((*t.body)
            .flags
            .intersects(MultipartFlags::HTP_MULTIPART_CD_INVALID));
    }
}

#[test]
fn InvalidContentDispositionSyntax() {
    let mut t = Test::new();
    let inputs = vec![
        // Parameter value not quoted.
        "form-data; name=field1",
        // Using single quotes around parameter value.
        "form-data; name='field1'",
        // No semicolon after form-data in the C-D header.
        "form-data name=\"field1\"",
        // No semicolon after C-D parameter.
        "form-data; name=\"file1\" filename=\"file.bin\"",
        // Missing terminating quote in C-D parameter value.
        "form-data; name=\"field1",
        // Backslash as the last character in parameter value
        "form-data; name=\"field1\\",
        // C-D header does not begin with "form-data".
        "invalid-syntax; name=\"field1",
        // Escape the terminating double quote.
        "name=\"field1\\\"",
        // Incomplete header.
        "form-data; ",
        // Incomplete header.
        "form-data; name",
        // Incomplete header.
        "form-data; name ",
        // Incomplete header.
        "form-data; name ?",
        // Incomplete header.
        "form-data; name=",
        // Incomplete header.
        "form-data; name= ",
    ];

    unsafe {
        for input in inputs {
            t.mpartp = htp_mpartp_create(t.cfg, bstr_dup_str("123"), MultipartFlags::empty());

            let mut part: *mut htp_multipart_part_t =
                calloc(1, ::std::mem::size_of::<htp_multipart_part_t>())
                    as *mut htp_multipart_part_t;
            (*part).headers = htp_table_t::with_capacity(4);
            (*part).parser = t.mpartp;

            let mut h: *mut htp_header_t =
                calloc(1, ::std::mem::size_of::<htp_header_t>()) as *mut htp_header_t;
            (*h).name = bstr_dup_str("Content-Disposition");
            (*h).value = bstr_dup_str(input);

            (*part).headers.add((*(*h).name).clone(), h);
            let rc: Status = htp_mpart_part_parse_c_d(part);

            assert_eq!(Status::DECLINED, rc);

            t.body = htp_mpartp_get_multipart(t.mpartp);
            assert!((*t.body)
                .flags
                .contains(MultipartFlags::HTP_MULTIPART_CD_SYNTAX_INVALID));
            assert!((*t.body)
                .flags
                .intersects(MultipartFlags::HTP_MULTIPART_CD_INVALID));

            htp_mpart_part_destroy(part, 0);
            htp_mpartp_destroy(t.mpartp);
            t.mpartp = std::ptr::null_mut();
        }
    }
}

#[test]
fn ParamValueEscaping() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"---\\\"---\\\\---\"\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequest(&headers, &data);
    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());
        assert_eq!(3, htp_list_array_size((*t.body).parts));

        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CD_INVALID));

        let field1 = htp_list_array_get((*t.body).parts, 0) as *mut htp_multipart_part_t;
        assert!(!field1.is_null());
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*field1).type_0);
        assert!(!(*field1).name.is_null());
        assert_eq!(0, bstr_cmp_str((*field1).name, "---\"---\\---"));
        assert!(!(*field1).value.is_null());
        assert_eq!(0, bstr_cmp_str((*field1).value, "ABCDEF"));
    }
}

#[test]
fn HeaderValueTrim() {
    let mut t = Test::new();
    let headers = vec![
        "POST / HTTP/1.0\r\n\
         Content-Type: multipart/form-data; boundary=0123456789\r\n",
    ];

    let data = vec![
        "--0123456789\r\n\
         Content-Disposition: form-data; name=\"field1\" \r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"file1\"; filename=\"file.bin\"\r\n\
         \r\n\
         FILEDATA\
         \r\n--0123456789\r\n\
         Content-Disposition: form-data; name=\"field2\"\r\n\
         \r\n\
         GHIJKL\
         \r\n--0123456789--",
    ];

    t.parseRequestThenVerify(&headers, &data);

    unsafe {
        assert!(!t.body.is_null());
        assert!(!(*t.body).parts.is_null());

        let field1 = htp_list_array_get((*t.body).parts, 0) as *mut htp_multipart_part_t;
        assert!(!field1.is_null());
        let h_opt = (*field1).headers.get_nocase_nozero("content-disposition");
        assert!(h_opt.is_some());
        let h = h_opt.unwrap().1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).value, "form-data; name=\"field1\" "));
    }
}
