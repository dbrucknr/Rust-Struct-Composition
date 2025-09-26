mod instances;
mod traits;

use std::net::{IpAddr, Ipv4Addr};

use instances::repository::IpAddrRepository;
use instances::service::IpAddrService;

use crate::traits::service::Service;

fn main() {
    let respository = IpAddrRepository::new(Vec::from([
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 2)),
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 3)),
    ]));

    let mut service = IpAddrService::new(respository);

    let all = service.find_all();
    println!("{:?}", all);

    if let Some(addr) = service.find_one(&IpAddr::V4(Ipv4Addr::new(192, 0, 0, 2))) {
        println!("{:?}", addr);
    }

    if let Some(addr) = service.update(
        &mut IpAddr::V4(Ipv4Addr::new(192, 0, 0, 2)),
        IpAddr::V4(Ipv4Addr::new(192, 0, 0, 102)),
    ) {
        println!("Updated: {:?}", addr);
    }

    let all_after_update = service.find_all();
    println!("After Update: {:?}", all_after_update);

    if let Some(addr) = service.remove(&mut IpAddr::V4(Ipv4Addr::new(192, 0, 0, 1))) {
        println!("Removed {:?}", addr);
    }

    let all_after_removal = service.find_all();
    println!("After Removal: {:?}", all_after_removal);
}
