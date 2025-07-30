use std::ptr;

use windows::{
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
    r: Option<HANDLE>,
    s: Option<*const SECURITY_DESCRIPTOR>,
) {
    // OBJECT_ATTRIBUTES is only 48 bytes so it should fit in u32
    #[allow(clippy::cast_possible_truncation)]
    let Length = size_of::<OBJECT_ATTRIBUTES>() as u32;
    let RootDirectory = if let Some(r) = r { r } else { HANDLE(ptr::null_mut()) };
    let SecurityDescriptor = if let Some(s) = s { s } else { ptr::null() };

    *p = OBJECT_ATTRIBUTES {
        Length,
        RootDirectory,
        ObjectName: n,
        Attributes: a,
        SecurityDescriptor,
        SecurityQualityOfService: ptr::null(),
    };
}
