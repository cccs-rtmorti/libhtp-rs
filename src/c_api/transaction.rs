use crate::bstr;
use crate::hook::DataExternalCallbackFn;
use crate::htp_config;
use crate::htp_connection_parser;
use crate::htp_decompressors;
use crate::htp_request;
use crate::htp_transaction;
use crate::htp_util;
use crate::Status;
use std::convert::{TryFrom, TryInto};

/// Creates a new transaction.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_create(
    connp: *mut htp_connection_parser::htp_connp_t,
) -> *mut htp_transaction::htp_tx_t {
    if let Some(connp) = connp.as_mut() {
        if let Ok(tx_id) = connp.create_tx() {
            connp.conn.tx_mut_ptr(tx_id)
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Destroys the supplied transaction.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_destroy(tx: *mut htp_transaction::htp_tx_t) -> Status {
    if let Some(tx) = tx.as_mut() {
        tx.destroy().into()
    } else {
        Status::ERROR
    }
}

/// Get a transaction's connection parser.
///
/// tx: Transaction pointer.
///
/// Returns the connection parser or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_connp(
    tx: *mut htp_transaction::htp_tx_t,
) -> *mut htp_connection_parser::htp_connp_t {
    if let Some(tx) = tx.as_ref() {
        tx.connp
    } else {
        std::ptr::null_mut()
    }
}

/// Get the transaction's configuration.
///
/// tx: Transaction pointer.
///
/// Returns a pointer to the configuration or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_cfg(
    tx: *mut htp_transaction::htp_tx_t,
) -> *mut htp_config::htp_cfg_t {
    if let Some(tx) = tx.as_mut() {
        tx.cfg
    } else {
        std::ptr::null_mut()
    }
}

/// Returns the user data associated with this transaction or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_user_data(
    tx: *const htp_transaction::htp_tx_t,
) -> *mut libc::c_void {
    if let Some(tx) = tx.as_ref() {
        tx.user_data()
    } else {
        std::ptr::null_mut()
    }
}

/// Associates user data with this transaction.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_user_data(
    tx: *mut htp_transaction::htp_tx_t,
    user_data: *mut libc::c_void,
) {
    if let Some(tx) = tx.as_mut() {
        tx.set_user_data(user_data)
    }
}

/// Get a transaction's request line.
///
/// tx: Transaction pointer.
///
/// Returns the request line or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_line(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_line
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's request method.
///
/// tx: Transaction pointer.
///
/// Returns the request method or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_method(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_method
    } else {
        std::ptr::null()
    }
}

/// Get the transaction's request method number.
///
/// tx: Transaction pointer.
///
/// Returns the request method number or HTP_M_ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_method_number(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_request::htp_method_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_method_number
    } else {
        htp_request::htp_method_t::HTP_M_ERROR
    }
}

/// Get a transaction's request uri.
///
/// tx: Transaction pointer.
///
/// Returns the request uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_uri(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_uri
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's request protocol.
///
/// tx: Transaction pointer.
///
/// Returns the protocol or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_protocol(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_protocol
    } else {
        std::ptr::null_mut()
    }
}

/// Get a transaction's request protocol number.
///
/// tx: Transaction pointer.
///
/// Returns the protocol number or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_protocol_number(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_transaction::Protocol {
    if let Some(tx) = tx.as_ref() {
        tx.request_protocol_number
    } else {
        htp_transaction::Protocol::ERROR
    }
}

/// Get whether a transaction's protocol is version 0.9.
///
/// tx: Transaction pointer.
///
/// Returns 1 if the version is 0.9 or 0 otherwise. A NULL argument will
/// also result in a return value of 0.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_is_protocol_0_9(tx: *const htp_transaction::htp_tx_t) -> i32 {
    if let Some(tx) = tx.as_ref() {
        tx.is_protocol_0_9
    } else {
        0
    }
}

/// Get a transaction's parsed uri.
///
/// tx: Transaction pointer.
///
/// Returns the parsed uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_parsed_uri(
    tx: *mut htp_transaction::htp_tx_t,
) -> *mut htp_util::htp_uri_t {
    if let Some(tx) = tx.as_mut() {
        tx.parsed_uri
    } else {
        std::ptr::null_mut()
    }
}

