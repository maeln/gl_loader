extern crate libc;

use std::ffi::CString;

extern "C" {
    fn open_gl() -> libc::c_int;
    fn close_gl();
    fn get_proc(name: *const libc::c_char) -> *const libc::c_void;
}

pub fn init_gl() -> i32 {
    unsafe { open_gl() }
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
    let c_str = CString::new(func.as_bytes()).unwrap().as_ptr();
    let p = get_proc_addr(c_str) as *const ();
    println!("fun: {}, p1: {:?}", func, p);
    p
}
