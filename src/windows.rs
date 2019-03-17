#![cfg(target_os = "windows")]
extern crate win;

/*
#[link(name = "opengl32")]
extern "C" {}
*/

use std::ffi::CString;
#[link(name = "opengl32")]
pub fn get_proc_address(func: &str) -> *const () {
    unsafe {
        win::open_gl();
        let c_str = CString::new(func.as_bytes()).unwrap().as_ptr();
        let p = win::get_proc(c_str) as *const ();
        println!("fun: {}, p1: {:?}", func, p);
        p
    }
}
