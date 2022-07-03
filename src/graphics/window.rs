#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="linux")]
mod linux;

use std::os::raw::c_void;

pub struct Window {
    window: *const c_void,
}

