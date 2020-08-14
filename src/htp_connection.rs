use crate::{htp_transaction, htp_util, list::List, log, Status};

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
    transactions: htp_transaction::htp_txs_t,
    /// Log messages associated with this connection.
    messages: log::htp_logs_t,
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

    /// Push a transaction to this connection's tx list.
    pub fn push_tx(&mut self, tx: htp_transaction::htp_tx_t) {
        self.transactions.push(tx)
    }

    /// Remove a transaction from this connection's tx list.
    pub fn remove_tx(&mut self, tx_id: usize) -> Result<(), Status> {
        self.transactions.remove(tx_id)
    }

    /// Get the transactions for this connection.
    pub fn txs(&self) -> &htp_transaction::htp_txs_t {
        &self.transactions
    }

    /// Get the transactions for this connection as a mutable reference.
    pub fn txs_mut(&mut self) -> &mut htp_transaction::htp_txs_t {
        &mut self.transactions
    }

    /// Get the number of transactions in this connection.
    pub fn tx_size(&self) -> usize {
        self.transactions.len()
    }

    /// Get a transaction by tx_id from this connection.
    pub fn tx(&self, tx_id: usize) -> Option<&htp_transaction::htp_tx_t> {
        self.transactions.get(tx_id)
    }

    /// Get a transaction by tx_id from this connection as a pointer.
    pub fn tx_ptr(&self, tx_id: usize) -> *const htp_transaction::htp_tx_t {
        self.transactions
            .get(tx_id)
            .map(|tx| tx as *const htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null())
    }

    /// Get a transaction by tx_id from this connection as a mutable reference.
    pub fn tx_mut(&mut self, tx_id: usize) -> Option<&mut htp_transaction::htp_tx_t> {
        self.transactions.get_mut(tx_id)
    }

    /// Get a transaction by tx_id from this connection as a mutable pointer.
    pub fn tx_mut_ptr(&mut self, tx_id: usize) -> *mut htp_transaction::htp_tx_t {
        self.transactions
            .get_mut(tx_id)
            .map(|tx| tx as *mut htp_transaction::htp_tx_t)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Push a log message to this connection's log list.
    pub fn push_message(&mut self, log: log::htp_log_t) {
        self.messages.push(log);
    }

    /// Get the log messages for this connection.
    pub fn messages(&self) -> &log::htp_logs_t {
        &self.messages
    }

    /// Get the number of log messages in this connection.
    pub fn message_size(&self) -> usize {
        self.messages.len()
    }

    /// Get a log message by id from this connection.
    pub fn message(&self, msg_id: usize) -> Option<&log::htp_log_t> {
        self.messages.get(msg_id)
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
