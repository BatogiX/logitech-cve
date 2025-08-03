use core::{mem, ptr};

use windows_sys::{
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{HANDLE, OBJECT_ATTRIBUTE_FLAGS, UNICODE_STRING},
        Security::SECURITY_DESCRIPTOR,
    },
};

/// Initializes an `OBJECT_ATTRIBUTES` structure for use with Windows API calls.
#[expect(clippy::many_single_char_names, non_snake_case, reason = "Windows API style")]
pub const fn InitializeObjectAttributes(
    p: &mut OBJECT_ATTRIBUTES,
    n: *const UNICODE_STRING,
    a: OBJECT_ATTRIBUTE_FLAGS,
    r: HANDLE,
    s: *const SECURITY_DESCRIPTOR,
) {
    #[expect(clippy::cast_possible_truncation, reason = "OBJECT_ATTRIBUTES is only 48 bytes")]
    const LENGTH: u32 = mem::size_of::<OBJECT_ATTRIBUTES>() as u32;

    p.Length = LENGTH;
    p.RootDirectory = r;
    p.ObjectName = n;
    p.Attributes = a;
    p.SecurityDescriptor = s;
    p.SecurityQualityOfService = ptr::null();
}
