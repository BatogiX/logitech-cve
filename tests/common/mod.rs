use std::{
    mem, ptr,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use windows_sys::Win32::{
    Foundation::{LPARAM, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        CallNextHookEx, GetMessageW, HHOOK, MSG, MSLLHOOKSTRUCT, SetWindowsHookExW, UnhookWindowsHookEx,
        WINDOWS_HOOK_ID, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEWHEEL,
    },
};

macro_rules! get_wheel_delta_wparam {
    ($wparam:expr) => {
        (($wparam >> 16) & 0xFFFF) as i16
    };
}

static mut HOOK_HANDLE: HHOOK = std::ptr::null_mut();
static RUNNING: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(false)));
static RESULT: LazyLock<Mutex<Vec<bool>>> = LazyLock::new(|| Mutex::new(vec![]));

#[allow(non_snake_case)]
extern "system" fn LowLevelMouseProc(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> isize {
    if nCode >= 0 {
        #[allow(clippy::cast_possible_truncation)]
        match wParam as u32 {
            WM_LBUTTONUP | WM_LBUTTONDOWN | WM_MOUSEWHEEL => {
                RESULT.lock().unwrap().push(true);
            }
            _ => {}
        }
    }

    unsafe { CallNextHookEx(std::ptr::null_mut(), nCode, wParam, lParam) }
}

pub fn start(idhook: WINDOWS_HOOK_ID) -> Result<(), String> {
    unsafe {
        HOOK_HANDLE = SetWindowsHookExW(idhook, Some(LowLevelMouseProc), GetModuleHandleW(ptr::null()), 0);

        if HOOK_HANDLE.is_null() {
            return Err("Failed to set mouse hook".to_string());
        }

        RUNNING.store(true, Ordering::SeqCst);
        println!("Mouse hook started");

        // Message loop
        let mut msg: MSG = mem::zeroed();
        while RUNNING.load(Ordering::SeqCst) {
            let result = GetMessageW(&raw mut msg, std::ptr::null_mut(), 0, 0);
            if result == -1 {
                break;
            }
        }
    }

    Ok(())
}

pub fn stop() -> Vec<bool> {
    RUNNING.store(false, Ordering::SeqCst);

    unsafe {
        if !HOOK_HANDLE.is_null() {
            UnhookWindowsHookEx(HOOK_HANDLE);
            HOOK_HANDLE = std::ptr::null_mut();
        }
    }

    println!("Mouse hook stopped.");
    let result = RESULT.lock().unwrap().clone();
    RESULT.lock().unwrap().clear();
    result
}
