extern crate libc;

use std::ffi::CString;

extern "C" {
    fn open_gl() -> libc::c_int;
    fn init_glad() -> libc::c_int;
    fn close_gl();
    fn get_proc(name: *const libc::c_char) -> *const libc::c_void;
}

pub fn init_gl() -> i32 {
    unsafe { open_gl() }
}

pub fn start_glad() -> i32 {
    unsafe { init_glad() }
}

pub fn end_gl() {
    unsafe {
        close_gl();
    }
}

pub fn get_proc_addr(name: *const libc::c_char) -> *const libc::c_void {
    unsafe { get_proc(name) }
}

pub fn get_proc_address(func: &str) -> *const () {
    let c_str = CString::new(func.as_bytes()).unwrap();
    let str_ptr = c_str.as_ptr();
    let p = get_proc_addr(str_ptr) as *const ();
    p
}
