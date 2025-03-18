use crate::utils::addr_response::AddrResponse;
use crate::utils::printable_cstring::PrintableCString;
use crate::utils::{resolved_addresses, libc_errno::preserve_errno};
use libc::{addrinfo, c_char, c_int};
use once_cell::sync::Lazy;
use std::ffi::CString;

#[allow(non_camel_case_types)]
pub type getaddrinfoFn = extern "C" fn(
    node: *const c_char,
    service: *const c_char,
    hints: *const addrinfo,
    res: *mut *mut addrinfo,
) -> c_int;

#[allow(non_upper_case_globals)]
pub static LIBC_GETADDRINFO: Lazy<getaddrinfoFn> = Lazy::new(|| {
    let fn_name = CString::new("getaddrinfo").unwrap();
    let fn_ptr = unsafe { libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr()) };
    if fn_ptr.is_null() {
        log::error!("Failed to load *getaddrinfo() in libc from dlsym()");
    }
    unsafe { std::mem::transmute(fn_ptr) }
});

#[no_mangle]
pub extern "C" fn getaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *const addrinfo,
    res: *mut *mut addrinfo,
) -> c_int {
    // Check whether egress connectivity is allowed for this node
    let printable_node = format!("{}", PrintableCString::from(node));
    let printable_service = format!("{}", PrintableCString::from(service));

    log::debug!("> getaddrinfo node={printable_node} service={printable_service}");

    // Resolve address
    let rv = LIBC_GETADDRINFO(node, service, hints, res);

    if rv != 0 {
        // RETURN is error; errno must be preserved
        preserve_errno(||{
            log::warn!("CANT RESOLVE node={printable_node} service={printable_service} rv={rv}");
        });
        return rv;
    }

    let ip_addresses = match AddrResponse::from_addrinfo(res) {
        None => {
            log::warn!("NO IPs node={printable_node} service={printable_service} rv={rv}");
            return rv;
        }
        Some(v) => v,
    };

    let mut count = 0;
    for ip_addr in ip_addresses {
        let ip = ip_addr.addr.to_string();
        log::debug!("RESOLVED node={printable_node} service={printable_service} ip={ip}");
        resolved_addresses::add(&printable_node, &ip);
        count += 1;
    }

    log::info!("DONE node={printable_node} service={printable_service} resolved_addresses_count={count} rv={rv}");
    rv
}
