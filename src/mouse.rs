use core::{cmp::Ordering, time::Duration};
use std::thread;

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
    #[inline]
    fn from(button: MouseButton) -> Self {
        button as Self
    }
}

/// A struct for controlling a virtual mouse.
///
/// It holds a reference to a `Device` which is used to send the mouse commands.
pub struct Mouse<'a> {
    /// Reference to the device used for sending mouse commands.
    device: &'a Device,
}

impl<'a> Mouse<'a> {
    /// Creates a new [`Mouse`].
    #[must_use]
    #[inline]
    pub const fn new(device: &'a Device) -> Self {
        Self { device }
    }

    /// Performs a click and release action with a specified button.
    ///
    /// The button is pressed, held for `millis` milliseconds, and then released.
    ///
    /// # Arguments
    ///
    /// * `button` - The `MouseButton` to click.
    /// * `millis` - The duration, in milliseconds, to hold the button down.
    #[inline]
    pub fn click(&self, button: MouseButton, millis: u64) {
        self.device.call_mouse(button, 0, 0, 0);
        thread::sleep(Duration::from_millis(millis));
        self.device.call_mouse(MouseButton::Release, 0, 0, 0);
    }

    /// Moves the mouse cursor to an absolute screen coordinate (x, y) with a simulated smooth movement.
    ///
    /// The movement is broken down into smaller steps, with a delay between each step.
    ///
    /// # Arguments
    ///
    /// * `button` - The `MouseButton` to hold down during the movement (e.g., for dragging). Use `MouseButton::Release` for no buttons.
    /// * `x` - The target horizontal coordinate.
    /// * `y` - The target vertical coordinate.
    /// * `millis` - The delay, in milliseconds, between each small movement step.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Casting is safe here because mouse movement steps are always within i8 range."
    )]
    #[inline]
    pub fn move_absolute(&self, button: MouseButton, x: u16, y: u16, millis: u64) {
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
        // SAFETY: `current_point` is a valid pointer to a POINT struct, as required by GetCursorPos.
        unsafe {
            GetCursorPos(&raw mut current_point);
        };

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

    /// Moves the mouse cursor by a relative offset from its current position.
    ///
    /// # Arguments
    ///
    /// * `button` - The `MouseButton` to hold down during the movement.
    /// * `x` - The horizontal offset. Positive values move right, negative move left.
    /// * `y` - The vertical offset. Positive values move down, negative move up.
    #[inline]
    pub fn move_relative(&self, button: MouseButton, x: i8, y: i8) {
        self.device.call_mouse(button, x, y, 0);
    }

    /// Presses and holds a specified mouse button.
    ///
    /// This method only presses the button; you must call `release()` to release it.
    ///
    /// # Arguments
    ///
    /// * `button` - The `MouseButton` to press.
    #[inline]
    pub fn press(&self, button: MouseButton) {
        self.device.call_mouse(button, 0, 0, 0);
    }

    /// Releases any currently pressed mouse buttons.
    ///
    /// This should be called after a `press()` action to release the button.
    #[inline]
    pub fn release(&self) {
        self.device.call_mouse(MouseButton::Release, 0, 0, 0);
    }

    /// Scrolls the mouse wheel.
    ///
    /// # Arguments
    ///
    /// * `button` - The `MouseButton` to hold down during the scroll.
    /// * `wheel` - The scroll amount. Positive values scroll up, negative values scroll down.
    #[inline]
    pub fn wheel(&self, button: MouseButton, wheel: i8) {
        self.device.call_mouse(button, 0, 0, wheel);
    }
}
