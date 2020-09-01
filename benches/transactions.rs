use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use htp::c_api::{htp_connp_create, htp_connp_destroy_all};
use htp::htp_config;
use htp::htp_config::htp_server_personality_t::*;
use htp::htp_connection_parser::*;
use htp::htp_request::*;
use htp::htp_response::*;
use std::convert::TryInto;
use std::fmt;
use std::iter::IntoIterator;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::Drop;
use std::time::Duration;

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
    cfg: *mut htp_config::htp_cfg_t,
    connp: *mut htp_connp_t,
}

impl Test {
    fn new() -> Self {
        unsafe {
            let cfg = htp_config::create();
            (*cfg).set_server_personality(HTP_SERVER_APACHE_2).unwrap();
            (*cfg).register_urlencoded_parser();
            (*cfg).register_multipart_parser();
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
            (*self.connp).open(
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                10000,
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                80,
                Some(tv_start),
            );

            let mut in_buf: Option<Vec<u8>> = None;
            let mut out_buf: Option<Vec<u8>> = None;
            for chunk in test {
                match chunk {
                    Chunk::Client(data) => {
                        let rc = htp_connp_req_data(
                            self.connp,
                            Some(tv_start),
                            data.as_ptr() as *const core::ffi::c_void,
                            data.len(),
                        );
                        if rc == htp_stream_state_t::HTP_STREAM_ERROR {
                            return Err(TestError::StreamError);
                        }

                        if rc == htp_stream_state_t::HTP_STREAM_DATA_OTHER {
                            // HTP_STREAM_DATA_OTHER = 5
                            let consumed = (*self.connp)
                                .req_data_consumed()
                                .try_into()
                                .expect("Error retrieving number of consumed bytes.");
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
                                Some(tv_start),
                                out_remaining.as_ptr() as *const core::ffi::c_void,
                                out_remaining.len(),
                            );
                            out_buf = None;
                            if rc == htp_stream_state_t::HTP_STREAM_ERROR {
                                return Err(TestError::StreamError);
                            }
                        }

                        // Now use up this data chunk
                        let rc = htp_connp_res_data(
                            self.connp,
                            Some(tv_start),
                            data.as_ptr() as *const core::ffi::c_void,
                            data.len(),
                        );
                        if rc == htp_stream_state_t::HTP_STREAM_ERROR {
                            return Err(TestError::StreamError);
                        }

                        if rc == htp_stream_state_t::HTP_STREAM_DATA_OTHER {
                            let consumed = (*self.connp)
                                .res_data_consumed()
                                .try_into()
                                .expect("Error retrieving number of consumed bytes.");
                            let mut remaining = Vec::with_capacity(data.len() - consumed);
                            remaining.extend_from_slice(&data[consumed..]);
                            out_buf = Some(remaining);
                        }

                        // And check if we also had some input data buffered
                        if let Some(in_remaining) = in_buf {
                            let rc = htp_connp_req_data(
                                self.connp,
                                Some(tv_start),
                                in_remaining.as_ptr() as *const core::ffi::c_void,
                                in_remaining.len(),
                            );
                            in_buf = None;
                            if rc == htp_stream_state_t::HTP_STREAM_ERROR {
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
                    Some(tv_start),
                    out_remaining.as_ptr() as *const core::ffi::c_void,
                    out_remaining.len(),
                );
                if rc == htp_stream_state_t::HTP_STREAM_ERROR {
                    return Err(TestError::StreamError);
                }
            }

            let mut tv_end = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            libc::gettimeofday(&mut tv_end, std::ptr::null_mut());
            (*self.connp).close(Some(tv_end));
        }
        Ok(())
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            htp_connp_destroy_all(self.connp);
            (*self.cfg).destroy();
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(2, 0)).sample_size(50).without_plots();
    targets = small_transaction
}
criterion_main!(benches);

pub fn small_transaction(c: &mut Criterion) {
    let input = TestInput {
        chunks: {
            vec![
                Chunk::Client(
                    b"GET /?p=%20 HTTP/1.0\r\n\
                      User-Agent: Mozilla\r\n\r\n"
                        .to_vec(),
                ),
                Chunk::Server(
                    b"HTTP/1.0 200 OK\r\n\
                      Date: Mon, 31 Aug 2009 20:25:50 GMT\r\n\
                      Server: Apache\r\n\
                      Connection: close\r\n\
                      Content-Type: text/html\r\n\
                      Content-Length: 12\r\n\
                      \r\n\
                      Hello World!\r\n"
                        .to_vec(),
                ),
            ]
        },
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
