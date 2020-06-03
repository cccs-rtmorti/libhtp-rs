#![allow(non_snake_case)]
use htp::bstr::*;
use htp::htp_config::htp_server_personality_t::*;
use htp::htp_config::*;
use htp::htp_connection_parser::*;
use htp::htp_decompressors::htp_content_encoding_t::*;
use htp::htp_decompressors::*;
use htp::htp_transaction::*;
use htp::Status;
use std::env;
use std::ffi::CString;
use std::path::PathBuf;

#[no_mangle]
extern "C" fn GUnzip_decompressor_callback(d: *mut htp_tx_data_t) -> Status {
    unsafe {
        let output_ptr: *mut *mut bstr_t = htp_tx_get_user_data((*d).tx) as *mut *mut bstr_t;
        *output_ptr = bstr_dup_mem((*d).data as *const core::ffi::c_void, (*d).len);
    }
    Status::OK
}

#[derive(Debug)]
struct Test {
    cfg: *mut htp_cfg_t,
    connp: *mut htp_connp_t,
    output: *mut bstr_t,
    o_boxing_wizards: *mut bstr_t,
    tx: *mut htp_tx_t,
    decompressor: *mut htp_decompressor_t,
}

enum TestError {
    Io(std::io::Error),
    Htp(Status),
}

impl Test {
    fn new() -> Self {
        unsafe {
            let cfg = htp_config_create();
            assert!(!cfg.is_null());
            htp_config_set_server_personality(&mut *cfg, HTP_SERVER_APACHE_2);
            let connp = htp_connp_create(cfg);
            assert!(!connp.is_null());
            let tx = htp_connp_tx_create(connp);
            assert!(!tx.is_null());
            let output = std::ptr::null_mut();

            let decompressor = htp_gzip_decompressor_create(connp, HTP_COMPRESSION_GZIP);
            (*decompressor).callback = Some(GUnzip_decompressor_callback);
            let o_boxing_wizards = bstr_dup_c(
                CString::new("The five boxing wizards jump quickly.")
                    .unwrap()
                    .as_ptr(),
            );

            Test {
                cfg,
                connp,
                output,
                o_boxing_wizards,
                tx,
                decompressor,
            }
        }
    }

    fn run(&mut self, filename: &str) -> Result<(), TestError> {
        let mut filepath = if let Ok(dir) = std::env::var("srcdir") {
            PathBuf::from(dir)
        } else {
            let mut base = PathBuf::from(
                env::var("CARGO_MANIFEST_DIR").expect("Could not determine test file directory"),
            );
            base.push("tests");
            base.push("files");
            base
        };
        filepath.push(filename);

        let mut data = std::fs::read(filepath).map_err(TestError::Io)?;
        unsafe {
            let output_ptr: *mut *mut bstr_t = &mut self.output;
            htp_tx_set_user_data(self.tx, output_ptr as *mut core::ffi::c_void);

            let mut tx: htp_tx_data_t = htp_tx_data_t {
                tx: self.tx,
                data: data.as_mut_ptr() as *const u8,
                len: data.len(),
                is_last: 0,
            };
            let rc = (*self.decompressor).decompress.unwrap()(self.decompressor, &mut tx);
            if rc == Status::OK {
                Ok(())
            } else {
                Err(TestError::Htp(rc))
            }
        }
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            bstr_free(self.output);
            bstr_free(self.o_boxing_wizards);
            (*self.decompressor).destroy.unwrap()(self.decompressor);
            htp_connp_destroy_all(self.connp);
            htp_config_destroy(self.cfg);
        }
    }
}

#[test]
fn GUnzip_Minimal() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-01-minimal.gz").is_ok());
        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_FNAME() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-02-fname.gz").is_ok());
        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_FEXTRA() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-05-fextra.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_FTEXT() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-06-ftext.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_Multipart() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-10-multipart.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_InvalidExtraFlags() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-14-invalid-xfl.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_InvalidHeaderCrc() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-15-invalid-fhcrc.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

/*
// These tests were disabled in libhtp
#[test]
fn GUnzip_FCOMMENT() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-03-fcomment.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_FHCRC() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-04-fhcrc.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_FRESERVED1() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-07-freserved1.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_FRESERVED2() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-08-freserved2.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_FRESERVED3() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-09-freserved3.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_InvalidMethod() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-11-invalid-method.gz.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_InvalidCrc() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-12-invalid-crc32.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}

#[test]
fn GUnzip_InvalidInputSize() {
    unsafe {
        let mut t = Test::new();
        assert!(t.run("gztest-13-invalid-isize.gz").is_ok());

        assert!(!t.output.is_null());
        assert_eq!(0, bstr_cmp(t.o_boxing_wizards, t.output));
    }
}
*/
