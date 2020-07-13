#![allow(non_snake_case)]
use htp::bstr::*;
use htp::htp_config;
use htp::htp_config::htp_decoder_ctx_t::*;
use htp::htp_config::htp_server_personality_t::*;
use htp::htp_connection_parser::*;
use htp::htp_request::*;
use htp::htp_response::*;
use htp::htp_transaction::htp_auth_type_t::*;
use htp::htp_transaction::htp_data_source_t::*;
use htp::htp_transaction::htp_tx_req_progress_t::*;
use htp::htp_transaction::htp_tx_res_progress_t::*;
use htp::htp_transaction::*;
use htp::htp_util::*;
use htp::log::*;
use htp::Status;
use std::cmp::Ordering;
use std::env;
use std::ffi::CString;
use std::iter::IntoIterator;
use std::ops::Drop;
use std::path::PathBuf;
use std::slice;

macro_rules! cstr {
    ( $x:expr ) => {{
        CString::new($x).unwrap().as_ptr()
    }};
}

#[derive(Debug)]
enum Chunk {
    Client(Vec<u8>),
    Server(Vec<u8>),
}

#[derive(Debug)]
struct TestInput {
    chunks: Vec<Chunk>,
}

impl IntoIterator for TestInput {
    type Item = Chunk;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.chunks.into_iter()
    }
}

impl TestInput {
    fn new(file: PathBuf) -> Self {
        let input = std::fs::read(file);
        assert!(input.is_ok());
        let input = input.unwrap();

        let mut test_input = TestInput { chunks: Vec::new() };
        let mut current = Vec::<u8>::new();
        let mut client = true;
        for line in input.split(|c| *c == b'\n') {
            if line.len() >= 3
                && ((line[0] == b'>' && line[1] == b'>' && line[2] == b'>')
                    || (line[0] == b'<' && line[1] == b'<' && line[2] == b'<'))
            {
                if !current.is_empty() {
                    // Pop off the CRLF from the last line, which
                    // just separates the previous data from the
                    // boundary <<< >>> chars and isn't actual data
                    if let Some(b'\n') = current.last() {
                        current.pop();
                    }
                    if let Some(b'\r') = current.last() {
                        current.pop();
                    }
                    test_input.append(client, current);
                    current = Vec::<u8>::new();
                }
                client = line[0] == b'>';
            } else {
                current.append(&mut line.to_vec());
                current.push(b'\n');
            }
        }
        // Remove the '\n' we would have appended for EOF
        current.pop();
        test_input.append(client, current);
        test_input
    }

    fn append(&mut self, client: bool, data: Vec<u8>) {
        if client {
            self.chunks.push(Chunk::Client(data));
        } else {
            self.chunks.push(Chunk::Server(data));
        }
    }
}

#[derive(Debug)]
enum TestError {
    //MultipleClientChunks,
    //MultipleServerChunks,
    StreamError,
}

struct Test {
    cfg: *mut htp_config::htp_cfg_t,
    connp: *mut htp_connp_t,
    basedir: PathBuf,
}

impl Test {
    fn new() -> Self {
        let basedir = if let Ok(dir) = std::env::var("srcdir") {
            PathBuf::from(dir)
        } else {
            let mut base = PathBuf::from(
                env::var("CARGO_MANIFEST_DIR").expect("Could not determine test file directory"),
            );
            base.push("tests");
            base.push("files");
            base
        };

        unsafe {
            let cfg: *mut htp_config::htp_cfg_t = htp_config::create();
            assert!(!cfg.is_null());
            (*cfg).set_server_personality(HTP_SERVER_APACHE_2);
            (*cfg).register_urlencoded_parser();
            (*cfg).register_multipart_parser();
            let connp = htp_connp_create(cfg);
            assert!(!connp.is_null());

            Test {
                cfg,
                connp,
                basedir,
            }
        }
    }

    fn run(&mut self, file: &str) -> Result<(), TestError> {
        unsafe {
            let mut tv_start = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            libc::gettimeofday(&mut tv_start, std::ptr::null_mut());
            htp_connp_open(
                self.connp,
                cstr!("127.0.0.1"),
                10000,
                cstr!("127.0.0.1"),
                80,
                &mut tv_start,
            );

            let mut path = self.basedir.clone();
            path.push(file);
            let test = TestInput::new(path);
            let mut in_buf: Option<Vec<u8>> = None;
            let mut out_buf: Option<Vec<u8>> = None;
            for chunk in test {
                match chunk {
                    Chunk::Client(data) => {
                        let rc = htp_connp_req_data(
                            self.connp,
                            &tv_start,
                            data.as_ptr() as *const core::ffi::c_void,
                            data.len(),
                        );
                        if rc == 3 {
                            // HTP_STREAM_ERROR = 3
                            return Err(TestError::StreamError);
                        }

                        if rc == 5 {
                            // HTP_STREAM_DATA_OTHER = 5
                            let consumed = htp_connp_req_data_consumed(self.connp);
                            let mut remaining = Vec::with_capacity(data.len() - consumed);
                            remaining.extend_from_slice(&data[consumed..]);
                            in_buf = Some(remaining);
                        }
                    }
                    Chunk::Server(data) => {
                        // If we have leftover data from before then use it first
                        if let Some(out_remaining) = out_buf {
                            let rc = htp_connp_res_data(
                                self.connp,
                                &tv_start,
                                out_remaining.as_ptr() as *const core::ffi::c_void,
                                out_remaining.len(),
                            );
                            out_buf = None;
                            if rc == 3 {
                                // HTP_STREAM_ERROR = 3
                                return Err(TestError::StreamError);
                            }
                        }

                        // Now use up this data chunk
                        let rc = htp_connp_res_data(
                            self.connp,
                            &tv_start,
                            data.as_ptr() as *const core::ffi::c_void,
                            data.len(),
                        );
                        if rc == 3 {
                            // HTP_STREAM_ERROR = 3
                            return Err(TestError::StreamError);
                        }

                        if rc == 5 {
                            // HTP_STREAM_DATA_OTHER
                            let consumed = htp_connp_res_data_consumed(self.connp);
                            let mut remaining = Vec::with_capacity(data.len() - consumed);
                            remaining.extend_from_slice(&data[consumed..]);
                            out_buf = Some(remaining);
                        }

                        // And check if we also had some input data buffered
                        if let Some(in_remaining) = in_buf {
                            let rc = htp_connp_req_data(
                                self.connp,
                                &tv_start,
                                in_remaining.as_ptr() as *const core::ffi::c_void,
                                in_remaining.len(),
                            );
                            in_buf = None;
                            if rc == 3 {
                                // HTP_STREAM_ERROR
                                return Err(TestError::StreamError);
                            }
                        }
                    }
                }
            }

            // Clean up any remaining server data
            if let Some(out_remaining) = out_buf {
                let rc = htp_connp_res_data(
                    self.connp,
                    &tv_start,
                    out_remaining.as_ptr() as *const core::ffi::c_void,
                    out_remaining.len(),
                );
                if rc == 3 {
                    // HTP_STREAM_ERROR = 3
                    return Err(TestError::StreamError);
                }
            }

            let mut tv_end = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            libc::gettimeofday(&mut tv_end, std::ptr::null_mut());
            htp_connp_close(self.connp, &mut tv_end);
        }
        Ok(())
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            htp_connp_destroy(self.connp);
            (*self.cfg).destroy();
        }
    }
}

