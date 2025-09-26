use crate::traits::service::Service;
use std::error::Error;

pub trait Controller {
    type Item;
    type Svc: Service<Item = Self::Item>;
    type Error: From<<Self::Svc as Service>::ServiceError> + Error;

    // Return Types
    type ListDto;
    type OneDto;

    // Allow the controller to interact with the service layer:
    fn service(&self) -> &Self::Svc;
    fn service_mut(&mut self) -> &mut Self::Svc;

    // Default interactions
    // Default “endpoints”
    fn index(&self) -> Result<Self::ListDto, Self::Error>;
    fn show(&self, key: &Self::Item) -> Result<Self::OneDto, Self::Error>;
    fn delete(&mut self, key: &Self::Item) -> Result<Self::OneDto, Self::Error>;
    fn patch(
        &mut self,
        original: &Self::Item,
        updated: Self::Item,
    ) -> Result<Self::OneDto, Self::Error>;
}
