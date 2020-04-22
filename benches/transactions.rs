use libhtp2::htp_config::htp_server_personality_t::*;
use libhtp2::htp_config::*;
use libhtp2::htp_connection_parser::*;
use libhtp2::htp_request::*;
use libhtp2::htp_response::*;
use std::ffi::CString;
use std::iter::IntoIterator;
use std::ops::Drop;
use std::fmt;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! cstr {
    ( $x:expr ) => {{
        CString::new($x).unwrap().as_ptr()
    }};
}

#[derive(Debug, Clone)]
enum Chunk {
    Client(Vec<u8>),
    Server(Vec<u8>),
}

#[derive(Debug, Clone)]
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

impl fmt::Display for TestInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "TestInput length: {}", self.chunks.len())
    }
}

#[derive(Debug)]
enum TestError {
    //MultipleClientChunks,
    //MultipleServerChunks,
    StreamError,
}

struct Test {
    cfg: *mut htp_cfg_t,
    connp: *mut htp_connp_t,
}

impl Test {
    fn new() -> Self {
        unsafe {
            let cfg: *mut htp_cfg_t = htp_config_create();
            assert!(!cfg.is_null());
            htp_config_set_server_personality(cfg, HTP_SERVER_APACHE_2);
            htp_config_register_urlencoded_parser(cfg);
            htp_config_register_multipart_parser(cfg);
            let connp = htp_connp_create(cfg);
            assert!(!connp.is_null());

            Test { cfg, connp }
        }
    }

    fn run(&mut self, test: TestInput) -> Result<(), TestError> {
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

            let mut in_buf: Option<Vec<u8>> = None;
            let mut out_buf: Option<Vec<u8>> = None;
            for chunk in test {
                match chunk {
                    Chunk::Client(data) => {
                        let rc = htp_connp_req_data(
                            self.connp,
                            &tv_start,
                            data.as_ptr() as *const core::ffi::c_void,
                            data.len() as u64,
                        );
                        if rc == 3 {
                            // HTP_STREAM_ERROR = 3
                            return Err(TestError::StreamError);
                        }

                        if rc == 5 {
                            // HTP_STREAM_DATA_OTHER = 5
                            let consumed = htp_connp_req_data_consumed(self.connp) as usize;
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
                                out_remaining.len() as u64,
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
                            data.len() as u64,
                        );
                        if rc == 3 {
                            // HTP_STREAM_ERROR = 3
                            return Err(TestError::StreamError);
                        }

                        if rc == 5 {
                            // HTP_STREAM_DATA_OTHER
                            let consumed = htp_connp_res_data_consumed(self.connp) as usize;
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
                                in_remaining.len() as u64,
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
                    out_remaining.len() as u64,
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
            htp_config_destroy(self.cfg);
        }
    }
}

criterion_group!(benches, small_transaction);
criterion_main!(benches);

pub fn small_transaction(c: &mut Criterion) {
    let input = TestInput {
        chunks: {
            vec![
                Chunk::Client(b"GET /?p=%20 HTTP/1.0\r\n\
                               User-Agent: Mozilla\r\n\r\n".to_vec()),
                Chunk::Server(b"HTTP/1.0 200 OK\r\n\
                               Date: Mon, 31 Aug 2009 20:25:50 GMT\r\n\
                               Server: Apache\r\n\
                               Connection: close\r\n\
                               Content-Type: text/html\r\n\
                               Content-Length: 12\r\n\
                               \r\n\
                               Hello World!\r\n".to_vec())
            ]
        }
    };

    c.bench_with_input(
        BenchmarkId::new("Small Transaction", input.clone()),
        &input,
        |b, i| {
            let mut test = Test::new();
            b.iter(|| test.run(i.clone()));
        },
    );
}
