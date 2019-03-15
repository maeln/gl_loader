#![cfg(target_os = "linux")]

pub fn get_proc_address(func: &str) -> *const () {
    0 as *const _
}
