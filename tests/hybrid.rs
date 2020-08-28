#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use htp::bstr::*;
use htp::c_api::{htp_connp_create, htp_connp_destroy_all};
use htp::htp_config;
use htp::htp_config::htp_server_personality_t::*;
use htp::htp_connection_parser::*;
use htp::htp_transaction::htp_data_source_t::*;
use htp::htp_transaction::*;
use htp::htp_util::*;
use htp::Status;
use std::ffi::CString;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::Drop;

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

fn HybridParsing_Get_Callback_REQUEST_START(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_START_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_REQUEST_LINE(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_LINE_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_REQUEST_HEADERS(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_HEADERS_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_REQUEST_COMPLETE(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_REQUEST_COMPLETE_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_RESPONSE_START(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_START_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_RESPONSE_LINE(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_LINE_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_RESPONSE_HEADERS(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_HEADERS_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_RESPONSE_BODY_DATA(d: *mut htp_tx_data_t) -> Status {
    unsafe {
        let user_data = (*(*d).tx()).user_data() as *mut HybridParsing_Get_User_Data;

        // Don't do anything if in errored state.
        if (*user_data).response_body_correctly_received == -1 {
            return Status::ERROR;
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
    Status::OK
}

fn HybridParsing_Get_Callback_RESPONSE_COMPLETE(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_RESPONSE_COMPLETE_invoked += 1;
    }
    Status::OK
}

fn HybridParsing_Get_Callback_TRANSACTION_COMPLETE(tx: *mut htp_tx_t) -> Status {
    unsafe {
        let user_data = (*tx).user_data() as *mut HybridParsing_Get_User_Data;
        (*user_data).callback_TRANSACTION_COMPLETE_invoked += 1;
    }
    Status::OK
}

struct HybridParsingTest {
    connp: *mut htp_connp_t,
    cfg: *mut htp_config::htp_cfg_t,
    connp_open: bool,
    user_data: HybridParsing_Get_User_Data,
}

impl HybridParsingTest {
    fn new() -> Self {
        unsafe {
            let cfg: *mut htp_config::htp_cfg_t = htp_config::create();
            assert!(!cfg.is_null());
            (*cfg).set_server_personality(HTP_SERVER_APACHE_2);
            (*cfg).register_urlencoded_parser();
            (*cfg).register_multipart_parser();
            let connp = htp_connp_create(cfg);
            assert!(!connp.is_null());
            (*connp).open(
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                32768,
                Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                80,
                None,
            );

            let user_data = HybridParsing_Get_User_Data::new();
            HybridParsingTest {
                connp,
                cfg,
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

    fn register_user_callbacks(&mut self) {
        unsafe {
            // Request callbacks
            (*self.cfg).register_request_start(HybridParsing_Get_Callback_REQUEST_START);
            (*self.cfg).register_request_line(HybridParsing_Get_Callback_REQUEST_LINE);
            (*self.cfg).register_request_headers(HybridParsing_Get_Callback_REQUEST_HEADERS);
            (*self.cfg).register_request_complete(HybridParsing_Get_Callback_REQUEST_COMPLETE);

            // Response callbacks
            (*self.cfg).register_response_start(HybridParsing_Get_Callback_RESPONSE_START);
            (*self.cfg).register_response_line(HybridParsing_Get_Callback_RESPONSE_LINE);
            (*self.cfg).register_response_headers(HybridParsing_Get_Callback_RESPONSE_HEADERS);
            (*self.cfg).register_response_body_data(HybridParsing_Get_Callback_RESPONSE_BODY_DATA);
            (*self.cfg).register_response_complete(HybridParsing_Get_Callback_RESPONSE_COMPLETE);

            // Transaction calllbacks
            (*self.cfg)
                .register_transaction_complete(HybridParsing_Get_Callback_TRANSACTION_COMPLETE);
        }
    }
}

impl Drop for HybridParsingTest {
    fn drop(&mut self) {
        unsafe {
            self.close_conn_parser();
            htp_connp_destroy_all(self.connp);
            (*self.cfg).destroy();
        }
    }
}

/// Test hybrid mode with one complete GET transaction; request then response
/// with a body. Most features are tested, including query string parameters and callbacks.
#[test]
fn GetTest() {
    unsafe {
        let mut t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Configure user data and callbacks
        (*tx).set_user_data(&mut t.user_data as *mut _ as *mut core::ffi::c_void);

        // Register callbacks
        t.register_user_callbacks();

        // Request begins
        (*tx).state_request_start().unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_START_invoked);

        // Request line data
        (*tx).req_set_line("GET /?p=1&q=2 HTTP/1.1").unwrap();

        // Request line complete
        (*tx).state_request_line().unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_LINE_invoked);

        // Check request line data
        assert!(!(*tx).request_method.is_null());
        assert!((*(*tx).request_method).eq("GET"));
        assert!(!(*tx).request_uri.is_null());
        assert!((*(*tx).request_uri).eq("/?p=1&q=2"));
        assert!(!(*tx).request_protocol.is_null());
        assert!((*(*tx).request_protocol).eq("HTTP/1.1"));

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert!((*(*(*tx).parsed_uri).path).eq("/"));

        assert!(!(*(*tx).parsed_uri).query.is_null());
        assert!((*(*(*tx).parsed_uri).query).eq("p=1&q=2"));

        // Check parameters
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "p")
            .unwrap()
            .value
            .eq("1"));
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "q")
            .unwrap()
            .value
            .eq("2"));

        // Request headers
        (*tx).req_set_header("Host", "www.example.com");
        (*tx).req_set_header("Connection", "keep-alive");
        (*tx).req_set_header("User-Agent", "Mozilla/5.0");

        // Request headers complete
        (*tx).state_request_headers().unwrap();

        // Check headers
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);

        assert_request_header_eq!(tx, "host", "www.example.com");
        assert_request_header_eq!(tx, "connection", "keep-alive");
        assert_request_header_eq!(tx, "user-agent", "Mozilla/5.0");

        // Request complete
        (*tx).state_request_complete().unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);

        // Response begins
        (*tx).state_response_start().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);

        // Response line data
        (*tx).res_set_status_line("HTTP/1.1 200 OK").unwrap();
        assert!((*(*tx).response_protocol).eq("HTTP/1.1"));
        assert_eq!(Protocol::V1_1, (*tx).response_protocol_number);
        assert_eq!(0, bstr_cmp_str((*tx).response_status, "200"));
        assert_eq!(200, (*tx).response_status_number);
        assert!((*(*tx).response_message).eq("OK"));

        // Response line complete
        (*tx).state_response_line().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);

        // Response header data
        (*tx).res_set_header("Content-Type", "text/html");
        (*tx).res_set_header("Server", "Apache");

        // Response headers complete
        (*tx).state_response_headers().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);

        // Check response headers
        assert_response_header_eq!(tx, "content-type", "text/html");
        assert_response_header_eq!(tx, "server", "Apache");

        // Response body data
        (*tx).res_process_body_data("<h1>Hello").unwrap();
        (*tx).res_process_body_data(" ").unwrap();
        (*tx).res_process_body_data("World!</h1>").unwrap();
        assert_eq!(1, t.user_data.response_body_correctly_received);

        (*tx).res_set_header("Content-Type", "text/html");
        (*tx).res_set_header("Server", "Apache");

        // Check trailing response headers
        assert_response_header_eq!(tx, "content-type", "text/html");
        assert_response_header_eq!(tx, "server", "Apache");

        (*tx).state_response_complete().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_COMPLETE_invoked);
    }
}