/// Get a transaction's request headers.
///
/// tx: Transaction pointer.
///
/// Returns the request headers or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_headers(
    tx: *const htp_transaction::htp_tx_t,
) -> *const htp_transaction::htp_headers_t {
    if let Some(tx) = tx.as_ref() {
        &tx.request_headers
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's request headers size.
///
/// tx: Transaction pointer.
///
/// Returns the size or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_headers_size(
    tx: *const htp_transaction::htp_tx_t,
) -> isize {
    if let Some(tx) = tx.as_ref() {
        isize::try_from(tx.request_headers.size()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Get the first request header value matching the key from a transaction.
///
/// tx: Transaction pointer.
/// ckey: Header name to match.
///
/// Returns the header or NULL when not found or on error
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_header(
    tx: *const htp_transaction::htp_tx_t,
    ckey: *const libc::c_char,
) -> *const htp_transaction::htp_header_t {
    if let Some(tx) = tx.as_ref() {
        super::htp_headers_get(&tx.request_headers, ckey)
    } else {
        std::ptr::null()
    }
}

/// Get the request header at the given index.
///
/// tx: Transaction pointer.
/// index: request header table index.
///
/// Returns the header or NULL on error
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_header_index(
    tx: *const htp_transaction::htp_tx_t,
    index: usize,
) -> *const htp_transaction::htp_header_t {
    if let Some(tx) = tx.as_ref() {
        if let Some((_, value)) = tx.request_headers.get(index) {
            value
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's request transfer coding.
///
/// tx: Transaction pointer.
///
/// Returns the transfer coding or HTP_CODING_ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_transfer_coding(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_transaction::htp_transfer_coding_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_transfer_coding
    } else {
        htp_transaction::htp_transfer_coding_t::HTP_CODING_ERROR
    }
}

/// Get a transaction's request content encoding.
///
/// tx: Transaction pointer.
///
/// Returns the content encoding or HTP_COMPRESSION_ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_content_encoding(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_decompressors::htp_content_encoding_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_content_encoding
    } else {
        htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_ERROR
    }
}

/// Get a transaction's request content type.
///
/// tx: Transaction pointer.
///
/// Returns the content type or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_content_type(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_content_type
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's request content length.
///
/// tx: Transaction pointer.
///
/// Returns the content length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_content_length(
    tx: *const htp_transaction::htp_tx_t,
) -> i64 {
    if let Some(tx) = tx.as_ref() {
        tx.request_content_length
    } else {
        -1
    }
}

/// Get the transaction's request authentication type.
///
/// tx: Transaction pointer.
///
/// Returns the auth type or HTP_AUTH_ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_auth_type(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_transaction::htp_auth_type_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_auth_type
    } else {
        htp_transaction::htp_auth_type_t::HTP_AUTH_ERROR
    }
}

/// Get a transaction's request hostname.
///
/// tx: Transaction pointer.
///
/// Returns the request hostname or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_hostname(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_hostname
    } else {
        std::ptr::null()
    }
}

/// Get the transaction's request port number.
///
/// tx: Transaction pointer.
///
/// Returns the request port number or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_port_number(tx: *const htp_transaction::htp_tx_t) -> i32 {
    if let Some(tx) = tx.as_ref() {
        tx.request_port_number
    } else {
        -1
    }
}

/// Get a transaction's request message length.
///
/// tx: Transaction pointer.
///
/// Returns the request message length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_message_len(tx: *const htp_transaction::htp_tx_t) -> i64 {
    if let Some(tx) = tx.as_ref() {
        tx.request_message_len
    } else {
        -1
    }
}

/// Get a transaction's request entity length.
///
/// tx: Transaction pointer.
///
/// Returns the request entity length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_entity_len(tx: *const htp_transaction::htp_tx_t) -> i64 {
    if let Some(tx) = tx.as_ref() {
        tx.request_entity_len
    } else {
        -1
    }
}

/// Get a transaction's response line.
///
/// tx: Transaction pointer.
///
/// Returns the response line or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_line(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.response_line
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's response protocol.
///
/// tx: Transaction pointer.
///
/// Returns the response protocol or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_protocol(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.response_protocol
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's response protocol number.
///
/// tx: Transaction pointer.
///
/// Returns the protocol number or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_protocol_number(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_transaction::Protocol {
    if let Some(tx) = tx.as_ref() {
        tx.response_protocol_number
    } else {
        htp_transaction::Protocol::ERROR
    }
}

