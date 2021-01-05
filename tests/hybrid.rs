#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use htp::{
    bstr::Bstr,
    c_api::{htp_connp_create, htp_connp_destroy_all},
    config::{Config, HtpServerPersonality},
    connection_parser::ConnectionParser,
    error::Result,
    transaction::{Data, Header, HtpDataSource, HtpProtocol, HtpResponseNumber, Transaction},
    uri::Uri,
    HtpStatus,
};
use std::{
    ffi::CString,
    net::{IpAddr, Ipv4Addr},
    ops::Drop,
};

// import common testing utilities
mod common;
use common::htp_connp_tx_create;

struct HybridParsing_Get_User_Data {
    // Request callback indicators.
    callback_REQUEST_START_invoked: i32,
    callback_REQUEST_LINE_invoked: i32,
    callback_REQUEST_HEADERS_invoked: i32,
    callback_REQUEST_COMPLETE_invoked: i32,

    // Response callback indicators.
    callback_RESPONSE_START_invoked: i32,
    callback_RESPONSE_LINE_invoked: i32,
    callback_RESPONSE_HEADERS_invoked: i32,
    callback_RESPONSE_COMPLETE_invoked: i32,

    // Transaction callback indicators.
    callback_TRANSACTION_COMPLETE_invoked: i32,

    // Response body handling fields.
    response_body_chunks_seen: i32,
    response_body_correctly_received: i32,
}

impl HybridParsing_Get_User_Data {
    pub fn new() -> Self {
        HybridParsing_Get_User_Data {
            callback_REQUEST_START_invoked: 0,
            callback_REQUEST_LINE_invoked: 0,
            callback_REQUEST_HEADERS_invoked: 0,
            callback_REQUEST_COMPLETE_invoked: 0,
            callback_RESPONSE_START_invoked: 0,
            callback_RESPONSE_LINE_invoked: 0,
            callback_RESPONSE_HEADERS_invoked: 0,
            callback_RESPONSE_COMPLETE_invoked: 0,
            callback_TRANSACTION_COMPLETE_invoked: 0,
            response_body_chunks_seen: 0,
            response_body_correctly_received: 0,
        }
    }
}