#[test]
fn AdHoc() {
    let mut t = Test::new();
    assert!(t.run("00-adhoc.t").is_ok());
}

#[test]
fn Get() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("01-get.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;

        assert!(!tx.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));
        assert_eq!(0, bstr_cmp_str((*tx).request_uri, "/?p=%20"));
        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).query.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).query, "p=%20"));

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "p")
                .unwrap()
                .value
                .cmp(" ")
        );
    }
}

#[test]
fn ApacheHeaderParsing() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("02-header-test-apache2.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(9, (*(*tx).request_headers).size());

        let mut res = &(*(*tx).request_headers)[0];
        let mut h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, " Invalid-Folding"));
        assert_eq!(0, bstr_cmp_str((*h).value, "1"));

        res = &(*(*tx).request_headers)[1];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, "Valid-Folding"));
        assert_eq!(0, bstr_cmp_str((*h).value, "2 2"));

        res = &(*(*tx).request_headers)[2];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, "Normal-Header"));
        assert_eq!(0, bstr_cmp_str((*h).value, "3"));

        res = &(*(*tx).request_headers)[3];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, "Invalid Header Name"));
        assert_eq!(0, bstr_cmp_str((*h).value, "4"));

        res = &(*(*tx).request_headers)[4];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, "Same-Name-Headers"));
        assert_eq!(0, bstr_cmp_str((*h).value, "5, 6"));

        res = &(*(*tx).request_headers)[5];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, "Empty-Value-Header"));
        assert_eq!(0, bstr_cmp_str((*h).value, ""));

        res = &(*(*tx).request_headers)[6];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, ""));
        assert_eq!(0, bstr_cmp_str((*h).value, "8, "));

        res = &(*(*tx).request_headers)[7];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, "Header-With-LWS-After"));
        assert_eq!(0, bstr_cmp_str((*h).value, "9"));

        res = &(*(*tx).request_headers)[8];
        h = res.1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).name, "Header-With-NUL"));
        assert_eq!(0, bstr_cmp_str((*h).value, "BEFORE"));
    }
}

#[test]
fn PostUrlencoded() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("03-post-urlencoded.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        // Transaction 1
        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "p")
                .unwrap()
                .value
                .cmp("0123456789")
        );

        assert_eq!((*tx).request_progress, HTP_REQUEST_COMPLETE);
        assert_eq!((*tx).response_progress, HTP_RESPONSE_COMPLETE);

        let h = (*(*tx).response_headers)
            .get_nocase_nozero("Server")
            .unwrap()
            .1;
        assert!(!h.is_null());
        assert!(!(*h).value.is_null());
        assert_eq!(0, bstr_cmp_str((*h).value, "Apache"));

        // Transaction 2
        let tx2: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx2.is_null());

        assert_eq!((*tx2).request_progress, HTP_REQUEST_COMPLETE);
        assert_eq!((*tx2).response_progress, HTP_RESPONSE_COMPLETE);

        let h2 = (*(*tx2).response_headers)
            .get_nocase_nozero("Server")
            .unwrap()
            .1;
        assert!(!h2.is_null());
        assert!(!(*h2).value.is_null());
        assert_eq!(0, bstr_cmp_str((*h2).value, "Apache"));
    }
}

#[test]
fn PostUrlencodedChunked() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("04-post-urlencoded-chunked.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "p")
                .unwrap()
                .value
                .cmp("0123456789")
        );
        assert_eq!(25, (*tx).request_message_len);
        assert_eq!(12, (*tx).request_entity_len);
    }
}

#[test]
fn Expect() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("05-expect.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        // The interim header from the 100 response should not be among the final headers.
        assert!((*(*tx).request_headers)
            .get_nocase_nozero("Header1")
            .is_none());
    }
}

#[test]
fn UriNormal() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("06-uri-normal.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
    }
}

#[test]
fn PipelinedConn() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("07-pipelined-connection.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        assert!((*(*t.connp).conn)
            .flags
            .contains(ConnectionFlags::HTP_CONN_PIPELINED));

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
    }
}

#[test]
fn NotPipelinedConn() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("08-not-pipelined-connection.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        assert!(!(*(*t.connp).conn)
            .flags
            .contains(ConnectionFlags::HTP_CONN_PIPELINED));

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).flags.contains(Flags::HTP_MULTI_PACKET_HEAD));
    }
}

#[test]
fn MultiPacketRequest() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("09-multi-packet-request-head.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_MULTI_PACKET_HEAD));
    }
}