/// Get the transaction's response status.
///
/// tx: Transaction pointer.
///
/// Returns the response status or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_status(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.response_status
    } else {
        std::ptr::null()
    }
}

/// Get the transaction's response status number.
///
/// tx: Transaction pointer.
///
/// Returns the response status number or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_status_number(
    tx: *const htp_transaction::htp_tx_t,
) -> i32 {
    if let Some(tx) = tx.as_ref() {
        tx.response_status_number
    } else {
        -1
    }
}
/// Get the transaction's response status expected number.
///
/// tx: Transaction pointer.
///
/// Returns the expected number or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_status_expected_number(
    tx: *const htp_transaction::htp_tx_t,
) -> i32 {
    if let Some(tx) = tx.as_ref() {
        tx.response_status_expected_number
    } else {
        -1
    }
}

/// Get a transaction's response message.
///
/// tx: Transaction pointer.
///
/// Returns the response message or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_message(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.response_message
    } else {
        std::ptr::null_mut()
    }
}

/// Get a transaction's response headers.
///
/// tx: Transaction pointer.
///
/// Returns the response headers or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_headers(
    tx: *const htp_transaction::htp_tx_t,
) -> *const htp_transaction::htp_headers_t {
    if let Some(tx) = tx.as_ref() {
        &tx.response_headers
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's response headers size.
///
/// tx: Transaction pointer.
///
/// Returns the size or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_headers_size(
    tx: *const htp_transaction::htp_tx_t,
) -> isize {
    if let Some(tx) = tx.as_ref() {
        isize::try_from(tx.response_headers.size()).unwrap_or(-1)
    } else {
        -1
    }
}

/// Get the first response header value matching the key from a transaction.
///
/// tx: Transaction pointer.
/// ckey: Header name to match.
///
/// Returns the header or NULL when not found or on error
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_header(
    tx: *const htp_transaction::htp_tx_t,
    ckey: *const libc::c_char,
) -> *const htp_transaction::htp_header_t {
    if let Some(tx) = tx.as_ref() {
        super::htp_headers_get(&tx.response_headers, ckey)
    } else {
        std::ptr::null()
    }
}

/// Get the response header at the given index.
///
/// tx: Transaction pointer.
/// index: response header table index.
///
/// Returns the header or NULL on error
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_header_index(
    tx: *const htp_transaction::htp_tx_t,
    index: usize,
) -> *const htp_transaction::htp_header_t {
    if let Some(tx) = tx.as_ref() {
        if let Some((_, value)) = tx.response_headers.get(index) {
            value
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

/// Get a transaction's response message length.
///
/// tx: Transaction pointer.
///
/// Returns the response message length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_message_len(tx: *const htp_transaction::htp_tx_t) -> i64 {
    if let Some(tx) = tx.as_ref() {
        tx.response_message_len
    } else {
        -1
    }
}

/// Get a transaction's response entity length.
///
/// tx: Transaction pointer.
///
/// Returns the response entity length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_entity_len(tx: *const htp_transaction::htp_tx_t) -> i64 {
    if let Some(tx) = tx.as_ref() {
        tx.response_entity_len
    } else {
        -1
    }
}

/// Get a transaction's response content length.
///
/// tx: Transaction pointer.
///
/// Returns the response content length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_content_length(
    tx: *const htp_transaction::htp_tx_t,
) -> i64 {
    if let Some(tx) = tx.as_ref() {
        tx.response_content_length
    } else {
        -1
    }
}

/// Get a transaction's response content type.
///
/// tx: Transaction pointer.
///
/// Returns the response content type or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_content_type(
    tx: *const htp_transaction::htp_tx_t,
) -> *const bstr::bstr_t {
    if let Some(tx) = tx.as_ref() {
        tx.response_content_type
    } else {
        std::ptr::null()
    }
}

/// Get the transaction's bit flags.
///
/// tx: Transaction pointer.
///
/// Returns the flags represented as an integer or 0 if the flags are empty
/// or a NULL ptr is passed as an argument.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_flags(tx: *const htp_transaction::htp_tx_t) -> u64 {
    if let Some(tx) = tx.as_ref() {
        tx.flags.bits()
    } else {
        0
    }
}

