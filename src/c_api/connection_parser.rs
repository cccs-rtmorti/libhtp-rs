#![deny(missing_docs)]
use crate::{
    config::Config,
    connection::Connection,
    connection_parser::{ConnectionParser, HtpStreamState, ParserData},
    transaction::Transaction,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::{
    convert::{TryFrom, TryInto},
    ffi::CStr,
};

/// Closes the connection associated with the supplied parser.
///
/// timestamp is optional
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_close(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
) {
    if let Some(connp) = connp.as_mut() {
        connp.close(
            timestamp
                .as_ref()
                .map(|val| {
                    NaiveDateTime::from_timestamp_opt(val.tv_sec as i64, val.tv_usec as u32)
                        .map(|time| DateTime::<Utc>::from_utc(time, Utc))
                })
                .unwrap_or(None),
        )
    }
}

/// Creates a new connection parser using the provided configuration or a default configuration if NULL provided.
/// Note the provided config will be copied into the created connection parser. Therefore, subsequent modification
/// to the original config will have no effect.
///
/// Returns a new connection parser instance, or NULL on error.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_create(cfg: *mut Config) -> *mut ConnectionParser {
    Box::into_raw(Box::new(ConnectionParser::new(
        cfg.as_ref().cloned().unwrap_or_default(),
    )))
}

/// Destroys the connection parser, its data structures, as well
/// as the connection and its transactions.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_destroy_all(connp: *mut ConnectionParser) {
    drop(Box::from_raw(connp));
}

/// Returns the connection associated with the connection parser.
///
/// Returns Connection instance, or NULL if one is not available.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_connection(connp: *const ConnectionParser) -> *const Connection {
    connp
        .as_ref()
        .map(|val| &val.conn as *const Connection)
        .unwrap_or(std::ptr::null())
}

/// Retrieve the user data associated with this connection parser.
/// Returns user data, or NULL if there isn't any.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_user_data(connp: *const ConnectionParser) -> *mut libc::c_void {
    connp
        .as_ref()
        .and_then(|val| val.user_data::<*mut libc::c_void>())
        .copied()
        .unwrap_or(std::ptr::null_mut())
}

/// Associate user data with the supplied parser.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_set_user_data(
    connp: *mut ConnectionParser,
    user_data: *mut libc::c_void,
) {
    if let Some(connp) = connp.as_mut() {
        connp.set_user_data(Box::new(user_data))
    }
}

/// Opens connection.
///
/// timestamp is optional
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_open(
    connp: *mut ConnectionParser,
    client_addr: *const libc::c_char,
    client_port: libc::c_int,
    server_addr: *const libc::c_char,
    server_port: libc::c_int,
    timestamp: *const libc::timeval,
) {
    if let Some(connp) = connp.as_mut() {
        connp.open(
            client_addr.as_ref().and_then(|client_addr| {
                CStr::from_ptr(client_addr)
                    .to_str()
                    .ok()
                    .and_then(|val| val.parse().ok())
            }),
            client_port.try_into().ok(),
            server_addr.as_ref().and_then(|server_addr| {
                CStr::from_ptr(server_addr)
                    .to_str()
                    .ok()
                    .and_then(|val| val.parse().ok())
            }),
            server_port.try_into().ok(),
            timestamp
                .as_ref()
                .map(|val| {
                    NaiveDateTime::from_timestamp_opt(val.tv_sec as i64, val.tv_usec as u32)
                        .map(|time| DateTime::<Utc>::from_utc(time, Utc))
                })
                .unwrap_or(None),
        )
    }
}

/// Closes the connection associated with the supplied parser.
///
/// timestamp is optional
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_request_close(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
) {
    if let Some(connp) = connp.as_mut() {
        connp.request_close(
            timestamp
                .as_ref()
                .map(|val| {
                    NaiveDateTime::from_timestamp_opt(val.tv_sec as i64, val.tv_usec as u32)
                        .map(|time| DateTime::<Utc>::from_utc(time, Utc))
                })
                .unwrap_or(None),
        )
    }
}

/// Process a chunk of inbound client request data
///
/// timestamp is optional
/// Returns HTP_STREAM_STATE_DATA, HTP_STREAM_STATE_ERROR or HTP_STREAM_STATE_DATA_OTHER (see QUICK_START).
///         HTP_STREAM_STATE_CLOSED and HTP_STREAM_STATE_TUNNEL are also possible.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_request_data(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
    data: *const libc::c_void,
    len: libc::size_t,
) -> HtpStreamState {
    connp
        .as_mut()
        .map(|connp| {
            connp.request_data(
                ParserData::from((data as *const u8, len)),
                timestamp
                    .as_ref()
                    .map(|val| {
                        NaiveDateTime::from_timestamp_opt(val.tv_sec as i64, val.tv_usec as u32)
                            .map(|time| DateTime::<Utc>::from_utc(time, Utc))
                    })
                    .unwrap_or(None),
            )
        })
        .unwrap_or(HtpStreamState::ERROR)
}

