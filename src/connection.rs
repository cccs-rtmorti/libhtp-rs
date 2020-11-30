use crate::{
    error::Result,
    list::List,
    log::Logs,
    transaction::{Transaction, Transactions},
    util::ConnectionFlags,
};
use chrono::{DateTime, Utc};
use std::{cell::RefCell, net::IpAddr, time::SystemTime};

pub struct Connection {
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
    transactions: Transactions,
    /// Log messages associated with this connection.
    pub messages: RefCell<Logs>,
    /// Parsing flags: HTP_CONN_PIPELINED.
    pub flags: ConnectionFlags,
    /// When was this connection opened? Can be NULL.
    pub open_timestamp: DateTime<Utc>,
    /// When was this connection closed? Can be NULL.
    pub close_timestamp: DateTime<Utc>,
    /// Inbound data counter.
    pub in_data_counter: i64,
    /// Outbound data counter.
    pub out_data_counter: i64,
}

impl Connection {
    pub fn new() -> Self {
        Self {
            client_addr: None,
            client_port: None,
            server_addr: None,
            server_port: None,
            transactions: List::with_capacity(16),
            messages: RefCell::new(List::with_capacity(8)),
            flags: ConnectionFlags::UNKNOWN,
            open_timestamp: DateTime::<Utc>::from(SystemTime::now()),
            close_timestamp: DateTime::<Utc>::from(SystemTime::now()),
            in_data_counter: 0,
            out_data_counter: 0,
        }
    }

    /// Push a transaction to this connection's tx list.
    pub fn push_tx(&mut self, tx: Transaction) {
        self.transactions.push(tx)
    }

    /// Remove a transaction from this connection's tx list.
    pub fn remove_tx(&mut self, tx_id: usize) -> Result<()> {
        self.transactions.remove(tx_id)
    }

    /// Get the transactions for this connection.
    pub fn txs(&self) -> &Transactions {
        &self.transactions
    }

    /// Get the transactions for this connection as a mutable reference.
    pub fn txs_mut(&mut self) -> &mut Transactions {
        &mut self.transactions
    }

    /// Get the number of transactions in this connection.
    pub fn tx_size(&self) -> usize {
        self.transactions.len()
    }

    /// Get a transaction by tx_id from this connection.
    pub fn tx(&self, tx_id: usize) -> Option<&Transaction> {
        self.transactions.get(tx_id)
    }

    /// Get a transaction by tx_id from this connection as a pointer.
    pub fn tx_ptr(&self, tx_id: usize) -> *const Transaction {
        self.transactions
            .get(tx_id)
            .map(|tx| tx as *const Transaction)
            .unwrap_or(std::ptr::null())
    }

    /// Get a transaction by tx_id from this connection as a mutable reference.
    pub fn tx_mut(&mut self, tx_id: usize) -> Option<&mut Transaction> {
        self.transactions.get_mut(tx_id)
    }

    /// Get a transaction by tx_id from this connection as a mutable pointer.
    pub fn tx_mut_ptr(&mut self, tx_id: usize) -> *mut Transaction {
        self.transactions
            .get_mut(tx_id)
            .map(|tx| tx as *mut Transaction)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Opens a connection. This function will essentially only store the provided data
    /// for future reference. The timestamp parameter is optional.
    pub fn open(
        &mut self,
        client_addr: Option<IpAddr>,
        client_port: Option<u16>,
        server_addr: Option<IpAddr>,
        server_port: Option<u16>,
        timestamp: Option<DateTime<Utc>>,
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
    pub fn close(&mut self, timestamp: Option<DateTime<Utc>>) {
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

impl PartialEq for Connection {
    fn eq(&self, rhs: &Self) -> bool {
        self.client_addr == rhs.client_addr
            && self.client_port == rhs.client_port
            && self.server_addr == rhs.server_addr
            && self.server_port == rhs.server_port
    }
}
