#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::*;
#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
pub use linux::*;

pub trait WindowImpl {
    fn new(width: i32, height: i32, title: &'static str, is_windowed: bool) -> Self;
    fn run(self, f: fn());
}