/// Process a chunk of outbound (server or response) data.
///
/// timestamp is optional.
/// Returns HTP_STREAM_STATE_OK on state change, HTP_STREAM_STATE_ERROR on error, or HTP_STREAM_STATE_DATA when more data is needed
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_response_data(
    connp: *mut ConnectionParser,
    timestamp: *const libc::timeval,
    data: *const libc::c_void,
    len: libc::size_t,
) -> HtpStreamState {
    connp
        .as_mut()
        .map(|connp| {
            connp.response_data(
                ParserData::from((data as *const u8, len)),
                timestamp
                    .as_ref()
                    .map(|val| {
                        NaiveDateTime::from_timestamp_opt(val.tv_sec as i64, val.tv_usec as u32)
                            .map(|time| DateTime::<Utc>::from_utc(time, Utc))
                    })
                    .unwrap_or(None),
            )
        })
        .unwrap_or(HtpStreamState::ERROR)
}

/// Get the number of transactions processed on this connection.
///
/// Returns the number of transactions or -1 on error.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_tx_size(connp: *const ConnectionParser) -> isize {
    connp
        .as_ref()
        .map(|connp| isize::try_from(connp.tx_size()).unwrap_or(-1))
        .unwrap_or(-1)
}

/// Get a transaction.
///
/// Returns the transaction or NULL on error.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_tx(
    connp: *mut ConnectionParser,
    tx_id: usize,
) -> *const Transaction {
    connp
        .as_ref()
        .map(|connp| {
            connp
                .tx(tx_id)
                .map(|tx| {
                    if tx.is_started() {
                        tx as *const Transaction
                    } else {
                        std::ptr::null()
                    }
                })
                .unwrap_or(std::ptr::null())
        })
        .unwrap_or(std::ptr::null())
}

/// Retrieves the pointer to the active response transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Can be NULL.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_response_tx(
    connp: *mut ConnectionParser,
) -> *const Transaction {
    connp
        .as_mut()
        .map(|connp| connp.response() as *const Transaction)
        .unwrap_or(std::ptr::null())
}

/// Retrieves the pointer to the active request transaction. In connection
/// parsing mode there can be many open transactions, and up to 2 active
/// transactions at any one time. This is due to HTTP pipelining. Call be NULL.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_get_request_tx(
    connp: *mut ConnectionParser,
) -> *const Transaction {
    connp
        .as_mut()
        .map(|connp| connp.request() as *const Transaction)
        .unwrap_or(std::ptr::null())
}

/// Invoke the transaction complete callback for each incomplete transaction.
/// The transactions passed to the callback will not have their request and
/// response state set to complete - they will simply be passed with the state
/// they have within the parser at the time of the call.
///
/// This function is intended to be used when a connection is closing and we want
/// to process any incomplete transactions that were in flight, or which never
/// completed due to packet loss or parsing errors.
///
/// This function will also cause these transactions to be removed from the parser.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_flush_incomplete_transactions(connp: *mut ConnectionParser) {
    if let Some(connp) = connp.as_mut() {
        connp.flush_incomplete_transactions()
    }
}

/// Returns the number of bytes consumed from the current data chunks so far or -1 on error.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_request_data_consumed(connp: *const ConnectionParser) -> i64 {
    connp
        .as_ref()
        .map(|connp| connp.request_data_consumed().try_into().ok().unwrap_or(-1))
        .unwrap_or(-1)
}

/// Returns the number of bytes consumed from the most recent outbound data chunk. Normally, an invocation
/// of htp_connp_response_data() will consume all data from the supplied buffer, but there are circumstances
/// where only partial consumption is possible. In such cases HTP_STREAM_DATA_OTHER will be returned.
/// Consumed bytes are no longer necessary, but the remainder of the buffer will be need to be saved
/// for later.
/// Returns the number of bytes consumed from the last data chunk sent for outbound processing
/// or -1 on error.
/// # Safety
/// When calling this method, you have to ensure that connp is either properly initialized or NULL
#[no_mangle]
pub unsafe extern "C" fn htp_connp_response_data_consumed(connp: *const ConnectionParser) -> i64 {
    connp
        .as_ref()
        .map(|connp| connp.response_data_consumed().try_into().ok().unwrap_or(-1))
        .unwrap_or(-1)
}
