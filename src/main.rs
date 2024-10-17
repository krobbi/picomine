mod tile;
mod window;

use tile::Tile;
use window::Window;

/// Opens a window and draws a test scene.
fn main() {
    let mut window = Window::new();

    draw_tile_colors(window.buffer_mut());

    while window.is_open() {
        window.update();
    }
}

/// Draws each tile's color to a framebuffer.
fn draw_tile_colors(buffer: &mut [u32]) {
    buffer[0] = Tile::Grass.get_color();
    buffer[1] = Tile::Stone.get_color();
}
