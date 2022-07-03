mod graphics;

use graphics::window::Window;

fn main() {
    Window::new(640, 480, "Title", true).run(|| {});
}
