#![allow(non_snake_case)]
use htp::bstr::*;
use htp::c_api::htp_connp_create;
use htp::htp_config;
use htp::htp_config::htp_server_personality_t::*;
use htp::htp_connection_parser::*;
use htp::htp_multipart::*;
use htp::htp_transaction::*;
use htp::Status;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};

// import common testing utilities
mod common;

struct Test {
    connp: *mut htp_connp_t,
    cfg: *mut htp_config::htp_cfg_t,
    body: *mut htp_multipart_t,
    mpartp: Option<htp_mpartp_t>,
    tx: *mut htp_tx_t,
}

impl Test {
    fn new() -> Self {
        unsafe {
            let cfg: *mut htp_config::htp_cfg_t = htp_config::create();
            assert!(!cfg.is_null());
            (*cfg).set_server_personality(HTP_SERVER_APACHE_2).unwrap();
            (*cfg).register_multipart_parser();
            let connp = htp_connp_create(cfg);
            assert!(!connp.is_null());
            let body = std::ptr::null_mut();
            let mpartp = None;
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
            (*self.connp).open(
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(32768),
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(80),
                None,
            );

            // Send headers
            for header in headers {
                (*self.connp).req_data(
                    None,
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
            (*self.connp).req_data(
                None,
                contentStr.as_ptr() as *const core::ffi::c_void,
                contentStr.chars().count(),
            );

            (*self.connp).req_data(None, "\r\n".as_ptr() as *const core::ffi::c_void, 2);

            // Send data.
            for d in data {
                (*self.connp).req_data(
                    None,
                    d.as_ptr() as *const core::ffi::c_void,
                    d.chars().count(),
                );
            }

            assert_eq!(1, (*self.connp).conn.tx_size());

            self.tx = (*self.connp).conn.tx_mut_ptr(0);

            assert!(!self.tx.is_null());
            assert!(!(*self.tx).request_mpartp.is_none());
            self.mpartp = (*self.tx).request_mpartp.clone();
            self.body = self.mpartp.as_mut().unwrap().get_multipart();
            assert!(!self.body.is_null());
        }
    }

    fn parseRequestThenVerify(&mut self, headers: &Vec<&str>, data: &Vec<&str>) {
        self.parseRequest(headers, data);
        unsafe {
            assert_eq!(3, (*self.body).parts.len());

            assert!(!(*self.body)
                .flags
                .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));

            // Field 1
            let field1 = (*self.body).parts.get(0);
            assert!(field1.is_some());
            let field1 = field1.unwrap();
            assert_eq!(
                htp_multipart_type_t::MULTIPART_PART_TEXT,
                (*(*field1)).type_0
            );
            assert!((*(*(*field1)).name).eq("field1"));
            assert!((*(*(*field1)).value).eq("ABCDEF"));

            // File 1
            let file1 = (*self.body).parts.get(1);
            assert!(file1.is_some());
            let file1 = file1.unwrap();
            assert_eq!(
                htp_multipart_type_t::MULTIPART_PART_FILE,
                (*(*file1)).type_0
            );
            assert!((*(*(*file1)).name).eq("file1"));

            assert!((*(*file1)).file.is_some());
            let file = (*(*file1)).file.as_ref().unwrap();
            assert!(file.filename.is_some());
            let filename = file.filename.as_ref().unwrap();
            assert!(filename.eq("file.bin"));

            // Field 2
            let field2 = (*self.body).parts.get(2);
            assert!(field2.is_some());
            let field2 = field2.unwrap();
            assert_eq!(
                htp_multipart_type_t::MULTIPART_PART_TEXT,
                (*(*field2)).type_0
            );
            assert!((*(*(*field2)).name).eq("field2"));
            assert!((*(*(*field2)).value).eq("GHIJKL"));
        }
    }
    fn parseParts(&mut self, parts: &Vec<&str>) {
        unsafe {
            self.mpartp = htp_mpartp_t::new(self.cfg, b"0123456789", MultipartFlags::empty());
            assert!(!self.mpartp.is_none());
            for part in parts {
                self.mpartp.as_mut().unwrap().parse(part.as_bytes());
            }

            self.mpartp.as_mut().unwrap().finalize().unwrap();
            self.body = self.mpartp.as_mut().unwrap().get_multipart();
            assert!(!self.body.is_null());
        }
    }

    fn parsePartsThenVerify(&mut self, parts: &Vec<&str>) {
        self.parseParts(parts);

        unsafe {
            // Examine the result
            self.body = self.mpartp.as_mut().unwrap().get_multipart();
            assert!(!self.body.is_null());
            assert_eq!(2, (*self.body).parts.len());

            let part = (*self.body).parts.get(0);
            assert!(part.is_some());
            let part = part.unwrap();
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*(*part)).type_0);
            assert!((*(*(*part)).name).eq("field1"));
            assert!((*(*(*part)).value).eq("ABCDEF"));

            let part = (*self.body).parts.get(1);
            assert!(part.is_some());
            let part = part.unwrap();
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*(*part)).type_0);
            assert!((*(*(*part)).name).eq("field2"));
            assert!((*(*(*part)).value).eq("GHIJKL"));
        }
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            (*self.cfg).destroy();
        }
    }
}

