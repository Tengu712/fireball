mod graphics;

use graphics::{vulkan::*, window::*};

fn main() {
    #[cfg(target_os = "windows")]
    let window = windows::Window::new(640, 480, "Title", true);
    #[cfg(target_os = "linux")]
    let window = XcbWindow::new(640, 480, "Title", true);
    let _ = Vulkan::new("appname", &window);
    window.run(|| {});
}
