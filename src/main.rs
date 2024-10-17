mod tile;
mod window;

use tile::Tile;
use window::Window;

/// Opens a window and draws a test scene.
fn main() {
    let mut window = Window::new();

    {
        let buffer = window.buffer_mut();
        draw_tile_colors(buffer);
        draw_center_dot(buffer);
    }

    while window.is_open() {
        window.update();
    }
}

/// Draws each tile's color to a framebuffer.
fn draw_tile_colors(buffer: &mut [u32]) {
    buffer[0] = Tile::Grass.get_color();
    buffer[1] = Tile::Stone.get_color();
}

/// Draws a dot at the center of a window-sized framebuffer.
fn draw_center_dot(buffer: &mut [u32]) {
    const INDEX: usize = Window::HEIGHT / 2 * Window::WIDTH + Window::WIDTH / 2;
    const COLOR: u32 = 0xf1_f2_f1;

    buffer[INDEX] = COLOR;
}