fn HybridParsing_Get_Callback_REQUEST_START(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_START_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_REQUEST_LINE(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_LINE_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_REQUEST_HEADERS(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_HEADERS_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_REQUEST_COMPLETE(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_COMPLETE_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_RESPONSE_START(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_START_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_RESPONSE_LINE(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_LINE_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_RESPONSE_HEADERS(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_HEADERS_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_RESPONSE_BODY_DATA(d: &mut Data) -> Result<()> {
    unsafe {
        let user_data = (*(*d).tx()).user_data() as *mut HybridParsing_Get_User_Data;

        // Don't do anything if in errored state.
        if (*user_data).response_body_correctly_received == -1 {
            return Err(HtpStatus::ERROR);
        }

        match (*user_data).response_body_chunks_seen {
            0 => {
                if (*d).len() == 9
                    && (libc::memcmp(
                        (*d).data() as *const core::ffi::c_void,
                        cstr!("<h1>Hello") as *const core::ffi::c_void,
                        9,
                    ) == 0)
                {
                    (*user_data).response_body_chunks_seen += 1;
                } else {
                    eprintln!("Mismatch in 1st chunk");
                    (*user_data).response_body_correctly_received = -1;
                }
            }
            1 => {
                if (*d).len() == 1
                    && (libc::memcmp(
                        (*d).data() as *const core::ffi::c_void,
                        cstr!(" ") as *const core::ffi::c_void,
                        1,
                    ) == 0)
                {
                    (*user_data).response_body_chunks_seen += 1;
                } else {
                    eprintln!("Mismatch in 2nd chunk");
                    (*user_data).response_body_correctly_received = -1;
                }
            }
            2 => {
                if (*d).len() == 11
                    && (libc::memcmp(
                        (*d).data() as *const core::ffi::c_void,
                        cstr!("World!</h1>") as *const core::ffi::c_void,
                        11,
                    ) == 0)
                {
                    (*user_data).response_body_chunks_seen += 1;
                    (*user_data).response_body_correctly_received = 1;
                } else {
                    eprintln!("Mismatch in 3rd chunk");
                    (*user_data).response_body_correctly_received = -1;
                }
            }
            _ => {
                eprintln!("Seen more than 3 chunks");
                (*user_data).response_body_correctly_received = -1;
            }
        }
    }
    Ok(())
}

fn HybridParsing_Get_Callback_RESPONSE_COMPLETE(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_COMPLETE_invoked += 1;
    }
    Ok(())
}

fn HybridParsing_Get_Callback_TRANSACTION_COMPLETE(tx: &mut Transaction) -> Result<()> {
    unsafe {
        let user_data = tx.user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_TRANSACTION_COMPLETE_invoked += 1;
    }
    Ok(())
}

/// Set one request header.
unsafe fn req_set_header<S: AsRef<[u8]>>(tx: &mut Transaction, name: S, value: S) {
    tx.request_headers.add(
        name.as_ref().into(),
        Header::new(name.as_ref().into(), value.as_ref().into()),
    )
}

/// Set request line. When used, this function should always be called first,
/// with more specific functions following. Must not contain line terminators.
unsafe fn req_set_line<S: AsRef<[u8]>>(connp: &mut ConnectionParser, line: S) -> Result<()> {
    connp.parse_request_line(line.as_ref())
}

/// Set response line. Use this function is you have a single buffer containing
/// the entire line. If you have individual request line pieces, use the other
/// available functions.
unsafe fn res_set_status_line<S: AsRef<[u8]>>(connp: &mut ConnectionParser, line: S) -> Result<()> {
    connp.parse_response_line(line.as_ref())
}
fn TestConfig() -> Config {
    let mut cfg = Config::default();
    cfg.set_server_personality(HtpServerPersonality::APACHE_2)
        .unwrap();
    cfg.register_urlencoded_parser();
    cfg.register_multipart_parser();
    return cfg;
}

fn register_user_callbacks(cfg: &mut Config) {
    // Request callbacks
    cfg.register_request_start(HybridParsing_Get_Callback_REQUEST_START);
    cfg.register_request_line(HybridParsing_Get_Callback_REQUEST_LINE);
    cfg.register_request_headers(HybridParsing_Get_Callback_REQUEST_HEADERS);
    cfg.register_request_complete(HybridParsing_Get_Callback_REQUEST_COMPLETE);

    // Response callbacks
    cfg.register_response_start(HybridParsing_Get_Callback_RESPONSE_START);
    cfg.register_response_line(HybridParsing_Get_Callback_RESPONSE_LINE);
    cfg.register_response_headers(HybridParsing_Get_Callback_RESPONSE_HEADERS);
    cfg.register_response_body_data(HybridParsing_Get_Callback_RESPONSE_BODY_DATA);
    cfg.register_response_complete(HybridParsing_Get_Callback_RESPONSE_COMPLETE);

    // Transaction calllbacks
    cfg.register_transaction_complete(HybridParsing_Get_Callback_TRANSACTION_COMPLETE);
}

struct HybridParsingTest {
    connp: *mut ConnectionParser,
    connp_open: bool,
    user_data: HybridParsing_Get_User_Data,
}

impl HybridParsingTest {
    fn new(mut cfg: Config) -> Self {
        unsafe {
            let connp = htp_connp_create(&mut cfg);
            assert!(!connp.is_null());
            (*connp).open(
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(32768),
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                Some(80),
                None,
            );

            let user_data = HybridParsing_Get_User_Data::new();
            HybridParsingTest {
                connp,
                connp_open: true,
                user_data,
            }
        }
    }

    fn close_conn_parser(&mut self) {
        unsafe {
            if self.connp_open {
                (*self.connp).close(None);
                self.connp_open = false;
            }
        }
    }
}

impl Drop for HybridParsingTest {
    fn drop(&mut self) {
        unsafe {
            self.close_conn_parser();
            htp_connp_destroy_all(self.connp);
        }
    }
}

/// Test hybrid mode with one complete GET transaction; request then response
/// with a body. Most features are tested, including query string parameters and callbacks.
#[test]
fn GetTest() {
    unsafe {
        let mut cfg = TestConfig();
        // Register callbacks
        register_user_callbacks(&mut cfg);
        let mut t = HybridParsingTest::new(cfg);
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Configure user data and callbacks
        (*tx).set_user_data(&mut t.user_data as *mut _ as *mut core::ffi::c_void);

        // Request begins
        (*tx).state_request_start(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_START_invoked);

        // Request line data
        req_set_line(&mut *t.connp, "GET /?p=1&q=2 HTTP/1.1").unwrap();

        // Request line complete
        (*tx).state_request_line(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_LINE_invoked);

        // Check request line data
        assert!((*tx).request_method.as_ref().unwrap().eq("GET"));
        assert!((*tx).request_uri.as_ref().unwrap().eq("/?p=1&q=2"));
        assert!((*tx).request_protocol.as_ref().unwrap().eq("HTTP/1.1"));
        assert!((*tx)
            .parsed_uri
            .as_ref()
            .unwrap()
            .path
            .as_ref()
            .unwrap()
            .eq("/"));
        assert!((*tx)
            .parsed_uri
            .as_ref()
            .unwrap()
            .query
            .as_ref()
            .unwrap()
            .eq("p=1&q=2"));

        // Check parameters
        assert_contains_param!(&(*tx).request_params, "p", "1");
        assert_contains_param!(&(*tx).request_params, "q", "2");

        // Request headers
        req_set_header(&mut *tx, "Host", "www.example.com");
        req_set_header(&mut *tx, "Connection", "keep-alive");
        req_set_header(&mut *tx, "User-Agent", "Mozilla/5.0");

        // Request headers complete
        (*tx).state_request_headers(&mut *t.connp).unwrap();

        // Check headers
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);

        assert_request_header_eq!(tx, "host", "www.example.com");
        assert_request_header_eq!(tx, "connection", "keep-alive");
        assert_request_header_eq!(tx, "user-agent", "Mozilla/5.0");

        // Request complete
        (*tx).state_request_complete(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);

        // Response begins
        (*tx).state_response_start(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);

        // Response line data
        res_set_status_line(&mut *t.connp, "HTTP/1.1 200 OK").unwrap();
        assert!((*tx).response_protocol.as_ref().unwrap().eq("HTTP/1.1"));
        assert_eq!(HtpProtocol::V1_1, (*tx).response_protocol_number);
        assert!((*tx).response_status.as_ref().unwrap().eq("200"));
        assert!((*tx).response_status_number.eq(200));
        assert!((*tx).response_message.as_ref().unwrap().eq("OK"));

        // Response line complete
        (*tx).state_response_line(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);

        // Response header data
        (*tx).res_set_header("Content-Type", "text/html");
        (*tx).res_set_header("Server", "Apache");

        // Response headers complete
        (*tx).state_response_headers(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);

        // Check response headers
        assert_response_header_eq!(tx, "content-type", "text/html");
        assert_response_header_eq!(tx, "server", "Apache");

        // Response body data
        (*tx)
            .res_process_body_data(&mut *t.connp, Some(b"<h1>Hello"))
            .unwrap();
        (*tx)
            .res_process_body_data(&mut *t.connp, Some(b" "))
            .unwrap();
        (*tx)
            .res_process_body_data(&mut *t.connp, Some(b"World!</h1>"))
            .unwrap();
        assert_eq!(1, t.user_data.response_body_correctly_received);

        (*tx).res_set_header("Content-Type", "text/html");
        (*tx).res_set_header("Server", "Apache");

        // Check trailing response headers
        assert_response_header_eq!(tx, "content-type", "text/html");
        assert_response_header_eq!(tx, "server", "Apache");

        (*tx).state_response_complete(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_COMPLETE_invoked);
    }
}

/// Use a POST request in order to test request body processing and parameter parsing.
#[test]
fn PostUrlecodedTest() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start(&mut *t.connp).unwrap();

        // Request line data
        req_set_line(&mut *t.connp, "POST / HTTP/1.1").unwrap();

        // Request line complete
        (*tx).state_request_line(&mut *t.connp).unwrap();

        // Configure headers to trigger the URLENCODED parser
        req_set_header(
            &mut *tx,
            "Content-Type",
            "application/x-www-form-urlencoded",
        );
        req_set_header(&mut *tx, "Content-Length", "7");

        // Request headers complete
        (*tx).state_request_headers(&mut *t.connp).unwrap();

        // Send request body
        (*tx)
            .req_process_body_data(&mut *t.connp, Some(b"p=1"))
            .unwrap();
        (*tx)
            .req_process_body_data(&mut *t.connp, Some(b""))
            .unwrap();
        (*tx)
            .req_process_body_data(&mut *t.connp, Some(b"&"))
            .unwrap();
        (*tx)
            .req_process_body_data(&mut *t.connp, Some(b"q=2"))
            .unwrap();

        req_set_header(&mut *tx, "Host", "www.example.com");
        req_set_header(&mut *tx, "Connection", "keep-alive");
        req_set_header(&mut *tx, "User-Agent", "Mozilla/5.0");

        assert_request_header_eq!(tx, "host", "www.example.com");
        assert_request_header_eq!(tx, "connection", "keep-alive");
        assert_request_header_eq!(tx, "user-agent", "Mozilla/5.0");

        // Request complete
        (*tx).state_request_complete(&mut *t.connp).unwrap();

        // Check parameters
        assert_contains_param!(&(*tx).request_params, "p", "1");
        assert_contains_param!(&(*tx).request_params, "q", "2");
    }
}

const HYBRID_PARSING_COMPRESSED_RESPONSE: &[u8] =
    b"H4sIAAAAAAAAAG2PwQ6CMBBE73xFU++tXk2pASliAiEhPegRYUOJYEktEP5eqB6dy2ZnJ5O3LJFZ\
      yj2WiCBah7zKVPBMT1AjCf2gTWnabmH0e/AY/QXDPLqj8HLO07zw8S52wkiKm1zXvRPeeg//2lbX\
      kwpQrauxh5dFqnyj3uVYgJJCxD5W1g5HSud5Jo3WTQek0mR8UgNlDYZOLcz0ZMuH3y+YKzDAaMDJ\
      SrihOVL32QceVXUy4QAAAA==";

extern "C" fn HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(
    connp: *mut ConnectionParser,
    tx: *mut Transaction,
) {
    unsafe {
        (*tx).state_request_start(&mut *connp).unwrap();

        req_set_line(&mut *connp, "GET / HTTP/1.1").unwrap();

        (*tx).state_request_line(&mut *connp).unwrap();
        (*tx).state_request_headers(&mut *connp).unwrap();
        (*tx).state_request_complete(&mut *connp).unwrap();

        (*tx).state_response_start(&mut *connp).unwrap();

        res_set_status_line(&mut *connp, "HTTP/1.1 200 OK").unwrap();
        (*tx).res_set_header("Content-Encoding", "gzip");
        (*tx).res_set_header("Content-Length", "187");

        (*tx).state_response_headers(&mut *connp).unwrap();

        let body = Bstr::from(base64::decode(HYBRID_PARSING_COMPRESSED_RESPONSE).unwrap());

        (*tx)
            .res_process_body_data(&mut *connp, Some(body.as_slice()))
            .unwrap();

        (*tx).state_response_complete(&mut *connp).unwrap();
    }
}

/// Test with a compressed response body and decompression enabled.
#[test]
fn CompressedResponse() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(t.connp, tx);

        assert_eq!(187, (*tx).response_message_len);
        assert_eq!(225, (*tx).response_entity_len);
    }
}

#[test]
fn ParamCaseSensitivity() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start(&mut *t.connp).unwrap();

        // Request line data
        req_set_line(&mut *t.connp, "GET /?p=1&Q=2 HTTP/1.1").unwrap();

        // Request line complete
        (*tx).state_request_line(&mut *t.connp).unwrap();

        // Check the parameters.
        assert_contains_param!(&(*tx).request_params, "p", "1");
        assert_contains_param!(&(*tx).request_params, "p", "1");
        assert_contains_param!(&(*tx).request_params, "q", "2");
        assert_contains_param_source!(&(*tx).request_params, HtpDataSource::QUERY_STRING, "q", "2");
        assert_contains_param_source!(&(*tx).request_params, HtpDataSource::QUERY_STRING, "Q", "2");
    }
}

