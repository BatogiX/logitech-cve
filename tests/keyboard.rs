use std::{thread, time::Duration};

use logitech_cve::{Device, Keyboard, KeyboardButton};
use windows_sys::Win32::UI::{
    Input::KeyboardAndMouse::{VK_A, VK_B, VK_C, VK_D, VK_E, VK_F},
    WindowsAndMessaging::WH_KEYBOARD_LL,
};

mod common;

#[test]
fn test_press_and_release() {
    let mut device = Device::try_new().unwrap();
    let mut keyboard = Keyboard::new(&mut device);

    thread::spawn(|| common::start(WH_KEYBOARD_LL).expect("Failed to start keyboard hook"));
    thread::sleep(Duration::from_millis(100));
    keyboard.press(KeyboardButton::A);
    keyboard.release();
    thread::sleep(Duration::from_millis(100));
    assert_eq!(common::stop(), vec![format!("{VK_A} DOWN"), format!("{VK_A} UP")]);
}

#[test]
fn test_multi_press() {
    let mut device = Device::try_new().unwrap();
    let mut keyboard = Keyboard::new(&mut device);

    thread::spawn(|| common::start(WH_KEYBOARD_LL).expect("Failed to start keyboard hook"));
    thread::sleep(Duration::from_millis(100));
    keyboard.multi_press(
        KeyboardButton::A,
        KeyboardButton::B,
        KeyboardButton::C,
        KeyboardButton::D,
        KeyboardButton::E,
        KeyboardButton::F,
    );
    thread::sleep(Duration::from_millis(100));
    assert_eq!(
        common::stop(),
        vec![
            format!("{VK_A} DOWN"),
            format!("{VK_B} DOWN"),
            format!("{VK_C} DOWN"),
            format!("{VK_D} DOWN"),
            format!("{VK_E} DOWN"),
            format!("{VK_F} DOWN"),
        ]
    );
    keyboard.release();
}