#[test]
fn HeaderHostParsing() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("10-host-in-headers.t").is_ok());
        assert_eq!(4, (*(*t.connp).conn).transactions.len());

        let tx1: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx1.is_null());
        assert!(!(*tx1).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx1).request_hostname, "www.example.com"));

        let tx2: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx2.is_null());
        assert!(!(*tx2).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx2).request_hostname, "www.example.com."));

        let tx3: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(2).unwrap() as *mut htp_tx_t;
        assert!(!tx3.is_null());
        assert!(!(*tx3).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx3).request_hostname, "www.example.com"));

        let tx4: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(3).unwrap() as *mut htp_tx_t;
        assert!(!tx4.is_null());
        assert!(!(*tx4).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx4).request_hostname, "www.example.com"));
    }
}

#[test]
fn ResponseWithoutContentLength() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("11-response-stream-closure.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));
    }
}

#[test]
fn FailedConnectRequest() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("12-connect-request.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(0, bstr_cmp_str((*tx).request_method, "CONNECT"));

        assert_eq!(405, (*tx).response_status_number);
    }
}

#[test]
fn CompressedResponseContentType() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("13-compressed-response-gzip-ct.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(187, (*tx).response_message_len);

        assert_eq!(225, (*tx).response_entity_len);
    }
}

#[test]
fn CompressedResponseChunked() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("14-compressed-response-gzip-chunked.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(28261, (*tx).response_message_len);

        assert_eq!(159_590, (*tx).response_entity_len);
    }
}

#[test]
fn SuccessfulConnectRequest() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("15-connect-complete.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        // TODO: Update the test_run() function to provide better
        //       simulation of real traffic. At the moment, it does not
        //       invoke inbound parsing after outbound parsing returns
        //       HTP_DATA_OTHER, which is why the check below fails.
        //assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(0, bstr_cmp_str((*tx).request_method, "CONNECT"));

        assert_eq!(200, (*tx).response_status_number);
    }
}

#[test]
fn ConnectRequestWithExtraData() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("16-connect-extra.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        let tx1: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx1.is_null());

        assert!(0 != htp_tx_is_complete(tx1));

        let tx2: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx2.is_null());

        assert!(0 != htp_tx_is_complete(tx2));
    }
}

#[test]
fn Multipart() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("17-multipart-1.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "field1")
                .unwrap()
                .value
                .cmp("0123456789")
        );
        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "field2")
                .unwrap()
                .value
                .cmp("9876543210")
        );
    }
}

#[test]
fn CompressedResponseDeflate() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("18-compressed-response-deflate.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(755, (*tx).response_message_len);

        assert_eq!(1433, (*tx).response_entity_len);
    }
}

#[test]
fn UrlEncoded() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("19-urlencoded-test.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(0, bstr_cmp_str((*tx).request_method, "POST"));
        assert_eq!(0, bstr_cmp_str((*tx).request_uri, "/?p=1&q=2"));

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param_ex(&*(*tx).request_params, HTP_SOURCE_BODY, "p")
                .unwrap()
                .value
                .cmp("3")
        );

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param_ex(&*(*tx).request_params, HTP_SOURCE_BODY, "q")
                .unwrap()
                .value
                .cmp("4")
        );

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param_ex(&*(*tx).request_params, HTP_SOURCE_BODY, "z")
                .unwrap()
                .value
                .cmp("5")
        );
    }
}

#[test]
fn AmbiguousHost() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("20-ambiguous-host.t").is_ok());

        assert_eq!(5, (*(*t.connp).conn).transactions.len());

        let tx1: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx1.is_null());
        assert!(0 != htp_tx_is_complete(tx1));
        assert!(!(*tx1).flags.contains(Flags::HTP_HOST_AMBIGUOUS));

        let tx2: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx2.is_null());
        assert!(0 != htp_tx_is_complete(tx2));
        assert!((*tx2).flags.contains(Flags::HTP_HOST_AMBIGUOUS));
        assert!(!(*tx2).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx2).request_hostname, "example.com"));

        let tx3: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(2).unwrap() as *mut htp_tx_t;
        assert!(!tx3.is_null());
        assert!(0 != htp_tx_is_complete(tx3));
        assert!(!(*tx3).flags.contains(Flags::HTP_HOST_AMBIGUOUS));
        assert!(!(*tx3).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx3).request_hostname, "www.example.com"));
        assert_eq!(8001, (*tx3).request_port_number);

        let tx4: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(3).unwrap() as *mut htp_tx_t;
        assert!(!tx4.is_null());
        assert!(0 != htp_tx_is_complete(tx4));
        assert!((*tx4).flags.contains(Flags::HTP_HOST_AMBIGUOUS));
        assert!(!(*tx4).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx4).request_hostname, "www.example.com"));
        assert_eq!(8002, (*tx4).request_port_number);

        let tx5: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(4).unwrap() as *mut htp_tx_t;
        assert!(!tx5.is_null());
        assert!(0 != htp_tx_is_complete(tx5));
        assert!(!(*tx5).flags.contains(Flags::HTP_HOST_AMBIGUOUS));
        assert!(!(*tx5).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx5).request_hostname, "www.example.com"));
        assert_eq!(80, (*tx5).request_port_number);
    }
}

#[test]
fn Http_0_9() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("21-http09.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());
        assert!(!(*(*t.connp).conn)
            .flags
            .contains(ConnectionFlags::HTP_CONN_HTTP_0_9_EXTRA));

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
    }
}

#[test]
fn Http11HostMissing() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("22-http_1_1-host_missing").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_HOST_MISSING));
    }
}

#[test]
fn Http_0_9_Multiple() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("23-http09-multiple.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());
        assert!((*(*t.connp).conn)
            .flags
            .contains(ConnectionFlags::HTP_CONN_HTTP_0_9_EXTRA));

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
    }
}

#[test]
fn Http_0_9_Explicit() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("24-http09-explicit.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
        assert_eq!(0, (*tx).is_protocol_0_9);
    }
}

#[test]
fn SmallChunks() {
    let mut t = Test::new();
    assert!(t.run("25-small-chunks.t").is_ok());
}

