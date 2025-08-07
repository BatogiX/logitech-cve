use core::{mem, ptr};

use windows_sys::{
    Wdk::{
        Foundation::OBJECT_ATTRIBUTES,
        Storage::FileSystem::{FILE_NON_DIRECTORY_FILE, FILE_OPEN_IF, FILE_SYNCHRONOUS_IO_NONALERT, NtCreateFile},
        System::{IO::NtDeviceIoControlFile, SystemServices::ZwClose},
    },
    Win32::{
        Foundation::{GENERIC_WRITE, HANDLE, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING},
        Storage::FileSystem::{FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, SYNCHRONIZE},
        System::{IO::IO_STATUS_BLOCK, WindowsProgramming::RtlInitUnicodeString},
    },
    core::PCWSTR,
};

use crate::{keyboard::Key, mouse::MouseButton, util::InitializeObjectAttributes};

/// I/O structure used to communicate mouse actions to the device driver.
#[repr(C)]
struct MouseIO {
    /// The mouse button state/action to perform.
    button: u8,
    /// The X-axis movement delta.
    x: i8,
    /// The Y-axis movement delta.
    y: i8,
    /// The mouse wheel movement delta.
    wheel: i8,
    /// Unknown field, always set to 0.
    unk1: i8,
}

impl MouseIO {
    /// Creates a new `MouseIO` instance with the specified parameters.
    ///
    /// # Arguments
    /// * `button` - The mouse button state/action to perform
    /// * `x` - The X-axis movement delta
    /// * `y` - The Y-axis movement delta
    /// * `wheel` - The mouse wheel movement delta
    ///
    /// # Returns
    /// A new `MouseIO` instance with `unk1` set to 0
    #[inline]
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
    /// Unknown field, always set to 0.
    unknown1: u8,
    /// Unknown field, always set to 0.
    unknown2: u8,
    /// State of keyboard button 1.
    button1: u8,
    /// State of keyboard button 2.
    button2: u8,
    /// State of keyboard button 3.
    button3: u8,
    /// State of keyboard button 4.
    button4: u8,
    /// State of keyboard button 5.
    button5: u8,
    /// State of keyboard button 6.
    button6: u8,
}

impl KeyboardIO {
    /// Creates a new `KeyboardIO` instance with the specified button states.
    ///
    /// # Arguments
    /// * `button1` through `button6` - The states of keyboard buttons 1-6
    ///
    /// # Returns
    /// A new `KeyboardIO` instance with unknown fields set to 0
    #[inline]
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
    /// Handle to the device file.
    filehandle: HANDLE,
}

impl Drop for Device {
    #[inline]
    fn drop(&mut self) {
        self.close();
    }
}

impl Device {
    /// Attempts to open the device and return a [`Device`] instance.
    ///
    /// # Errors
    /// Returns an error if the device cannot be opened (e.g., G HUB not installed or incompatible version).
    #[inline]
    pub fn try_new() -> Result<Self, &'static str> {
        let filehandle = HANDLE::default();

        let mut device = Self { filehandle };

        if !device.open() {
            return Err("Device not found. Consider to download Logitech G HUB 2021.11.1775");
        }

