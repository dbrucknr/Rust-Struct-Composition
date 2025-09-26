use std::{mem::replace, net::IpAddr};

use crate::traits::repository::Repository;

pub struct IpAddrRepository {
    resources: Vec<IpAddr>,
}
impl IpAddrRepository {
    pub fn new(resources: Vec<IpAddr>) -> Self {
        Self { resources }
    }
}
impl Repository for IpAddrRepository {
    type Item = IpAddr;

    fn all(&self) -> &[Self::Item] {
        &self.resources
    }

    fn one(&self, ip: &IpAddr) -> Option<&IpAddr> {
        self.resources.iter().find(|addr| *addr == ip)
    }

    fn remove(&mut self, ip: &IpAddr) -> Option<IpAddr> {
        if let Some(index) = self.resources.iter().position(|addr| addr == ip) {
            Some(self.resources.remove(index))
        } else {
            None
        }
    }

    fn update(&mut self, original: &IpAddr, updated: IpAddr) -> Option<IpAddr> {
        if let Some(index) = self.resources.iter().position(|addr| addr == original) {
            Some(replace(&mut self.resources[index], updated))
        } else {
            None
        }
    }
}
