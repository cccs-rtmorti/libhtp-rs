use crate::{htp_list, htp_transaction, htp_util, Status};
use ::libc;

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_conn_t {
    /// Client IP address.
    pub client_addr: *mut libc::c_char,
    /// Client port.
    pub client_port: libc::c_int,
    /// Server IP address.
    pub server_addr: *mut libc::c_char,
    /// Server port.
    pub server_port: libc::c_int,

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
    pub in_data_counter: int64_t,
    /// Outbound data counter.
    pub out_data_counter: int64_t,
}
pub type htp_time_t = libc::timeval;

/// Creates a new connection structure.
///
/// Returns A new connection structure on success, NULL on memory allocation failure.
pub unsafe fn htp_conn_create() -> *mut htp_conn_t {
    let mut conn: *mut htp_conn_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_conn_t>() as libc::c_ulong,
    ) as *mut htp_conn_t;
    if conn.is_null() {
        return 0 as *mut htp_conn_t;
    }
    (*conn).transactions = htp_list::htp_list_array_create(16 as libc::c_int as size_t);
    if (*conn).transactions.is_null() {
        free(conn as *mut libc::c_void);
        return 0 as *mut htp_conn_t;
    }
    (*conn).messages = htp_list::htp_list_array_create(8 as libc::c_int as size_t);
    if (*conn).messages.is_null() {
        htp_list::htp_list_array_destroy((*conn).transactions);
        (*conn).transactions = 0 as *mut htp_list::htp_list_array_t;
        free(conn as *mut libc::c_void);
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
            &mut (*conn).close_timestamp as *mut htp_time_t as *mut libc::c_void,
            timestamp as *const libc::c_void,
            ::std::mem::size_of::<htp_time_t>() as libc::c_ulong,
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
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_list::htp_list_array_size((*conn).transactions);
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
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        let mut n_0: size_t = htp_list::htp_list_array_size((*conn).messages);
        while i_0 < n_0 {
            let mut l: *mut htp_util::htp_log_t =
                htp_list::htp_list_array_get((*conn).messages, i_0) as *mut htp_util::htp_log_t;
            free((*l).msg as *mut libc::c_void);
            free(l as *mut libc::c_void);
            i_0 = i_0.wrapping_add(1)
        }
        htp_list::htp_list_array_destroy((*conn).messages);
        (*conn).messages = 0 as *mut htp_list::htp_list_array_t
    }
    if !(*conn).server_addr.is_null() {
        free((*conn).server_addr as *mut libc::c_void);
    }
    if !(*conn).client_addr.is_null() {
        free((*conn).client_addr as *mut libc::c_void);
    }
    free(conn as *mut libc::c_void);
}

/// Opens a connection. This function will essentially only store the provided data
/// for future reference. The timestamp parameter is optional.
pub unsafe fn htp_conn_open(
    mut conn: *mut htp_conn_t,
    mut client_addr: *const libc::c_char,
    mut client_port: libc::c_int,
    mut server_addr: *const libc::c_char,
    mut server_port: libc::c_int,
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
                free((*conn).client_addr as *mut libc::c_void);
            }
            return Status::ERROR;
        }
    }
    (*conn).server_port = server_port;
    // Remember when the connection was opened.
    if !timestamp.is_null() {
        memcpy(
            &mut (*conn).open_timestamp as *mut htp_time_t as *mut libc::c_void,
            timestamp as *const libc::c_void,
            ::std::mem::size_of::<htp_time_t>() as libc::c_ulong,
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
        0 as *mut libc::c_void,
    );
}

/// Keeps track of inbound packets and data.
pub unsafe fn htp_conn_track_inbound_data(
    mut conn: *mut htp_conn_t,
    mut len: size_t,
    mut _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).in_data_counter =
        ((*conn).in_data_counter as libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
}

/// Keeps track of outbound packets and data.
pub unsafe fn htp_conn_track_outbound_data(
    mut conn: *mut htp_conn_t,
    mut len: size_t,
    mut _timestamp: *const htp_time_t,
) {
    if conn.is_null() {
        return;
    }
    (*conn).out_data_counter =
        ((*conn).out_data_counter as libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
}