#[test]
fn Test1() {
    let mut t = Test::new();
    unsafe {
        t.mpartp = htp_mpartp_t::new(
            t.cfg,
            b"---------------------------41184676334",
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
            t.mpartp.as_mut().unwrap().parse(part.as_bytes());
        }

        t.mpartp.as_mut().unwrap().finalize().unwrap();

        // Examine the result
        t.body = t.mpartp.as_mut().unwrap().get_multipart();
        assert!(!t.body.is_null());
        assert_eq!(5, (*t.body).parts.len());

        let part = (*t.body).parts.get(0);
        assert!(part.is_some());
        let part = part.unwrap();
        assert!((*(*(*part)).name).eq("field1"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*(*part)).type_0);
        assert!((*(*(*part)).value).eq("0123456789"));

        let part = (*t.body).parts.get(1);
        assert!(part.is_some());
        let part = part.unwrap();
        assert!((*(*(*part)).name).eq("field2"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*(*part)).type_0);
        assert!((*(*(*part)).value).eq("0123456789\r\n----------------------------X"));

        let part = (*t.body).parts.get(2);
        assert!(part.is_some());
        let part = part.unwrap();
        assert!((*(*(*part)).name).eq("field3"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_TEXT, (*(*part)).type_0);
        assert!((*(*(*part)).value).eq("9876543210"));

        let part = (*t.body).parts.get(3);
        assert!(part.is_some());
        let part = part.unwrap();
        assert!((*(*(*part)).name).eq("file1"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*(*part)).type_0);

        let part = (*t.body).parts.get(4);
        assert!(part.is_some());
        let part = part.unwrap();
        assert!((*(*(*part)).name).eq("file2"));
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*(*part)).type_0);

        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_PART_INCOMPLETE));
    }
}

#[test]
fn Test2() {
    let mut t = Test::new();
    unsafe {
        t.mpartp = htp_mpartp_t::new(t.cfg, b"BBB", MultipartFlags::empty());

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
            t.mpartp.as_mut().unwrap().parse(part.as_bytes());
        }

        t.mpartp.as_mut().unwrap().finalize().unwrap();

        t.body = t.mpartp.as_mut().unwrap().get_multipart();
        assert!(!t.body.is_null());
        assert_eq!(4, (*t.body).parts.len());

        let part = (*t.body).parts.get(0);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_PREAMBLE,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("x0000x"));

        let part = (*t.body).parts.get(1);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_UNKNOWN,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("x1111x\n--\nx2222x"));

        let part = (*t.body).parts.get(2);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_UNKNOWN,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("x3333x\n--BB\n\nx4444x\n--BB"));

        let part = (*t.body).parts.get(3);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_UNKNOWN,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("x5555x\r\n--x6666x\r--"));

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_INCOMPLETE));
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
fn BeginsWithCrLf1() {
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
fn BeginsWithCrLf2() {
    let mut t = Test::new();
    let parts = vec![
        "\r",
        "\n",
        "--01234",
        "56789\r\n\
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
fn BeginsWithLf1() {
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
fn BeginsWithLf2() {
    let mut t = Test::new();
    let parts = vec![
        "\n",
        "--0123456789",
        "\r\n\
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
        assert_eq!(3, (*t.body).parts.len());

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_PREAMBLE));

        let part = (*t.body).parts.get(0);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_PREAMBLE,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("Preamble"));
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
        assert_eq!(3, (*t.body).parts.len());

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let part = (*t.body).parts.get(2);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("Epilogue"));
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
        assert_eq!(3, (*t.body).parts.len());

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let part = (*t.body).parts.get(2).unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("Epi\nlogue"));
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
        assert_eq!(3, (*t.body).parts.len());

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let part = (*t.body).parts.get(2).unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*(*part)).type_0
        );
        assert!((*(*(*part)).value).eq("Epi\r\n--logue"));
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
        assert_eq!(4, (*t.body).parts.len());

        assert!((*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_HAS_EPILOGUE));

        let ep1 = (*t.body).parts.get(2);
        assert!(ep1.is_some());
        let ep1 = ep1.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*(*ep1)).type_0
        );
        assert!((*(*(*ep1)).value).eq("Epilogue1"));

        let ep2 = (*t.body).parts.get(3);
        assert!(ep2.is_some());
        let ep2 = ep2.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_EPILOGUE,
            (*(*ep2)).type_0
        );
        assert!((*(*(*ep2)).value).eq("Epilogue2"));

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
        assert_eq!(2, (*t.body).parts.len());

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
        assert_eq!(1, (*t.body).parts.len());

        let part = (*t.body).parts.get(0);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_UNKNOWN,
            (*(*part)).type_0
        );
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
        assert_eq!(2, (*t.body).parts.len());

        let part = (*t.body).parts.get(1);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*(*part)).type_0);
        assert_eq!(
            Some(bstr_t::from("application/octet-stream")),
            (*(*part)).content_type
        );
        assert!((*(*part)).file.is_some());
        let file = (*(*part)).file.as_ref().unwrap();
        assert!(file.filename.is_some());
        let filename = file.filename.as_ref().unwrap();
        assert!(filename.eq("test.bin"));
        assert_eq!(6, file.len);
    }
}

