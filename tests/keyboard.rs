use core::time::Duration;
use logitech_cve::{
    device::Device,
    keyboard::{Key, Keyboard},
};
use std::thread;
use windows_sys::Win32::UI::{
    Input::KeyboardAndMouse::{VK_A, VK_B, VK_C, VK_D, VK_E, VK_F},
    WindowsAndMessaging::WH_KEYBOARD_LL,
};

mod common;

#[test]
fn press_and_release() {
    let device = Device::try_new().unwrap();
    let keyboard = Keyboard::new(&device);

    thread::spawn(|| common::start(WH_KEYBOARD_LL));
    thread::sleep(Duration::from_millis(100));
    keyboard.press(Key::A);
    keyboard.release();
    thread::sleep(Duration::from_millis(100));
    assert_eq!(common::stop(), vec![format!("{VK_A} DOWN"), format!("{VK_A} UP")]);
}

#[test]
fn multi_press() {
    let device = Device::try_new().unwrap();
    let keyboard = Keyboard::new(&device);

    thread::spawn(|| common::start(WH_KEYBOARD_LL));
    thread::sleep(Duration::from_millis(100));
    keyboard.multi_press(Key::A, Key::B, Key::C, Key::D, Key::E, Key::F);
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
