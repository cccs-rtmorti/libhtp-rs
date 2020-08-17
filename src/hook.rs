use crate::htp_transaction::{htp_tx_data_t, htp_tx_t};
use crate::htp_util::htp_file_data_t;
use crate::log::htp_log_t;
use crate::Status;

/// External (C) callback function prototype
pub type TxExternalCallbackFn = unsafe extern "C" fn(tx: *mut htp_tx_t) -> Status;

/// Native (rust) callback function prototype
pub type TxNativeCallbackFn = fn(tx: *mut htp_tx_t) -> Status;

/// Hook for htp_tx_t
pub type TxHook = Hook<TxExternalCallbackFn, TxNativeCallbackFn>;

/// External (C) callback function prototype
pub type DataExternalCallbackFn = unsafe extern "C" fn(data: *mut htp_tx_data_t) -> Status;

/// Native (rust) callback function prototype
pub type DataNativeCallbackFn = fn(data: *mut htp_tx_data_t) -> Status;

/// Hook for htp_tx_data_t
pub type DataHook = Hook<DataExternalCallbackFn, DataNativeCallbackFn>;

/// External (C) callback function prototype
pub type FileDataExternalCallbackFn = unsafe extern "C" fn(data: *mut htp_file_data_t) -> Status;

/// Native (rust) callback function prototype
pub type FileDataNativeCallbackFn = fn(data: *mut htp_file_data_t) -> Status;

/// Hook for htp_tx_filedata_t
pub type FileDataHook = Hook<FileDataExternalCallbackFn, FileDataNativeCallbackFn>;

/// External (C) callback function prototype
pub type LogExternalCallbackFn = unsafe extern "C" fn(log: *mut htp_log_t) -> Status;

/// Native (rust) callback function prototype
pub type LogNativeCallbackFn = fn(log: *mut htp_log_t) -> Status;

/// Hook for htp_log_t
pub type LogHook = Hook<LogExternalCallbackFn, LogNativeCallbackFn>;

/// Callback list
#[derive(Clone)]
pub struct Hook<E, N> {
    pub callbacks: Vec<Callback<E, N>>,
}

impl<E, N> Hook<E, N> {
    /// Create a new callback list
    pub fn new() -> Self {
        Hook {
            callbacks: Vec::new(),
        }
    }

    /// Register a native (rust) callback function
    pub fn register(&mut self, cbk_fn: N) {
        self.callbacks.push(Callback::Native(cbk_fn))
    }

    /// Register an external (C) callback function
    pub fn register_extern(&mut self, cbk_fn: E) {
        self.callbacks.push(Callback::External(cbk_fn))
    }
}

impl TxHook {
    /// Run all callbacks on the list
    ///
    /// This function will exit early if a callback fails to return Status::OK
    /// or Status::DECLINED.
    pub fn run_all(&self, data: *mut htp_tx_t) -> Status {
        for cbk_fn in &self.callbacks {
            let result = match cbk_fn {
                Callback::External(cbk_fn) => unsafe { cbk_fn(data) },
                Callback::Native(cbk_fn) => cbk_fn(data),
            };

            if result != Status::OK && result != Status::DECLINED {
                return result;
            }
        }
        Status::OK
    }
}

impl DataHook {
    /// Run all callbacks on the list
    ///
    /// This function will exit early if a callback fails to return Status::OK
    /// or Status::DECLINED.
    pub fn run_all(&self, data: *mut htp_tx_data_t) -> Status {
        for cbk_fn in &self.callbacks {
            let result = match cbk_fn {
                Callback::External(cbk_fn) => unsafe { cbk_fn(data) },
                Callback::Native(cbk_fn) => cbk_fn(data),
            };

            if result != Status::OK && result != Status::DECLINED {
                return result;
            }
        }
        Status::OK
    }
}

impl FileDataHook {
    /// Run all callbacks on the list
    ///
    /// This function will exit early if a callback fails to return Status::OK
    /// or Status::DECLINED.
    pub fn run_all(&self, data: *mut htp_file_data_t) -> Status {
        for cbk_fn in &self.callbacks {
            let result = match cbk_fn {
                Callback::External(cbk_fn) => unsafe { cbk_fn(data) },
                Callback::Native(cbk_fn) => cbk_fn(data),
            };

            if result != Status::OK && result != Status::DECLINED {
                return result;
            }
        }
        Status::OK
    }
}

impl LogHook {
    /// Run all callbacks on the list
    ///
    /// This function will exit early if a callback fails to return Status::OK
    /// or Status::DECLINED.
    pub fn run_all(&self, log: *mut htp_log_t) -> Status {
        for cbk_fn in &self.callbacks {
            let result = match cbk_fn {
                Callback::External(cbk_fn) => unsafe { cbk_fn(log) },
                Callback::Native(cbk_fn) => cbk_fn(log),
            };

            if result != Status::OK && result != Status::DECLINED {
                return result;
            }
        }
        Status::OK
    }
}

/// Type of callbacks
#[derive(Copy, Clone)]
pub enum Callback<E, N> {
    /// External (C) callback function
    External(E),
    /// Native (rust) callback function
    Native(N),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_callback() {
        unsafe extern "C" fn foo(_: *mut htp_tx_data_t) -> Status {
            Status::OK
        }
        let mut hook = DataHook::new();
        let mut data = htp_tx_data_t::new(std::ptr::null_mut(), std::ptr::null_mut(), 0, false);

        hook.register(|_| Status::OK);
        hook.register_extern(foo);

        assert_eq!(hook.run_all(&mut data), Status::OK);
    }
}
