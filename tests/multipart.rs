#![allow(non_snake_case)]
use htp::{
    bstr::Bstr,
    config::{Config, HtpServerPersonality},
    connection_parser::ConnectionParser,
    multipart::*,
    transaction::{Header, Transaction},
    util::FlagOperations,
    HtpStatus,
};
use std::{
    fs,
    net::{IpAddr, Ipv4Addr},
    rc::Rc,
};

// import common testing utilities
mod common;

fn TestConfig() -> Config {
    let mut cfg = Config::default();
    cfg.set_server_personality(HtpServerPersonality::APACHE_2)
        .unwrap();
    cfg.set_parse_multipart(true);
    cfg
}

struct Test {
    connp: ConnectionParser,
}

impl Test {
    fn new(cfg: Config) -> Self {
        let connp = ConnectionParser::new(cfg);
        Test { connp }
    }

    fn tx(&mut self) -> &mut Transaction {
        self.connp.tx_mut(0).unwrap()
    }

    fn set_mpartp(&mut self, boundary: &[u8]) {
        // Ensure there is a tx for those tests where we just
        // make a Parser without feeding any data through the connp
        let _tx = self.connp.request();
        // And set the parser
        self.tx().request_mpartp = Some(Parser::new(&self.connp.cfg, boundary, 0));
    }

    fn mpartp(&mut self) -> &mut Parser {
        self.tx().request_mpartp.as_mut().unwrap()
    }

    fn body(&mut self) -> &mut Multipart {
        self.mpartp().get_multipart()
    }

    fn parseRequest(&mut self, headers: &[&str], data: &[&str]) {
        // Open connection
        self.connp.open(
            Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            Some(32768),
            Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            Some(80),
            None,
        );

        // Send headers
        for header in headers {
            self.connp.request_data(header.as_bytes().into(), None);
        }

        // Calculate body length.
        let mut bodyLen: usize = 0;
        for d in data {
            bodyLen += d.chars().count();
        }

        let contentStr = format!("Content-Length: {}\r\n", bodyLen);
        self.connp.request_data(contentStr.as_bytes().into(), None);

        self.connp.request_data((b"\r\n" as &[u8]).into(), None);

        // Send data.
        for d in data {
            self.connp.request_data(d.as_bytes().into(), None);
        }

        assert_eq!(1, self.connp.tx_size());
    }

    fn parseRequestThenVerify(&mut self, headers: &[&str], data: &[&str]) {
        self.parseRequest(headers, data);
        assert_eq!(3, self.body().parts.len());

        assert!(!self.body().flags.is_set(Flags::INCOMPLETE));

        // Field 1
        let field1 = self.body().parts.get(0);
        assert!(field1.is_some());
        let field1 = field1.unwrap();
        assert_eq!(HtpMultipartType::TEXT, field1.type_0);
        assert!(field1.name.eq_slice("field1"));
        assert!(field1.value.eq_slice("ABCDEF"));

        // File 1
        let file1 = self.body().parts.get(1);
        assert!(file1.is_some());
        let file1 = file1.unwrap();
        assert_eq!(HtpMultipartType::FILE, file1.type_0);
        assert!(file1.name.eq_slice("file1"));

        assert!(file1.file.is_some());
        let file = file1.file.as_ref().unwrap();
        assert!(file.filename.is_some());
        let filename = file.filename.as_ref().unwrap();
        assert!(filename.eq_slice("file.bin"));

        // Field 2
        let field2 = self.body().parts.get(2);
        assert!(field2.is_some());
        let field2 = field2.unwrap();
        assert_eq!(HtpMultipartType::TEXT, field2.type_0);
        assert!(field2.name.eq_slice("field2"));
        assert!(field2.value.eq_slice("GHIJKL"));
    }
    fn parseParts(&mut self, parts: &[&str]) {
        self.set_mpartp(b"0123456789");
        for part in parts {
            self.mpartp().parse(part.as_bytes());
        }

        self.mpartp().finalize().unwrap();
    }

    fn parsePartsThenVerify(&mut self, parts: &[&str]) {
        self.parseParts(parts);

        // Examine the result
        assert_eq!(2, self.body().parts.len());

        let part = self.body().parts.get(0);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(HtpMultipartType::TEXT, part.type_0);
        assert!(part.name.eq_slice("field1"));
        assert!(part.value.eq_slice("ABCDEF"));

        let part = self.body().parts.get(1);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(HtpMultipartType::TEXT, part.type_0);
        assert!(part.name.eq_slice("field2"));
        assert!(part.value.eq_slice("GHIJKL"));
    }
}

