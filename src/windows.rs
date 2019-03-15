#![cfg(target_os = "windows")]

use winapi::um::wingdi;

pub fn get_proc_address(func: &str) -> *const () {
    wingdi::wglGetProcAddress(func) as *const _
}
