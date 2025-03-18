use crate::utils::{
    libc_errno::preserve_errno,
    unified_ip_addr::UnifiedIpAddr,
    monitored_sockets,
    resolved_addresses};
use libc::{c_int, sockaddr, socklen_t};
use once_cell::sync::Lazy;
use std::ffi::CString;

#[allow(non_camel_case_types)]
pub type connectFn = extern "C" fn(fd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;

#[allow(non_upper_case_globals)]
pub static LIBC_CONNECT: Lazy<connectFn> = Lazy::new(|| {
    let fn_name = CString::new("connect").unwrap();
    let fn_ptr = unsafe { libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr()) };
    if fn_ptr.is_null() {
        log::error!("Failed to load *connect() in libc from dlsym()");
    }
    unsafe { std::mem::transmute(fn_ptr) }
});

#[no_mangle]
pub unsafe extern "C" fn connect(fd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int {
    let ip = match UnifiedIpAddr::from_sockaddr_ptr(addr) {
        None => String::from("(not IPv4/IPv6)"),
        Some(addr) => addr.addr.to_string(),
    };
    let node = resolved_addresses::get_node_by_ip(&ip);
    log::debug!("> connect fd={fd} addr={ip} node={node}");

    let rv = LIBC_CONNECT(fd, addr, addrlen);

    preserve_errno(||{
        monitored_sockets::set_socket_addr(fd, &ip);
        log::info!("DONE fd={fd} addr={ip} node={node} rv={rv}");
    });
    rv
}
