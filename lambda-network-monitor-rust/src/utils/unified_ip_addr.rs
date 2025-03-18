use std::fmt;
use std::net::{IpAddr, SocketAddr};
use libc::{sockaddr, sockaddr_in, sockaddr_in6};

fn sockaddr_in_from_sockaddr(addr: &sockaddr) -> &sockaddr_in {
    assert_eq!(addr.sa_family as i32, libc::AF_INET, "Cannot cast non-AF_INET into sockaddr_in");
    let ptr: *const sockaddr_in = (addr as *const sockaddr).cast();

    // SAFETY: pointer derived from reference and sockaddr castable to sockaddr_in after checking `sa_family`
    unsafe {
        return ptr.as_ref().unwrap();
    }
}
fn sockaddr_in6_from_sockaddr(addr: &sockaddr) -> &sockaddr_in6 {
    assert_eq!(addr.sa_family as i32, libc::AF_INET6, "Cannot cast non-AF_INET6 into sockaddr_in6");
    let ptr: *const sockaddr_in6 = (addr as *const sockaddr).cast();

    // SAFETY: pointer derived from reference and sockaddr castable to sockaddr_in6 after checking `sa_family`
    unsafe {
        return ptr.as_ref().unwrap();
    }
}

/// Like {SocketAddr}, but the port is optional, so that approve-listing can potentially limit to IP and port combination.
#[derive(Clone)]
pub struct UnifiedIpAddr {
    pub addr: IpAddr,
    pub port: Option<u16>,
}
impl UnifiedIpAddr {
    pub fn from_sockaddr_ptr(addr: *const libc::sockaddr) -> Option<Self> {
        if addr.is_null() {
            return None;
        }

        // SAFETY: dereferenced pointer is not NULL
        unsafe {
            Self::from_sockaddr(&*addr)
        }
    }
    pub fn from_sockaddr(addr: &libc::sockaddr) -> Option<Self> {
        match addr.sa_family as i32 {
            libc::AF_INET => {
                let sock_in = sockaddr_in_from_sockaddr(addr);
                let ipv4_addr = std::net::Ipv4Addr::from(u32::from_be(sock_in.sin_addr.s_addr));
                let port = u16::from_be(sock_in.sin_port);

                Some(Self {
                    addr: IpAddr::V4(ipv4_addr),
                    port: Some(port),
                })
            }
            libc::AF_INET6 => {
                let sock_in6 = sockaddr_in6_from_sockaddr(addr);
                let ipv6_addr = std::net::Ipv6Addr::from(sock_in6.sin6_addr.s6_addr);
                let port = u16::from_be(sock_in6.sin6_port);

                Some(Self {
                    addr: IpAddr::V6(ipv6_addr),
                    port: Some(port),
                })
            }
            _ => None,
        }
    }

}

impl PartialEq for UnifiedIpAddr {
    fn eq(&self, other: &Self) -> bool {
        self.addr.eq(&other.addr) && self.port.eq(&other.port)
    }
}

impl fmt::Display for UnifiedIpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.addr)?;
        if let Some(p) = self.port {
            write!(f, ":{}", p)?;
        }
        Ok(())
    }
}
impl fmt::Debug for UnifiedIpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.addr)?;
        if let Some(p) = self.port {
            write!(f, ":{}", p)?;
        }
        Ok(())
    }
}

/// Fail-safe conversion of {UnifiedIpAddr} into {std::net::SocketAddr} with Result.
/// Succeeds iif `port` is `Some(...)`.
impl TryFrom<UnifiedIpAddr> for SocketAddr {
    type Error = ();

    fn try_from(value: UnifiedIpAddr) -> Result<Self, Self::Error> {
        match value.port {
            Some(port) => Ok(SocketAddr::new(value.addr, port)),
            None => Err(())
        }
    }
}

