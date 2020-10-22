use crate::HtpStatus;
use std::convert::Into;

pub type Result<T> = std::result::Result<T, HtpStatus>;

impl<T> Into<HtpStatus> for Result<T> {
    fn into(self) -> HtpStatus {
        match self {
            Ok(_) => HtpStatus::OK,
            Err(e) => e,
        }
    }
}

impl Into<Result<()>> for HtpStatus {
    fn into(self) -> Result<()> {
        if self == HtpStatus::OK {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl From<std::io::Error> for HtpStatus {
    fn from(_: std::io::Error) -> Self {
        HtpStatus::ERROR
    }
}
