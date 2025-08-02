use std::{cmp::Ordering, thread, time::Duration};

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

    #[allow(clippy::cast_possible_truncation)]
    pub fn move_absolute(&mut self, button: MouseButton, x: u16, y: u16, millis: u64) {
        const MIN_STEP_SIZE: i8 = -127; // -128 Does not work for some reason
        const MAX_STEP_SIZE: i8 = 127;

        #[inline]
        fn calculate_steps_and_size(delta: i32) -> (i32, i8) {
            if delta < 0 {
                return (delta / i32::from(MIN_STEP_SIZE), MIN_STEP_SIZE);
            }
            (delta / i32::from(MAX_STEP_SIZE), MAX_STEP_SIZE)
        }

        // Get current mouse position
        let mut current_point = POINT::default();
        unsafe { GetCursorPos(&raw mut current_point) };

        // Calculate deltas
        let delta_x = i32::from(x) - current_point.x;
        let delta_y = i32::from(y) - current_point.y;

        // Calculate the number of steps and step sizes for both X and Y
        let (steps_x, mut x_step) = calculate_steps_and_size(delta_x);
        let (steps_y, mut y_step) = calculate_steps_and_size(delta_y);

        let (final_step_x, final_step_y);
        if steps_x > 0 || steps_y > 0 {
            // Determine which axis takes more steps
            let steps;
            match steps_x.cmp(&steps_y) {
                Ordering::Greater => {
                    steps = steps_x; 
                    y_step = (delta_y / steps) as i8; 
                }
                Ordering::Less => {
                    steps = steps_y;
                    x_step = (delta_x / steps) as i8;
                }
                Ordering::Equal => {
                    steps = steps_x; // or steps_y, they are equal
                }
            }

            final_step_x = (delta_x - (i32::from(x_step) * steps)) as i8;
            final_step_y = (delta_y - (i32::from(y_step) * steps)) as i8;
            // Perform the movement in steps
            for _ in 0..steps {
                self.move_relative(button, x_step, y_step);
                thread::sleep(Duration::from_millis(millis));
            }
        } else {
        final_step_x = delta_x as i8;
        final_step_y = delta_y as i8;
        }

        // Ensure the final move reaches the target
        self.move_relative(button, final_step_x, final_step_y);
    }

    #[inline]
    pub fn move_relative(&mut self, button: MouseButton, x: i8, y: i8) {
        self.device.send_mouse(button, x, y, 0);
    }

    #[inline]
    pub fn press(&mut self, button: MouseButton) {
        self.device.send_mouse(button, 0, 0, 0);
    }

    #[inline]
    pub fn release(&mut self) {
        self.device.send_mouse(MouseButton::Release, 0, 0, 0);
    }

    #[inline]
    pub fn wheel(&mut self, button: MouseButton, wheel: i8) {
        self.device.send_mouse(button, 0, 0, wheel);
    }
}
