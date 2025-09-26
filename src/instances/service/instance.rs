use std::net::IpAddr;

use crate::traits::repository::Repository;
use crate::traits::service::Service;

use crate::instances::repository::errors::IpAddrRepoError;
use crate::instances::service::errors::IpAddrServiceError;

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

    // We can overwrite the default methods provided by the Service trait.
}
