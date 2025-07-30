#![allow(non_snake_case)]

use std::{thread, time::Duration};

use crate::device::Device;

#[repr(u8)]
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

    pub fn move_absolute(&mut self, button: MouseButton, x: i8, y: i8) {
        self.device.send_mouse(button, x, y, 0);
        todo!("Absolute move is not yet implemented");
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
