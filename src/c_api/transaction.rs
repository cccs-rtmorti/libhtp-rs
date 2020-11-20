use crate::{
    bstr::Bstr, config::Config, connection_parser::ConnectionParser,
    decompressors::HtpContentEncoding, hook::DataExternalCallbackFn, request::HtpMethod,
    transaction::*, uri::Uri, HtpStatus,
};
use std::convert::{TryFrom, TryInto};

/// Creates a new transaction.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_create(connp: *mut ConnectionParser) -> *mut Transaction {
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
pub unsafe extern "C" fn htp_tx_destroy(tx: *mut Transaction) -> HtpStatus {
    if let Some(tx) = tx.as_mut() {
        tx.destroy().into()
    } else {
        HtpStatus::ERROR
    }
}

/// Get a transaction's normalized parsed uri.
///
/// tx: Transaction pointer.
///
/// Returns the complete normalized uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_normalized_uri(tx: *mut Transaction, all: bool) -> *const Bstr {
    if all {
        if let Some(uri) = tx
            .as_ref()
            .and_then(|tx| tx.complete_normalized_uri.as_ref())
            .map(|uri| uri)
        {
            return uri;
        }
    } else if let Some(uri) = tx
        .as_ref()
        .and_then(|tx| tx.partial_normalized_uri.as_ref())
        .map(|uri| uri)
    {
        return uri;
    }
    std::ptr::null()
}

/// Get a transaction's connection parser.
///
/// tx: Transaction pointer.
///
/// Returns the connection parser or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_connp(tx: *mut Transaction) -> *mut ConnectionParser {
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
pub unsafe extern "C" fn htp_tx_cfg(tx: *mut Transaction) -> *mut Config {
    if let Some(tx) = tx.as_mut() {
        tx.cfg
    } else {
        std::ptr::null_mut()
    }
}

/// Returns the user data associated with this transaction or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_user_data(tx: *const Transaction) -> *mut libc::c_void {
    if let Some(tx) = tx.as_ref() {
        tx.user_data()
    } else {
        std::ptr::null_mut()
    }
}

