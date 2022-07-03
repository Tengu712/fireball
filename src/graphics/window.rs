#[cfg(target_os="windows")]
mod windows;

use std::os::raw::c_void;

pub struct Window {
    window: *const c_void,
}

