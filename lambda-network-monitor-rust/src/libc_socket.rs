use libc::c_int;
use once_cell::sync::Lazy;
use std::ffi::CString;
use crate::utils::{monitored_sockets, libc_errno::preserve_errno};

#[allow(non_camel_case_types)]
pub type socketFn = extern "C" fn(domain: c_int, type_: c_int, protocol: c_int) -> c_int;

#[allow(non_upper_case_globals)]
pub static LIBC_SOCKET: Lazy<socketFn> = Lazy::new(|| {
    let fn_name = CString::new("socket").unwrap();
    let fn_ptr = unsafe { libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr()) };
    if fn_ptr.is_null() {
        log::error!("Failed to load *socket() in libc from dlsym()");
    }
    unsafe { std::mem::transmute(fn_ptr) }
});

#[no_mangle]
pub extern "C" fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    log::debug!("> socket domain={domain} type={type_} protocol={protocol}");
    let rv = LIBC_SOCKET(domain, type_, protocol);

    if rv < 0 {
        preserve_errno(||{
            log::info!("socket() failed, rv={rv}");
        });
        return rv;
    }

    // Only monitor sockets that are using IPv4 and IPv6
    if domain == libc::AF_INET || domain == libc::AF_INET6 {
        monitored_sockets::add(rv);
    }

    log::info!("DONE domain={domain} type={type_} protocol={protocol} rv={rv}");
    rv
}
