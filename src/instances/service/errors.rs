use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::instances::repository::errors::IpAddrRepoError;

// Service Error Enum
#[derive(Debug)]
pub enum IpAddrServiceError {
    Repo(IpAddrRepoError),
}
impl Display for IpAddrServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            IpAddrServiceError::Repo(ip) => write!(f, "IP not found: {ip}"),
        }
    }
}
impl Error for IpAddrServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IpAddrServiceError::Repo(e) => Some(e),
            // _ => None,
        }
    }
}
// Implement From here because of the Service trait's
// requirement of type ServiceError: From<<Self::Repo as Repository>::RepoError> + Error;
// This guarantees the error translation.
impl From<IpAddrRepoError> for IpAddrServiceError {
    fn from(e: IpAddrRepoError) -> Self {
        IpAddrServiceError::Repo(e)
    }
}