#[test]
fn WithFileExternallyStored() {
    let tmpfile = {
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
            (*t.cfg).extract_request_files = true;
            (*t.cfg).tmpdir = "/tmp".to_string();

            t.parseParts(&parts);

            assert!(!t.body.is_null());
            assert_eq!(2, (*t.body).parts.len());

            let part = (*t.body).parts.get(1);
            assert!(part.is_some());
            let part = part.unwrap();
            assert_eq!(htp_multipart_type_t::MULTIPART_PART_FILE, (*(*part)).type_0);

            assert_eq!(
                (*(*part)).content_type,
                Some(bstr_t::from("application/octet-stream"))
            );
            assert!((*(*part)).file.is_some());
            let file = (*(*part)).file.as_ref().unwrap();
            assert!(file.filename.is_some());
            let filename = file.filename.as_ref().unwrap();
            assert!(filename.eq("test.bin"));
            assert_eq!(6, file.len);

            assert!(file.tmpfile.is_some());
            let name = file.tmpfile.as_ref().unwrap().path().to_path_buf();

            let contents = fs::read_to_string(&name).unwrap();
            assert_eq!(6, contents.chars().count());
            assert_eq!(contents, "GHIJKL");
            name
        }
    };
    assert!(!tmpfile.exists());
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
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(3, (*t.body).parts.len());

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

    // No header value. Header values are non-optional: see https://tools.ietf.org/html/rfc7230#section-3.2

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
        assert_eq!(3, (*t.body).parts.len());

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
    t.mpartp = htp_mpartp_t::new(t.cfg, b"0123456789", MultipartFlags::empty());

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
        t.mpartp.as_mut().unwrap().parse(i1.as_bytes());
        t.mpartp.as_mut().unwrap().parse(i2.as_bytes());
        t.mpartp.as_mut().unwrap().parse(i3.as_bytes());
        t.mpartp.as_mut().unwrap().finalize().unwrap();

        t.body = t.mpartp.as_mut().unwrap().get_multipart();
        assert!(!t.body.is_null());
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(
            (*t.tx).request_content_type,
            Some(bstr_t::from("multipart/form-data"))
        );
    }
}

#[test]
fn BoundaryNormal() {
    let inputs: Vec<&[u8]> = vec![
        b"multipart/form-data; boundary=----WebKitFormBoundaryT4AfwQCOgIxNVwlD",
        b"multipart/form-data; boundary=---------------------------21071316483088",
        b"multipart/form-data; boundary=---------------------------7dd13e11c0452",
        b"multipart/form-data; boundary=----------2JL5oh7QWEDwyBllIRc7fh",
        b"multipart/form-data; boundary=----WebKitFormBoundaryre6zL3b0BelnTY5S",
    ];

    let outputs: Vec<&[u8]> = vec![
        b"----WebKitFormBoundaryT4AfwQCOgIxNVwlD",
        b"---------------------------21071316483088",
        b"---------------------------7dd13e11c0452",
        b"----------2JL5oh7QWEDwyBllIRc7fh",
        b"----WebKitFormBoundaryre6zL3b0BelnTY5S",
    ];
    for i in 0..inputs.len() {
        let mut flags: MultipartFlags = MultipartFlags::empty();
        assert_eq!(find_boundary(inputs[i], &mut flags).unwrap(), outputs[i]);
        assert_eq!(MultipartFlags::empty(), flags);
    }
}