        Ok(device)
    }

    /// Calls the device IOCTL.
    ///
    /// # Arguments
    /// * `button` - The mouse button action to perform (e.g., left click, right click, release)
    /// * `x` - Horizontal movement delta in pixels. Positive values move right, negative values move left
    /// * `y` - Vertical movement delta in pixels. Positive values move down, negative values move up
    /// * `wheel` - Mouse wheel scroll delta. Positive values scroll up, negative values scroll down
    ///
    /// # Returns
    /// `true` if the IOCTL call was successful, `false` otherwise.
    #[expect(
        clippy::must_use_candidate,
        reason = "This function is used to send mouse input commands"
    )]
    #[inline]
    pub fn call_mouse(&self, button: MouseButton, x: i8, y: i8, wheel: i8) -> bool {
        #[expect(clippy::cast_possible_truncation, reason = "MouseIO is only 5 bytes")]
        const INPUTBUFFERLENGTH: u32 = mem::size_of::<MouseIO>() as u32;
        let mut iostatusblock = IO_STATUS_BLOCK::default();
        let inputbuffer = MouseIO::new(button.into(), x, y, wheel);

        // SAFETY: All pointers passed to NtDeviceIoControlFile are either valid, null, or point to properly initialized structures as required by the API.
        let status = unsafe {
            NtDeviceIoControlFile(
                self.filehandle,
                ptr::null_mut(),
                None,
                ptr::null(),
                &raw mut iostatusblock,
                0x002A_2010,
                (&raw const inputbuffer).cast(),
                INPUTBUFFERLENGTH,
                ptr::null_mut(),
                0,
            )
        };
        status == STATUS_SUCCESS
    }

    /// Calls the device IOCTL.
    ///
    /// # Arguments
    /// * `button1` through `button6` - The states of keyboard buttons 1-6. Each parameter represents
    ///   the desired state of a specific key position. Use `Key::None`
    ///   for keys that should not be pressed.
    ///
    /// # Returns
    /// `true` if the IOCTL call was successful, `false` otherwise.
    #[expect(
        clippy::must_use_candidate,
        reason = "This function is used to send keyboard input commands"
    )]
    #[inline]
    pub fn call_keyboard(
        &self,
        button1: Key,
        button2: Key,
        button3: Key,
        button4: Key,
        button5: Key,
        button6: Key,
    ) -> bool {
        #[expect(clippy::cast_possible_truncation, reason = "KeyboardIO is only 8 bytes")]
        const INPUTBUFFERLENGTH: u32 = mem::size_of::<KeyboardIO>() as u32;
        let mut iostatusblock = IO_STATUS_BLOCK::default();
        let inputbuffer = KeyboardIO::new(
            button1.into(),
            button2.into(),
            button3.into(),
            button4.into(),
            button5.into(),
            button6.into(),
        );

        // SAFETY: All pointers passed to NtDeviceIoControlFile are either valid, null, or point to properly initialized structures as required by the API.
        let status = unsafe {
            NtDeviceIoControlFile(
                self.filehandle,
                ptr::null_mut(),
                None,
                ptr::null(),
                &raw mut iostatusblock,
                0x002A_200C,
                (&raw const inputbuffer).cast(),
                INPUTBUFFERLENGTH,
                ptr::null_mut(),
                0,
            )
        };
        status == STATUS_SUCCESS
    }

    /// Tries to open the device by testing multiple known device paths.
    ///
    /// # Returns
    /// `true` if a device was successfully opened, `false` otherwise.
    fn open(&mut self) -> bool {
        let buffers: [Vec<u16>; 2] = [
            "\\??\\ROOT#SYSTEM#0001#{1abc05c0-c378-41b9-9cef-df1aba82b015}\0"
                .encode_utf16()
                .collect(),
            "\\??\\ROOT#SYSTEM#0002#{1abc05c0-c378-41b9-9cef-df1aba82b015}\0"
                .encode_utf16()
                .collect(),
        ];

        for buffer in buffers {
            if self.device_initialize(buffer.as_ptr()) == STATUS_SUCCESS {
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
        let mut iostatusblock = IO_STATUS_BLOCK::default();

        // SAFETY: RtlInitUnicodeString requires a valid pointer to a UNICODE_STRING and a valid PCWSTR.
        unsafe {
            RtlInitUnicodeString(&raw mut name, device_name);
        };
        InitializeObjectAttributes(&mut attr, &raw const name, 0, ptr::null_mut(), ptr::null());
        // SAFETY: NtCreateFile requires properly initialized pointers and structures as per API contract.
        unsafe {
            NtCreateFile(
                &raw mut self.filehandle,
                GENERIC_WRITE | SYNCHRONIZE,
                &raw const attr,
                &raw mut iostatusblock,
                ptr::null::<i64>(), // AllocationSize (optional)
                FILE_ATTRIBUTE_NORMAL,
                FILE_SHARE_NONE,
                FILE_OPEN_IF, // CreateDisposition (OPEN_EXISTING)
                FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT,
                ptr::null(),
                0,
            )
        }
    }

    /// Closes the handle to the device.
    ///
    /// This method safely closes the device handle if it's currently open,
    /// and sets the handle to null to prevent double-closing.
    fn close(&mut self) {
        if !self.filehandle.is_null() {
            // SAFETY: ZwClose is only called if filehandle is not null, and filehandle is set to null after closing to prevent double-closing.
            unsafe {
                ZwClose(self.filehandle);
            };
            self.filehandle = ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_close() {
        let mut device = Device {
            filehandle: HANDLE::default(),
        };
        
        assert!(device.open());
        device.close();
        assert!(device.filehandle.is_null());
    }
}
