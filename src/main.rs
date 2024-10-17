mod camera;
mod tile;
mod window;

use camera::Camera;
use tile::Tile;
use window::Window;

/// Opens a window and draws a test scene.
fn main() {
    let mut window = Window::new();
    Camera::draw_tile(Tile::Grass, 0, 0, &mut window);
    Camera::draw_tile(Tile::Grass, 1, 1, &mut window);
    Camera::draw_tile(Tile::Stone, 1, 2, &mut window);
    Camera::draw_tile(Tile::Stone, 3, 2, &mut window);
    draw_center_dot(&mut window);

    while window.is_open() {
        window.update();
    }
}

/// Draws a dot to the center of a window.
fn draw_center_dot(window: &mut Window) {
    const INDEX: usize = Window::HEIGHT / 2 * Window::WIDTH + Window::WIDTH / 2;
    const COLOR: u32 = 0xf1_f2_f1;

    window.buffer_mut()[INDEX] = COLOR;
}