#[test]
fn BoundaryParsing() {
    let inputs: Vec<&[u8]> = vec![
        b"multipart/form-data; boundary=1 ",
        b"multipart/form-data; boundary=1, boundary=2",
        b"multipart/form-data; boundary=\"1\"",
        b"multipart/form-data; boundary=\"1\" ",
        b"multipart/form-data; boundary=\"1",
    ];

    let outputs: Vec<&[u8]> = vec![b"1", b"1", b"1", b"1", b"\"1"];

    for i in 0..inputs.len() {
        let mut flags: MultipartFlags = MultipartFlags::empty();
        assert_eq!(find_boundary(inputs[i], &mut flags).unwrap(), outputs[i]);
    }
}

#[test]
fn BoundaryInvalid() {
    let inputs: Vec<&[u8]> = vec![
    b"multipart/form-data boundary=1",
    b"multipart/form-data ; boundary=1",
    b"multipart/form-data, boundary=1",
    b"multipart/form-data , boundary=1",
    b"multipart/form-datax; boundary=1",
    b"multipart/; boundary=1",
    b"multipart; boundary=1",
    b"application/octet-stream; boundary=1",
    b"boundary=1",
    b"multipart/form-data; boundary",
    b"multipart/form-data; boundary=",
    b"multipart/form-data; boundaryX=",
    b"multipart/form-data; boundary=\"\"",
    b"multipart/form-data; bounDary=1",
    b"multipart/form-data; boundary=1; boundary=2",
    b"multipart/form-data; boundary=1 2",
    b"multipart/form-data boundary=01234567890123456789012345678901234567890123456789012345678901234567890123456789",
];

    for input in inputs {
        let mut flags: MultipartFlags = MultipartFlags::empty();
        find_boundary(input, &mut flags);
        assert!(flags.contains(MultipartFlags::HTP_MULTIPART_HBOUNDARY_INVALID));
    }
}

#[test]
fn BoundaryUnusual() {
    let inputs: Vec<&[u8]> = vec![
        b"multipart/form-data; boundary=1 ",
        b"multipart/form-data; boundary =1",
        b"multipart/form-data; boundary= 1",
        b"multipart/form-data; boundary=\"1\"",
        b"multipart/form-data; boundary=\" 1 \"",
        b"multipart/form-data; boundary=\"1?2\"",
    ];
    for input in inputs {
        let mut flags: MultipartFlags = MultipartFlags::empty();
        assert!(find_boundary(input, &mut flags).is_some());
        assert!(flags.contains(MultipartFlags::HTP_MULTIPART_HBOUNDARY_UNUSUAL));
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
        assert_eq!(2, (*t.body).parts.len());
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
        assert_eq!(3, (*t.body).parts.len());

        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_UNKNOWN,
            (*(*(*t.body).parts.get(0).unwrap())).type_0
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
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(3, (*t.body).parts.len());

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
        assert_eq!(3, (*t.body).parts.len());

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
            t.mpartp = htp_mpartp_t::new(t.cfg, b"123", MultipartFlags::empty());

            let mut part: htp_multipart_part_t =
                htp_multipart_part_t::new(t.mpartp.as_mut().unwrap());
            let header = htp_header_t::new(b"Content-Disposition".to_vec().into(), input.into());
            part.headers.add(header.name.clone(), header);
            assert_err!(part.parse_c_d(), Status::DECLINED);

            t.body = t.mpartp.as_mut().unwrap().get_multipart();
            assert!((*t.body)
                .flags
                .contains(MultipartFlags::HTP_MULTIPART_CD_SYNTAX_INVALID));
            assert!((*t.body)
                .flags
                .intersects(MultipartFlags::HTP_MULTIPART_CD_INVALID));
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
        assert_eq!(3, (*t.body).parts.len());

        assert!(!(*t.body)
            .flags
            .contains(MultipartFlags::HTP_MULTIPART_CD_INVALID));

        let field1 = (*t.body).parts.get(0);
        assert!(field1.is_some());
        let field1 = field1.unwrap();
        assert_eq!(
            htp_multipart_type_t::MULTIPART_PART_TEXT,
            (*(*field1)).type_0
        );
        assert!((*(*(*field1)).name).eq("---\"---\\---"));
        assert!((*(*(*field1)).value).eq("ABCDEF"));
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

        let field1 = (*t.body).parts.get(0).unwrap().as_ref().unwrap();
        let header = &field1
            .headers
            .get_nocase_nozero("content-disposition")
            .unwrap()
            .1;
        assert_eq!(header.value, "form-data; name=\"field1\" ");
    }
}
