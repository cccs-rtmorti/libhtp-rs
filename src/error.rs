use crate::Status;
use std::convert::Into;

pub type Result<T> = std::result::Result<T, Status>;

impl<T> Into<Status> for Result<T> {
    fn into(self) -> Status {
        match self {
            Ok(_) => Status::OK,
            Err(e) => e,
        }
    }
}

impl Into<Result<()>> for Status {
    fn into(self) -> Result<()> {
        if self == Status::OK {
            Ok(())
        } else {
            Err(self)
        }
    }
}
