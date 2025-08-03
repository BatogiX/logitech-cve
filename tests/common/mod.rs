use core::{
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};
use std::sync::{LazyLock, Mutex};
extern crate alloc;
use alloc::sync::Arc;

use windows_sys::Win32::{
    Foundation::{LPARAM, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        CallNextHookEx, GetMessageW, HHOOK, KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT, SetWindowsHookExW,
        UnhookWindowsHookEx, WH_MOUSE_LL, WINDOWS_HOOK_ID, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP,
        WM_MOUSEWHEEL, WM_SYSKEYDOWN, WM_SYSKEYUP,
    },
};

#[expect(non_snake_case, reason = "Windows API style")]
const fn GET_WHEEL_DELTA_WPARAM(wParam: u32) -> i16 {
    ((wParam >> 16) & 0xFFFF) as i16
}

static mut HOOK_HANDLE: HHOOK = ptr::null_mut();
static RUNNING: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(false)));
static RESULT: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));

#[expect(non_snake_case, reason = "Windows API style")]
extern "system" fn LowLevelMouseProc(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> isize {
    if nCode >= 0 {
        #[expect(clippy::cast_possible_truncation, reason = "Windows API uses u32 for wParam")]
        match wParam as u32 {
            WM_LBUTTONUP => {
                RESULT.lock().unwrap().push("LBUTTON UP".to_owned());
            }
            WM_LBUTTONDOWN => {
                RESULT.lock().unwrap().push("LBUTTON DOWN".to_owned());
            }
            WM_MOUSEWHEEL => {
                // SAFETY: lParam is guaranteed by Windows to be a valid pointer to MSLLHOOKSTRUCT.
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

    // SAFETY: CallNextHookEx is called with the same parameters received by the hook procedure, as required by the Windows API.
    unsafe { CallNextHookEx(ptr::null_mut(), nCode, wParam, lParam) }
}

#[expect(non_snake_case, reason = "Windows API style")]
extern "system" fn LowLevelKeyboardProc(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> isize {
    if nCode >= 0 {
        // SAFETY: lParam is guaranteed by Windows to be a valid pointer to KBDLLHOOKSTRUCT.
        let kb_struct = unsafe { &*(lParam as *const KBDLLHOOKSTRUCT) };

        #[expect(clippy::cast_possible_truncation, reason = "Windows API uses u32 for wParam")]
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

    // SAFETY: CallNextHookEx is called with the same parameters received by the hook procedure, as required by the Windows API.
    unsafe { CallNextHookEx(ptr::null_mut(), nCode, wParam, lParam) }
}

pub fn start(idhook: WINDOWS_HOOK_ID) {
    // SAFETY: GetModuleHandleW is called with a null pointer to get a handle to the current process's module, which is safe here.
    let lpmodulename = unsafe { GetModuleHandleW(ptr::null()) };
    let hook_handle = if idhook == WH_MOUSE_LL {
        // SAFETY: SetWindowsHookExW and GetModuleHandleW are FFI calls, and HOOK_HANDLE is a static mut.
        unsafe { SetWindowsHookExW(idhook, Some(LowLevelMouseProc), lpmodulename, 0) }
    } else {
        // SAFETY: SetWindowsHookExW and GetModuleHandleW are FFI calls, and HOOK_HANDLE is a static mut.
        unsafe { SetWindowsHookExW(idhook, Some(LowLevelKeyboardProc), lpmodulename, 0) }
    };

    // SAFETY: Writing to static mut HOOK_HANDLE. This is safe assuming single-threaded access or proper synchronization.
    unsafe {
        HOOK_HANDLE = hook_handle;
    };

    RUNNING.store(true, Ordering::SeqCst);

    // Message loop
    let mut msg = MSG::default();
    while RUNNING.load(Ordering::SeqCst) {
        // SAFETY: GetMessageW is an FFI call and requires a mutable pointer to MSG.
        let result = unsafe { GetMessageW(&raw mut msg, ptr::null_mut(), 0, 0) };
        if result == -1 {
            break;
        }
    }
}

pub fn stop() -> Vec<String> {
    RUNNING.store(false, Ordering::SeqCst);

    // SAFETY: Accessing HOOK_HANDLE is safe here because stop() is only called from a single thread and HOOK_HANDLE is only modified here and in start().
    let hook_handle_is_not_null = unsafe { !HOOK_HANDLE.is_null() };
    if hook_handle_is_not_null {
        // SAFETY: Accessing HOOK_HANDLE is safe here because stop() is only called from a single thread and HOOK_HANDLE is only modified here and in start().
        let hook_handle = unsafe { HOOK_HANDLE };
        // SAFETY: UnhookWindowsHookEx is called with a valid hook handle, as ensured by the previous check.
        unsafe {
            UnhookWindowsHookEx(hook_handle);
        };
        // SAFETY: Resetting HOOK_HANDLE to null is safe as we have just unhooked it.
        unsafe {
            HOOK_HANDLE = ptr::null_mut();
        };
    }

    let result = RESULT.lock().unwrap().clone();
    RESULT.lock().unwrap().clear();
    result
}
