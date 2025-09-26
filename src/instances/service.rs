use std::{error::Error, fmt, net::IpAddr};

use crate::traits::repository::Repository;
use crate::traits::service::Service;

use crate::instances::repository::IpAddrRepoError;

// Service Error Enum
#[derive(Debug)]
pub enum IpAddrServiceError {
    Repo(IpAddrRepoError),
}
impl fmt::Display for IpAddrServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

// Base IpAddrService Struct Defintion
pub struct IpAddrService<R: Repository<Item = IpAddr>> {
    repo: R,
}
// Struct Specific Methods
impl<R: Repository<Item = IpAddr>> IpAddrService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}
// Implement Service Trait for IpAddrService
impl<R> Service for IpAddrService<R>
where
    R: Repository<Item = IpAddr, RepoError = IpAddrRepoError>,
{
    type Item = IpAddr;
    type Repo = R;
    type ServiceError = IpAddrServiceError;

    fn repo(&self) -> &Self::Repo {
        &self.repo
    }
    fn repo_mut(&mut self) -> &mut Self::Repo {
        &mut self.repo
    }
}
