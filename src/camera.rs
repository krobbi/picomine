use crate::{tile::Tile, window::Window, world::World};

/// A camera for drawing in world space.
#[derive(Clone, Copy)]
pub struct Camera {
    /// The camera's center X position.
    x: f32,

    /// The camera's center Y position.
    y: f32,
}

impl Camera {
    /// Creates a new camera.
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Sets the camera's center position.
    pub fn set_position(&mut self, x: f32, y: f32) {
        (self.x, self.y) = (x, y);
    }

    /// Draws a world to a window.
    pub fn draw_world(self, window: &mut Window) {
        #[allow(clippy::cast_precision_loss)]
        const TILE_WIDTH: f32 = Tile::WIDTH as f32;

        #[allow(clippy::cast_precision_loss)]
        const TILE_HEIGHT: f32 = Tile::HEIGHT as f32;

        #[allow(clippy::cast_precision_loss)]
        const EXTENT_X: f32 = Window::WIDTH as f32 / TILE_WIDTH / 2.0;

        #[allow(clippy::cast_precision_loss)]
        const EXTENT_Y: f32 = Window::HEIGHT as f32 / TILE_HEIGHT / 2.0;

        let (offset_x, offset_y) = (self.x - EXTENT_X, self.y - EXTENT_Y);
        let (x, y) = (offset_x.ceil(), offset_y.ceil());

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (offset_x, offset_y) = (
            ((offset_x - x) * -TILE_WIDTH) as usize,
            ((offset_y - y) * -TILE_HEIGHT) as usize,
        );

        #[allow(clippy::cast_possible_truncation)]
        let (x, y) = (x as i32, y as i32);

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let tiles_down = ((Window::HEIGHT - offset_y) / Tile::HEIGHT) as i32;

        let tiles_across = (Window::WIDTH - offset_x) / Tile::WIDTH;
        let pixels_across = tiles_across * Tile::WIDTH;

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let tiles_across = tiles_across as i32;

        let buffer = window.buffer_mut();
        let mut index = offset_x + offset_y * Window::WIDTH;

        for y in y..y + tiles_down {
            const ROW_OFFSET: usize = Tile::HEIGHT * Window::WIDTH;

            for x in x..x + tiles_across {
                let color = World::get_tile(x, y).get_color();

                for _ in 0..Tile::HEIGHT {
                    buffer[index..index + Tile::WIDTH].fill(color);
                    index += Window::WIDTH;
                }

                index = index - ROW_OFFSET + Tile::WIDTH;
            }

            index = index + ROW_OFFSET - pixels_across;
        }
    }
}
