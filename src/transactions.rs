use crate::{config::Config, log::Logger, transaction::Transaction};
use std::collections::BTreeMap;
use std::rc::Rc;

/// Transaction is a structure which tracks request and response
/// transactions, and guarantees that the current request or
/// response transaction will always exist.
pub struct Transactions {
    config: Rc<Config>,
    logger: Logger,
    request: usize,
    response: usize,
    transactions: BTreeMap<usize, Transaction>,
}

impl Transactions {
    /// Make a new Transactions struct with the given config
    pub fn new(cfg: &Rc<Config>, logger: &Logger) -> Self {
        Self {
            config: Rc::clone(cfg),
            logger: logger.clone(),
            request: 0,
            response: 0,
            transactions: BTreeMap::default(),
        }
    }

    /// Return the number of transactions processed.
    /// The value returned may wrap around if the number of transactions
    /// exceeds the storage size available to `usize`.
    pub fn size(&self) -> usize {
        // The total number of transactions is just the maximum
        // of the request or response transaction index + 1 (if
        // that transaction is started), or zero if neither
        // request or response transaction exist yet
        match (
            self.transactions.get(&self.request),
            self.transactions.get(&self.response),
        ) {
            (Some(req), Some(res)) => {
                if req.index > res.index {
                    req.index.wrapping_add(req.is_started() as usize)
                } else {
                    res.index.wrapping_add(res.is_started() as usize)
                }
            }
            (Some(req), None) => req.index.wrapping_add(req.is_started() as usize),
            (None, Some(res)) => res.index.wrapping_add(res.is_started() as usize),
            (None, None) => 0,
        }
    }

    /// Get the current request transaction index
    pub fn request_index(&self) -> usize {
        self.request
    }

    /// Get the current request transaction
    pub fn request(&mut self) -> &Transaction {
        self.request_mut()
    }

    /// Get the current request transaction
    pub fn request_mut(&mut self) -> &mut Transaction {
        let cfg = &self.config;
        let logger = &self.logger;
        let request = self.request;
        self.transactions
            .entry(request)
            .or_insert_with(|| Transaction::new(cfg, logger, request))
    }

    /// Get the current response transaction index
    pub fn response_index(&self) -> usize {
        self.response
    }

    /// Get the current response transaction
    pub fn response(&mut self) -> &Transaction {
        self.response_mut()
    }

    /// Get the current response transaction
    pub fn response_mut(&mut self) -> &mut Transaction {
        let cfg = &self.config;
        let logger = &self.logger;
        let response = self.response;
        self.transactions
            .entry(response)
            .or_insert_with(|| Transaction::new(cfg, logger, response))
    }

    /// Increment the request transaction number.
    /// May cause the previous transaction to be freed if configured to auto-destroy.
    /// Returns the new request transaction index
    pub fn request_next(&mut self) -> usize {
        self.check_free(self.request);
        self.request = self.request.wrapping_add(1);
        self.request
    }

    /// Increment the response transaction number.
    /// May cause the previous transaction to be freed if configured to auto-destroy.
    /// Returns the new response transaction index
    pub fn response_next(&mut self) -> usize {
        self.check_free(self.response);
        self.response = self.response.wrapping_add(1);
        self.response
    }

    /// Check if any old transactions can be freed
    fn check_free(&mut self, index: usize) {
        if self.config.tx_auto_destroy {
            if let Some(tx) = self.transactions.get(&index) {
                if !tx.is_complete() {
                    return;
                }
            }
            self.transactions.remove(&index);
        }
    }

    /// Remove the transaction at the given index. If the transaction
    /// existed, it is returned.
    pub fn remove(&mut self, index: usize) -> Option<Transaction> {
        self.transactions.remove(&index)
    }

    /// Get the given transaction by index number
    pub fn get(&self, index: usize) -> Option<&Transaction> {
        self.transactions.get(&index)
    }

    /// Get the given transaction by index number
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Transaction> {
        self.transactions.get_mut(&index)
    }
}

/// An iterator over Transactions
pub struct TransactionsIterator<'a> {
    iter: std::collections::btree_map::IterMut<'a, usize, Transaction>,
}

impl<'a> Iterator for TransactionsIterator<'a> {
    type Item = &'a mut Transaction;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_index, tx)) = self.iter.next() {
            Some(tx)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a mut Transactions {
    type Item = &'a mut Transaction;
    type IntoIter = TransactionsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TransactionsIterator {
            iter: self.transactions.iter_mut(),
        }
    }
}
