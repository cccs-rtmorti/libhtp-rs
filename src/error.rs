use crate::HtpStatus;

/// Alias for libhtp Result type. Result types are classified by `HtpStatus`.
pub type Result<T> = std::result::Result<T, HtpStatus>;

impl<T> From<Result<T>> for HtpStatus {
    /// Returns HtpStatus from result.
    fn from(res: Result<T>) -> HtpStatus {
        match res {
            Ok(_) => HtpStatus::OK,
            Err(e) => e,
        }
    }
}

impl From<HtpStatus> for Result<()> {
    /// Returns Result from `HtpStatus`
    fn from(status: HtpStatus) -> Result<()> {
        if status == HtpStatus::OK {
            Ok(())
        } else {
            Err(status)
        }
    }
}

impl From<std::io::Error> for HtpStatus {
    fn from(_: std::io::Error) -> Self {
        HtpStatus::ERROR
    }
}
