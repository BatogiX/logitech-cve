use std::ptr;

use windows_sys::{
    Wdk::Foundation::OBJECT_ATTRIBUTES,
    Win32::{
        Foundation::{HANDLE, OBJECT_ATTRIBUTE_FLAGS, UNICODE_STRING},
        Security::SECURITY_DESCRIPTOR,
    },
};

#[allow(clippy::many_single_char_names, non_snake_case)]
pub const fn InitializeObjectAttributes(
    p: &mut OBJECT_ATTRIBUTES,
    n: *const UNICODE_STRING,
    a: OBJECT_ATTRIBUTE_FLAGS,
    r: HANDLE,
    s: *const SECURITY_DESCRIPTOR,
) {
    // OBJECT_ATTRIBUTES is only 48 bytes so it should fit in u32
    #[allow(clippy::cast_possible_truncation)]
    let Length = size_of::<OBJECT_ATTRIBUTES>() as u32;

    p.Length = Length;
    p.RootDirectory = r;
    p.ObjectName = n;
    p.Attributes = a;
    p.SecurityDescriptor = s;
    p.SecurityQualityOfService = ptr::null();
}
