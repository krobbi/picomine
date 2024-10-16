mod window;

use window::Window;

/// Open a window.
fn main() {
    let mut window = Window::new();

    while window.is_open() {
        window.update();
    }
}
