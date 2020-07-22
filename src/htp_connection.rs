use crate::{htp_transaction, htp_util, list::List, log::htp_logs_free, Status};

extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
    #[no_mangle]
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: libc::size_t,
    ) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}

pub type htp_time_t = libc::timeval;

pub struct htp_conn_t {
    /// Client IP address.
    pub client_addr: *mut i8,
    /// Client port.
    pub client_port: i32,
    /// Server IP address.
    pub server_addr: *mut i8,
    /// Server port.
    pub server_port: i32,

    /// Transactions carried out on this connection. The list may contain
    /// NULL elements when some of the transactions are deleted (and then
    /// removed from a connection by calling htp_conn_remove_tx().
    pub transactions: List<*mut core::ffi::c_void>,
    /// Log messages associated with this connection.
    pub messages: List<*mut core::ffi::c_void>,
    /// Parsing flags: HTP_CONN_PIPELINED.
    pub flags: htp_util::ConnectionFlags,
    /// When was this connection opened? Can be NULL.
    pub open_timestamp: htp_time_t,
    /// When was this connection closed? Can be NULL.
    pub close_timestamp: htp_time_t,
    /// Inbound data counter.
    pub in_data_counter: i64,
    /// Outbound data counter.
    pub out_data_counter: i64,
}

impl htp_conn_t {
    pub fn new() -> Self {
        Self {
            client_addr: std::ptr::null_mut(),
            client_port: 0,
            server_addr: std::ptr::null_mut(),
            server_port: 0,
            transactions: List::with_capacity(16),
            messages: List::with_capacity(8),
            flags: htp_util::ConnectionFlags::HTP_CONN_UNKNOWN,
            open_timestamp: htp_time_t {
                tv_sec: 0,
                tv_usec: 0,
            },
            close_timestamp: htp_time_t {
                tv_sec: 0,
                tv_usec: 0,
            },
            in_data_counter: 0,
            out_data_counter: 0,
        }
    }
}

/// Creates a new connection structure.
///
/// Returns A new connection structure on success, NULL on memory allocation failure.
pub fn htp_conn_create() -> *mut htp_conn_t {
    Box::into_raw(Box::new(htp_conn_t::new()))
}

/// Closes the connection.
pub unsafe fn htp_conn_close(conn: *mut htp_conn_t, timestamp: *const htp_time_t) {
    if conn.is_null() {
        return;
    }
    // Update timestamp.
    if !timestamp.is_null() {
        memcpy(
            &mut (*conn).close_timestamp as *mut htp_time_t as *mut core::ffi::c_void,
            timestamp as *const core::ffi::c_void,
            ::std::mem::size_of::<htp_time_t>(),
        );
    };
}

/// Destroys a connection, as well as all the transactions it contains. It is
/// not possible to destroy a connection structure yet leave any of its
/// transactions intact. This is because transactions need its connection and
/// connection structures hold little data anyway. The opposite is true, though
/// it is possible to delete a transaction but leave its connection alive.
pub unsafe fn htp_conn_destroy(conn: *mut htp_conn_t) {
    if conn.is_null() {
        return;
    }

    // retake ownership of the connection
    let conn = Box::from_raw(conn);

    // Destroy individual transactions. Do note that iterating
    // using the iterator does not work here because some of the
    // list element may be NULL (and with the iterator it is impossible
    // to distinguish a NULL element from the end of the list).
    for tx in &conn.transactions {
        if !tx.is_null() {
            htp_transaction::htp_tx_destroy_incomplete(*tx as *mut htp_transaction::htp_tx_t);
        }
    }

    htp_logs_free(&conn.messages);

    if !(*conn).server_addr.is_null() {
        free((*conn).server_addr as *mut core::ffi::c_void);
    }
    if !(*conn).client_addr.is_null() {
        free((*conn).client_addr as *mut core::ffi::c_void);
    }
}

/// Opens a connection. This function will essentially only store the provided data
/// for future reference. The timestamp parameter is optional.
pub unsafe fn htp_conn_open(
    mut conn: *mut htp_conn_t,
    client_addr: *const i8,
    client_port: i32,
    server_addr: *const i8,
    server_port: i32,
    timestamp: *const htp_time_t,
) -> Status {
    if conn.is_null() {
        return Status::ERROR;
    }
    if !client_addr.is_null() {
        (*conn).client_addr = strdup(client_addr);
        if (*conn).client_addr.is_null() {
            return Status::ERROR;
        }
    }
    (*conn).client_port = client_port;
    if !server_addr.is_null() {
        (*conn).server_addr = strdup(server_addr);
        if (*conn).server_addr.is_null() {
            if !(*conn).client_addr.is_null() {
                free((*conn).client_addr as *mut core::ffi::c_void);
            }
            return Status::ERROR;
        }
    }
    (*conn).server_port = server_port;
    // Remember when the connection was opened.
    if !timestamp.is_null() {
        memcpy(
            &mut (*conn).open_timestamp as *mut htp_time_t as *mut core::ffi::c_void,
            timestamp as *const core::ffi::c_void,
            ::std::mem::size_of::<htp_time_t>(),
        );
    }
    Status::OK
}

/// Removes the given transaction structure, which makes it possible to
/// safely destroy it. It is safe to destroy transactions in this way
/// because the index of the transactions (in a connection) is preserved.
///
/// Returns HTP_OK if transaction was removed (replaced with NULL) or HTP_ERROR if it wasn't found.
pub unsafe fn htp_conn_remove_tx(
    conn: *mut htp_conn_t,
    tx: *const htp_transaction::htp_tx_t,
) -> Result<(), Status> {
    if tx.is_null() || conn.is_null() {
        Err(Status::ERROR)
    } else {
        (*conn).transactions.remove((*tx).index)
    }
}

/// Keeps track of inbound packets and data.
pub unsafe fn htp_conn_track_inbound_data(
    mut conn: *mut htp_conn_t,
    len: usize,
    _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).in_data_counter = ((*conn).in_data_counter as u64).wrapping_add(len as u64) as i64;
}

/// Keeps track of outbound packets and data.
pub unsafe fn htp_conn_track_outbound_data(
    mut conn: *mut htp_conn_t,
    len: usize,
    _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).out_data_counter = ((*conn).out_data_counter as u64).wrapping_add(len as u64) as i64;
}
