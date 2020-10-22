use crate::error::Result;
use crate::log::Log;
use crate::transaction::{Data, Transaction};
use crate::util::FileData;
use crate::Status;

/// External (C) callback function prototype
pub type TxExternalCallbackFn = unsafe extern "C" fn(tx: *mut Transaction) -> Status;

/// Native (rust) callback function prototype
pub type TxNativeCallbackFn = fn(tx: *mut Transaction) -> Result<()>;

/// Hook for Transaction
pub type TxHook = Hook<TxExternalCallbackFn, TxNativeCallbackFn>;

/// External (C) callback function prototype
pub type DataExternalCallbackFn = unsafe extern "C" fn(data: *mut Data) -> Status;

/// Native (rust) callback function prototype
pub type DataNativeCallbackFn = fn(data: *mut Data) -> Result<()>;

/// Hook for Data
pub type DataHook = Hook<DataExternalCallbackFn, DataNativeCallbackFn>;

/// External (C) callback function prototype
pub type FileDataExternalCallbackFn = unsafe extern "C" fn(data: *mut FileData) -> Status;

/// Native (rust) callback function prototype
pub type FileDataNativeCallbackFn = fn(data: *mut FileData) -> Result<()>;

/// Hook for htp_tx_filedata_t
pub type FileDataHook = Hook<FileDataExternalCallbackFn, FileDataNativeCallbackFn>;

/// External (C) callback function prototype
pub type LogExternalCallbackFn = unsafe extern "C" fn(log: *mut Log) -> Status;

/// Native (rust) callback function prototype
pub type LogNativeCallbackFn = fn(log: *mut Log) -> Result<()>;

/// Hook for Log
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
    pub fn run_all(&self, tx: *mut Transaction) -> Result<()> {
        for cbk_fn in &self.callbacks {
            match cbk_fn {
                Callback::External(cbk_fn) => {
                    let result = unsafe { cbk_fn(tx) };
                    if result != Status::OK && result != Status::DECLINED {
                        return Err(result);
                    }
                }
                Callback::Native(cbk_fn) => {
                    if let Err(e) = cbk_fn(tx) {
                        if e != Status::DECLINED {
                            return Err(e);
                        }
                    }
                }
            };
        }
        Ok(())
    }
}

impl DataHook {
    /// Run all callbacks on the list
    ///
    /// This function will exit early if a callback fails to return Status::OK
    /// or Status::DECLINED.
    pub fn run_all(&self, data: *mut Data) -> Result<()> {
        for cbk_fn in &self.callbacks {
            match cbk_fn {
                Callback::External(cbk_fn) => {
                    let result = unsafe { cbk_fn(data) };
                    if result != Status::OK && result != Status::DECLINED {
                        return Err(result);
                    }
                }
                Callback::Native(cbk_fn) => {
                    if let Err(e) = cbk_fn(data) {
                        if e != Status::DECLINED {
                            return Err(e);
                        }
                    }
                }
            };
        }
        Ok(())
    }
}

impl FileDataHook {
    /// Run all callbacks on the list
    ///
    /// This function will exit early if a callback fails to return Status::OK
    /// or Status::DECLINED.
    pub fn run_all(&self, data: *mut FileData) -> Result<()> {
        for cbk_fn in &self.callbacks {
            match cbk_fn {
                Callback::External(cbk_fn) => {
                    let result = unsafe { cbk_fn(data) };
                    if result != Status::OK && result != Status::DECLINED {
                        return Err(result);
                    }
                }
                Callback::Native(cbk_fn) => {
                    if let Err(e) = cbk_fn(data) {
                        if e != Status::DECLINED {
                            return Err(e);
                        }
                    }
                }
            };
        }
        Ok(())
    }
}

impl LogHook {
    /// Run all callbacks on the list
    ///
    /// This function will exit early if a callback fails to return Status::OK
    /// or Status::DECLINED.
    pub fn run_all(&self, log: *mut Log) -> Result<()> {
        for cbk_fn in &self.callbacks {
            match cbk_fn {
                Callback::External(cbk_fn) => {
                    let result = unsafe { cbk_fn(log) };
                    if result != Status::OK && result != Status::DECLINED {
                        return Err(result);
                    }
                }
                Callback::Native(cbk_fn) => {
                    if let Err(e) = cbk_fn(log) {
                        if e != Status::DECLINED {
                            return Err(e);
                        }
                    }
                }
            };
        }
        Ok(())
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
        unsafe extern "C" fn foo(_: *mut Data) -> Status {
            Status::OK
        }
        let mut hook = DataHook::new();
        let mut data = Data::new(std::ptr::null_mut(), None, false);

        hook.register(|_| Ok(()));
        hook.register_extern(foo);

        assert!(hook.run_all(&mut data).is_ok());
    }
}