/// Use a POST request in order to test request body processing and parameter
/// parsing. In hybrid mode, we expect that the body arrives to us dechunked.
#[test]
fn PostUrlecodedChunked() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction.
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Request begins.
        (*tx).state_request_start(&mut *t.connp).unwrap();

        // Request line data.
        req_set_line(&mut *t.connp, "POST / HTTP/1.1").unwrap();
        (*tx).state_request_line(&mut *t.connp).unwrap();

        // Configure headers to trigger the URLENCODED parser.
        req_set_header(
            &mut *tx,
            "Content-Type",
            "application/x-www-form-urlencoded",
        );
        req_set_header(&mut *tx, "Transfer-Encoding", "chunked");

        // Request headers complete.
        (*tx).state_request_headers(&mut *t.connp).unwrap();

        // Send request body.
        (*tx)
            .req_process_body_data(&mut *t.connp, Some(b"p=1"))
            .unwrap();
        (*tx)
            .req_process_body_data(&mut *t.connp, Some(b"&"))
            .unwrap();
        (*tx)
            .req_process_body_data(&mut *t.connp, Some(b"q=2"))
            .unwrap();

        // Request complete.
        (*tx).state_request_complete(&mut *t.connp).unwrap();

        // Check the parameters.
        assert_contains_param!(&(*tx).request_params, "p", "1");
        assert_contains_param!(&(*tx).request_params, "q", "2");
    }
}