/// Get the transaction's request progress.
///
/// tx: Transaction pointer.
///
/// Returns the progress or HTP_REQUEST_ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_progress(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_transaction::htp_tx_req_progress_t {
    if let Some(tx) = tx.as_ref() {
        tx.request_progress
    } else {
        htp_transaction::htp_tx_req_progress_t::HTP_REQUEST_ERROR
    }
}

/// Set the transaction's request progress.
///
/// tx: Transaction pointer.
///
/// Returns HTP_OK on success or HTP_ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_request_progress(
    tx: *mut htp_transaction::htp_tx_t,
    progress: htp_transaction::htp_tx_req_progress_t,
) -> Status {
    if let Some(tx) = tx.as_mut() {
        tx.request_progress = progress;
        Status::OK
    } else {
        Status::ERROR
    }
}

/// Get the transaction's response progress.
///
/// tx: Transaction pointer.
///
/// Returns the progress or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_progress(
    tx: *const htp_transaction::htp_tx_t,
) -> htp_transaction::htp_tx_res_progress_t {
    if let Some(tx) = tx.as_ref() {
        tx.response_progress
    } else {
        htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_ERROR
    }
}

/// Get the transaction's index.
///
/// tx: Transaction pointer.
///
/// Returns an index or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_index(tx: *const htp_transaction::htp_tx_t) -> isize {
    if let Some(tx) = tx.as_ref() {
        isize::try_from(tx.index).unwrap_or(-1)
    } else {
        -1
    }
}

/// Set the transaction's response progress.
///
/// tx: Transaction pointer.
///
/// Returns HTP_OK on success or HTP_ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_response_progress(
    tx: *mut htp_transaction::htp_tx_t,
    progress: htp_transaction::htp_tx_res_progress_t,
) -> Status {
    if let Some(tx) = tx.as_mut() {
        tx.response_progress = progress;
        Status::OK
    } else {
        Status::ERROR
    }
}

/// Change transaction state to REQUEST and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_request_complete(
    tx: *mut htp_transaction::htp_tx_t,
) -> Status {
    if let Some(tx) = tx.as_mut() {
        tx.state_request_complete().into()
    } else {
        Status::ERROR
    }
}

/// Change transaction state to RESPONSE and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns HTP_OK on success; HTP_ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_response_complete(
    tx: *mut htp_transaction::htp_tx_t,
) -> Status {
    if let Some(tx) = tx.as_mut() {
        tx.state_response_complete().into()
    } else {
        Status::ERROR
    }
}

/// Register callback for the transaction-specific RESPONSE_BODY_DATA hook.
#[no_mangle]
pub unsafe fn htp_tx_register_response_body_data(
    tx: *mut htp_transaction::htp_tx_t,
    cbk_fn: DataExternalCallbackFn,
) {
    if let Some(tx) = tx.as_mut() {
        tx.hook_response_body_data.register_extern(cbk_fn);
    }
}

/// Get the data's transaction.
///
/// Returns the transaction or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_data_tx(
    data: *mut htp_transaction::htp_tx_data_t,
) -> *mut htp_transaction::htp_tx_t {
    if let Some(data) = data.as_ref() {
        data.tx()
    } else {
        std::ptr::null_mut()
    }
}

/// Get the data pointer.
///
/// Returns the data or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_data_data(
    data: *const htp_transaction::htp_tx_data_t,
) -> *const u8 {
    if let Some(data) = data.as_ref() {
        data.data()
    } else {
        std::ptr::null()
    }
}

/// Get the length of the data.
///
/// Returns the length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_data_len(data: *const htp_transaction::htp_tx_data_t) -> isize {
    if let Some(data) = data.as_ref() {
        data.len().try_into().unwrap_or(-1)
    } else {
        -1
    }
}

/// Get whether this is the last chunk of data.
///
/// Returns true if this is the last chunk of data or false otherwise.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_data_is_last(data: *const htp_transaction::htp_tx_data_t) -> bool {
    if let Some(data) = data.as_ref() {
        data.is_last()
    } else {
        false
    }
}

/// Get whether this data is empty.
///
/// Returns true if data is NULL or zero-length.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_data_is_empty(data: *const htp_transaction::htp_tx_data_t) -> bool {
    if let Some(data) = data.as_ref() {
        data.is_empty()
    } else {
        true
    }
}
