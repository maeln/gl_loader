extern crate libc;

#[link(name = "opengl32")]
extern "C" {
    pub fn open_gl() -> i32;
    pub fn close_gl();
    pub fn get_proc(name: *const libc::c_char) -> *const libc::c_void;
}
