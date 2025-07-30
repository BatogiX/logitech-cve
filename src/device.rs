use std::{ffi, mem, ptr};

use windows::{
    Wdk::{
        Foundation::OBJECT_ATTRIBUTES,
        Storage::FileSystem::{
            FILE_NON_DIRECTORY_FILE, FILE_SYNCHRONOUS_IO_NONALERT, NTCREATEFILE_CREATE_DISPOSITION,
            NTCREATEFILE_CREATE_OPTIONS, NtCreateFile,
        },
        System::{IO::NtDeviceIoControlFile, SystemServices::ZwClose},
    },
    Win32::{
        Foundation::{GENERIC_WRITE, HANDLE, NTSTATUS, OBJECT_ATTRIBUTE_FLAGS, STATUS_SUCCESS, UNICODE_STRING},
        Storage::FileSystem::{
            FILE_ACCESS_RIGHTS, FILE_ATTRIBUTE_NORMAL, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, SYNCHRONIZE,
        },
        System::{IO::IO_STATUS_BLOCK, WindowsProgramming::RtlInitUnicodeString},
    },
    core::PCWSTR,
};

use crate::{KeyboardButton, MouseButton, util::InitializeObjectAttributes};

/// I/O structure used to communicate mouse actions to the device driver.
#[repr(C)]
struct MouseIO {
    button: u8,
    x: i8,
    y: i8,
    wheel: i8,
    unk1: i8,
}

impl MouseIO {
    const fn new(button: u8, x: i8, y: i8, wheel: i8) -> Self {
        let unk1 = 0;
        Self {
            button,
            x,
            y,
            wheel,
            unk1,
        }
    }
}

/// I/O structure used to communicate keyboard button states to the device driver.
#[repr(C)]
struct KeyboardIO {
    unknown1: u8,
    unknown2: u8,
    button1: u8,
    button2: u8,
    button3: u8,
    button4: u8,
    button5: u8,
    button6: u8,
}

impl KeyboardIO {
    const fn new(button1: u8, button2: u8, button3: u8, button4: u8, button5: u8, button6: u8) -> Self {
        let unknown1 = 0;
        let unknown2 = 0;
        Self {
            unknown1,
            unknown2,
            button1,
            button2,
            button3,
            button4,
            button5,
            button6,
        }
    }
}

/// Represents a handle to the virtual input device.
pub struct Device {
    filehandle: HANDLE,
    iostatusblock: IO_STATUS_BLOCK,
}

impl Drop for Device {
    fn drop(&mut self) {
        self.close();
    }
}

impl Device {
    /// Attempts to open the device and return a [`Device`] instance.
    ///
    /// # Errors
    /// Returns an error if the device cannot be opened (e.g., G HUB not installed or incompatible version).
    pub fn try_new() -> Result<Self, &'static str> {
        let filehandle = HANDLE::default();
        let iostatusblock = IO_STATUS_BLOCK::default();

        let mut device = Self {
            filehandle,
            iostatusblock,
        };

        if !device.open() {
            return Err("Device not found. Consider to download Logitech G HUB 2021.11.1775");
        }

