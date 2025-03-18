use crate::utils::unified_ip_addr::UnifiedIpAddr;

pub struct AddrResponse {
    ptr: *mut libc::addrinfo,
}
impl AddrResponse {
    pub fn from_addrinfo(addrinfo: *mut *mut libc::addrinfo) -> Option<Self> {
        if addrinfo.is_null() {
            return None;
        }

        // SAFETY: addrinfo is not null
        unsafe {
            Some(AddrResponse { ptr: (*addrinfo) })
        }
    }
}

impl Iterator for AddrResponse {
    type Item = UnifiedIpAddr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            return None;
        }

        // SAFETY:
        // - self.ptr is not null
        // - ai_next is either null or points to a valid addrinfo struct
        unsafe {
            let addr_ref: &libc::sockaddr = (*self.ptr).ai_addr.as_ref()?;
            let addr = UnifiedIpAddr::from_sockaddr(addr_ref);
            self.ptr = (*self.ptr).ai_next;
            addr
        }
    }
}
