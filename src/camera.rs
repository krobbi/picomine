use crate::{tile::Tile, window::Window};

/// A camera for drawing in world space.
pub struct Camera;

impl Camera {
    /// Draws a tile at a position to a window.
    pub fn draw_tile(tile: Tile, x: i32, y: i32, window: &mut Window) {
        const CAMERA_X: i32 = -5;
        const CAMERA_Y: i32 = -10;

        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let mut index = (x * Tile::WIDTH - CAMERA_X
            + (y * Tile::HEIGHT - CAMERA_Y) * Window::WIDTH as i32)
            as usize;

        let buffer = window.buffer_mut();
        let color = tile.get_color();

        for _ in 0..Tile::HEIGHT {
            buffer[index..index + Tile::WIDTH as usize].fill(color);
            index += Window::WIDTH;
        }
    }
}