#[no_mangle]
extern "C" fn ConnectionParsing_RequestHeaderData_REQUEST_HEADER_DATA(
    d: *mut htp_tx_data_t,
) -> Status {
    unsafe {
        static mut COUNTER: i32 = 0;
        let len = (*d).len;
        let data: &[u8] = slice::from_raw_parts((*d).data, len);
        match COUNTER {
            0 => {
                if !((len == 11) && data == b"User-Agent:") {
                    eprintln!("Mismatch in chunk 0");
                    COUNTER = -1;
                }
            }
            1 => {
                if !((len == 5) && data == b" Test") {
                    eprintln!("Mismatch in chunk 1");
                    COUNTER = -1;
                }
            }
            2 => {
                if !((len == 5) && data == b" User") {
                    eprintln!("Mismatch in chunk 2");
                    COUNTER = -1;
                }
            }
            3 => {
                if !((len == 30) && data == b" Agent\nHost: www.example.com\n\n") {
                    eprintln!("Mismatch in chunk 3");
                    COUNTER = -1;
                }
            }
            _ => {
                if COUNTER >= 0 {
                    eprintln!("Seen more than 4 chunks");
                    COUNTER = -1;
                }
            }
        }

        if COUNTER >= 0 {
            COUNTER += 1;
        }

        let counter_ptr: *mut i32 = &mut COUNTER;
        htp_tx_set_user_data((*d).tx, counter_ptr as *mut core::ffi::c_void);

        Status::OK
    }
}

#[test]
fn RequestHeaderData() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).register_request_header_data(Some(
            ConnectionParsing_RequestHeaderData_REQUEST_HEADER_DATA,
        ));
        assert!(t.run("26-request-headers-raw.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        let counter: *mut i32 = htp_tx_get_user_data(tx) as *mut i32;
        assert!(!counter.is_null());
        assert_eq!(4, *counter);
    }
}

#[no_mangle]
extern "C" fn ConnectionParsing_RequestTrailerData_REQUEST_TRAILER_DATA(
    d: *mut htp_tx_data_t,
) -> Status {
    unsafe {
        static mut COUNTER: i32 = 0;
        let len = (*d).len;
        let data: &[u8] = slice::from_raw_parts((*d).data, len);
        match COUNTER {
            0 => {
                if !((len == 7) && (data == b"Cookie:")) {
                    eprintln!("Mismatch in chunk 0");
                    COUNTER = -1;
                }
            }
            1 => {
                if !((len == 6) && (data == b" 2\r\n\r\n")) {
                    eprintln!("Mismatch in chunk 1");
                    COUNTER = -2;
                }
            }
            _ => {
                if COUNTER >= 0 {
                    eprintln!("Seen more than 4 chunks");
                    COUNTER = -3;
                }
            }
        }

        if COUNTER >= 0 {
            COUNTER += 1;
        }

        let counter_ptr: *mut i32 = &mut COUNTER;
        htp_tx_set_user_data((*d).tx, counter_ptr as *mut core::ffi::c_void);

        Status::OK
    }
}

#[test]
fn RequestTrailerData() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).register_request_trailer_data(Some(
            ConnectionParsing_RequestTrailerData_REQUEST_TRAILER_DATA,
        ));
        assert!(t.run("27-request-trailer-raw.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        let counter: *mut i32 = htp_tx_get_user_data(tx) as *mut i32;
        assert!(!counter.is_null());
        assert_eq!(2, *counter);
    }
}

#[no_mangle]
extern "C" fn ConnectionParsing_ResponseHeaderData_RESPONSE_HEADER_DATA(
    d: *mut htp_tx_data_t,
) -> Status {
    unsafe {
        static mut COUNTER: i32 = 0;
        let len = (*d).len;
        let data: &[u8] = slice::from_raw_parts((*d).data, len);
        match COUNTER {
        0 => {
            if !((len == 5) && (data == b"Date:")) {
                eprintln!("Mismatch in chunk 0");
                COUNTER = -1;
            }
        }
        1 => {
            if !((len == 5) && (data == b" Mon,")) {
                eprintln!("Mismatch in chunk 1");
                COUNTER = -2;
            }
        }
        2 => {
            if !((len == 34) && (data == " 31 Aug 2009 20:25:50 GMT\r\nServer:".as_bytes())) {
                eprintln!("Mismatch in chunk 2");
                COUNTER = -3;
            }
        }
        3 => {
            if !((len == 83) && (data == " Apache\r\nConnection: close\r\nContent-Type: text/html\r\nTransfer-Encoding: chunked\r\n\r\n".as_bytes())) {
                eprintln!("Mismatch in chunk 3");
                COUNTER = -4;
            }
        }
        _ => {
            if COUNTER >= 0 {
                eprintln!("Seen more than 4 chunks");
                COUNTER = -5;
            }
        }
    }

        if COUNTER >= 0 {
            COUNTER += 1;
        }

        let counter_ptr: *mut i32 = &mut COUNTER;
        htp_tx_set_user_data((*d).tx, counter_ptr as *mut core::ffi::c_void);

        Status::OK
    }
}

#[test]
fn ResponseHeaderData() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).register_response_header_data(Some(
            ConnectionParsing_ResponseHeaderData_RESPONSE_HEADER_DATA,
        ));
        assert!(t.run("28-response-headers-raw.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        let counter: *mut i32 = htp_tx_get_user_data(tx) as *mut i32;
        assert!(!counter.is_null());
        assert_eq!(4, *counter);
    }
}

#[no_mangle]
extern "C" fn ConnectionParsing_ResponseTrailerData_RESPONSE_TRAILER_DATA(
    d: *mut htp_tx_data_t,
) -> Status {
    unsafe {
        static mut COUNTER: i32 = 0;
        let len = (*d).len;
        let data: &[u8] = slice::from_raw_parts((*d).data, len);
        match COUNTER {
            0 => {
                if !((len == 11) && (data == b"Set-Cookie:")) {
                    eprintln!("Mismatch in chunk 0");
                    COUNTER = -1;
                }
            }

            1 => {
                if !((len == 6) && (data == b" name=")) {
                    eprintln!("Mismatch in chunk 1");
                    COUNTER = -2;
                }
            }

            2 => {
                if !((len == 22) && (data == b"value\r\nAnother-Header:")) {
                    eprintln!("Mismatch in chunk 1");
                    COUNTER = -3;
                }
            }

            3 => {
                if !((len == 17) && (data == b" Header-Value\r\n\r\n")) {
                    eprintln!("Mismatch in chunk 1");
                    COUNTER = -4;
                }
            }

            _ => {
                if COUNTER >= 0 {
                    eprintln!("Seen more than 4 chunks");
                    COUNTER = -5;
                }
            }
        }

        if COUNTER >= 0 {
            COUNTER += 1;
        }

        let counter_ptr: *mut i32 = &mut COUNTER;
        htp_tx_set_user_data((*d).tx, counter_ptr as *mut core::ffi::c_void);

        Status::OK
    }
}

