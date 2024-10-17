use crate::{tile::Tile, window::Window};

/// A camera for drawing in world space.
#[derive(Clone, Copy)]
pub struct Camera {
    /// The camera's left edge in world pixel space.
    x: i32,

    /// The camera's top edge in world pixel space.
    y: i32,
}

impl Camera {
    /// Creates a new camera.
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Sets the camera's center position.
    pub fn set_position(&mut self, x: f32, y: f32) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        const OFFSET_X: i32 = Window::WIDTH as i32 / 2;

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        const OFFSET_Y: i32 = Window::HEIGHT as i32 / 2;

        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        {
            self.x = (x * Tile::WIDTH as f32) as i32 - OFFSET_X;
            self.y = (y * Tile::HEIGHT as f32) as i32 - OFFSET_Y;
        }
    }

    /// Draws a tile at a position to a window.
    pub fn draw_tile(self, tile: Tile, x: i32, y: i32, window: &mut Window) {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let mut index = (x * Tile::WIDTH - self.x
            + (y * Tile::HEIGHT - self.y) * Window::WIDTH as i32) as usize;

        let buffer = window.buffer_mut();
        let color = tile.get_color();

        for _ in 0..Tile::HEIGHT {
            buffer[index..index + Tile::WIDTH as usize].fill(color);
            index += Window::WIDTH;
        }
    }
}
