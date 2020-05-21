use crate::{htp_list, htp_transaction, htp_util, Status};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
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

#[repr(C)]
#[derive(Copy, Clone)]
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
    pub transactions: *mut htp_list::htp_list_array_t,
    /// Log messages associated with this connection.
    pub messages: *mut htp_list::htp_list_array_t,
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
pub type htp_time_t = libc::timeval;

/// Creates a new connection structure.
///
/// Returns A new connection structure on success, NULL on memory allocation failure.
pub unsafe fn htp_conn_create() -> *mut htp_conn_t {
    let mut conn: *mut htp_conn_t =
        calloc(1, ::std::mem::size_of::<htp_conn_t>()) as *mut htp_conn_t;
    if conn.is_null() {
        return 0 as *mut htp_conn_t;
    }
    (*conn).transactions = htp_list::htp_list_array_create(16);
    if (*conn).transactions.is_null() {
        free(conn as *mut core::ffi::c_void);
        return 0 as *mut htp_conn_t;
    }
    (*conn).messages = htp_list::htp_list_array_create(8);
    if (*conn).messages.is_null() {
        htp_list::htp_list_array_destroy((*conn).transactions);
        (*conn).transactions = 0 as *mut htp_list::htp_list_array_t;
        free(conn as *mut core::ffi::c_void);
        return 0 as *mut htp_conn_t;
    }
    return conn;
}

/// Closes the connection.
pub unsafe fn htp_conn_close(mut conn: *mut htp_conn_t, mut timestamp: *const htp_time_t) {
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
pub unsafe fn htp_conn_destroy(mut conn: *mut htp_conn_t) {
    if conn.is_null() {
        return;
    }
    if !(*conn).transactions.is_null() {
        // Destroy individual transactions. Do note that iterating
        // using the iterator does not work here because some of the
        // list element may be NULL (and with the iterator it is impossible
        // to distinguish a NULL element from the end of the list).
        let mut i: usize = 0;
        let mut n: usize = htp_list::htp_list_array_size((*conn).transactions);
        while i < n {
            let mut tx: *mut htp_transaction::htp_tx_t =
                htp_list::htp_list_array_get((*conn).transactions, i)
                    as *mut htp_transaction::htp_tx_t;
            if !tx.is_null() {
                htp_transaction::htp_tx_destroy_incomplete(tx);
            }
            i = i.wrapping_add(1)
        }
        htp_list::htp_list_array_destroy((*conn).transactions);
        (*conn).transactions = 0 as *mut htp_list::htp_list_array_t
    }
    if !(*conn).messages.is_null() {
        // Destroy individual messages.
        let mut i_0: usize = 0;
        let mut n_0: usize = htp_list::htp_list_array_size((*conn).messages);
        while i_0 < n_0 {
            let mut l: *mut htp_util::htp_log_t =
                htp_list::htp_list_array_get((*conn).messages, i_0) as *mut htp_util::htp_log_t;
            free((*l).msg as *mut core::ffi::c_void);
            free(l as *mut core::ffi::c_void);
            i_0 = i_0.wrapping_add(1)
        }
        htp_list::htp_list_array_destroy((*conn).messages);
        (*conn).messages = 0 as *mut htp_list::htp_list_array_t
    }
    if !(*conn).server_addr.is_null() {
        free((*conn).server_addr as *mut core::ffi::c_void);
    }
    if !(*conn).client_addr.is_null() {
        free((*conn).client_addr as *mut core::ffi::c_void);
    }
    free(conn as *mut core::ffi::c_void);
}

/// Opens a connection. This function will essentially only store the provided data
/// for future reference. The timestamp parameter is optional.
pub unsafe fn htp_conn_open(
    mut conn: *mut htp_conn_t,
    mut client_addr: *const i8,
    mut client_port: i32,
    mut server_addr: *const i8,
    mut server_port: i32,
    mut timestamp: *const htp_time_t,
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
    return Status::OK;
}

/// Removes the given transaction structure, which makes it possible to
/// safely destroy it. It is safe to destroy transactions in this way
/// because the index of the transactions (in a connection) is preserved.
///
/// Returns HTP_OK if transaction was removed (replaced with NULL) or HTP_ERROR if it wasn't found.
pub unsafe fn htp_conn_remove_tx(
    mut conn: *mut htp_conn_t,
    mut tx: *const htp_transaction::htp_tx_t,
) -> Status {
    if tx.is_null() || conn.is_null() {
        return Status::ERROR;
    }
    if (*conn).transactions.is_null() {
        return Status::ERROR;
    }
    return htp_list::htp_list_array_replace(
        (*conn).transactions,
        (*tx).index,
        0 as *mut core::ffi::c_void,
    );
}

/// Keeps track of inbound packets and data.
pub unsafe fn htp_conn_track_inbound_data(
    mut conn: *mut htp_conn_t,
    mut len: usize,
    mut _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).in_data_counter = ((*conn).in_data_counter as u64).wrapping_add(len as u64) as i64;
}

/// Keeps track of outbound packets and data.
pub unsafe fn htp_conn_track_outbound_data(
    mut conn: *mut htp_conn_t,
    mut len: usize,
    mut _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).out_data_counter = ((*conn).out_data_counter as u64).wrapping_add(len as u64) as i64;
}