#[test]
fn ResponseTrailerData() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).register_response_trailer_data(Some(
            ConnectionParsing_ResponseTrailerData_RESPONSE_TRAILER_DATA,
        ));
        assert!(t.run("29-response-trailer-raw.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        let counter: *mut i32 = htp_tx_get_user_data(tx) as *mut i32;
        assert!(!counter.is_null());
        assert_eq!(4, *counter);
    }
}

#[test]
fn GetIPv6() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("30-get-ipv6.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).request_method.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));

        assert!(!(*tx).request_uri.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*tx).request_uri, "http://[::1]:8080/?p=%20")
        );

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).hostname, "[::1]"));
        assert_eq!(8080, (*(*tx).parsed_uri).port_number);

        assert!(!(*(*tx).parsed_uri).query.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).query, "p=%20"));

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "p")
                .unwrap()
                .value
                .cmp(" ")
        );
    }
}

#[test]
fn GetRequestLineNul() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("31-get-request-line-nul.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).request_uri.is_null());

        assert_eq!(0, bstr_cmp_str((*tx).request_uri, "/?p=%20"));
    }
}

#[test]
fn InvalidHostname1() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("32-invalid-hostname.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_HOSTH_INVALID));
        assert!((*tx).flags.contains(Flags::HTP_HOSTU_INVALID));
        assert!((*tx).flags.contains(Flags::HTP_HOST_INVALID));
    }
}

#[test]
fn InvalidHostname2() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("33-invalid-hostname.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).flags.contains(Flags::HTP_HOSTH_INVALID));
        assert!((*tx).flags.contains(Flags::HTP_HOSTU_INVALID));
        assert!((*tx).flags.intersects(Flags::HTP_HOST_INVALID));
    }
}

#[test]
fn InvalidHostname3() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("34-invalid-hostname.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_HOSTH_INVALID));
        assert!(!(*tx).flags.contains(Flags::HTP_HOSTU_INVALID));
        assert!((*tx).flags.intersects(Flags::HTP_HOST_INVALID));
    }
}

#[test]
fn API_connp_get_connection() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("34-invalid-hostname.t").is_ok());

        assert_eq!((*t.connp).conn, htp_connp_get_connection(t.connp));
    }
}

#[test]
fn EarlyResponse() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("35-early-response.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));
    }
}

#[test]
fn InvalidRequest1() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("36-invalid-request-1-invalid-c-l.t").is_err());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_HEADERS, (*tx).request_progress);

        assert!((*tx).flags.contains(Flags::HTP_REQUEST_INVALID));
        assert!((*tx).flags.contains(Flags::HTP_REQUEST_INVALID_C_L));

        assert!(!(*tx).request_hostname.is_null());
    }
}

#[test]
fn InvalidRequest2() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("37-invalid-request-2-t-e-and-c-l.t").is_ok());
        // No error, flags only.

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert!((*tx).flags.contains(Flags::HTP_REQUEST_SMUGGLING));

        assert!(!(*tx).request_hostname.is_null());
    }
}

#[test]
fn InvalidRequest3() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("38-invalid-request-3-invalid-t-e.t").is_err());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_HEADERS, (*tx).request_progress);

        assert!((*tx).flags.contains(Flags::HTP_REQUEST_INVALID));
        assert!((*tx).flags.contains(Flags::HTP_REQUEST_INVALID_T_E));

        assert!(!(*tx).request_hostname.is_null());
    }
}

#[test]
fn AutoDestroyCrash() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_tx_auto_destroy(1);
        assert!(t.run("39-auto-destroy-crash.t").is_ok());

        assert_eq!(4, (*(*t.connp).conn).transactions.len());
    }
}

#[test]
fn AuthBasic() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("40-auth-basic.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(HTP_AUTH_BASIC, (*tx).request_auth_type);

        assert!(!(*tx).request_auth_username.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_auth_username, "ivanr"));

        assert!(!(*tx).request_auth_password.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_auth_password, "secret"));
    }
}

#[test]
fn AuthDigest() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("41-auth-digest.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(HTP_AUTH_DIGEST, (*tx).request_auth_type);

        assert!(!(*tx).request_auth_username.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_auth_username, "ivanr"));

        assert!((*tx).request_auth_password.is_null());
    }
}

#[test]
fn Unknown_MethodOnly() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("42-unknown-method_only.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert!(!(*tx).request_method.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "HELLO"));

        assert!((*tx).request_uri.is_null());

        assert_eq!(1, (*tx).is_protocol_0_9);
    }
}

#[test]
fn InvalidProtocol() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("43-invalid-protocol.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(-2, (*tx).request_protocol_number); // HTP_PROTOCOL_INVALID,
    }
}

#[test]
fn AuthBasicInvalid() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("44-auth-basic-invalid.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(HTP_AUTH_BASIC, (*tx).request_auth_type);

        assert!((*tx).request_auth_username.is_null());

        assert!((*tx).request_auth_password.is_null());

        assert!((*tx).flags.contains(Flags::HTP_AUTH_INVALID));
    }
}

#[test]
fn AuthDigestUnquotedUsername() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("45-auth-digest-unquoted-username.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(HTP_AUTH_DIGEST, (*tx).request_auth_type);

        assert!((*tx).request_auth_username.is_null());

        assert!((*tx).request_auth_password.is_null());

        assert!((*tx).flags.contains(Flags::HTP_AUTH_INVALID));
    }
}

#[test]
fn AuthDigestInvalidUsername1() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("46-auth-digest-invalid-username.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(HTP_AUTH_DIGEST, (*tx).request_auth_type);

        assert!((*tx).request_auth_username.is_null());

        assert!((*tx).request_auth_password.is_null());

        assert!((*tx).flags.contains(Flags::HTP_AUTH_INVALID));
    }
}

