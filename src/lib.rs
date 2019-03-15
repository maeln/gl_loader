#[cfg(target_os = "linux")]
#[path = "linux.rs"]
pub mod loader;

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
pub mod loader;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
pub mod loader;
