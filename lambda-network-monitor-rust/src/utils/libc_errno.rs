//! Errno isn't provided in the Rust libc bindings on some platforms.
//! It is helpful to have a stub for those platforms so that IDE
//! tools can provide autocomplete without displaying errors that will not
//! exist when building for the build target.
//!
//!
//! ## Preserve `errno`
//!
//! When we proxy *libc* functions, we need to make sure
//! our function preserves the errno value set from the proxied call
//! and not the errno value that may be set by internal network-monitor
//! activities like printing messages.  For example:
//!
//! ```rust,ignore
//! pub extern "C" fn close(fd: c_int) -> c_int {
//!     let rv = LIBC_CLOSE(fd);
//!     println!("Closed fd {}", fd);      // This *could* modify `errno` if the underlying `write()` encounters an error,
//!                                        // and it could appear to the application that `close()` set an errno if `rv != 0`.
//!     return rv;
//! }
//! ```
//!
//!
//!
//!

/// Save and restore `errno` across the function-call body.
/// Useful to wrap code that *might* mutate errno such as `println!()`, or writes.
///
#[inline]
pub fn preserve_errno(f: impl Fn()) {
    let errno = libc_errno();
    f();
    set_libc_errno(errno);
}

#[allow(dead_code)]
pub fn libc_errno() -> libc::c_int {
    #[cfg(target_os = "macos")]
    // SAFETY: __error will be valid if non-NULL
    unsafe {
        libc::__error()
            .as_ref()
            .copied()
            .unwrap_or(0)
    }

    #[cfg(not(target_os = "macos"))]
    // SAFETY: __ernno_location will be valid if non_NULL
    unsafe {
        libc::__errno_location()
            .as_ref()
            .copied()
            .unwrap_or(0)
    }
}

#[allow(dead_code)]
pub fn set_libc_errno(value: libc::c_int) {
    #[cfg(target_os = "macos")]
    // SAFETY: __error() is valid if non-NULL
    unsafe {
        if let Some(ptr) = libc::__error().as_mut() {
            *ptr = value;
        }
    }
    #[cfg(not(target_os = "macos"))]
    // SAFETY: __ernno_location() will be valid if non-NULL
    unsafe {
        if let Some(ptr) = libc::__errno_location().as_mut() {
            *ptr = value;
        }
    }
}