#[test]
fn AuthUnrecognized() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("47-auth-unrecognized.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(HTP_AUTH_UNRECOGNIZED, (*tx).request_auth_type);

        assert!((*tx).request_auth_username.is_null());

        assert!((*tx).request_auth_password.is_null());
    }
}

#[test]
fn InvalidResponseHeaders1() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("48-invalid-response-headers-1.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        assert_eq!(8, (*(*tx).response_headers).size());

        let h_empty = (*(*tx).response_headers).get_nocase_nozero("").unwrap().1;
        assert!(!h_empty.is_null());
        assert_eq!(0, bstr_cmp_str((*h_empty).value, "No Colon"));
        assert!((*h_empty).flags.contains(Flags::HTP_FIELD_INVALID));
        assert!((*h_empty).flags.contains(Flags::HTP_FIELD_UNPARSEABLE));

        let h_lws = (*(*tx).response_headers)
            .get_nocase_nozero("Lws")
            .unwrap()
            .1;
        assert!(!h_lws.is_null());
        assert_eq!(0, bstr_cmp_str((*h_lws).value, "After Header Name"));
        assert!((*h_lws).flags.contains(Flags::HTP_FIELD_INVALID));

        let h_nottoken = (*(*tx).response_headers)
            .get_nocase_nozero("Header@Name")
            .unwrap()
            .1;
        assert!(!h_nottoken.is_null());
        assert_eq!(0, bstr_cmp_str((*h_nottoken).value, "Not Token"));
        assert!((*h_nottoken).flags.contains(Flags::HTP_FIELD_INVALID));
    }
}

#[test]
fn InvalidResponseHeaders2() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("49-invalid-response-headers-2.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        assert_eq!(6, (*(*tx).response_headers).size());

        let h_empty = (*(*tx).response_headers).get_nocase_nozero("").unwrap().1;
        assert!(!h_empty.is_null());
        assert_eq!(0, bstr_cmp_str((*h_empty).value, "Empty Name"));
        assert!((*h_empty).flags.contains(Flags::HTP_FIELD_INVALID));
    }
}

#[test]
fn Util() {
    use htp::htp_log;
    let mut t = Test::new();
    unsafe {
        assert!(t.run("50-util.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        // A message that should not be logged.
        let log_message_count = (*(*(*tx).connp).conn).messages.len();
        (*(*(*tx).connp).cfg).log_level = htp_log_level_t::HTP_LOG_NONE;
        htp_log!(
            (*tx).connp,
            htp_log_level_t::HTP_LOG_ERROR,
            htp_log_code::UNKNOWN,
            "Log message"
        );
        assert_eq!(log_message_count, (*(*(*tx).connp).conn).messages.len());
    }
}

#[test]
fn GetIPv6Invalid() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("51-get-ipv6-invalid.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).request_method.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));

        assert!(!(*tx).request_uri.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*tx).request_uri, "http://[::1:8080/?p=%20")
        );

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).hostname, "[::1:8080"));
    }
}

#[test]
fn InvalidPath() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("52-invalid-path.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).request_method.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));

        assert!(!(*tx).request_uri.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_uri, "invalid/path?p=%20"));

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).path, "invalid/path"));
    }
}

#[test]
fn PathUtf8_None() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("53-path-utf8-none.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).flags.contains(Flags::HTP_PATH_UTF8_VALID));
        assert!(!(*tx).flags.contains(Flags::HTP_PATH_UTF8_OVERLONG));
        assert!(!(*tx).flags.contains(Flags::HTP_PATH_HALF_FULL_RANGE));
    }
}

#[test]
fn PathUtf8_Valid() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("54-path-utf8-valid.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_VALID));
    }
}

#[test]
fn PathUtf8_Overlong2() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("55-path-utf8-overlong-2.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_OVERLONG));
    }
}

#[test]
fn PathUtf8_Overlong3() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("56-path-utf8-overlong-3.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_OVERLONG));
    }
}

#[test]
fn PathUtf8_Overlong4() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("57-path-utf8-overlong-4.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_OVERLONG));
    }
}

#[test]
fn PathUtf8_Invalid() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("58-path-utf8-invalid.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_INVALID));
        assert!(!(*tx).flags.contains(Flags::HTP_PATH_UTF8_VALID));
    }
}

#[test]
fn PathUtf8_FullWidth() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("59-path-utf8-fullwidth.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_HALF_FULL_RANGE));
    }
}

#[test]
fn PathUtf8_Decode_Valid() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_utf8_convert_bestfit(HTP_DECODER_URL_PATH, true);
        assert!(t.run("54-path-utf8-valid.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).path, "/Ristic.txt"));
    }
}

#[test]
fn PathUtf8_Decode_Overlong2() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_utf8_convert_bestfit(HTP_DECODER_URL_PATH, true);
        assert!(t.run("55-path-utf8-overlong-2.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_OVERLONG));

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).path, "/&.txt"));
    }
}

#[test]
fn PathUtf8_Decode_Overlong3() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_utf8_convert_bestfit(HTP_DECODER_URL_PATH, true);
        assert!(t.run("56-path-utf8-overlong-3.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_OVERLONG));

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).path, "/&.txt"));
    }
}

#[test]
fn PathUtf8_Decode_Overlong4() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_utf8_convert_bestfit(HTP_DECODER_URL_PATH, true);
        assert!(t.run("57-path-utf8-overlong-4.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_OVERLONG));

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).path, "/&.txt"));
    }
}

#[test]
fn PathUtf8_Decode_Invalid() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_utf8_convert_bestfit(HTP_DECODER_URL_PATH, true);
        assert!(t.run("58-path-utf8-invalid.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_UTF8_INVALID));
        assert!(!(*tx).flags.contains(Flags::HTP_PATH_UTF8_VALID));

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());

        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).path, "/Ristic?.txt"));
    }
}