/// Use a POST request in order to test request body processing and parameter parsing.
#[test]
fn PostUrlecodedTest() {
    unsafe {
        let t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start().unwrap();

        // Request line data
        (*tx).req_set_line("POST / HTTP/1.1").unwrap();

        // Request line complete
        (*tx).state_request_line().unwrap();

        // Configure headers to trigger the URLENCODED parser
        (*tx).req_set_header("Content-Type", "application/x-www-form-urlencoded");
        (*tx).req_set_header("Content-Length", "7");

        // Request headers complete
        (*tx).state_request_headers().unwrap();

        // Send request body
        (*tx).req_process_body_data("p=1").unwrap();
        (*tx).req_process_body_data("").unwrap();
        (*tx).req_process_body_data("&").unwrap();
        (*tx).req_process_body_data("q=2").unwrap();

        (*tx).req_set_header("Host", "www.example.com");
        (*tx).req_set_header("Connection", "keep-alive");
        (*tx).req_set_header("User-Agent", "Mozilla/5.0");

        assert_request_header_eq!(tx, "host", "www.example.com");
        assert_request_header_eq!(tx, "connection", "keep-alive");
        assert_request_header_eq!(tx, "user-agent", "Mozilla/5.0");

        // Request complete
        (*tx).state_request_complete().unwrap();

        // Check parameters
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "p")
            .unwrap()
            .value
            .eq("1"));
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "q")
            .unwrap()
            .value
            .eq("2"));
    }
}