#[test]
fn RequestLineParsing1() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start(&mut *t.connp).unwrap();

        // Request line data
        req_set_line(&mut *t.connp, "GET /?p=1&q=2 HTTP/1.0").unwrap();

        // Request line complete
        (*tx).state_request_line(&mut *t.connp).unwrap();

        assert!((*tx).request_method.as_ref().unwrap().eq("GET"));
        assert!((*tx).request_uri.as_ref().unwrap().eq("/?p=1&q=2"));
        assert!((*tx).request_protocol.as_ref().unwrap().eq("HTTP/1.0"));
        assert!((*tx)
            .parsed_uri
            .as_ref()
            .unwrap()
            .query
            .as_ref()
            .unwrap()
            .eq("p=1&q=2"));

        // Check parameters
        assert_contains_param!(&(*tx).request_params, "p", "1");
        assert_contains_param!(&(*tx).request_params, "q", "2");
    }
}

#[test]
fn RequestLineParsing2() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start(&mut *t.connp).unwrap();
        req_set_line(&mut *t.connp, "GET /").unwrap();
        (*tx).state_request_line(&mut *t.connp).unwrap();

        // Check the results now.

        assert!((*tx).request_method.as_ref().unwrap().eq("GET"));
        assert!((*tx).is_protocol_0_9);
        assert_eq!(HtpProtocol::V0_9, (*tx).request_protocol_number);
        assert!((*tx).request_protocol.is_none());
        assert!((*tx).request_uri.as_ref().unwrap().eq("/"));
    }
}