#[test]
fn PathUtf8_Decode_FullWidth() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_utf8_convert_bestfit(HTP_DECODER_URL_PATH, true);
        assert!(t.run("59-path-utf8-fullwidth.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!((*tx).flags.contains(Flags::HTP_PATH_HALF_FULL_RANGE));

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).path, "/&.txt"));
    }
}

#[test]
fn RequestCookies() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("60-request-cookies.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(3, (*(*tx).request_cookies).size());

        let mut res = &(*(*tx).request_cookies)[0];
        assert_eq!(Ordering::Equal, res.0.cmp("p"));
        assert!(!res.1.is_null());
        assert_eq!(0, bstr_cmp_str(res.1, "1"));

        res = &(*(*tx).request_cookies)[1];
        assert_eq!(Ordering::Equal, res.0.cmp("q"));
        assert!(!res.1.is_null());
        assert_eq!(0, bstr_cmp_str(res.1, "2"));

        res = &(*(*tx).request_cookies)[2];
        assert_eq!(Ordering::Equal, res.0.cmp("z"));
        assert!(!res.1.is_null());
        assert_eq!(0, bstr_cmp_str(res.1, ""));
    }
}

#[test]
fn EmptyLineBetweenRequests() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("61-empty-line-between-requests.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        /*part of previous request body assert_eq!(1, (*tx).request_ignored_lines);*/
    }
}

#[test]
fn PostNoBody() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("62-post-no-body.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        let tx1: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx1.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx1).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx1).response_progress);

        let tx2: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx2.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx2).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx2).response_progress);
    }
}

#[test]
fn PostChunkedInvalid1() {
    let mut t = Test::new();
    assert!(t.run("63-post-chunked-invalid-1.t").is_err());
}

#[test]
fn PostChunkedInvalid2() {
    let mut t = Test::new();
    assert!(t.run("64-post-chunked-invalid-2.t").is_err());
}

#[test]
fn PostChunkedInvalid3() {
    let mut t = Test::new();
    assert!(t.run("65-post-chunked-invalid-3.t").is_err());
}

#[test]
fn PostChunkedSplitChunk() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("66-post-chunked-split-chunk.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "p")
                .unwrap()
                .value
                .cmp("0123456789")
        );
    }
}

#[test]
fn LongRequestLine1() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("67-long-request-line.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(
            0,
            bstr_cmp_str((*tx).request_uri, "/0123456789/0123456789/")
        );
    }
}

#[test]
fn LongRequestLine2() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_field_limits(0, 16);
        assert!(t.run("67-long-request-line.t").is_err());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_LINE, (*tx).request_progress);
    }
}

#[test]
fn InvalidRequestHeader() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("68-invalid-request-header.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        let h = (*(*tx).request_headers)
            .get_nocase_nozero("Header-With-NUL")
            .unwrap()
            .1;
        assert!(!h.is_null());
        assert_eq!(0, bstr_cmp_str((*h).value, "BEFORE"));
    }
}

#[test]
fn TestGenericPersonality() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_server_personality(HTP_SERVER_IDS);
        assert!(t.run("02-header-test-apache2.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
    }
}

#[test]
fn LongResponseHeader() {
    let mut t = Test::new();
    unsafe {
        (*t.cfg).set_field_limits(0, 16);
        assert!(t.run("69-long-response-header.t").is_err());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        //error first assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_HEADERS, (*tx).response_progress);
    }
}

#[test]
fn ResponseInvalidChunkLength() {
    let mut t = Test::new();
    assert!(t.run("70-response-invalid-chunk-length.t").is_ok());
}

#[test]
fn ResponseSplitChunk() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("71-response-split-chunk.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
    }
}

#[test]
fn ResponseBody() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("72-response-split-body.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
    }
}

#[test]
fn ResponseContainsTeAndCl() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("73-response-te-and-cl.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        assert!((*tx).flags.contains(Flags::HTP_REQUEST_SMUGGLING));
    }
}

#[test]
fn ResponseMultipleCl() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("74-response-multiple-cl.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        assert!((*tx).flags.contains(Flags::HTP_REQUEST_SMUGGLING));

        let h = (*(*tx).response_headers)
            .get_nocase_nozero("Content-Length")
            .unwrap()
            .1;
        assert!(!h.is_null());
        assert!(!(*h).value.is_null());
        assert!((*h).flags.contains(Flags::HTP_FIELD_REPEATED));

        assert_eq!(0, bstr_cmp_str((*h).value, "12"));
    }
}

#[test]
fn ResponseMultipleClMismatch() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("88-response-multiple-cl-mismatch.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        assert!((*tx).flags.contains(Flags::HTP_REQUEST_SMUGGLING));

        let h = (*(*tx).response_headers)
            .get_nocase_nozero("Content-Length")
            .unwrap()
            .1;
        assert!(!h.is_null());
        assert!(!(*h).value.is_null());
        assert!((*h).flags.contains(Flags::HTP_FIELD_REPEATED));

        assert_eq!(0, bstr_cmp_str((*h).value, "12"));

        assert_eq!(2, (*(*tx).conn).messages.len());
        let log: *mut htp_log_t = *(*(*tx).conn).messages.get(1).unwrap() as *mut htp_log_t;
        assert!(!log.is_null());
        assert_eq!((*log).msg, "Ambiguous response C-L value");
        assert_eq!(htp_log_level_t::HTP_LOG_WARNING, (*log).level);
    }
}

#[test]
fn ResponseInvalidCl() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("75-response-invalid-cl.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        assert!(!(*tx).flags.contains(Flags::HTP_REQUEST_SMUGGLING));
    }
}

#[test]
fn ResponseNoBody() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("76-response-no-body.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        let tx1: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx1.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx1).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx1).response_progress);

        let h = (*(*tx1).response_headers)
            .get_nocase_nozero("Server")
            .unwrap()
            .1;
        assert!(!h.is_null());
        assert!(!(*h).value.is_null());

        assert_eq!(0, bstr_cmp_str((*h).value, "Apache"));

        let tx2: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx2.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx2).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx2).response_progress);

        assert!(tx1 != tx2);
    }
}

