#![deny(missing_docs)]
use crate::{connection::Connection, log::Log};

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
