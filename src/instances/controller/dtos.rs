/// DTOs - Data Transfer Objects
use std::net::IpAddr;

#[derive(Debug)]
pub struct IpDto {
    pub ip: IpAddr,
    pub is_v6: bool,
    pub is_private: bool,
}

pub fn to_dto(ip: IpAddr) -> IpDto {
    let is_private = match ip {
        IpAddr::V4(v4) => v4.is_private(),
        IpAddr::V6(v6) => v6.is_unique_local(),
    };
    IpDto {
        ip,
        is_v6: matches!(ip, IpAddr::V6(_)),
        is_private,
    }
}