const HYBRID_PARSING_COMPRESSED_RESPONSE: &[u8] =
    b"H4sIAAAAAAAAAG2PwQ6CMBBE73xFU++tXk2pASliAiEhPegRYUOJYEktEP5eqB6dy2ZnJ5O3LJFZ\
      yj2WiCBah7zKVPBMT1AjCf2gTWnabmH0e/AY/QXDPLqj8HLO07zw8S52wkiKm1zXvRPeeg//2lbX\
      kwpQrauxh5dFqnyj3uVYgJJCxD5W1g5HSud5Jo3WTQek0mR8UgNlDYZOLcz0ZMuH3y+YKzDAaMDJ\
      SrihOVL32QceVXUy4QAAAA==";

extern "C" fn HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(tx: *mut htp_tx_t) {
    unsafe {
        (*tx).state_request_start().unwrap();

        (*tx).req_set_line("GET / HTTP/1.1").unwrap();

        (*tx).state_request_line().unwrap();
        (*tx).state_request_headers().unwrap();
        (*tx).state_request_complete().unwrap();

        (*tx).state_response_start().unwrap();

        (*tx).res_set_status_line("HTTP/1.1 200 OK").unwrap();
        (*tx).res_set_header("Content-Encoding", "gzip");
        (*tx).res_set_header("Content-Length", "187");

        (*tx).state_response_headers().unwrap();

        let body = bstr_t::from(base64::decode(HYBRID_PARSING_COMPRESSED_RESPONSE).unwrap());

        (*tx).res_process_body_data(body.as_slice()).unwrap();

        (*tx).state_response_complete().unwrap();
    }
}

/// Test with a compressed response body and decompression enabled.
#[test]
fn CompressedResponse() {
    unsafe {
        let t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(tx);

        assert_eq!(187, (*tx).response_message_len);
        assert_eq!(225, (*tx).response_entity_len);
    }
}

#[test]
fn ParamCaseSensitivity() {
    unsafe {
        let t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start().unwrap();

        // Request line data
        (*tx).req_set_line("GET /?p=1&Q=2 HTTP/1.1").unwrap();

        // Request line complete
        (*tx).state_request_line().unwrap();

        // Check the parameters.
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "p")
            .unwrap()
            .value
            .eq("1"));
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "p")
            .unwrap()
            .value
            .eq("1"));
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "q")
            .unwrap()
            .value
            .eq("2"));
        assert!(
            htp_tx_req_get_param_ex(&*(*tx).request_params, HTP_SOURCE_QUERY_STRING, "q")
                .unwrap()
                .value
                .eq("2")
        );
        assert!(
            htp_tx_req_get_param_ex(&*(*tx).request_params, HTP_SOURCE_QUERY_STRING, "Q")
                .unwrap()
                .value
                .eq("2")
        );
    }
}

