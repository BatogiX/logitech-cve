use std::{thread, time::Duration};

use windows_sys::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};

use crate::device::Device;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MouseButton {
    Left = 1,
    Right = 2,
    Middle = 4,
    LeftRight = 3,
    LeftMiddle = 5,
    RightMiddle = 6,
    All = 7,
    Release = 0,
}

impl From<MouseButton> for u8 {
    fn from(button: MouseButton) -> Self {
        button as Self
    }
}

pub struct Mouse<'a> {
    device: &'a mut Device,
}

impl<'a> Mouse<'a> {
    pub const fn new(device: &'a mut Device) -> Self {
        Self { device }
    }

    pub fn click(&mut self, button: MouseButton, millis: u64) {
        self.device.send_mouse(button, 0, 0, 0);
        thread::sleep(Duration::from_millis(millis));
        self.device.send_mouse(MouseButton::Release, 0, 0, 0);
    }

    pub fn move_absolute(&mut self, button: MouseButton, x: u16, y: u16, millis: u64) {
        // Get current mouse position
        let mut current_point = POINT::default();
        unsafe { GetCursorPos(&raw mut current_point) };

        // Calculate deltas
        let delta_x = i32::from(x) - current_point.x;
        let delta_y = i32::from(y) - current_point.y;

        // Calculate the number of steps and step sizes for both X and Y
        let (count_x, mut x_step): (i32, i8) = if current_point.x > i32::from(x) {
            (delta_x / -127, -127)
        } else {
            (delta_x / 127, 127)
        };

        let (count_y, mut y_step): (i32, i8) = if current_point.y > i32::from(y) {
            (delta_y / -127, -127)
        } else {
            (delta_y / 127, 127)
        };

        let (final_x, final_y);
        if count_x > 0 || count_y > 0 {
            // Determine which axis takes more steps
            let count;
            if count_x > count_y {
                count = count_x;
                y_step = (delta_y / count) as i8;
            } else if count_y > count_x {
                count = count_y;
                x_step = (delta_x / count) as i8;
            } else {
                count = count_x; // or count_y, they are equal
            }

            final_x = (delta_x - (x_step as i32 * count)) as i8;
            final_y = (delta_y - (y_step as i32 * count)) as i8;
            // Perform the movement in steps
            for _ in 0..count {
                self.move_relative(button, x_step, y_step);
                thread::sleep(Duration::from_millis(millis));
            }
        } else {
            final_x = (delta_x) as i8;
            final_y = (delta_y) as i8;
        }

        // Ensure the final move reaches the target
        self.move_relative(button, final_x, final_y);
    }

    pub fn move_relative(&mut self, button: MouseButton, x: i8, y: i8) {
        self.device.send_mouse(button, x, y, 0);
    }

    pub fn press(&mut self, button: MouseButton) {
        self.device.send_mouse(button, 0, 0, 0);
    }

    pub fn release(&mut self) {
        self.device.send_mouse(MouseButton::Release, 0, 0, 0);
    }

    pub fn wheel(&mut self, button: MouseButton, wheel: i8) {
        self.device.send_mouse(button, 0, 0, wheel);
    }
}
