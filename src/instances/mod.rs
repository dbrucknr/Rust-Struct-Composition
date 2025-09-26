pub mod controller;
pub mod repository;
pub mod service;

use std::net::IpAddr;

use crate::instances::controller::instance::IpAddrController;
use crate::instances::repository::instance::IpAddrRepository;
use crate::instances::service::instance::IpAddrService;

pub struct IpModule {
    controller: IpAddrController<IpAddrService<IpAddrRepository>>,
}
impl IpModule {
    pub fn new(ip_addrs: Vec<IpAddr>) -> Self {
        let repository = IpAddrRepository::new(ip_addrs);
        let service = IpAddrService::new(repository);
        let controller = IpAddrController::new(service);

        Self { controller }
    }

    // Borrow to keep module alive:
    pub fn controller(&self) -> &IpAddrController<IpAddrService<IpAddrRepository>> {
        &self.controller
    }

    pub fn controller_mut(&mut self) -> &mut IpAddrController<IpAddrService<IpAddrRepository>> {
        &mut self.controller
    }

    // Move if running an app context:
    pub fn into_controller(self) -> IpAddrController<IpAddrService<IpAddrRepository>> {
        self.controller
    }
}
