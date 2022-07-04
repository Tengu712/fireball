mod graphics;

use graphics::window::*;

fn main() {
    Window::new(640, 480, "Title", true).run(|| {});
}
