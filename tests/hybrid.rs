#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use htp::bstr::*;
use htp::htp_base64::*;
use htp::htp_config::htp_server_personality_t::*;
use htp::htp_config::*;
use htp::htp_connection_parser::*;
use htp::htp_decompressors::htp_content_encoding_t::*;
use htp::htp_request::htp_method_t::*;
use htp::htp_table::*;
use htp::htp_transaction::htp_data_source_t::*;
use htp::htp_transaction::*;
use htp::htp_util::*;
use htp::Status;
use std::ffi::CString;
use std::ops::Drop;

macro_rules! cstr {
    ( $x:expr ) => {{
        CString::new($x).unwrap().as_ptr()
    }};
}

const HTP_URLENCODED_MIME_TYPE: &'static [u8; 34] = b"application/x-www-form-urlencoded\x00";

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
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_START_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_REQUEST_LINE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_LINE_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_REQUEST_HEADERS(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_HEADERS_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_REQUEST_COMPLETE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_REQUEST_COMPLETE_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_START(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_START_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_LINE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_LINE_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_HEADERS(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_HEADERS_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_BODY_DATA(
    d: *mut htp_tx_data_t,
) -> Status {
    let user_data = htp_tx_get_user_data((*d).tx) as *mut HybridParsing_Get_User_Data;

    // Don't do anything if in errored state.
    if (*user_data).response_body_correctly_received == -1 {
        return Status::ERROR;
    }

    match (*user_data).response_body_chunks_seen {
        0 => {
            if (*d).len == 9
                && (libc::memcmp(
                    (*d).data as *const core::ffi::c_void,
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
            if (*d).len == 1
                && (libc::memcmp(
                    (*d).data as *const core::ffi::c_void,
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
            if (*d).len == 11
                && (libc::memcmp(
                    (*d).data as *const core::ffi::c_void,
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
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_RESPONSE_COMPLETE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_RESPONSE_COMPLETE_invoked += 1;
    return Status::OK;
}

unsafe extern "C" fn HybridParsing_Get_Callback_TRANSACTION_COMPLETE(tx: *mut htp_tx_t) -> Status {
    let user_data = htp_tx_get_user_data(tx) as *mut HybridParsing_Get_User_Data;
    (*user_data).callback_TRANSACTION_COMPLETE_invoked += 1;
    return Status::OK;
}

struct HybridParsingTest {
    connp: *mut htp_connp_t,
    cfg: *mut htp_cfg_t,
    connp_open: bool,
    user_data: HybridParsing_Get_User_Data,
}

impl HybridParsingTest {
    fn new() -> Self {
        unsafe {
            let cfg: *mut htp_cfg_t = htp_config_create();
            assert!(!cfg.is_null());
            htp_config_set_server_personality(cfg, HTP_SERVER_APACHE_2);
            htp_config_register_urlencoded_parser(cfg);
            htp_config_register_multipart_parser(cfg);
            let connp = htp_connp_create(cfg);
            assert!(!connp.is_null());
            htp_connp_open(
                connp,
                cstr!("127.0.0.1"),
                32768,
                cstr!("127.0.0.1"),
                80,
                std::ptr::null_mut(),
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
                htp_connp_close(self.connp, std::ptr::null_mut());
                self.connp_open = false;
            }
        }
    }

    fn register_user_callbacks(&mut self) {
        unsafe {
            // Request callbacks
            htp_config_register_request_start(
                self.cfg,
                Some(HybridParsing_Get_Callback_REQUEST_START),
            );
            htp_config_register_request_line(
                self.cfg,
                Some(HybridParsing_Get_Callback_REQUEST_LINE),
            );
            htp_config_register_request_headers(
                self.cfg,
                Some(HybridParsing_Get_Callback_REQUEST_HEADERS),
            );
            htp_config_register_request_complete(
                self.cfg,
                Some(HybridParsing_Get_Callback_REQUEST_COMPLETE),
            );

            // Response callbacks
            htp_config_register_response_start(
                self.cfg,
                Some(HybridParsing_Get_Callback_RESPONSE_START),
            );
            htp_config_register_response_line(
                self.cfg,
                Some(HybridParsing_Get_Callback_RESPONSE_LINE),
            );
            htp_config_register_response_headers(
                self.cfg,
                Some(HybridParsing_Get_Callback_RESPONSE_HEADERS),
            );
            htp_config_register_response_body_data(
                self.cfg,
                Some(HybridParsing_Get_Callback_RESPONSE_BODY_DATA),
            );
            htp_config_register_response_complete(
                self.cfg,
                Some(HybridParsing_Get_Callback_RESPONSE_COMPLETE),
            );

            // Transaction calllbacks
            htp_config_register_transaction_complete(
                self.cfg,
                Some(HybridParsing_Get_Callback_TRANSACTION_COMPLETE),
            );
        }
    }
}

impl Drop for HybridParsingTest {
    fn drop(&mut self) {
        unsafe {
            self.close_conn_parser();
            htp_connp_destroy_all(self.connp);
            htp_config_destroy(self.cfg);
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
        htp_tx_req_set_method(tx, cstr!("GET"), 3, HTP_ALLOC_COPY);
        htp_tx_req_set_method_number(tx, HTP_M_GET as u32);
        htp_tx_req_set_uri(tx, cstr!("/?p=1&q=2"), 9, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol(tx, cstr!("HTTP/1.1"), 8, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol_number(tx, Protocol::V1_1 as libc::c_int);
        htp_tx_req_set_protocol_0_9(tx, 0);

        // Request line complete
        htp_tx_state_request_line(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_LINE_invoked);

        // Check request line data
        assert!(!(*tx).request_method.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_method, cstr!("GET")));
        assert!(!(*tx).request_uri.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_uri, cstr!("/?p=1&q=2")));
        assert!(!(*tx).request_protocol.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_protocol, cstr!("HTTP/1.1")));

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_c((*(*tx).parsed_uri).path, cstr!("/")));

        assert!(!(*(*tx).parsed_uri).query.is_null());
        assert_eq!(0, bstr_cmp_c((*(*tx).parsed_uri).query, cstr!("p=1&q=2")));

        // Check parameters
        let param_p = htp_tx_req_get_param(tx, cstr!("p"), 1) as *mut htp_param_t;
        assert!(!param_p.is_null());
        assert_eq!(0, bstr_cmp_c((*param_p).value, cstr!("1")));

        let param_q = htp_tx_req_get_param(tx, cstr!("q"), 1) as *mut htp_param_t;
        assert!(!param_q.is_null());
        assert_eq!(0, bstr_cmp_c((*param_q).value, cstr!("2")));

        // Request headers
        htp_tx_req_set_header(
            tx,
            cstr!("Host"),
            4,
            cstr!("www.example.com"),
            15,
            HTP_ALLOC_COPY,
        );
        htp_tx_req_set_header(
            tx,
            cstr!("Connection"),
            10,
            cstr!("keep-alive"),
            10,
            HTP_ALLOC_COPY,
        );
        htp_tx_req_set_header(
            tx,
            cstr!("User-Agent"),
            10,
            cstr!("Mozilla/5.0"),
            11,
            HTP_ALLOC_COPY,
        );

        // Request headers complete
        htp_tx_state_request_headers(tx);

        // Check headers
        assert_eq!(1, t.user_data.callback_REQUEST_HEADERS_invoked);

        let h_host = htp_table_get_c((*tx).request_headers, cstr!("host")) as *mut htp_header_t;
        assert!(!h_host.is_null());
        assert_eq!(0, bstr_cmp_c((*h_host).value, cstr!("www.example.com")));

        let h_connection =
            htp_table_get_c((*tx).request_headers, cstr!("connection")) as *mut htp_header_t;
        assert!(!h_connection.is_null());
        assert_eq!(0, bstr_cmp_c((*h_connection).value, cstr!("keep-alive")));

        let h_ua = htp_table_get_c((*tx).request_headers, cstr!("user-agent")) as *mut htp_header_t;
        assert!(!h_ua.is_null());
        assert_eq!(0, bstr_cmp_c((*h_ua).value, cstr!("Mozilla/5.0")));

        // Request complete
        htp_tx_state_request_complete(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_COMPLETE_invoked);

        // Response begins
        htp_tx_state_response_start(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_START_invoked);

        // Response line data
        htp_tx_res_set_status_line(tx, cstr!("HTTP/1.1 200 OK"), 15, HTP_ALLOC_COPY);
        assert_eq!(0, bstr_cmp_c((*tx).response_protocol, cstr!("HTTP/1.1")));
        assert_eq!(
            Protocol::V1_1 as libc::c_int,
            (*tx).response_protocol_number
        );
        assert_eq!(200, (*tx).response_status_number);
        assert_eq!(0, bstr_cmp_c((*tx).response_message, cstr!("OK")));

        htp_tx_res_set_protocol_number(tx, Protocol::V1_0 as libc::c_int);
        assert_eq!(
            Protocol::V1_0 as libc::c_int,
            (*tx).response_protocol_number
        );

        htp_tx_res_set_status_code(tx, 500);
        assert_eq!(500, (*tx).response_status_number);

        htp_tx_res_set_status_message(tx, cstr!("Internal Server Error"), 21, HTP_ALLOC_COPY);
        assert_eq!(
            0,
            bstr_cmp_c((*tx).response_message, cstr!("Internal Server Error"))
        );

        // Response line complete
        htp_tx_state_response_line(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_LINE_invoked);

        // Response header data
        htp_tx_res_set_header(
            tx,
            cstr!("Content-Type"),
            12,
            cstr!("text/html"),
            9,
            HTP_ALLOC_COPY,
        );
        htp_tx_res_set_header(tx, cstr!("Server"), 6, cstr!("Apache"), 6, HTP_ALLOC_COPY);

        // Response headers complete
        htp_tx_state_response_headers(tx);
        assert_eq!(1, t.user_data.callback_RESPONSE_HEADERS_invoked);

        // Check response headers
        let mut h_content_type =
            htp_table_get_c((*tx).response_headers, cstr!("content-type")) as *mut htp_header_t;
        assert!(!h_content_type.is_null());
        assert_eq!(0, bstr_cmp_c((*h_content_type).value, cstr!("text/html")));

        let mut h_server =
            htp_table_get_c((*tx).response_headers, cstr!("server")) as *mut htp_header_t;
        assert!(!h_server.is_null());
        assert_eq!(0, bstr_cmp_c((*h_server).value, cstr!("Apache")));

        // Response body data
        htp_tx_res_process_body_data(tx, cstr!("<h1>Hello") as *const core::ffi::c_void, 9);
        htp_tx_res_process_body_data(tx, cstr!(" ") as *const core::ffi::c_void, 1);
        htp_tx_res_process_body_data(tx, cstr!("World!</h1>") as *const core::ffi::c_void, 11);
        assert_eq!(1, t.user_data.response_body_correctly_received);

        // Check that the API is rejecting std::ptr::null_mut() data.
        assert_eq!(
            Status::ERROR,
            htp_tx_res_process_body_data(tx, std::ptr::null_mut(), 1)
        );

        // Trailing response headers
        htp_tx_res_set_headers_clear(tx);
        assert_eq!(0, htp_table_size((*tx).response_headers));

        htp_tx_res_set_header(
            tx,
            cstr!("Content-Type"),
            12,
            cstr!("text/html"),
            9,
            HTP_ALLOC_COPY,
        );
        htp_tx_res_set_header(tx, cstr!("Server"), 6, cstr!("Apache"), 6, HTP_ALLOC_COPY);

        // Check trailing response headers
        h_content_type =
            htp_table_get_c((*tx).response_headers, cstr!("content-type")) as *mut htp_header_t;
        assert!(!h_content_type.is_null());
        assert_eq!(0, bstr_cmp_c((*h_content_type).value, cstr!("text/html")));

        h_server = htp_table_get_c((*tx).response_headers, cstr!("server")) as *mut htp_header_t;
        assert!(!h_server.is_null());
        assert_eq!(0, bstr_cmp_c((*h_server).value, cstr!("Apache")));

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
        htp_tx_req_set_method(tx, cstr!("POST"), 4, HTP_ALLOC_COPY);
        htp_tx_req_set_method_number(tx, HTP_M_GET as libc::c_uint);
        htp_tx_req_set_uri(tx, cstr!("/"), 1, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol(tx, cstr!("HTTP/1.1"), 8, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol_number(tx, Protocol::V1_1 as libc::c_int);
        htp_tx_req_set_protocol_0_9(tx, 0);

        // Request line complete
        htp_tx_state_request_line(tx);

        // Configure headers to trigger the URLENCODED parser
        htp_tx_req_set_header(
            tx,
            cstr!("Content-Type"),
            12,
            HTP_URLENCODED_MIME_TYPE.as_ptr() as *const libc::c_char,
            libc::strlen(HTP_URLENCODED_MIME_TYPE.as_ptr() as *const libc::c_char) as u64,
            HTP_ALLOC_COPY,
        );
        htp_tx_req_set_header(
            tx,
            cstr!("Content-Length"),
            14,
            cstr!("7"),
            1,
            HTP_ALLOC_COPY,
        );

        // Request headers complete
        htp_tx_state_request_headers(tx);

        // Send request body
        htp_tx_req_process_body_data(tx, cstr!("p=1") as *const core::ffi::c_void, 3);
        htp_tx_req_process_body_data(tx, std::ptr::null_mut(), 0);
        htp_tx_req_process_body_data(tx, cstr!("&") as *const core::ffi::c_void, 1);
        htp_tx_req_process_body_data(tx, cstr!("q=2") as *const core::ffi::c_void, 3);

        // Check that the API is rejecting std::ptr::null_mut() data.
        assert_eq!(
            Status::ERROR,
            htp_tx_req_process_body_data(tx, std::ptr::null_mut(), 1)
        );

        // Trailing request headers
        htp_tx_req_set_headers_clear(tx);
        assert_eq!(0, htp_table_size((*tx).request_headers));

        htp_tx_req_set_header(
            tx,
            cstr!("Host"),
            4,
            cstr!("www.example.com"),
            15,
            HTP_ALLOC_COPY,
        );
        htp_tx_req_set_header(
            tx,
            cstr!("Connection"),
            10,
            cstr!("keep-alive"),
            10,
            HTP_ALLOC_COPY,
        );
        htp_tx_req_set_header(
            tx,
            cstr!("User-Agent"),
            10,
            cstr!("Mozilla/5.0"),
            11,
            HTP_ALLOC_COPY,
        );

        let h_host = htp_table_get_c((*tx).request_headers, cstr!("host")) as *mut htp_header_t;
        assert!(!h_host.is_null());
        assert_eq!(0, bstr_cmp_c((*h_host).value, cstr!("www.example.com")));

        let h_connection =
            htp_table_get_c((*tx).request_headers, cstr!("connection")) as *mut htp_header_t;
        assert!(!h_connection.is_null());
        assert_eq!(0, bstr_cmp_c((*h_connection).value, cstr!("keep-alive")));

        let h_ua = htp_table_get_c((*tx).request_headers, cstr!("user-agent")) as *mut htp_header_t;
        assert!(!h_ua.is_null());
        assert_eq!(0, bstr_cmp_c((*h_ua).value, cstr!("Mozilla/5.0")));

        // Request complete
        htp_tx_state_request_complete(tx);

        // Check parameters

        let param_p = htp_tx_req_get_param(tx, cstr!("p"), 1) as *mut htp_param_t;
        assert!(!param_p.is_null());
        assert_eq!(0, bstr_cmp_c((*param_p).value, cstr!("1")));

        let param_q = htp_tx_req_get_param(tx, cstr!("q"), 1) as *mut htp_param_t;
        assert!(!param_q.is_null());
        assert_eq!(0, bstr_cmp_c((*param_q).value, cstr!("2")));
    }
}

const HYBRID_PARSING_COMPRESSED_RESPONSE: &'static [u8; 253] =
    b"H4sIAAAAAAAAAG2PwQ6CMBBE73xFU++tXk2pASliAiEhPegRYUOJYEktEP5eqB6dy2ZnJ5O3LJFZ\
      yj2WiCBah7zKVPBMT1AjCf2gTWnabmH0e/AY/QXDPLqj8HLO07zw8S52wkiKm1zXvRPeeg//2lbX\
      kwpQrauxh5dFqnyj3uVYgJJCxD5W1g5HSud5Jo3WTQek0mR8UgNlDYZOLcz0ZMuH3y+YKzDAaMDJ\
      SrihOVL32QceVXUy4QAAAA==\x00";

extern "C" fn HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(tx: *mut htp_tx_t) {
    unsafe {
        htp_tx_state_request_start(tx);

        // We need owned versions of these because we use HTP_ALLOC_REUSE below.
        let get = CString::new("GET").unwrap();
        let version = CString::new("HTTP/1.1").unwrap();
        let response_line = CString::new("HTTP/1.1 200 OK").unwrap();
        let content_encoding = CString::new("Content-Encoding").unwrap();
        let content_encoding_value = CString::new("gzip").unwrap();
        let content_length = CString::new("Content-Length").unwrap();
        let content_length_value = CString::new("187").unwrap();

        htp_tx_req_set_method(tx, get.as_ptr(), 3, HTP_ALLOC_REUSE);
        htp_tx_req_set_method_number(tx, HTP_M_GET as libc::c_uint);
        htp_tx_req_set_uri(tx, cstr!("/"), 1, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol(tx, version.as_ptr(), 8, HTP_ALLOC_REUSE);
        htp_tx_req_set_protocol_number(tx, Protocol::V1_1 as libc::c_int);
        htp_tx_req_set_protocol_0_9(tx, 0);

        htp_tx_state_request_line(tx);
        htp_tx_state_request_headers(tx);
        htp_tx_state_request_complete(tx);

        htp_tx_state_response_start(tx);

        htp_tx_res_set_status_line(tx, response_line.as_ptr(), 15, HTP_ALLOC_REUSE);
        htp_tx_res_set_header(
            tx,
            content_encoding.as_ptr(),
            16,
            content_encoding_value.as_ptr(),
            4,
            HTP_ALLOC_REUSE,
        );
        htp_tx_res_set_header(
            tx,
            content_length.as_ptr(),
            14,
            content_length_value.as_ptr(),
            3,
            HTP_ALLOC_REUSE,
        );

        htp_tx_state_response_headers(tx);

        let body: *mut bstr_t = htp_base64_decode_mem(
            HYBRID_PARSING_COMPRESSED_RESPONSE.as_ptr() as *const libc::c_void,
            libc::strlen(HYBRID_PARSING_COMPRESSED_RESPONSE.as_ptr() as *const libc::c_char) as u64,
        );
        assert!(!body.is_null());

        htp_tx_res_process_body_data(
            tx,
            bstr_ptr(body) as *const core::ffi::c_void,
            bstr_len(body),
        );
        bstr_free(body);

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

/// Test with a compressed response body and decompression disabled.
#[test]
fn CompressedResponseNoDecompression() {
    unsafe {
        let t = HybridParsingTest::new();
        // Disable decompression
        htp_config_set_response_decompression(t.cfg, 0);

        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(tx);

        assert_eq!(187, (*tx).response_message_len);
        assert_eq!(187, (*tx).response_entity_len);
    }
}

extern "C" fn HybridParsing_ForcedDecompressionTest_Callback_RESPONSE_HEADERS(
    tx: *mut htp_tx_t,
) -> Status {
    unsafe {
        (*tx).response_content_encoding_processing = HTP_COMPRESSION_GZIP;
        return Status::OK;
    }
}

/// Test forced decompression.
#[test]
fn ForcedDecompression() {
    unsafe {
        let t = HybridParsingTest::new();
        // Disable decompression
        htp_config_set_response_decompression(t.cfg, 0);

        // Register a callback that will force decompression
        htp_config_register_response_headers(
            t.cfg,
            Some(HybridParsing_ForcedDecompressionTest_Callback_RESPONSE_HEADERS),
        );

        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(tx);

        assert_eq!(187, (*tx).response_message_len);
        assert_eq!(225, (*tx).response_entity_len);
    }
}

extern "C" fn HybridParsing_DisableDecompressionTest_Callback_RESPONSE_HEADERS(
    tx: *mut htp_tx_t,
) -> Status {
    unsafe {
        (*tx).response_content_encoding_processing = HTP_COMPRESSION_NONE;
        return Status::OK;
    }
}

/// Test disabling decompression from a callback.
#[test]
fn DisableDecompression() {
    unsafe {
        let t = HybridParsingTest::new();
        // Disable decompression
        htp_config_set_response_decompression(t.cfg, 0);

        // Register a callback that will force decompression
        htp_config_register_response_headers(
            t.cfg,
            Some(HybridParsing_DisableDecompressionTest_Callback_RESPONSE_HEADERS),
        );

        // Create a new LibHTP transaction
        let tx = htp_connp_tx_create(t.connp) as *mut htp_tx_t;
        assert!(!tx.is_null());

        HYBRID_PARSING_COMPRESSED_RESPONSE_Setup(tx);

        assert_eq!(187, (*tx).response_message_len);
        assert_eq!(187, (*tx).response_entity_len);
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
        htp_tx_req_set_method(tx, cstr!("GET"), 3, HTP_ALLOC_COPY);
        htp_tx_req_set_method_number(tx, HTP_M_GET as libc::c_uint);
        htp_tx_req_set_uri(tx, cstr!("/?p=1&Q=2"), 9, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol(tx, cstr!("HTTP/1.1"), 8, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol_number(tx, Protocol::V1_1 as libc::c_int);
        htp_tx_req_set_protocol_0_9(tx, 0);

        // Request line complete
        htp_tx_state_request_line(tx);

        // Check the parameters.

        let mut param_p = htp_tx_req_get_param(tx, cstr!("p"), 1) as *mut htp_param_t;
        assert!(!param_p.is_null());
        assert_eq!(0, bstr_cmp_c((*param_p).value, cstr!("1")));

        param_p = htp_tx_req_get_param(tx, cstr!("P"), 1);
        assert!(!param_p.is_null());
        assert_eq!(0, bstr_cmp_c((*param_p).value, cstr!("1")));

        let mut param_q = htp_tx_req_get_param(tx, cstr!("q"), 1) as *mut htp_param_t;
        assert!(!param_q.is_null());
        assert_eq!(0, bstr_cmp_c((*param_q).value, cstr!("2")));

        param_q = htp_tx_req_get_param_ex(tx, HTP_SOURCE_QUERY_STRING, cstr!("q"), 1);
        assert!(!param_q.is_null());
        assert_eq!(0, bstr_cmp_c((*param_q).value, cstr!("2")));

        param_q = htp_tx_req_get_param_ex(tx, HTP_SOURCE_QUERY_STRING, cstr!("Q"), 1);
        assert!(!param_q.is_null());
        assert_eq!(0, bstr_cmp_c((*param_q).value, cstr!("2")));
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
        htp_tx_req_set_method(tx, cstr!("POST"), 4, HTP_ALLOC_COPY);
        htp_tx_req_set_method_number(tx, HTP_M_GET as libc::c_uint);
        htp_tx_req_set_uri(tx, cstr!("/"), 1, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol(tx, cstr!("HTTP/1.1"), 8, HTP_ALLOC_COPY);
        htp_tx_req_set_protocol_number(tx, Protocol::V1_1 as libc::c_int);
        htp_tx_req_set_protocol_0_9(tx, 0);
        htp_tx_state_request_line(tx);

        // Configure headers to trigger the URLENCODED parser.
        htp_tx_req_set_header(
            tx,
            cstr!("Content-Type"),
            12,
            HTP_URLENCODED_MIME_TYPE.as_ptr() as *const libc::c_char,
            libc::strlen(HTP_URLENCODED_MIME_TYPE.as_ptr() as *const libc::c_char) as u64,
            HTP_ALLOC_COPY,
        );
        htp_tx_req_set_header(
            tx,
            cstr!("Transfer-Encoding"),
            17,
            cstr!("chunked"),
            7,
            HTP_ALLOC_COPY,
        );

        // Request headers complete.
        htp_tx_state_request_headers(tx);

        // Send request body.
        htp_tx_req_process_body_data(tx, cstr!("p=1") as *const libc::c_void, 3);
        htp_tx_req_process_body_data(tx, cstr!("&") as *const libc::c_void, 1);
        htp_tx_req_process_body_data(tx, cstr!("q=2") as *const libc::c_void, 3);

        // Request complete.
        htp_tx_state_request_complete(tx);

        // Check the parameters.

        let param_p = htp_tx_req_get_param(tx, cstr!("p"), 1) as *mut htp_param_t;
        assert!(!param_p.is_null());
        assert_eq!(0, bstr_cmp_c((*param_p).value, cstr!("1")));

        let param_q = htp_tx_req_get_param(tx, cstr!("q"), 1) as *mut htp_param_t;
        assert!(!param_q.is_null());
        assert_eq!(0, bstr_cmp_c((*param_q).value, cstr!("2")));
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
        htp_tx_req_set_line(tx, cstr!("GET /?p=1&q=2 HTTP/1.0"), 22, HTP_ALLOC_COPY);

        // Request line complete
        htp_tx_state_request_line(tx);

        assert_eq!(0, bstr_cmp_c((*tx).request_method, cstr!("GET")));
        assert_eq!(0, bstr_cmp_c((*tx).request_uri, cstr!("/?p=1&q=2")));
        assert_eq!(0, bstr_cmp_c((*tx).request_protocol, cstr!("HTTP/1.0")));

        assert!(!(*tx).parsed_uri.is_null());
        assert_eq!(0, bstr_cmp_c((*(*tx).parsed_uri).query, cstr!("p=1&q=2")));

        // Check parameters
        let param_p = htp_tx_req_get_param(tx, cstr!("p"), 1) as *mut htp_param_t;
        assert!(!param_p.is_null());
        assert_eq!(0, bstr_cmp_c((*param_p).value, cstr!("1")));

        let param_q = htp_tx_req_get_param(tx, cstr!("q"), 1) as *mut htp_param_t;
        assert!(!param_q.is_null());
        assert_eq!(0, bstr_cmp_c((*param_q).value, cstr!("2")));
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
        htp_tx_req_set_line(tx, cstr!("GET /"), 5, HTP_ALLOC_COPY);
        htp_tx_state_request_line(tx);

        // Check the results now.

        assert_eq!(0, bstr_cmp_c((*tx).request_method, cstr!("GET")));
        assert_eq!(1, (*tx).is_protocol_0_9);
        assert_eq!(Protocol::V0_9 as libc::c_int, (*tx).request_protocol_number);
        assert!((*tx).request_protocol.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_uri, cstr!("/")));
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
        htp_tx_req_set_line(tx, cstr!("GET /?p=1&q=2 HTTP/1.0"), 22, HTP_ALLOC_COPY);

        //htp_uri_t *u = htp_uri_alloc();
        let u = htp_uri_alloc();
        (*u).path = bstr_dup_c(cstr!("/123"));
        htp_tx_req_set_parsed_uri(tx, u);

        htp_tx_state_request_line(tx);

        // Check the results now.

        assert_eq!(0, bstr_cmp_c((*tx).request_method, cstr!("GET")));
        assert!(!(*tx).request_protocol.is_null());
        assert_eq!(Protocol::V1_0 as libc::c_int, (*tx).request_protocol_number);
        assert!(!(*tx).request_uri.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_uri, cstr!("/?p=1&q=2")));

        assert!(!(*tx).parsed_uri.is_null());
        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_c((*(*tx).parsed_uri).path, cstr!("/123")));
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
        htp_tx_req_set_line(tx, cstr!("GET / HTTP/1.0"), 14, HTP_ALLOC_COPY);

        // Request line complete
        htp_tx_state_request_line(tx);
        assert_eq!(1, t.user_data.callback_REQUEST_LINE_invoked);

        // Check request line data
        assert!(!(*tx).request_method.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_method, cstr!("GET")));
        assert!(!(*tx).request_uri.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_uri, cstr!("/")));
        assert!(!(*tx).request_protocol.is_null());
        assert_eq!(0, bstr_cmp_c((*tx).request_protocol, cstr!("HTTP/1.0")));

        assert!(!(*tx).parsed_uri.is_null());

        assert!(!(*(*tx).parsed_uri).path.is_null());
        assert_eq!(0, bstr_cmp_c((*(*tx).parsed_uri).path, cstr!("/")));

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
        htp_tx_res_set_status_line(tx, cstr!("HTTP/1.1 200 OK\r\n"), 17, HTP_ALLOC_COPY);

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
        htp_tx_req_set_line(tx, cstr!("GET / HTTP/1.0"), 14, HTP_ALLOC_COPY);

        assert_eq!(htp_tx_destroy(tx), Status::ERROR);

        // Close connection
        t.close_conn_parser();
    }
}
