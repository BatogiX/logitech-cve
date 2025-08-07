use core::time::Duration;
use std::thread;

use logitech_cve::{
    device::Device,
    mouse::{Mouse, MouseButton},
};
use windows_sys::Win32::{
    Foundation::POINT,
    UI::WindowsAndMessaging::{GetCursorPos, WH_MOUSE_LL},
};

mod common;

#[test]
fn press_and_release() {
    let device = Device::try_new().unwrap();
    let mouse = Mouse::new(&device);

    thread::spawn(|| common::start(WH_MOUSE_LL));
    thread::sleep(Duration::from_millis(100));
    mouse.press(MouseButton::Left);
    mouse.release();
    thread::sleep(Duration::from_millis(100));

    assert_eq!(vec!["LBUTTON DOWN", "LBUTTON UP"], common::stop());
}

#[test]
fn wheel() {
    let device = Device::try_new().unwrap();
    let mouse = Mouse::new(&device);

    thread::spawn(|| common::start(WH_MOUSE_LL));
    thread::sleep(Duration::from_millis(100));
    mouse.wheel(MouseButton::Release, 1);
    mouse.wheel(MouseButton::Release, -1);
    thread::sleep(Duration::from_millis(100));

    assert_eq!(vec!["WHEEL UP", "WHEEL DOWN"], common::stop());
}

#[test]
fn move_relative() {
    let device = Device::try_new().unwrap();
    let mouse = Mouse::new(&device);

    let mut start_point = POINT::default();
    let mut current_point = POINT::default();
    // SAFETY: `start_point` is a valid pointer to a POINT struct, as required by GetCursorPos.
    unsafe {
        GetCursorPos(&raw mut start_point);
    };
    mouse.move_relative(MouseButton::Release, 1, 1);
    thread::sleep(Duration::from_millis(100));
    // SAFETY: `current_point` is a valid pointer to a POINT struct, as required by GetCursorPos.
    unsafe {
        GetCursorPos(&raw mut current_point);
    };

    assert_eq!(start_point.x + 1, current_point.x);
    assert_eq!(start_point.y + 1, current_point.y);
}

#[test]
fn move_absolute() {
    let device = Device::try_new().unwrap();
    let mouse = Mouse::new(&device);
    let mut current_point = POINT::default();

    mouse.move_absolute(MouseButton::Release, 500, 500, 10);
    thread::sleep(Duration::from_millis(10));
    // SAFETY: `current_point` is a valid pointer to a POINT struct, as required by GetCursorPos.
    unsafe {
        GetCursorPos(&raw mut current_point);
    };
    assert_eq!(500, current_point.x);
    assert_eq!(500, current_point.y);

    mouse.move_absolute(MouseButton::Release, 600, 600, 10);
    thread::sleep(Duration::from_millis(10));
    // SAFETY: `current_point` is a valid pointer to a POINT struct, as required by GetCursorPos.
    unsafe {
        GetCursorPos(&raw mut current_point);
    };
    assert_eq!(600, current_point.x);
    assert_eq!(600, current_point.y);

    mouse.move_absolute(MouseButton::Release, 750, 750, 10);
    thread::sleep(Duration::from_millis(10));
    // SAFETY: `current_point` is a valid pointer to a POINT struct, as required by GetCursorPos.
    unsafe {
        GetCursorPos(&raw mut current_point);
    };
    assert_eq!(750, current_point.x);
    assert_eq!(750, current_point.y);

    mouse.move_absolute(MouseButton::Release, 1, 1, 10);
    thread::sleep(Duration::from_millis(10));
    // SAFETY: `current_point` is a valid pointer to a POINT struct, as required by GetCursorPos.
    unsafe {
        GetCursorPos(&raw mut current_point);
    };
    assert_eq!(1, current_point.x);
    assert_eq!(1, current_point.y);
}
