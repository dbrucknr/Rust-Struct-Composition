mod instances;
mod traits;

use std::net::{IpAddr, Ipv4Addr};

use instances::controller::instance::IpAddrController;
use instances::repository::instance::IpAddrRepository;
use instances::service::instance::IpAddrService;

use crate::traits::controller::Controller;

fn main() {
    let respository = IpAddrRepository::new(Vec::from([
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 2)),
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 3)),
    ]));

    let service = IpAddrService::new(respository);
    let mut controller = IpAddrController::new(service);

    // let all = controller.index();
    if let Ok(result) = controller.index() {
        println!("All Values: {:?}", result);
    }

    if let Ok(addr) = controller.show(&IpAddr::V4(Ipv4Addr::new(192, 0, 0, 2))) {
        println!("Found: {:?}", addr);
    }

    if let Ok(addr) = controller.patch(
        &mut IpAddr::V4(Ipv4Addr::new(192, 0, 0, 2)),
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 102)),
    ) {
        println!("Updated: {:?}", addr);
    }

    if let Ok(all_after_update) = controller.index() {
        println!("After Update: {:?}", all_after_update);
    }

    if let Ok(addr) = controller.delete(&mut IpAddr::V4(Ipv4Addr::new(192, 0, 0, 1))) {
        println!("Removed {:?}", addr);
    }

    if let Ok(all_after_removal) = controller.index() {
        println!("After Removal: {:?}", all_after_removal);
    } else {
        println!("Something went wrong")
    }
}
