use core::time::Duration;
use logitech_cve::{
    device::Device,
    keyboard::{Key, Keyboard},
};
use std::thread;
use windows_sys::Win32::UI::{
    Input::KeyboardAndMouse::{
        VK_1, VK_A, VK_B, VK_C, VK_D, VK_E, VK_F, VK_H, VK_L, VK_LSHIFT, VK_O, VK_OEM_COMMA, VK_R, VK_SPACE, VK_W,
    },
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

#[test]
fn type_string() {
    let device = Device::try_new().unwrap();
    let keyboard = Keyboard::new(&device);

    thread::spawn(|| common::start(WH_KEYBOARD_LL));
    thread::sleep(Duration::from_millis(100));
    keyboard.type_string("Hello, World!", 50).expect("Should be OK");
    thread::sleep(Duration::from_millis(100));
    assert_eq!(
        common::stop(),
        vec![
            format!("{VK_LSHIFT} DOWN"),
            format!("{VK_H} DOWN"),
            format!("{VK_LSHIFT} UP"),
            format!("{VK_H} UP"),
            format!("{VK_E} DOWN"),
            format!("{VK_E} UP"),
            format!("{VK_L} DOWN"),
            format!("{VK_L} UP"),
            format!("{VK_L} DOWN"),
            format!("{VK_L} UP"),
            format!("{VK_O} DOWN"),
            format!("{VK_O} UP"),
            format!("{VK_OEM_COMMA} DOWN"),
            format!("{VK_OEM_COMMA} UP"),
            format!("{VK_SPACE} DOWN"),
            format!("{VK_SPACE} UP"),
            format!("{VK_LSHIFT} DOWN"),
            format!("{VK_W} DOWN"),
            format!("{VK_LSHIFT} UP"),
            format!("{VK_W} UP"),
            format!("{VK_O} DOWN"),
            format!("{VK_O} UP"),
            format!("{VK_R} DOWN"),
            format!("{VK_R} UP"),
            format!("{VK_L} DOWN"),
            format!("{VK_L} UP"),
            format!("{VK_D} DOWN"),
            format!("{VK_D} UP"),
            format!("{VK_LSHIFT} DOWN"),
            format!("{VK_1} DOWN"),
            format!("{VK_LSHIFT} UP"),
            format!("{VK_1} UP"),
        ]
    );
    keyboard.release();
}
