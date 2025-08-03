# logitech-cve

A Rust library for interacting with Logitech virtual driver. 

# Usage
## Mouse
### Example
```rust
use logitech_cve::{
    device::Device,
    mouse::{Mouse, MouseButton}
};

fn main() {
	let device = Device::try_new().expect("Logitech G HUB 2021.11.1775 is not installed"); // Req for Driver Handling 
	let mouse = Mouse::new(&device); // Init Mouse

	mouse.move_relative(100, 100); // (x,y) relative
    mouse.move_absolute(100, 100); // (x,y) absolute
	
	mouse.wheel(1); // Scroll up
	mouse.wheel(-1); // Scroll down
	
	mouse.click(MouseButtons::Left, 120); // Press and sleeps for 120ms before release

	// OR

	mouse.press(MouseButtons::Left); // Press
	std::thread::sleep(std::time::Duration(100)); // Custom sleep
	mouse.release(); // Release
}
```

## Keyboard
### Example
```rust
use logitech_cve::{
    device::Device,
    keyboard::{Keyboard, Key}
};

fn main() {
	let device = Device::try_new().expect("Logitech G HUB 2021.11.1775 is not installed"); // Req for Driver Handling 
	let keyboard = Keyboard::new(&device); // Init Keyboard

	keyboard.press_and_release(Key::A, 120); // Press and sleeps for 120ms before release

    // Press multiple buttons
	keyboard.multi_press(Key::A, Key::B, Key::C, Key::NONE, Key::NONE, Key::NONE); 
	std::thread::sleep(std::time::Duration(100)); // Custom sleep
	keyboard.release(); // Release all buttons
}
```

## Requirements

- Logitech G HUB 2021.11.1775
