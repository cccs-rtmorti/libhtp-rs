use crate::{
    error::Result,
    list::List,
    log::{Log, Message},
    transaction::{Transaction, Transactions},
};
use chrono::{DateTime, Utc};
use std::{
    cell::RefCell,
    net::IpAddr,
    sync::mpsc::{channel, Receiver, Sender},
    time::SystemTime,
};

/// Export Connection Flags
pub struct Flags;

/// `Connection` Flags
impl Flags {
    /// Default, no flags raised.
    pub const UNKNOWN: u8 = 0x00;
    /// Seen pipelined requests.
    pub const PIPELINED: u8 = 0x01;
    /// Seen extra data after a HTTP 0.9 communication.
    pub const HTTP_0_9_EXTRA: u8 = 0x02;
}

/// Stores information about the session.
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
    /// None elements when some of the transactions are deleted.
    transactions: Transactions,
    /// Messages channel associated with this connection.
    log_channel: (Sender<Message>, Receiver<Message>),
    /// Log Messages associated with this connection. This is popualted by draining the
    /// receiver of the log_channel by calling get_messages
    messages: RefCell<Vec<Log>>,
    /// Parsing flags.
    pub flags: u8,
    /// When was this connection opened?
    pub open_timestamp: DateTime<Utc>,
    /// When was this connection closed?
    pub close_timestamp: DateTime<Utc>,
    /// Inbound data counter.
    pub in_data_counter: i64,
    /// Outbound data counter.
    pub out_data_counter: i64,
}

impl Connection {
    /// Returns a new Connection instance with default values.
    pub fn new() -> Self {
        Self {
            client_addr: None,
            client_port: None,
            server_addr: None,
            server_port: None,
            transactions: List::with_capacity(16),
            log_channel: channel(),
            messages: RefCell::new(Vec::with_capacity(8)),
            flags: 0,
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
    /// for future reference.
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

    /// Return the log channel sender
    pub fn get_sender(&self) -> &Sender<Message> {
        &self.log_channel.0
    }

    /// Returns all logged messages
    pub fn get_messages(&self) -> &RefCell<Vec<Log>> {
        while let Ok(message) = self.log_channel.1.try_recv() {
            self.messages.borrow_mut().push(Log::new(self, message))
        }
        &self.messages
    }
}

impl PartialEq for Connection {
    /// Returns true if connections are the same, false otherwise.
    fn eq(&self, rhs: &Self) -> bool {
        self.client_addr == rhs.client_addr
            && self.client_port == rhs.client_port
            && self.server_addr == rhs.server_addr
            && self.server_port == rhs.server_port
    }
}
