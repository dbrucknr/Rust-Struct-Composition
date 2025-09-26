use std::{error::Error, fmt, mem::replace, net::IpAddr};

use crate::traits::repository::Repository;

// Repository Error Enum
#[derive(Debug)]
pub enum IpAddrRepoError {
    IpAddrNotFound(IpAddr),
}
impl fmt::Display for IpAddrRepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddrRepoError::IpAddrNotFound(ip) => write!(f, "IP not found: {ip}"),
        }
    }
}
impl Error for IpAddrRepoError {}

// Base IpAddrRepository Struct Definition
pub struct IpAddrRepository {
    resources: Vec<IpAddr>,
}
// IpAddrRepository Struct Methods
impl IpAddrRepository {
    pub fn new(resources: Vec<IpAddr>) -> Self {
        Self { resources }
    }
}
// Implement Repository Trair for IpAddrRepository
impl Repository for IpAddrRepository {
    type Item = IpAddr;
    type RepoError = IpAddrRepoError;

    fn all(&self) -> Result<&[Self::Item], Self::RepoError> {
        Ok(&self.resources)
    }

    fn one(&self, ip: &IpAddr) -> Result<&Self::Item, Self::RepoError> {
        if let Some(addr) = self.resources.iter().find(|addr| *addr == ip) {
            Ok(addr)
        } else {
            Err(IpAddrRepoError::IpAddrNotFound(*ip))
        }
    }

    fn remove(&mut self, ip: &IpAddr) -> Result<Self::Item, Self::RepoError> {
        if let Some(index) = self.resources.iter().position(|addr| addr == ip) {
            Ok(self.resources.remove(index))
        } else {
            Err(IpAddrRepoError::IpAddrNotFound(*ip))
        }
    }

    fn update(
        &mut self,
        original: &IpAddr,
        updated: IpAddr,
    ) -> Result<Self::Item, Self::RepoError> {
        if let Some(index) = self.resources.iter().position(|addr| addr == original) {
            Ok(replace(&mut self.resources[index], updated))
        } else {
            Err(IpAddrRepoError::IpAddrNotFound(*original))
        }
    }
}
