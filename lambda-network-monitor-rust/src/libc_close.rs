use crate::utils::{monitored_sockets, libc_errno::preserve_errno};
use libc::c_int;
use once_cell::sync::Lazy;
use std::ffi::CString;

#[allow(non_camel_case_types)]
pub type closeFn = extern "C" fn(fd: c_int) -> c_int;

#[allow(non_upper_case_globals)]
pub static LIBC_CLOSE: Lazy<closeFn> = Lazy::new(|| {
    let fn_name = CString::new("close").unwrap();
    let fn_ptr = unsafe { libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr()) };
    if fn_ptr.is_null() {
        log::error!("Failed to load *close() in libc from dlsym()");
    }
    unsafe { std::mem::transmute(fn_ptr) }
});

#[no_mangle]
pub extern "C" fn close(fd: c_int) -> c_int {
    let rv = LIBC_CLOSE(fd);

    preserve_errno(||{
        if monitored_sockets::contains(fd) {
            log::debug!("> fd={fd}");
            monitored_sockets::remove(fd);
            log::info!("DONE fd={fd} rv={rv}");
        }
    });
    rv
}
