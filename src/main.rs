mod graphics;

use graphics::window::*;
use graphics::vulkan::*;

fn main() {
    let window = Window::new(640, 480, "Title", true);
    let _ = Vulkan::new("appname");
    window.run(|| {});
}
