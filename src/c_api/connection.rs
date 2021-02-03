#![deny(missing_docs)]
use crate::{connection::Connection, log::Log, transaction::Transaction};
use std::convert::TryFrom;

/// Get the number of transactions in a connection
///
/// Returns the number of transactions or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_conn_tx_size(conn: *const Connection) -> isize {
    conn.as_ref()
        .and_then(|conn| isize::try_from(conn.tx_size()).ok())
        .unwrap_or(-1)
}

/// Get a transaction in a connection.
///
/// Returns the transaction or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_conn_tx(conn: *mut Connection, tx_id: usize) -> *mut Transaction {
    conn.as_mut()
        .map(|conn| conn.tx_mut_ptr(tx_id))
        .unwrap_or(std::ptr::null_mut())
}

/// Returns the in_data_counter
#[no_mangle]
pub unsafe extern "C" fn htp_conn_in_data_counter(conn: *const Connection) -> i64 {
    conn.as_ref().map(|conn| conn.in_data_counter).unwrap_or(0)
}

/// Returns the out_data_counter
#[no_mangle]
pub unsafe extern "C" fn htp_conn_out_data_counter(conn: *const Connection) -> i64 {
    conn.as_ref().map(|conn| conn.out_data_counter).unwrap_or(0)
}

/// Get the next logged message from the connection
///
/// Returns the next log or NULL on error.
/// The caller must free this result with htp_log_free
#[no_mangle]
pub unsafe extern "C" fn htp_conn_next_log(conn: *const Connection) -> *mut Log {
    conn.as_ref()
        .and_then(|conn| conn.get_next_log())
        .map(|log| Box::into_raw(Box::new(log)))
        .unwrap_or(std::ptr::null_mut())
}
