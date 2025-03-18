pub struct PrintableCString<'a> {
    slice: Option<&'a [u8]>,
}
impl<'a> From<*const libc::c_char> for PrintableCString<'a> {
    fn from(ptr: *const libc::c_char) -> Self {
        if ptr.is_null() {
            return Self { slice: None };
        }

        // SAFETY: ptr is not null
        unsafe {
            Self {
                slice: Some(core::slice::from_raw_parts(ptr as *const u8, libc::strlen(ptr))),
            }
        }
    }
}
impl<'a> std::fmt::Display for PrintableCString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write;

        if self.slice.is_none() {
            return Ok(());
        }

        let slice = self.slice.as_ref().unwrap();

        // iterate over the cstring, escaping unprintable characters into "\xHH" representation
        for c in *slice {
            if (0x20..=0x7e).contains(c) { // printable ascii characters
                let c = *c as char;
                f.write_char(c)?;
            } else {
                write!(f, "!\\x{:02x}", *c)?;
            }
        }
        Ok(())
    }
}