        Ok(device)
    }

    /// Sends a mouse command to the device.
    pub fn send_mouse(&mut self, button: MouseButton, x: i8, y: i8, wheel: i8) {
        let mut io = MouseIO::new(button.into(), x, y, wheel);

        if !self.call_mouse(&mut io) {
            self.close();
            self.open(); // Attempt to re-open if call failed
        }
    }

    /// Sends a keyboard command to the device.
    pub fn send_keyboard(
        &mut self,
        button1: KeyboardButton,
        button2: KeyboardButton,
        button3: KeyboardButton,
        button4: KeyboardButton,
        button5: KeyboardButton,
        button6: KeyboardButton,
    ) {
        let mut buffer = KeyboardIO::new(
            button1.into(),
            button2.into(),
            button3.into(),
            button4.into(),
            button5.into(),
            button6.into(),
        );

        if !self.call_keyboard(&mut buffer) {
            self.close();
            self.open(); // Attempt to re-open if call failed
        }
    }

    /// Tries to open the device by testing multiple known device paths.
    ///
    /// # Returns
    /// `true` if a device was successfully opened, `false` otherwise.
    fn open(&mut self) -> bool {
        let buffers: [Vec<u16>; 2] = [
            "\\??\\ROOT#SYSTEM#0001#{1abc05c0-c378-41b9-9cef-df1aba82b015}"
                .encode_utf16()
                .collect(),
            "\\??\\ROOT#SYSTEM#0002#{1abc05c0-c378-41b9-9cef-df1aba82b015}"
                .encode_utf16()
                .collect(),
        ];

        for buffer in buffers {
            if self.device_initialize(PCWSTR(buffer.as_ptr())) == STATUS_SUCCESS {
                return true;
            }
        }

        false
    }

    /// Initializes the device by opening a handle to it.
    ///
    /// # Arguments
    /// * `device_name` - A `PCWSTR` representing the path to the device.
    ///
    /// # Returns
    /// An `NTSTATUS` indicating the success or failure of the operation.
    fn device_initialize(&mut self, device_name: PCWSTR) -> NTSTATUS {
        let mut name = UNICODE_STRING::default();
        let mut attr = OBJECT_ATTRIBUTES::default();

        unsafe {
            RtlInitUnicodeString(&raw mut name, device_name);
            InitializeObjectAttributes(&mut attr, &raw const name, OBJECT_ATTRIBUTE_FLAGS(0), None, None);

            NtCreateFile(
                &raw mut self.filehandle,
                FILE_ACCESS_RIGHTS(GENERIC_WRITE.0 | SYNCHRONIZE.0),
                &raw const attr,
                &raw mut self.iostatusblock,
                None, // AllocationSize (optional)
                FILE_FLAGS_AND_ATTRIBUTES(FILE_ATTRIBUTE_NORMAL.0),
                FILE_SHARE_MODE(0),
                NTCREATEFILE_CREATE_DISPOSITION(3), // CreateDisposition (OPEN_EXISTING)
                NTCREATEFILE_CREATE_OPTIONS(FILE_NON_DIRECTORY_FILE.0 | FILE_SYNCHRONOUS_IO_NONALERT.0),
                None,
                0,
            )
        }
    }

    /// Calls the device IOCTL.
    ///
    /// # Arguments
    /// * `buffer` - A mutable reference to a `MouseIO` struct containing the mouse action data.
    ///
    /// # Returns
    /// `true` if the IOCTL call was successful, `false` otherwise.
    fn call_mouse(&self, buffer: &mut MouseIO) -> bool {
        #[allow(clippy::cast_possible_truncation)] // MouseIO is only 5 bytes
        const INPUTBUFFERLENGTH: u32 = mem::size_of::<MouseIO>() as u32;

        let mut block = IO_STATUS_BLOCK::default();

        let status = unsafe {
            NtDeviceIoControlFile(
                self.filehandle,
                None,
                None,
                None,
                &raw mut block,
                0x002A_2010,
                Some(ptr::from_mut(buffer).cast::<ffi::c_void>()),
                INPUTBUFFERLENGTH,
                None,
                0,
            )
        };
        status == STATUS_SUCCESS
    }

    /// Calls the device IOCTL.
    ///
    /// # Arguments
    /// * `buffer` - A mutable reference to a `KeyboardIO` struct containing the keyboard action data.
    ///
    /// # Returns
    /// `true` if the IOCTL call was successful, `false` otherwise.
    fn call_keyboard(&self, buffer: &mut KeyboardIO) -> bool {
        #[allow(clippy::cast_possible_truncation)] // KeyboardIO is only 8 bytes
        const INPUTBUFFERLENGTH: u32 = mem::size_of::<KeyboardIO>() as u32;

        let mut block = IO_STATUS_BLOCK::default();

        let status = unsafe {
            NtDeviceIoControlFile(
                self.filehandle,
                None,
                None,
                None,
                &raw mut block,
                0x002A_200C,
                Some(ptr::from_mut(buffer).cast::<ffi::c_void>()),
                INPUTBUFFERLENGTH,
                None,
                0,
            )
        };
        status == STATUS_SUCCESS
    }

    /// Closes the handle to the device.
    fn close(&mut self) {
        unsafe {
            if !self.filehandle.0.is_null() {
                let _ = ZwClose(self.filehandle);
                self.filehandle = HANDLE(ptr::null_mut());
            }
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_open_close() {
        let mut device = Device {
            filehandle: HANDLE::default(),
            iostatusblock: IO_STATUS_BLOCK::default(),
        };
        assert!(device.open(), "Device not opened");
        device.close();
        assert!(device.filehandle.is_invalid());
    }

    #[test]
    fn test_call_mouse() {
        let device = Device::try_new().unwrap();
        let mut buffer = MouseIO::new(0, 0, 0, 0);
        assert!(device.call_mouse(&mut buffer));
    }

    #[test]
    fn test_call_keyboard() {
        let device = Device::try_new().unwrap();
        let mut buffer = KeyboardIO::new(0, 0, 0, 0, 0, 0);
        assert!(device.call_keyboard(&mut buffer));
    }
}
