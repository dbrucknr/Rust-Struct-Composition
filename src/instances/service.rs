use std::net::IpAddr;

use crate::traits::repository::Repository;
use crate::traits::service::Service;

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
impl<R: Repository<Item = IpAddr>> Service for IpAddrService<R> {
    type Item = IpAddr;
    type Repo = R;

    fn repo(&self) -> &Self::Repo {
        &self.repo
    }

    fn repo_mut(&mut self) -> &mut Self::Repo {
        &mut self.repo
    }

    // Can add service specific logic over-writing here...
}
