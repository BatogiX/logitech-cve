use std::{thread, time::Duration};

use logitech_cve::{
    Device,
    mouse::{Mouse, MouseButton},
};
use windows_sys::Win32::{
    Foundation::POINT,
    UI::WindowsAndMessaging::{GetCursorPos, WH_MOUSE_LL},
};

mod common;

#[test]
fn test_press_and_release() {
    let mut device = Device::try_new().unwrap();
    let mut mouse = Mouse::new(&mut device);

    thread::spawn(|| common::start(WH_MOUSE_LL).expect("Failed to start mouse hook"));
    thread::sleep(Duration::from_millis(100));
    mouse.press(MouseButton::Left);
    mouse.release();
    thread::sleep(Duration::from_millis(1000));

    assert_eq!(common::stop(), vec![true, true]);
}

#[test]
fn test_wheel() {
    let mut device = Device::try_new().unwrap();
    let mut mouse = Mouse::new(&mut device);

    thread::spawn(|| common::start(WH_MOUSE_LL).expect("Failed to start mouse hook"));
    thread::sleep(Duration::from_millis(100));
    mouse.wheel(MouseButton::Release, 1);
    mouse.wheel(MouseButton::Release, -1);
    thread::sleep(Duration::from_millis(1000));

    assert_eq!(common::stop(), vec![true, true]);
}

#[test]
fn test_move_relative() {
    let mut device = Device::try_new().unwrap();
    let mut mouse = Mouse::new(&mut device);

    let mut start_point = POINT { x: 0, y: 0 };
    let mut end_point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&raw mut start_point) };
    mouse.move_relative(MouseButton::Release, 1, 1);
    thread::sleep(Duration::from_millis(100));
    unsafe { GetCursorPos(&raw mut end_point) };

    assert_eq!(start_point.x + 1, end_point.x);
    assert_eq!(start_point.y + 1, end_point.y);
}