#[test]
fn Test1() {
    let mut t = Test::new(TestConfig());
    t.set_mpartp(b"---------------------------41184676334");

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
        t.mpartp().parse(part.as_bytes());
    }

    t.mpartp().finalize().unwrap();

    // Examine the result
    assert_eq!(5, t.body().parts.len());

    let part = t.body().parts.get(0);
    assert!(part.is_some());
    let part = part.unwrap();
    assert!(part.name.eq_slice("field1"));
    assert_eq!(HtpMultipartType::TEXT, part.type_0);
    assert!(part.value.eq_slice("0123456789"));

    let part = t.body().parts.get(1);
    assert!(part.is_some());
    let part = part.unwrap();
    assert!(part.name.eq_slice("field2"));
    assert_eq!(HtpMultipartType::TEXT, part.type_0);
    assert!(part
        .value
        .eq_slice("0123456789\r\n----------------------------X"));

    let part = t.body().parts.get(2);
    assert!(part.is_some());
    let part = part.unwrap();
    assert!(part.name.eq_slice("field3"));
    assert_eq!(HtpMultipartType::TEXT, part.type_0);
    assert!(part.value.eq_slice("9876543210"));

    let part = t.body().parts.get(3);
    assert!(part.is_some());
    let part = part.unwrap();
    assert!(part.name.eq_slice("file1"));
    assert_eq!(HtpMultipartType::FILE, part.type_0);

    let part = t.body().parts.get(4);
    assert!(part.is_some());
    let part = part.unwrap();
    assert!(part.name.eq_slice("file2"));
    assert_eq!(HtpMultipartType::FILE, part.type_0);

    assert!(!t.body().flags.is_set(Flags::PART_INCOMPLETE));
}

#[test]
fn Test2() {
    let mut t = Test::new(TestConfig());
    t.set_mpartp(b"BBB");

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
        t.mpartp().parse(part.as_bytes());
    }

    t.mpartp().finalize().unwrap();

    assert_eq!(4, t.body().parts.len());

    let part = t.body().parts.get(0);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::PREAMBLE, part.type_0);
    assert!(part.value.eq_slice("x0000x"));

    let part = t.body().parts.get(1);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::UNKNOWN, part.type_0);
    assert!(part.value.eq_slice("x1111x\n--\nx2222x"));

    let part = t.body().parts.get(2);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::UNKNOWN, part.type_0);
    assert!(part.value.eq_slice("x3333x\n--BB\n\nx4444x\n--BB"));

    let part = t.body().parts.get(3);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::UNKNOWN, part.type_0);
    assert!(part.value.eq_slice("x5555x\r\n--x6666x\r--"));

    assert!(t.body().flags.is_set(Flags::INCOMPLETE));
}

#[test]
fn Test3() {
    let mut t = Test::new(TestConfig());
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
    let mut t = Test::new(TestConfig());
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
    let mut t = Test::new(TestConfig());
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
    let mut t = Test::new(TestConfig());
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
    let mut t = Test::new(TestConfig());
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
    let mut t = Test::new(TestConfig());
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
    let mut t = Test::new(TestConfig());
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

    assert!(!t.body().flags.is_set(Flags::LF_LINE));
    assert!(t.body().flags.is_set(Flags::CRLF_LINE));
}

#[test]
fn LfLineEndings() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::LF_LINE));
    assert!(!t.body().flags.is_set(Flags::CRLF_LINE));
}

#[test]
fn CrAndLfLineEndings1() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::LF_LINE));
    assert!(t.body().flags.is_set(Flags::CRLF_LINE));
}

#[test]
fn CrAndLfLineEndings2() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::LF_LINE));
    assert!(t.body().flags.is_set(Flags::CRLF_LINE));
}

#[test]
fn CrAndLfLineEndings3() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::LF_LINE));
    assert!(t.body().flags.is_set(Flags::CRLF_LINE));
}

#[test]
fn CrAndLfLineEndings4() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::LF_LINE));
    assert!(t.body().flags.is_set(Flags::CRLF_LINE));
}

#[test]
fn BoundaryInstanceWithLwsAfter() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::BBOUNDARY_LWS_AFTER));
}

#[test]
fn BoundaryInstanceWithNonLwsAfter1() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::BBOUNDARY_NLWS_AFTER));
}

#[test]
fn BoundaryInstanceWithNonLwsAfter2() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::BBOUNDARY_NLWS_AFTER));
}

#[test]
fn BoundaryInstanceWithNonLwsAfter3() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::BBOUNDARY_NLWS_AFTER));
}

#[test]
fn WithPreamble() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::HAS_PREAMBLE));

    let part = t.body().parts.get(0);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::PREAMBLE, part.type_0);
    assert!(part.value.eq_slice("Preamble"));
}

#[test]
fn WithEpilogue1() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::HAS_EPILOGUE));

    let part = t.body().parts.get(2);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::EPILOGUE, part.type_0);
    assert!(part.value.eq_slice("Epilogue"));
    assert!(!t.body().flags.is_set(Flags::INCOMPLETE));
    assert!(!t.body().flags.is_set(Flags::PART_INCOMPLETE));
}

