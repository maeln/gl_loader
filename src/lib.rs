use core_foundation::base::TCFType;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::string::CFString;
use std::str::FromStr;

// (very) heavely borrowed from glutin
#[cfg(target_os = "macos")]
pub fn get_proc_address(func: &str) -> *const () {
    let symbole_name: CFString = FromStr::from_str(func).unwrap();
    let framework_name: CFString = FromStr::from_str("com.apple.opengl").unwrap();

    let framework =
        unsafe { CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef()) };

    let symbole_ptr =
        unsafe { CFBundleGetFunctionPointerForName(framework, symbole_name.as_concrete_TypeRef()) };

    symbole_ptr as *const _
}

#[cfg(target_os = "windows")]
pub fn get_proc_address(func: &str) -> *const () {
    0 as *const _
}

#[cfg(target_os = "linux")]
pub fn get_proc_address(func: &str) -> *const () {
    0 as *const _
}