/// Use a POST request in order to test request body processing and parameter
/// parsing. In hybrid mode, we expect that the body arrives to us dechunked.
#[test]
fn PostUrlecodedChunked() {
    unsafe {
        let t = HybridParsingTest::new();
        // Create a new LibHTP transaction.
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Request begins.
        (*tx).state_request_start().unwrap();

        // Request line data.
        (*tx).req_set_line("POST / HTTP/1.1").unwrap();
        (*tx).state_request_line().unwrap();

        // Configure headers to trigger the URLENCODED parser.
        (*tx).req_set_header("Content-Type", "application/x-www-form-urlencoded");
        (*tx).req_set_header("Transfer-Encoding", "chunked");

        // Request headers complete.
        (*tx).state_request_headers().unwrap();

        // Send request body.
        (*tx).req_process_body_data("p=1").unwrap();
        (*tx).req_process_body_data("&").unwrap();
        (*tx).req_process_body_data("q=2").unwrap();

        // Request complete.
        (*tx).state_request_complete().unwrap();

        // Check the parameters.
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "p")
            .unwrap()
            .value
            .eq("1"));
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "q")
            .unwrap()
            .value
            .eq("2"));
    }
}

#[test]
fn RequestLineParsing1() {
    unsafe {
        let t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start().unwrap();

        // Request line data
        (*tx).req_set_line("GET /?p=1&q=2 HTTP/1.0").unwrap();

        // Request line complete
        (*tx).state_request_line().unwrap();

        assert!((*(*tx).request_method).eq("GET"));
        assert!((*(*tx).request_uri).eq("/?p=1&q=2"));
        assert!((*(*tx).request_protocol).eq("HTTP/1.0"));

        assert!(!(*tx).parsed_uri.is_null());
        assert!((*(*(*tx).parsed_uri).query).eq("p=1&q=2"));

        // Check parameters
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "p")
            .unwrap()
            .value
            .eq("1"));
        assert!(htp_tx_req_get_param(&*(*tx).request_params, "q")
            .unwrap()
            .value
            .eq("2"));
    }
}

#[test]
fn RequestLineParsing2() {
    unsafe {
        let t = HybridParsingTest::new();
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start().unwrap();
        (*tx).req_set_line("GET /").unwrap();
        (*tx).state_request_line().unwrap();

        // Check the results now.

        assert!((*(*tx).request_method).eq("GET"));
        assert_eq!(1, (*tx).is_protocol_0_9);
        assert_eq!(Protocol::V0_9, (*tx).request_protocol_number);
        assert!((*tx).request_protocol.is_null());
        assert!((*(*tx).request_uri).eq("/"));
    }
}

#[test]
fn RequestLineParsing3() {
    unsafe {
        let t = HybridParsingTest::new();
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start().unwrap();
        (*tx).req_set_line("GET / HTTP  / 01.1").unwrap();
        (*tx).state_request_line().unwrap();

        // Check the results now.

        assert!((*(*tx).request_method).eq("GET"));
        assert_eq!(Protocol::V1_1, (*tx).request_protocol_number);
        assert!(!(*tx).request_protocol.is_null());
        assert!((*(*tx).request_protocol).eq("HTTP  / 01.1"));
        assert!((*(*tx).request_uri).eq("/"));
    }
}

#[test]
fn RequestLineParsing4() {
    unsafe {
        let t = HybridParsingTest::new();
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start().unwrap();
        (*tx).req_set_line("GET / HTTP  / 01.10").unwrap();
        (*tx).state_request_line().unwrap();

        // Check the results now.

        assert!((*(*tx).request_method).eq("GET"));
        assert_eq!(Protocol::INVALID, (*tx).request_protocol_number);
        assert!(!(*tx).request_protocol.is_null());
        assert!((*(*tx).request_protocol).eq("HTTP  / 01.10"));
        assert!((*(*tx).request_uri).eq("/"));
    }
}
#[test]
fn ParsedUriSupplied() {
    unsafe {
        let t = HybridParsingTest::new();
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Feed data to the parser.

        (*tx).state_request_start().unwrap();
        (*tx).req_set_line("GET /?p=1&q=2 HTTP/1.0").unwrap();

        //htp_uri_t *u = htp_uri_alloc();
        let u = htp_uri_alloc();
        (*u).path = bstr_dup_str("/123");
        (*tx).req_set_parsed_uri(u);

        (*tx).state_request_line().unwrap();

        // Check the results now.

        assert!((*(*tx).request_method).eq("GET"));
        assert!(!(*tx).request_protocol.is_null());
        assert_eq!(Protocol::V1_0, (*tx).request_protocol_number);
        assert!(!(*tx).request_uri.is_null());
        assert!((*(*tx).request_uri).eq("/?p=1&q=2"));

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert!((*(*(*tx).parsed_uri).path).eq("/123"));
    }
}