#[test]
fn WithEpilogue2() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::HAS_EPILOGUE));

    let part = t.body().parts.get(2).unwrap();
    assert_eq!(HtpMultipartType::EPILOGUE, part.type_0);
    assert!(part.value.eq_slice("Epi\nlogue"));
    assert!(!t.body().flags.is_set(Flags::INCOMPLETE));
    assert!(!t.body().flags.is_set(Flags::PART_INCOMPLETE));
}

#[test]
fn WithEpilogue3() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::HAS_EPILOGUE));

    let part = t.body().parts.get(2).unwrap();
    assert_eq!(HtpMultipartType::EPILOGUE, part.type_0);
    assert!(part.value.eq_slice("Epi\r\n--logue"));
    assert!(!t.body().flags.is_set(Flags::INCOMPLETE));
    assert!(!t.body().flags.is_set(Flags::PART_INCOMPLETE));
}

#[test]
fn WithEpilogue4() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(4, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::HAS_EPILOGUE));

    let ep1 = t.body().parts.get(2);
    assert!(ep1.is_some());
    let ep1 = ep1.unwrap();
    assert_eq!(HtpMultipartType::EPILOGUE, ep1.type_0);
    assert!(ep1.value.eq_slice("Epilogue1"));

    let ep2 = t.body().parts.get(3);
    assert!(ep2.is_some());
    let ep2 = ep2.unwrap();
    assert_eq!(HtpMultipartType::EPILOGUE, ep2.type_0);
    assert!(ep2.value.eq_slice("Epilogue2"));

    assert!(!t.body().flags.is_set(Flags::INCOMPLETE));
    assert!(!t.body().flags.is_set(Flags::PART_INCOMPLETE));
}

#[test]
fn HasLastBoundary() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(2, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::SEEN_LAST_BOUNDARY));
}

#[test]
fn DoesNotHaveLastBoundary() {
    let mut t = Test::new(TestConfig());
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

    assert!(!t.body().flags.is_set(Flags::SEEN_LAST_BOUNDARY));
}

#[test]
fn PartAfterLastBoundary() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::SEEN_LAST_BOUNDARY));
}

#[test]
fn UnknownPart() {
    let mut t = Test::new(TestConfig());
    let parts = vec![
        "--0123456789\r\n\
         \r\n\
         ABCDEF\
         \r\n--0123456789--",
    ];

    t.parseParts(&parts);

    assert_eq!(1, t.body().parts.len());

    let part = t.body().parts.get(0);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::UNKNOWN, part.type_0);
}

#[test]
fn WithFile() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(2, t.body().parts.len());

    let part = t.body().parts.get(1);
    assert!(part.is_some());
    let part = part.unwrap();
    assert_eq!(HtpMultipartType::FILE, part.type_0);
    assert_eq!(
        Some(Bstr::from("application/octet-stream")),
        part.content_type
    );
    assert!(part.file.is_some());
    let file = part.file.as_ref().unwrap();
    assert!(file.filename.is_some());
    let filename = file.filename.as_ref().unwrap();
    assert!(filename.eq_slice("test.bin"));
    assert_eq!(6, file.len);
}

#[test]
fn WithFileExternallyStored() {
    let tmpfile = {
        let mut cfg = TestConfig();
        cfg.multipart_cfg.extract_request_files = true;
        let mut t = Test::new(cfg);
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

        assert_eq!(2, t.body().parts.len());

        let part = t.body().parts.get(1);
        assert!(part.is_some());
        let part = part.unwrap();
        assert_eq!(HtpMultipartType::FILE, part.type_0);

        assert_eq!(
            part.content_type,
            Some(Bstr::from("application/octet-stream"))
        );
        assert!(part.file.is_some());
        let file = part.file.as_ref().unwrap();
        assert!(file.filename.is_some());
        let filename = file.filename.as_ref().unwrap();
        assert!(filename.eq_slice("test.bin"));
        assert_eq!(6, file.len);

        assert!(file.tmpfile.is_some());
        let name = file
            .tmpfile
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .path()
            .to_path_buf();

        let contents = fs::read_to_string(&name).unwrap();
        assert_eq!(6, contents.chars().count());
        assert_eq!(contents, "GHIJKL");
        name
    };
    assert!(!tmpfile.exists());
}

#[test]
fn PartHeadersEmptyLineBug() {
    let mut t = Test::new(TestConfig());
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
    let mut t = Test::new(TestConfig());
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

    assert!(!t.body().flags.is_set(Flags::PART_HEADER_FOLDING));
}

