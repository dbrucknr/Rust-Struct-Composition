use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    net::IpAddr,
};

// Repository Error Enum
#[derive(Debug)]
pub enum IpAddrRepoError {
    IpAddrNotFound(IpAddr),
}
impl Display for IpAddrRepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            IpAddrRepoError::IpAddrNotFound(ip) => write!(f, "IP not found: {ip}"),
        }
    }
}
impl Error for IpAddrRepoError {}