#[test]
fn RequestLineParsing3() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start(&mut *t.connp).unwrap();
        req_set_line(&mut *t.connp, "GET / HTTP  / 01.1").unwrap();
        (*tx).state_request_line(&mut *t.connp).unwrap();

        // Check the results now.

        assert!((*tx).request_method.as_ref().unwrap().eq("GET"));
        assert_eq!(HtpProtocol::V1_1, (*tx).request_protocol_number);
        assert!((*tx).request_protocol.as_ref().unwrap().eq("HTTP  / 01.1"));
        assert!((*tx).request_uri.as_ref().unwrap().eq("/"));
    }
}

#[test]
fn RequestLineParsing4() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start(&mut *t.connp).unwrap();
        req_set_line(&mut *t.connp, "GET / HTTP  / 01.10").unwrap();
        (*tx).state_request_line(&mut *t.connp).unwrap();

        // Check the results now.

        assert!((*tx).request_method.as_ref().unwrap().eq("GET"));
        assert_eq!(HtpProtocol::INVALID, (*tx).request_protocol_number);
        assert!((*tx).request_protocol.as_ref().unwrap().eq("HTTP  / 01.10"));
        assert!((*tx).request_uri.as_ref().unwrap().eq("/"));
    }
}
#[test]
fn ParsedUriSupplied() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start(&mut *t.connp).unwrap();
        req_set_line(&mut *t.connp, "GET /?p=1&q=2 HTTP/1.0").unwrap();

        let mut u = Uri::default();
        u.path = Some(Bstr::from("/123"));
        (*tx).parsed_uri = Some(u);
        (*tx).state_request_line(&mut *t.connp).unwrap();

        // Check the results now.

        assert!((*tx).request_method.as_ref().unwrap().eq("GET"));
        assert_eq!(HtpProtocol::V1_0, (*tx).request_protocol_number);
        assert!((*tx).request_uri.as_ref().unwrap().eq("/?p=1&q=2"));
        assert!((*tx)
            .parsed_uri
            .as_ref()
            .unwrap()
            .path
            .as_ref()
            .unwrap()
            .eq("/123"));
    }
}

