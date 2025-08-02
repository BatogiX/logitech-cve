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
        CallNextHookEx, GetMessageW, HHOOK, KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT, SetWindowsHookExW,
        UnhookWindowsHookEx, WH_MOUSE_LL, WINDOWS_HOOK_ID, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP,
        WM_MOUSEWHEEL, WM_SYSKEYDOWN, WM_SYSKEYUP,
    },
};

#[allow(non_snake_case)]
const fn GET_WHEEL_DELTA_WPARAM(wParam: u32) -> i16 {
    ((wParam >> 16) & 0xFFFF) as i16
}

static mut HOOK_HANDLE: HHOOK = std::ptr::null_mut();
static RUNNING: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(false)));
static RESULT: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));

#[allow(non_snake_case)]
extern "system" fn LowLevelMouseProc(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> isize {
    if nCode >= 0 {
        #[allow(clippy::cast_possible_truncation)]
        match wParam as u32 {
            WM_LBUTTONUP => {
                RESULT.lock().unwrap().push("LBUTTON UP".to_owned());
            }
            WM_LBUTTONDOWN => {
                RESULT.lock().unwrap().push("LBUTTON DOWN".to_owned());
            }
            WM_MOUSEWHEEL => {
                let pMouseStruct = unsafe { *(lParam as *const MSLLHOOKSTRUCT) };
                let wheelDelta = GET_WHEEL_DELTA_WPARAM(pMouseStruct.mouseData);

                if wheelDelta > 0 {
                    RESULT.lock().unwrap().push("WHEEL UP".to_owned());
                } else {
                    RESULT.lock().unwrap().push("WHEEL DOWN".to_owned());
                }
            }
            _ => {}
        }
    }

    unsafe { CallNextHookEx(std::ptr::null_mut(), nCode, wParam, lParam) }
}

#[allow(non_snake_case)]
extern "system" fn LowLevelKeyboardProc(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> isize {
    if nCode >= 0 {
        let kb_struct = unsafe { &*(lParam as *const KBDLLHOOKSTRUCT) };

        #[allow(clippy::cast_possible_truncation)]
        match wParam as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                RESULT.lock().unwrap().push(format!("{} DOWN", kb_struct.vkCode));
            }
            WM_KEYUP | WM_SYSKEYUP => {
                RESULT.lock().unwrap().push(format!("{} UP", kb_struct.vkCode));
            }
            _ => {}
        }
    }

    unsafe { CallNextHookEx(std::ptr::null_mut(), nCode, wParam, lParam) }
}

pub fn start(idhook: WINDOWS_HOOK_ID) -> Result<(), String> {
    unsafe {
        if idhook == WH_MOUSE_LL {
            HOOK_HANDLE = SetWindowsHookExW(idhook, Some(LowLevelMouseProc), GetModuleHandleW(ptr::null()), 0);
        } else {
            HOOK_HANDLE = SetWindowsHookExW(idhook, Some(LowLevelKeyboardProc), GetModuleHandleW(ptr::null()), 0);
        }

        if HOOK_HANDLE.is_null() {
            return Err("Failed to set hook".to_string());
        }

        RUNNING.store(true, Ordering::SeqCst);

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

pub fn stop() -> Vec<String> {
    RUNNING.store(false, Ordering::SeqCst);

    unsafe {
        if !HOOK_HANDLE.is_null() {
            UnhookWindowsHookEx(HOOK_HANDLE);
            HOOK_HANDLE = std::ptr::null_mut();
        }
    }

    let result = RESULT.lock().unwrap().clone();
    RESULT.lock().unwrap().clear();
    result
}