/// Associates user data with this transaction.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_user_data(tx: *mut Transaction, user_data: *mut libc::c_void) {
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
pub unsafe extern "C" fn htp_tx_request_line(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.request_line.as_ref())
        .map(|line| line as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's request method.
///
/// tx: Transaction pointer.
///
/// Returns the request method or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_method(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.request_method.as_ref())
        .map(|method| method as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get the transaction's request method number.
///
/// tx: Transaction pointer.
///
/// Returns the request method number or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_method_number(tx: *const Transaction) -> HtpMethod {
    if let Some(tx) = tx.as_ref() {
        tx.request_method_number
    } else {
        HtpMethod::ERROR
    }
}

/// Get a transaction's request uri.
///
/// tx: Transaction pointer.
///
/// Returns the request uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_uri(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.request_uri.as_ref())
        .map(|uri| uri as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's request protocol.
///
/// tx: Transaction pointer.
///
/// Returns the protocol or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_protocol(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.request_protocol.as_ref())
        .map(|protocol| protocol as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's request protocol number.
///
/// tx: Transaction pointer.
///
/// Returns the protocol number or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_protocol_number(tx: *const Transaction) -> HtpProtocol {
    if let Some(tx) = tx.as_ref() {
        tx.request_protocol_number
    } else {
        HtpProtocol::ERROR
    }
}

/// Get whether a transaction's protocol is version 0.9.
///
/// tx: Transaction pointer.
///
/// Returns 1 if the version is 0.9 or 0 otherwise. A NULL argument will
/// also result in a return value of 0.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_is_protocol_0_9(tx: *const Transaction) -> i32 {
    if let Some(tx) = tx.as_ref() {
        tx.is_protocol_0_9 as i32
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
pub unsafe extern "C" fn htp_tx_parsed_uri(tx: *mut Transaction) -> *const Uri {
    tx.as_ref()
        .and_then(|tx| tx.parsed_uri.as_ref())
        .map(|uri| uri as *const Uri)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's request headers.
///
/// tx: Transaction pointer.
///
/// Returns the request headers or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_headers(tx: *const Transaction) -> *const Headers {
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
pub unsafe extern "C" fn htp_tx_request_headers_size(tx: *const Transaction) -> isize {
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
    tx: *const Transaction,
    ckey: *const libc::c_char,
) -> *const Header {
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
    tx: *const Transaction,
    index: usize,
) -> *const Header {
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
/// Returns the transfer coding or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_transfer_coding(
    tx: *const Transaction,
) -> HtpTransferCoding {
    if let Some(tx) = tx.as_ref() {
        tx.request_transfer_coding
    } else {
        HtpTransferCoding::ERROR
    }
}

/// Get a transaction's request content encoding.
///
/// tx: Transaction pointer.
///
/// Returns the content encoding or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_content_encoding(
    tx: *const Transaction,
) -> HtpContentEncoding {
    if let Some(tx) = tx.as_ref() {
        tx.request_content_encoding
    } else {
        HtpContentEncoding::ERROR
    }
}

/// Get a transaction's request content type.
///
/// tx: Transaction pointer.
///
/// Returns the content type or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_content_type(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.request_content_type.as_ref())
        .map(|content_type| content_type as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's request content length.
///
/// tx: Transaction pointer.
///
/// Returns the content length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_content_length(tx: *const Transaction) -> i64 {
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
pub unsafe extern "C" fn htp_tx_request_auth_type(tx: *const Transaction) -> HtpAuthType {
    if let Some(tx) = tx.as_ref() {
        tx.request_auth_type
    } else {
        HtpAuthType::ERROR
    }
}

/// Get a transaction's request hostname.
///
/// tx: Transaction pointer.
///
/// Returns the request hostname or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_hostname(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.request_hostname.as_ref())
        .map(|hostname| hostname as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get the transaction's request port number.
///
/// tx: Transaction pointer.
///
/// Returns the request port number or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_port_number(tx: *const Transaction) -> i32 {
    tx.as_ref()
        .and_then(|tx| tx.request_port_number.as_ref())
        .map(|port| *port as i32)
        .unwrap_or(-1)
}

/// Get a transaction's request message length.
///
/// tx: Transaction pointer.
///
/// Returns the request message length or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_message_len(tx: *const Transaction) -> i64 {
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
pub unsafe extern "C" fn htp_tx_request_entity_len(tx: *const Transaction) -> i64 {
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
pub unsafe extern "C" fn htp_tx_response_line(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.response_line.as_ref())
        .map(|response_line| response_line as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's response protocol.
///
/// tx: Transaction pointer.
///
/// Returns the response protocol or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_protocol(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.response_protocol.as_ref())
        .map(|response_protocol| response_protocol as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's response protocol number.
///
/// tx: Transaction pointer.
///
/// Returns the protocol number or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_protocol_number(tx: *const Transaction) -> HtpProtocol {
    if let Some(tx) = tx.as_ref() {
        tx.response_protocol_number
    } else {
        HtpProtocol::ERROR
    }
}

/// Get the transaction's response status.
///
/// tx: Transaction pointer.
///
/// Returns the response status or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_status(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.response_status.as_ref())
        .map(|response_status| response_status as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get the transaction's response status number.
///
/// tx: Transaction pointer.
///
/// Returns the response status number or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_status_number(tx: *const Transaction) -> i32 {
    tx.as_ref()
        .map(|tx| match tx.response_status_number {
            HtpResponseNumber::UNKNOWN => 0,
            HtpResponseNumber::INVALID => -1,
            HtpResponseNumber::VALID(status) => status as i32,
        })
        .unwrap_or(-1)
}
/// Get the transaction's response status expected number.
///
/// tx: Transaction pointer.
///
/// Returns the expected number or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_status_expected_number(tx: *const Transaction) -> i32 {
    tx.as_ref()
        .map(|tx| tx.response_status_expected_number as i32)
        .unwrap_or(-1)
}

/// Get a transaction's response message.
///
/// tx: Transaction pointer.
///
/// Returns the response message or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_message(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.response_message.as_ref())
        .map(|response_message| response_message as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get a transaction's response headers.
///
/// tx: Transaction pointer.
///
/// Returns the response headers or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_headers(tx: *const Transaction) -> *const Headers {
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
pub unsafe extern "C" fn htp_tx_response_headers_size(tx: *const Transaction) -> isize {
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
    tx: *const Transaction,
    ckey: *const libc::c_char,
) -> *const Header {
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
    tx: *const Transaction,
    index: usize,
) -> *const Header {
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
pub unsafe extern "C" fn htp_tx_response_message_len(tx: *const Transaction) -> i64 {
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
pub unsafe extern "C" fn htp_tx_response_entity_len(tx: *const Transaction) -> i64 {
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
pub unsafe extern "C" fn htp_tx_response_content_length(tx: *const Transaction) -> i64 {
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
pub unsafe extern "C" fn htp_tx_response_content_type(tx: *const Transaction) -> *const Bstr {
    tx.as_ref()
        .and_then(|tx| tx.response_content_type.as_ref())
        .map(|response_content_type| response_content_type as *const Bstr)
        .unwrap_or(std::ptr::null())
}

/// Get the transaction's bit flags.
///
/// tx: Transaction pointer.
///
/// Returns the flags represented as an integer or 0 if the flags are empty
/// or a NULL ptr is passed as an argument.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_flags(tx: *const Transaction) -> u64 {
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
pub unsafe extern "C" fn htp_tx_request_progress(tx: *const Transaction) -> HtpRequestProgress {
    if let Some(tx) = tx.as_ref() {
        tx.request_progress
    } else {
        HtpRequestProgress::ERROR
    }
}

/// Set the transaction's request progress.
///
/// tx: Transaction pointer.
///
/// Returns OK on success or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_request_progress(
    tx: *mut Transaction,
    progress: HtpRequestProgress,
) -> HtpStatus {
    if let Some(tx) = tx.as_mut() {
        tx.request_progress = progress;
        HtpStatus::OK
    } else {
        HtpStatus::ERROR
    }
}

/// Get the transaction's response progress.
///
/// tx: Transaction pointer.
///
/// Returns the progress or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_progress(tx: *const Transaction) -> HtpResponseProgress {
    if let Some(tx) = tx.as_ref() {
        tx.response_progress
    } else {
        HtpResponseProgress::ERROR
    }
}

/// Get the transaction's index.
///
/// tx: Transaction pointer.
///
/// Returns an index or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_index(tx: *const Transaction) -> isize {
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
/// Returns OK on success or ERROR on error.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_set_response_progress(
    tx: *mut Transaction,
    progress: HtpResponseProgress,
) -> HtpStatus {
    if let Some(tx) = tx.as_mut() {
        tx.response_progress = progress;
        HtpStatus::OK
    } else {
        HtpStatus::ERROR
    }
}

/// Change transaction state to REQUEST and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns OK on success; ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_request_complete(tx: *mut Transaction) -> HtpStatus {
    if let Some(tx) = tx.as_mut() {
        tx.state_request_complete().into()
    } else {
        HtpStatus::ERROR
    }
}

/// Change transaction state to RESPONSE and invoke registered callbacks.
///
/// tx: Transaction pointer. Must not be NULL.
///
/// Returns OK on success; ERROR on error, HTP_STOP if one of the
///         callbacks does not want to follow the transaction any more.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_state_response_complete(tx: *mut Transaction) -> HtpStatus {
    if let Some(tx) = tx.as_mut() {
        tx.state_response_complete().into()
    } else {
        HtpStatus::ERROR
    }
}

/// Register callback for the transaction-specific RESPONSE_BODY_DATA hook.
#[no_mangle]
pub unsafe extern "C" fn htp_tx_register_response_body_data(
    tx: *mut Transaction,
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
pub unsafe extern "C" fn htp_tx_data_tx(data: *mut Data) -> *mut Transaction {
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
pub unsafe extern "C" fn htp_tx_data_data(data: *const Data) -> *const u8 {
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
pub unsafe extern "C" fn htp_tx_data_len(data: *const Data) -> isize {
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
pub unsafe extern "C" fn htp_tx_data_is_last(data: *const Data) -> bool {
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
pub unsafe extern "C" fn htp_tx_data_is_empty(data: *const Data) -> bool {
    if let Some(data) = data.as_ref() {
        data.is_empty()
    } else {
        true
    }
}
