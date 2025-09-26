use std::net::IpAddr;

use crate::instances::controller::dtos::{IpDto, to_dto};
use crate::instances::controller::errors::IpControllerError;
use crate::instances::service::errors::IpAddrServiceError;
use crate::traits::controller::Controller;
use crate::traits::service::Service;

pub struct IpAddrController<Svc: Service<Item = IpAddr>> {
    svc: Svc,
}
// Struct Specific Methods
impl<Svc: Service<Item = IpAddr>> IpAddrController<Svc> {
    pub fn new(svc: Svc) -> Self {
        Self { svc }
    }
}
// Implement Controller Trait for IpAddrController
impl<Svc> Controller for IpAddrController<Svc>
where
    Svc: Service<Item = IpAddr, ServiceError = IpAddrServiceError>,
{
    type Item = IpAddr;
    type Svc = Svc;
    type Error = IpControllerError;
    type ListDto = Vec<IpDto>;
    type OneDto = IpDto;

    fn service(&self) -> &Self::Svc {
        &self.svc
    }
    fn service_mut(&mut self) -> &mut Self::Svc {
        &mut self.svc
    }

    // List -> map borrowed slice to owned DTOs
    fn index(&self) -> Result<Self::ListDto, Self::Error> {
        let items = self.service().find_all()?; // &[IpAddr]
        let dtos: Vec<IpDto> = items.iter().map(|ip| to_dto(*ip)).collect();
        Ok(dtos)
    }

    // Show -> copy to owned DTO (IpAddr is Copy; for non-Copy types, clone or adjust service)
    fn show(&self, key: &Self::Item) -> Result<Self::OneDto, Self::Error> {
        let ip = *self.service().find_one(key)?; // &IpAddr -> IpAddr
        Ok(to_dto(ip))
    }

    // Delete -> owned from service, then DTO
    fn delete(&mut self, key: &Self::Item) -> Result<Self::OneDto, Self::Error> {
        let removed = self.service_mut().remove(key)?; // IpAddr
        Ok(to_dto(removed))
    }

    // Patch -> owned from service, then DTO
    fn patch(
        &mut self,
        original: &Self::Item,
        updated: Self::Item,
    ) -> Result<Self::OneDto, Self::Error> {
        let old = self.service_mut().update(original, updated)?; // IpAddr (old value)
        Ok(to_dto(old))
    }
}
