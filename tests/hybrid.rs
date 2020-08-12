#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use htp::bstr::*;
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

unsafe extern "C" fn HybridParsing_Get_Callback_REQUEST_START(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_START_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_REQUEST_LINE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_LINE_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_REQUEST_HEADERS(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_HEADERS_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_REQUEST_COMPLETE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_COMPLETE_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_START(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_START_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_LINE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_LINE_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_HEADERS(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_HEADERS_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_BODY_DATA(
    d: *mut htp_tx_data_t,
) -> Status {
    let user_data = htp_tx_user_data((*d).tx()) as *mut HybridParsing_Get_User_Data;

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
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_COMPLETE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_COMPLETE_invoked += 1;
    Status::OK
}

unsafe extern "C" fn HybridParsing_Get_Callback_TRANSACTION_COMPLETE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_TRANSACTION_COMPLETE_invoked += 1;
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
            htp_connp_open(
                connp,
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
                htp_connp_close(self.connp, None);
                self.connp_open = false;
            }
        }
    }

    fn register_user_callbacks(&mut self) {
        unsafe {
            // Request callbacks
            (*self.cfg).register_request_start(Some(HybridParsing_Get_Callback_REQUEST_START));
            (*self.cfg).register_request_line(Some(HybridParsing_Get_Callback_REQUEST_LINE));
            (*self.cfg).register_request_headers(Some(HybridParsing_Get_Callback_REQUEST_HEADERS));
            (*self.cfg)
                .register_request_complete(Some(HybridParsing_Get_Callback_REQUEST_COMPLETE));

            // Response callbacks
            (*self.cfg).register_response_start(Some(HybridParsing_Get_Callback_RESPONSE_START));
            (*self.cfg).register_response_line(Some(HybridParsing_Get_Callback_RESPONSE_LINE));
            (*self.cfg)
                .register_response_headers(Some(HybridParsing_Get_Callback_RESPONSE_HEADERS));
            (*self.cfg)
                .register_response_body_data(Some(HybridParsing_Get_Callback_RESPONSE_BODY_DATA));
            (*self.cfg)
                .register_response_complete(Some(HybridParsing_Get_Callback_RESPONSE_COMPLETE));

            // Transaction calllbacks
            (*self.cfg).register_transaction_complete(Some(
                HybridParsing_Get_Callback_TRANSACTION_COMPLETE,
            ));
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
        htp_tx_set_user_data(tx, &mut t.user_data as *mut _ as *mut core::ffi::c_void);

        // Register callbacks
        t.register_user_callbacks();

        // Request begins
        htp_tx_state_request_start(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_START_invoked);

        // Request line data
        htp_tx_req_set_line(tx, "GET /?p=1&q=2 HTTP/1.1");

        // Request line complete
        htp_tx_state_request_line(tx);
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
        htp_tx_req_set_header(tx, "Host", "www.example.com");
        htp_tx_req_set_header(tx, "Connection", "keep-alive");
        htp_tx_req_set_header(tx, "User-Agent", "Mozilla/5.0");

        // Request headers complete
        htp_tx_state_request_headers(tx);

        // Check headers
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);

        assert_request_header_eq!(tx, "host", "www.example.com");
        assert_request_header_eq!(tx, "connection", "keep-alive");
        assert_request_header_eq!(tx, "user-agent", "Mozilla/5.0");

        // Request complete
        htp_tx_state_request_complete(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);

        // Response begins
        htp_tx_state_response_start(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);

        // Response line data
        htp_tx_res_set_status_line(tx, "HTTP/1.1 200 OK");
        assert!((*(*tx).response_protocol).eq("HTTP/1.1"));
        assert_eq!(Protocol::V1_1, (*tx).response_protocol_number);
        assert_eq!(200, (*tx).response_status_number);
        assert!((*(*tx).response_message).eq("OK"));

        // Response line complete
        htp_tx_state_response_line(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);

        // Response header data
        htp_tx_res_set_header(tx, "Content-Type", "text/html");
        htp_tx_res_set_header(tx, "Server", "Apache");

        // Response headers complete
        htp_tx_state_response_headers(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);

        // Check response headers
        assert_response_header_eq!(tx, "content-type", "text/html");
        assert_response_header_eq!(tx, "server", "Apache");

        // Response body data
        htp_tx_res_process_body_data(tx, "<h1>Hello");
        htp_tx_res_process_body_data(tx, " ");
        htp_tx_res_process_body_data(tx, "World!</h1>");
        assert_eq!(1, t.user_data.response_body_correctly_received);

        htp_tx_res_set_header(tx, "Content-Type", "text/html");
        htp_tx_res_set_header(tx, "Server", "Apache");

        // Check trailing response headers
        assert_response_header_eq!(tx, "content-type", "text/html");
        assert_response_header_eq!(tx, "server", "Apache");

        htp_tx_state_response_complete(tx);
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
        htp_tx_state_request_start(tx);

        // Request line data
        htp_tx_req_set_line(tx, "POST / HTTP/1.1");

        // Request line complete
        htp_tx_state_request_line(tx);

        // Configure headers to trigger the URLENCODED parser
        htp_tx_req_set_header(tx, "Content-Type", "application/x-www-form-urlencoded");
        htp_tx_req_set_header(tx, "Content-Length", "7");

        // Request headers complete
        htp_tx_state_request_headers(tx);

        // Send request body
        htp_tx_req_process_body_data(tx, "p=1");
        htp_tx_req_process_body_data(tx, "");
        htp_tx_req_process_body_data(tx, "&");
        htp_tx_req_process_body_data(tx, "q=2");

        htp_tx_req_set_header(tx, "Host", "www.example.com");
        htp_tx_req_set_header(tx, "Connection", "keep-alive");
        htp_tx_req_set_header(tx, "User-Agent", "Mozilla/5.0");

        assert_request_header_eq!(tx, "host", "www.example.com");
        assert_request_header_eq!(tx, "connection", "keep-alive");
        assert_request_header_eq!(tx, "user-agent", "Mozilla/5.0");

        // Request complete
        htp_tx_state_request_complete(tx);

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
        htp_tx_state_request_start(tx);

        htp_tx_req_set_line(tx, "GET / HTTP/1.1");

        htp_tx_state_request_line(tx);
        htp_tx_state_request_headers(tx);
        htp_tx_state_request_complete(tx);

        htp_tx_state_response_start(tx);

        htp_tx_res_set_status_line(tx, "HTTP/1.1 200 OK");
        htp_tx_res_set_header(tx, "Content-Encoding", "gzip");
        htp_tx_res_set_header(tx, "Content-Length", "187");

        htp_tx_state_response_headers(tx);

        let body = bstr_t::from(base64::decode(HYBRID_PARSING_COMPRESSED_RESPONSE).unwrap());

        htp_tx_res_process_body_data(tx, body.as_slice());

        htp_tx_state_response_complete(tx);
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
        htp_tx_state_request_start(tx);

        // Request line data
        htp_tx_req_set_line(tx, "GET /?p=1&Q=2 HTTP/1.1");

        // Request line complete
        htp_tx_state_request_line(tx);

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
        htp_tx_state_request_start(tx);

        // Request line data.
        htp_tx_req_set_line(tx, "POST / HTTP/1.1");
        htp_tx_state_request_line(tx);

        // Configure headers to trigger the URLENCODED parser.
        htp_tx_req_set_header(tx, "Content-Type", "application/x-www-form-urlencoded");
        htp_tx_req_set_header(tx, "Transfer-Encoding", "chunked");

        // Request headers complete.
        htp_tx_state_request_headers(tx);

        // Send request body.
        htp_tx_req_process_body_data(tx, "p=1");
        htp_tx_req_process_body_data(tx, "&");
        htp_tx_req_process_body_data(tx, "q=2");

        // Request complete.
        htp_tx_state_request_complete(tx);

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
        htp_tx_state_request_start(tx);

        // Request line data
        htp_tx_req_set_line(tx, "GET /?p=1&q=2 HTTP/1.0");

        // Request line complete
        htp_tx_state_request_line(tx);

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

        htp_tx_state_request_start(tx);
        htp_tx_req_set_line(tx, "GET /");
        htp_tx_state_request_line(tx);

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

        htp_tx_state_request_start(tx);
        htp_tx_req_set_line(tx, "GET / HTTP  / 01.1");
        htp_tx_state_request_line(tx);

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

        htp_tx_state_request_start(tx);
        htp_tx_req_set_line(tx, "GET / HTTP  / 01.10");
        htp_tx_state_request_line(tx);

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

        htp_tx_state_request_start(tx);
        htp_tx_req_set_line(tx, "GET /?p=1&q=2 HTTP/1.0");

        //htp_uri_t *u = htp_uri_alloc();
        let u = htp_uri_alloc();
        (*u).path = bstr_dup_str("/123");
        htp_tx_req_set_parsed_uri(tx, u);

        htp_tx_state_request_line(tx);

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
        htp_tx_set_user_data(tx, &mut t.user_data as *mut _ as *mut core::ffi::c_void);

        // Request callbacks
        t.register_user_callbacks();

        // Request begins
        htp_tx_state_request_start(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_START_invoked);

        // Request line data
        htp_tx_req_set_line(tx, "GET / HTTP/1.0");

        // Request line complete
        htp_tx_state_request_line(tx);
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
        htp_tx_state_request_headers(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);

        // Request complete
        htp_tx_state_request_complete(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);

        // Response begins
        htp_tx_state_response_start(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);

        // Response line data
        htp_tx_res_set_status_line(tx, "HTTP/1.1 200 OK\r\n");

        // Response line complete
        htp_tx_state_response_line(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);

        // Response headers complete
        htp_tx_state_response_headers(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);

        // Response complete
        htp_tx_state_response_complete(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_COMPLETE_invoked);

        assert_eq!(htp_tx_destroy(tx), Status::OK);

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
        htp_tx_state_request_start(tx);

        // Request line data
        htp_tx_req_set_line(tx, "GET / HTTP/1.0");

        assert_eq!(htp_tx_destroy(tx), Status::ERROR);

        // Close connection
        t.close_conn_parser();
    }
}