#[test]
fn InvalidHeader1() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::PART_HEADER_INVALID));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidHeader2() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::PART_HEADER_INVALID));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidHeader3() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::PART_HEADER_INVALID));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidHeader4() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::PART_HEADER_INVALID));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidHeader5() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::PART_HEADER_INVALID));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidHeader6() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::PART_HEADER_INVALID));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn NullByte() {
    let mut t = Test::new(TestConfig());
    t.set_mpartp(b"0123456789");

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

    t.mpartp().parse(i1.as_bytes());
    t.mpartp().parse(i2.as_bytes());
    t.mpartp().parse(i3.as_bytes());
    t.mpartp().finalize().unwrap();

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::NUL_BYTE));
    assert!(t.body().flags.is_set(Flags::INVALID));
}

#[test]
fn MultipleContentTypeHeadersEvasion() {
    let mut t = Test::new(TestConfig());
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
    assert_eq!(
        t.tx().request_content_type,
        Some(Bstr::from("multipart/form-data"))
    );
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
        let mut flags: u64 = 0;
        assert_eq!(find_boundary(inputs[i], &mut flags).unwrap(), outputs[i]);
        assert_eq!(0, flags);
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
        let mut flags: u64 = 0;
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
        let mut flags: u64 = 0;
        find_boundary(input, &mut flags);
        assert!(flags.is_set(Flags::HBOUNDARY_INVALID));
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
        let mut flags: u64 = 0;
        assert!(find_boundary(input, &mut flags).is_some());
        assert!(flags.is_set(Flags::HBOUNDARY_UNUSUAL));
    }
}

#[test]
fn CaseInsensitiveBoundaryMatching() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(2, t.body().parts.len());
}

#[test]
fn FoldedContentDisposition() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::PART_HEADER_FOLDING));
}

#[test]
fn FoldedContentDisposition2() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::PART_HEADER_FOLDING));
}

#[test]
fn InvalidPartNoData() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert_eq!(
        HtpMultipartType::UNKNOWN,
        t.body().parts.get(0).unwrap().type_0
    );

    assert!(t.body().flags.is_set(Flags::PART_INCOMPLETE));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidPartNoContentDisposition() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::PART_UNKNOWN));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidPartMultipleCD() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::PART_HEADER_REPEATED));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidPartUnknownHeader() {
    let mut t = Test::new(TestConfig());
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

    assert!(t.body().flags.is_set(Flags::PART_HEADER_UNKNOWN));
    assert!(t.body().flags.is_set(Flags::PART_INVALID));
}

#[test]
fn InvalidContentDispositionMultipleParams1() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::CD_PARAM_REPEATED));
    assert!(t.body().flags.is_set(Flags::CD_INVALID));
}

#[test]
fn InvalidContentDispositionMultipleParams2() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::CD_PARAM_REPEATED));
    assert!(t.body().flags.is_set(Flags::CD_INVALID));
}

#[test]
fn InvalidContentDispositionUnknownParam() {
    let mut t = Test::new(TestConfig());
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

    assert_eq!(3, t.body().parts.len());

    assert!(t.body().flags.is_set(Flags::CD_PARAM_UNKNOWN));
    assert!(t.body().flags.is_set(Flags::CD_INVALID));
}

#[test]
fn InvalidContentDispositionSyntax() {
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
    let cfg = Rc::new(Config::default());
    for input in inputs {
        let parser = &mut Parser::new(&cfg, b"123", 0);
        parser.multipart.parts.push(Part::default());
        parser.current_part_idx = Some(0);
        let part = parser.get_current_part().unwrap();
        let header = Header::new(b"Content-Disposition".to_vec().into(), input.into());
        part.headers.add(header.name.clone(), header);
        assert_err!(parser.parse_c_d(), HtpStatus::DECLINED);
        assert!(parser.multipart.flags.is_set(Flags::CD_SYNTAX_INVALID));
        assert!(parser.multipart.flags.is_set(Flags::CD_INVALID));
    }
}

#[test]
fn ParamValueEscaping() {
    let mut t = Test::new(TestConfig());
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
    assert_eq!(3, t.body().parts.len());

    assert!(!t.body().flags.is_set(Flags::CD_INVALID));

    let field1 = t.body().parts.get(0);
    assert!(field1.is_some());
    let field1 = field1.unwrap();
    assert_eq!(HtpMultipartType::TEXT, field1.type_0);
    assert!(field1.name.eq_slice("---\"---\\---"));
    assert!(field1.value.eq_slice("ABCDEF"));
}

#[test]
fn HeaderValueTrim() {
    let mut t = Test::new(TestConfig());
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

    let field1 = t.body().parts.get(0).unwrap();
    let header = &field1
        .headers
        .get_nocase_nozero("content-disposition")
        .unwrap()
        .1;
    assert_eq!(header.value, "form-data; name=\"field1\" ");
}