#[test]
fn ResponseFoldedHeaders() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("77-response-folded-headers.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        let tx1: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx1.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx1).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx1).response_progress);

        let h = (*(*tx1).response_headers)
            .get_nocase_nozero("Server")
            .unwrap()
            .1;
        assert!(!h.is_null());
        assert!(!(*h).value.is_null());

        assert_eq!(0, bstr_cmp_str((*h).value, "Apache Server"));

        let tx2: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx2.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx2).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx2).response_progress);
    }
}

#[test]
fn ResponseNoStatusHeaders() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("78-response-no-status-headers.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
    }
}

#[test]
fn ConnectInvalidHostport() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("79-connect-invalid-hostport.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());
    }
}

#[test]
fn HostnameInvalid1() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("80-hostname-invalid-1.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());
    }
}

#[test]
fn HostnameInvalid2() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("81-hostname-invalid-2.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());
    }
}

#[test]
fn Put() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("82-put.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_hostname, "www.example.com"));
    }
}

#[test]
fn AuthDigestInvalidUsername2() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("83-auth-digest-invalid-username-2.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert_eq!(HTP_AUTH_DIGEST, (*tx).request_auth_type);

        assert!((*tx).request_auth_username.is_null());

        assert!((*tx).request_auth_password.is_null());

        assert!((*tx).flags.contains(Flags::HTP_AUTH_INVALID));
    }
}

#[test]
fn ResponseNoStatusHeaders2() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("84-response-no-status-headers-2.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
    }
}

// Test was commented out of libhtp
//#[test]
//fn ZeroByteRequestTimeout() {
//    let mut t = Test::new();
//unsafe {
//    assert!(t.run("85-zero-byte-request-timeout.t").is_ok());
//
//    assert_eq!(1, (*(*t.connp).conn).transactions.len());
//
//    let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
//    assert!(!tx.is_null());
//
//    assert_eq!(HTP_REQUEST_NOT_STARTED, (*tx).request_progress);
//    assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
//}}

#[test]
fn PartialRequestTimeout() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("86-partial-request-timeout.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
    }
}

#[test]
fn IncorrectHostAmbiguousWarning() {
    let mut t = Test::new();
    unsafe {
        assert!(t
            .run("87-issue-55-incorrect-host-ambiguous-warning.t")
            .is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(!(*tx).parsed_uri_raw.is_null());

        assert!(!(*(*tx).parsed_uri_raw).port.is_null());
        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri_raw).port, "443"));

        assert!(!(*(*tx).parsed_uri_raw).hostname.is_null());
        assert_eq!(
            0,
            bstr_cmp_str((*(*tx).parsed_uri_raw).hostname, "www.example.com")
        );

        assert_eq!(443, (*(*tx).parsed_uri_raw).port_number);

        assert!(!(*tx).request_hostname.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_hostname, "www.example.com"));

        assert!(!(*tx).flags.contains(Flags::HTP_HOST_AMBIGUOUS));
    }
}

#[test]
fn GetWhitespace() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("89-get-whitespace.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(0, bstr_cmp_str((*tx).request_method, " GET"));

        assert_eq!(0, bstr_cmp_str((*tx).request_uri, "/?p=%20"));

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).query.is_null());

        assert_eq!(0, bstr_cmp_str((*(*tx).parsed_uri).query, "p=%20"));

        assert_eq!(
            Ordering::Equal,
            htp_tx_req_get_param(&*(*tx).request_params, "p")
                .unwrap()
                .value
                .cmp(" ")
        );
    }
}

#[test]
fn RequestUriTooLarge() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("90-request-uri-too-large.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
    }
}

#[test]
fn RequestInvalid() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("91-request-unexpected-body.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());

        let mut tx: *mut htp_tx_t =
            *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "POST"));
        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        tx = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));
        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(HTP_RESPONSE_NOT_STARTED, (*tx).response_progress);
    }
}

#[test]
fn Http_0_9_MethodOnly() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("92-http_0_9-method_only.t").is_ok());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        assert!(!(*tx).request_method.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));

        assert_eq!(0, bstr_cmp_str((*tx).request_uri, "/"));

        assert_eq!(1, (*tx).is_protocol_0_9);
    }
}

#[test]
fn CompressedResponseDeflateAsGzip() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("93-compressed-response-deflateasgzip.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(755, (*tx).response_message_len);

        assert_eq!(1433, (*tx).response_entity_len);
    }
}

#[test]
fn CompressedResponseMultiple() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("94-compressed-response-multiple.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(51, (*tx).response_message_len);

        assert_eq!(25, (*tx).response_entity_len);
    }
}

#[test]
fn CompressedResponseGzipAsDeflate() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("95-compressed-response-gzipasdeflate.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(187, (*tx).response_message_len);

        assert_eq!(225, (*tx).response_entity_len);
    }
}

#[test]
fn CompressedResponseLzma() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("96-compressed-response-lzma.t").is_ok());

        assert_eq!(1, (*(*t.connp).conn).transactions.len());

        let tx: *mut htp_tx_t = *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());

        assert!(0 != htp_tx_is_complete(tx));

        assert_eq!(90, (*tx).response_message_len);

        assert_eq!(68, (*tx).response_entity_len);
    }
}

#[test]
fn RequestsCut() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("97-requests-cut.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());
        let mut tx: *mut htp_tx_t =
            *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));
        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);

        tx = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));
        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
    }
}

#[test]
fn ResponsesCut() {
    let mut t = Test::new();
    unsafe {
        assert!(t.run("98-responses-cut.t").is_ok());

        assert_eq!(2, (*(*t.connp).conn).transactions.len());
        let mut tx: *mut htp_tx_t =
            *(*(*t.connp).conn).transactions.get(0).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));
        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(200, (*tx).response_status_number);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);

        tx = *(*(*t.connp).conn).transactions.get(1).unwrap() as *mut htp_tx_t;
        assert!(!tx.is_null());
        assert_eq!(0, bstr_cmp_str((*tx).request_method, "GET"));
        assert_eq!(HTP_REQUEST_COMPLETE, (*tx).request_progress);
        assert_eq!(200, (*tx).response_status_number);
        assert_eq!(HTP_RESPONSE_COMPLETE, (*tx).response_progress);
    }
}