/// Test hybrid mode with one complete GET transaction; request then response
/// with no body. Used to crash in htp_connp_close().
#[test]
fn TestRepeatCallbacks() {
    unsafe {
        let mut t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Configure user data and callbacks
        (*tx).set_user_data(&mut t.user_data as *mut _ as *mut core::ffi::c_void);

        // Request callbacks
        t.register_user_callbacks();

        // Request begins
        (*tx).state_request_start().unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_START_invoked);

        // Request line data
        (*tx).req_set_line("GET / HTTP/1.0").unwrap();

        // Request line complete
        (*tx).state_request_line().unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_LINE_invoked);

        // Check request line data
        assert!(!(*tx).request_method.is_null());
        assert!((*(*tx).request_method).eq("GET"));
        assert!(!(*tx).request_uri.is_null());
        assert!((*(*tx).request_uri).eq("/"));
        assert!(!(*tx).request_protocol.is_null());
        assert!((*(*tx).request_protocol).eq("HTTP/1.0"));

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert!((*(*(*tx).parsed_uri).path).eq("/"));

        // Request headers complete
        (*tx).state_request_headers().unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);

        // Request complete
        (*tx).state_request_complete().unwrap();
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);

        // Response begins
        (*tx).state_response_start().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);

        // Response line data
        (*tx).res_set_status_line("HTTP/1.1 200 OK\r\n").unwrap();

        // Response line complete
        (*tx).state_response_line().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);

        // Response headers complete
        (*tx).state_response_headers().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);

        // Response complete
        (*tx).state_response_complete().unwrap();
        assert_eq!(1, t.user_data.callback_RESPONSE_COMPLETE_invoked);

        (*tx).destroy().unwrap();

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
        let mut t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        // Request begins
        (*tx).state_request_start().unwrap();

        // Request line data
        (*tx).req_set_line("GET / HTTP/1.0").unwrap();

        assert_err!((*tx).destroy(), Status::ERROR);

        // Close connection
        t.close_conn_parser();
    }
}

/// Try response line with missing response code and message
#[test]
fn ResponseLineIncomplete() {
    unsafe {
        let t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;

        assert!(!tx.is_null());
        (*tx).state_response_start().unwrap();
        (*tx).res_set_status_line("HTTP/1.1").unwrap();
        assert!((*(*tx).response_protocol).eq("HTTP/1.1"));
        assert_eq!(Protocol::V1_1, (*tx).response_protocol_number);
        assert!((*tx).response_status.is_null());
        assert_eq!(-1, (*tx).response_status_number);
        assert!((*tx).response_message.is_null());
        (*tx).state_response_complete().unwrap();
    }
}

/// Try response line with missing response message
#[test]
fn ResponseLineIncomplete1() {
    unsafe {
        let t = HybridParsingTest::new();
        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;

        assert!(!tx.is_null());
        (*tx).state_response_start().unwrap();
        (*tx).res_set_status_line("HTTP/1.1 200").unwrap();
        assert!((*(*tx).response_protocol).eq("HTTP/1.1"));
        assert_eq!(Protocol::V1_1, (*tx).response_protocol_number);
        assert_eq!(0, bstr_cmp_str((*tx).response_status, "200"));
        assert_eq!(200, (*tx).response_status_number);
        assert!((*tx).response_message.is_null());
        (*tx).state_response_complete().unwrap();
    }
}
