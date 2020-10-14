use crate::error::Result;
use crate::{list::List, log, transaction, util};
use std::net::IpAddr;

pub type htp_time_t = libc::timeval;

pub struct htp_conn_t {
    /// Client IP address.
    pub client_addr: Option<IpAddr>,
    /// Client port.
    pub client_port: Option<u16>,
    /// Server IP address.
    pub server_addr: Option<IpAddr>,
    /// Server port.
    pub server_port: Option<u16>,

    /// Transactions carried out on this connection. The list may contain
    /// NULL elements when some of the transactions are deleted (and then
    /// removed from a connection by calling htp_conn_remove_tx().
    transactions: transaction::htp_txs_t,
    /// Log messages associated with this connection.
    messages: log::htp_logs_t,
    /// Parsing flags: HTP_CONN_PIPELINED.
    pub flags: util::ConnectionFlags,
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
            client_addr: None,
            client_port: None,
            server_addr: None,
            server_port: None,
            transactions: List::with_capacity(16),
            messages: List::with_capacity(8),
            flags: util::ConnectionFlags::HTP_CONN_UNKNOWN,
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
    pub fn push_tx(&mut self, tx: transaction::htp_tx_t) {
        self.transactions.push(tx)
    }

    /// Remove a transaction from this connection's tx list.
    pub fn remove_tx(&mut self, tx_id: usize) -> Result<()> {
        self.transactions.remove(tx_id)
    }

    /// Get the transactions for this connection.
    pub fn txs(&self) -> &transaction::htp_txs_t {
        &self.transactions
    }

    /// Get the transactions for this connection as a mutable reference.
    pub fn txs_mut(&mut self) -> &mut transaction::htp_txs_t {
        &mut self.transactions
    }

    /// Get the number of transactions in this connection.
    pub fn tx_size(&self) -> usize {
        self.transactions.len()
    }

    /// Get a transaction by tx_id from this connection.
    pub fn tx(&self, tx_id: usize) -> Option<&transaction::htp_tx_t> {
        self.transactions.get(tx_id)
    }

    /// Get a transaction by tx_id from this connection as a pointer.
    pub fn tx_ptr(&self, tx_id: usize) -> *const transaction::htp_tx_t {
        self.transactions
            .get(tx_id)
            .map(|tx| tx as *const transaction::htp_tx_t)
            .unwrap_or(std::ptr::null())
    }

    /// Get a transaction by tx_id from this connection as a mutable reference.
    pub fn tx_mut(&mut self, tx_id: usize) -> Option<&mut transaction::htp_tx_t> {
        self.transactions.get_mut(tx_id)
    }

    /// Get a transaction by tx_id from this connection as a mutable pointer.
    pub fn tx_mut_ptr(&mut self, tx_id: usize) -> *mut transaction::htp_tx_t {
        self.transactions
            .get_mut(tx_id)
            .map(|tx| tx as *mut transaction::htp_tx_t)
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

    /// Opens a connection. This function will essentially only store the provided data
    /// for future reference. The timestamp parameter is optional.
    pub fn open(
        &mut self,
        client_addr: Option<IpAddr>,
        client_port: Option<u16>,
        server_addr: Option<IpAddr>,
        server_port: Option<u16>,
        timestamp: Option<htp_time_t>,
    ) {
        self.client_addr = client_addr;
        self.client_port = client_port;
        self.server_addr = server_addr;
        self.server_port = server_port;

        // Remember when the connection was opened.
        if let Some(timestamp) = timestamp {
            self.open_timestamp = timestamp;
        }
    }

    /// Closes the connection.
    pub fn close(&mut self, timestamp: Option<htp_time_t>) {
        // Update timestamp.
        if let Some(timestamp) = timestamp {
            self.close_timestamp = timestamp;
        }
    }

    /// Keeps track of inbound packets and data.
    pub fn track_inbound_data(&mut self, len: usize) {
        self.in_data_counter = (self.in_data_counter as u64).wrapping_add(len as u64) as i64;
    }

    /// Keeps track of outbound packets and data.
    pub fn track_outbound_data(&mut self, len: usize) {
        self.out_data_counter = (self.out_data_counter as u64).wrapping_add(len as u64) as i64;
    }
}

impl PartialEq for htp_conn_t {
    fn eq(&self, rhs: &Self) -> bool {
        self.client_addr == rhs.client_addr
            && self.client_port == rhs.client_port
            && self.server_addr == rhs.server_addr
            && self.server_port == rhs.server_port
    }
}
