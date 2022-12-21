use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use htp::{
    config::{Config, HtpServerPersonality},
    connection_parser::*,
};
use std::{
    fmt,
    iter::IntoIterator,
    net::{IpAddr, Ipv4Addr},
    time::{Duration, SystemTime},
};
use time::OffsetDateTime;

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
    connp: ConnectionParser,
}

impl Test {
    fn new() -> Self {
        let mut cfg = Config::default();
        cfg.set_server_personality(HtpServerPersonality::APACHE_2)
            .unwrap();
        cfg.set_parse_urlencoded(true);
        cfg.set_parse_multipart(true);
        let connp = ConnectionParser::new(cfg);

        Test { connp }
    }

    fn run(&mut self, test: TestInput) -> Result<(), TestError> {
        let tv_start = OffsetDateTime::from(SystemTime::now());
        self.connp.open(
            Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            Some(10000),
            Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            Some(80),
            Some(tv_start),
        );

        let mut request_buf: Option<Vec<u8>> = None;
        let mut response_buf: Option<Vec<u8>> = None;
        for chunk in test {
            match chunk {
                Chunk::Client(ref data) => {
                    let rc = self.connp.request_data(data.into(), Some(tv_start));
                    if rc == HtpStreamState::ERROR {
                        return Err(TestError::StreamError);
                    }

                    if rc == HtpStreamState::DATA_OTHER {
                        // HTP_STREAM_DATA_OTHER = 5
                        let consumed = self.connp.request_data_consumed();
                        let mut remaining = Vec::with_capacity(data.len() - consumed);
                        remaining.extend_from_slice(&data[consumed..]);
                        request_buf = Some(remaining);
                    }
                }
                Chunk::Server(ref data) => {
                    // If we have leftover data from before then use it first
                    if let Some(ref response_remaining) = response_buf {
                        let rc = self
                            .connp
                            .response_data(response_remaining.into(), Some(tv_start));
                        response_buf = None;
                        if rc == HtpStreamState::ERROR {
                            return Err(TestError::StreamError);
                        }
                    }

                    // Now use up this data chunk
                    let rc = self.connp.response_data(data.into(), Some(tv_start));
                    if rc == HtpStreamState::ERROR {
                        return Err(TestError::StreamError);
                    }

                    if rc == HtpStreamState::DATA_OTHER {
                        let consumed = self.connp.response_data_consumed();
                        let mut remaining = Vec::with_capacity(data.len() - consumed);
                        remaining.extend_from_slice(&data[consumed..]);
                        response_buf = Some(remaining);
                    }

                    // And check if we also had some input data buffered
                    if let Some(ref request_remaining) = request_buf {
                        let rc = self
                            .connp
                            .request_data(request_remaining.into(), Some(tv_start));
                        request_buf = None;
                        if rc == HtpStreamState::ERROR {
                            return Err(TestError::StreamError);
                        }
                    }
                }
            }
        }

        // Clean up any remaining server data
        if let Some(ref response_remaining) = response_buf {
            let rc = self
                .connp
                .response_data(response_remaining.into(), Some(tv_start));
            if rc == HtpStreamState::ERROR {
                return Err(TestError::StreamError);
            }
        }

        self.connp
            .close(Some(OffsetDateTime::from(SystemTime::now())));
        Ok(())
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
