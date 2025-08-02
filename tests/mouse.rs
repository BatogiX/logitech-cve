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
    thread::sleep(Duration::from_millis(100));

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
    thread::sleep(Duration::from_millis(100));

    assert_eq!(common::stop(), vec![true, true]);
}

#[test]
fn test_move_relative() {
    let mut device = Device::try_new().unwrap();
    let mut mouse = Mouse::new(&mut device);

    let mut start_point = POINT::default();
    let mut current_point = POINT::default();
    unsafe { GetCursorPos(&raw mut start_point) };
    mouse.move_relative(MouseButton::Release, 1, 1);
    thread::sleep(Duration::from_millis(100));
    unsafe { GetCursorPos(&raw mut current_point) };

    assert_eq!(start_point.x + 1, current_point.x);
    assert_eq!(start_point.y + 1, current_point.y);
}

#[test]
fn test_move_absolute() {
    let mut device = Device::try_new().unwrap();
    let mut mouse = Mouse::new(&mut device);
    let mut current_point = POINT::default();

    mouse.move_absolute(MouseButton::Release, 500, 500, 10);
    thread::sleep(Duration::from_millis(10));
    unsafe { GetCursorPos(&raw mut current_point) };
    assert_eq!(current_point.x, 500);
    assert_eq!(current_point.y, 500);

    mouse.move_absolute(MouseButton::Release, 600, 600, 10);
    thread::sleep(Duration::from_millis(10));
    unsafe { GetCursorPos(&raw mut current_point) };
    assert_eq!(current_point.x, 600);
    assert_eq!(current_point.y, 600);

    mouse.move_absolute(MouseButton::Release, 750, 750, 10);
    thread::sleep(Duration::from_millis(10));
    unsafe { GetCursorPos(&raw mut current_point) };
    assert_eq!(current_point.x, 750);
    assert_eq!(current_point.y, 750);

    mouse.move_absolute(MouseButton::Release, 1, 1, 10);
    thread::sleep(Duration::from_millis(10));
    unsafe { GetCursorPos(&raw mut current_point) };
    assert_eq!(current_point.x, 1);
    assert_eq!(current_point.y, 1);
}
