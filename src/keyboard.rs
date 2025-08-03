use crate::device::Device;
use core::time::Duration;
use std::thread;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Key {
    A = 0x4,
    B = 0x5,
    C = 0x6,
    D = 0x7,
    E = 0x8,
    F = 0x9,
    G = 0xA,
    H = 0xB,
    I = 0xC,
    J = 0xD,
    K = 0xE,
    L = 0xF,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    N1 = 0x1E,
    N2 = 0x1F,
    N3 = 0x20,
    N4 = 0x21,
    N5 = 0x22,
    N6 = 0x23,
    N7 = 0x24,
    N8 = 0x25,
    N9 = 0x26,
    N0 = 0x27,
    Enter = 0x28,
    Esc = 0x29,
    BackSpace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    SquareBracketLeft = 0x2F,
    SquareBracketRight = 0x30,
    BackSlash = 0x31,
    BackSlash_ = 0x32,
    Column = 0x33,
    Quote = 0x34,
    BackTick = 0x35,
    Comma = 0x36,
    Period = 0x37,
    Slash = 0x38,
    Cap = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    Snapshot = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Del = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
    Numlock = 0x53,
    NumpadDiv = 0x54,
    NumpadMul = 0x55,
    NumpadMinus = 0x56,
    NumpadPlus = 0x57,
    NumpadEnter = 0x58,
    Numpad1 = 0x59,
    Numpad2 = 0x5A,
    Numpad3 = 0x5B,
    Numpad4 = 0x5C,
    Numpad5 = 0x5D,
    Numpad6 = 0x5E,
    Numpad7 = 0x5F,
    Numpad8 = 0x60,
    Numpad9 = 0x61,
    Numpad0 = 0x62,
    NumpadDec = 0x63,
    Apps = 0x65,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6A,
    F16 = 0x6B,
    F17 = 0x6C,
    F18 = 0x6D,
    F19 = 0x6E,
    F20 = 0x6F,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Rwin = 0x8C,
    F24_ = 0x94,
    Lctrl = 0xE0,
    Lshift = 0xE1,
    Lalt = 0xE2,
    Lwin = 0xE3,
    Rctrl = 0xE4,
    Rshift = 0xE5,
    Ralt = 0xE6,
    Rwin_ = 0xE7,
    NONE = 0x0,
}

impl From<Key> for u8 {
    #[inline]
    fn from(button: Key) -> Self {
        button as Self
    }
}

/// A struct for controlling a virtual keyboard.
///
/// It holds a reference to a `Device` which is used to send the keyboard commands.
pub struct Keyboard<'a> {
    /// A reference to the device used to send keyboard commands.
    device: &'a Device,
}

impl<'a> Keyboard<'a> {
    /// Creates a new [`Keyboard`].
    #[inline]
    #[must_use]
    pub const fn new(device: &'a Device) -> Self {
        Self { device }
    }

    /// Presses a single keyboard button.
    ///
    /// The button is held down until a `release()` or `multi_press()` with `Key::NONE` is called.
    ///
    /// # Arguments
    ///
    /// * `button` - The `Key` to press.
    #[inline]
    pub fn press(&self, button: Key) {
        self.device
            .call_keyboard(button, Key::NONE, Key::NONE, Key::NONE, Key::NONE, Key::NONE);
    }

    /// Releases all currently pressed keyboard buttons.
    ///
    /// This effectively sends a "no keys pressed" command to the device.
    #[inline]
    pub fn release(&self) {
        self.device
            .call_keyboard(Key::NONE, Key::NONE, Key::NONE, Key::NONE, Key::NONE, Key::NONE);
    }

    /// Presses and releases a single keyboard button.
    ///
    /// The button is pressed down, held for the specified duration, then released.
    ///
    /// # Arguments
    ///
    /// * `button` - The `Key` to press and release.
    /// * `millis` - The duration in milliseconds to hold the button down before releasing it.
    #[inline]
    pub fn press_and_release(&self, button: Key, millis: u64) {
        self.device
            .call_keyboard(button, Key::NONE, Key::NONE, Key::NONE, Key::NONE, Key::NONE);
        thread::sleep(Duration::from_millis(millis));
        self.device
            .call_keyboard(Key::NONE, Key::NONE, Key::NONE, Key::NONE, Key::NONE, Key::NONE);
    }

    /// Presses up to six keyboard buttons simultaneously.
    ///
    /// This can be used for pressing modifier keys and other keys at the same time.
    ///
    /// # Arguments
    ///
    /// * `button1` - The first `Key` to press.
    /// * `button2` - The second `Key` to press.
    /// * `button3` - The third `Key` to press.
    /// * `button4` - The fourth `Key` to press.
    /// * `button5` - The fifth `Key` to press.
    /// * `button6` - The sixth `Key` to press.
    #[inline]
    pub fn multi_press(&self, button1: Key, button2: Key, button3: Key, button4: Key, button5: Key, button6: Key) {
        self.device
            .call_keyboard(button1, button2, button3, button4, button5, button6);
    }
}
