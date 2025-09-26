use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::instances::service::errors::IpAddrServiceError;

#[derive(Debug)]
pub enum IpControllerError {
    Service(IpAddrServiceError),
    // BadParam(&'static str),
}
impl Display for IpControllerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            IpControllerError::Service(e) => write!(f, "service error: {e}"),
            // IpControllerError::BadParam(msg) => write!(f, "bad parameter: {msg}"),
        }
    }
}
impl Error for IpControllerError {}
// Ensure translation between the error types:
impl From<IpAddrServiceError> for IpControllerError {
    fn from(e: IpAddrServiceError) -> Self {
        IpControllerError::Service(e)
    }
}