/// Test hybrid mode with one complete GET transaction; request then response
/// with no body. Used to crash in htp_connp_close().
#[test]
fn TestRepeatCallbacks() {
    unsafe {
        let mut cfg = TestConfig();
        // Request callbacks
        register_user_callbacks(&mut cfg);
        let mut t = HybridParsingTest::new(cfg);
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Configure user data and callbacks
        (*tx).set_user_data(&mut t.user_data as *mut _ as *mut core::ffi::c_void);

        // Request begins
        (*tx).state_request_start(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_START_invoked);

        // Request line data
        req_set_line(&mut *t.connp, "GET / HTTP/1.0").unwrap();

        // Request line complete
        (*tx).state_request_line(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_LINE_invoked);

        // Check request line data
        assert!((*tx).request_method.as_ref().unwrap().eq("GET"));
        assert!((*tx).request_uri.as_ref().unwrap().eq("/"));
        assert!((*tx).request_protocol.as_ref().unwrap().eq("HTTP/1.0"));
        assert!((*tx)
            .parsed_uri
            .as_ref()
            .unwrap()
            .path
            .as_ref()
            .unwrap()
            .eq("/"));

        // Request headers complete
        (*tx).state_request_headers(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);

        // Request complete
        (*tx).state_request_complete(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);

        // Response begins
        (*tx).state_response_start(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);

        // Response line data
        res_set_status_line(&mut *t.connp, "HTTP/1.1 200 OK\r\n").unwrap();

        // Response line complete
        (*tx).state_response_line(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);

        // Response headers complete
        (*tx).state_response_headers(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);

        // Response complete
        (*tx).state_response_complete(&mut *t.connp).unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_COMPLETE_invoked);

        (*tx).destroy(&mut *t.connp).unwrap();

        // Close connection
        t.close_conn_parser();

        assert_eq!(1, t.user_data.callback_REQUEST_START_invoked);
        assert_eq!(1, t.user_data.callback_REQUEST_LINE_invoked);
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);
        assert_eq!(1, t.user_data.callback_RESPONSE_COMPLETE_invoked);
        assert_eq!(1, t.user_data.callback_TRANSACTION_COMPLETE_invoked);
    }
}

/// Try to delete a transaction before it is complete.
#[test]
fn DeleteTransactionBeforeComplete() {
    unsafe {
        let mut t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start(&mut *t.connp).unwrap();

        // Request line data
        req_set_line(&mut *t.connp, "GET / HTTP/1.0").unwrap();

        assert_err!((*tx).destroy(&mut *t.connp), HtpStatus::ERROR);

        // Close connection
        t.close_conn_parser();
    }
}

/// Try response line with missing response code and message
#[test]
fn ResponseLineIncomplete() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;

        assert!(!tx.is_null());
        (*tx).state_response_start(&mut *t.connp).unwrap();
        res_set_status_line(&mut *t.connp, "HTTP/1.1").unwrap();
        assert!((*tx).response_protocol.as_ref().unwrap().eq("HTTP/1.1"));
        assert_eq!(HtpProtocol::V1_1, (*tx).response_protocol_number);
        assert!((*tx).response_status.is_none());
        assert_eq!(HtpResponseNumber::INVALID, (*tx).response_status_number);
        assert!((*tx).response_message.is_none());
        (*tx).state_response_complete(&mut *t.connp).unwrap();
    }
}

/// Try response line with missing response message
#[test]
fn ResponseLineIncomplete1() {
    unsafe {
        let t = HybridParsingTest::new(TestConfig());
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut Transaction;

        assert!(!tx.is_null());
        (*tx).state_response_start(&mut *t.connp).unwrap();
        res_set_status_line(&mut *t.connp, "HTTP/1.1 200").unwrap();
        assert!((*tx).response_protocol.as_ref().unwrap().eq("HTTP/1.1"));
        assert_eq!(HtpProtocol::V1_1, (*tx).response_protocol_number);
        assert!((*tx).response_status.as_ref().unwrap().eq("200"));
        assert!((*tx).response_status_number.eq(200));
        assert!((*tx).response_message.is_none());
        (*tx).state_response_complete(&mut *t.connp).unwrap();
    }
}
