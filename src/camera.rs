use crate::{tile::Tile, window::Window, world::World};

/// A camera with a world position.
#[derive(Clone, Copy)]
pub struct Camera {
    /// The camera's left edge in world space.
    x: f32,

    /// The camera's top edge in world space.
    y: f32,
}

impl Camera {
    /// A tile's width in pixels as a floating-point number.
    #[allow(clippy::cast_precision_loss)]
    const TILE_WIDTH: f32 = Tile::WIDTH as f32;

    /// A tile's height in pixels as a floating-point number.
    #[allow(clippy::cast_precision_loss)]
    const TILE_HEIGHT: f32 = Tile::HEIGHT as f32;

    /// Creates a new camera.
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Sets the camera's center world position.
    pub fn set_position(&mut self, x: f32, y: f32) {
        #[allow(clippy::cast_precision_loss)]
        const OFFSET_X: f32 = Window::WIDTH as f32 / Camera::TILE_WIDTH / 2.0;

        #[allow(clippy::cast_precision_loss)]
        const OFFSET_Y: f32 = Window::HEIGHT as f32 / Camera::TILE_HEIGHT / 2.0;

        (self.x, self.y) = (x - OFFSET_X, y - OFFSET_Y);
    }

    /// Returns a tile position from a screen position relative to the camera.
    pub fn screen_to_tile_position(self, x: f32, y: f32) -> (i32, i32) {
        #[allow(clippy::cast_possible_truncation)]
        (
            (self.x + x / Self::TILE_WIDTH).floor() as i32,
            (self.y + y / Self::TILE_HEIGHT).floor() as i32,
        )
    }

    /// Draws a world to a window.
    pub fn draw_world(self, world: &mut World, window: &mut Window) {
        let (tile_x, tile_y) = (self.x.ceil(), self.y.ceil());

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (pixel_x, pixel_y) = (
            ((self.x - tile_x) * -Self::TILE_WIDTH) as usize,
            ((self.y - tile_y) * -Self::TILE_HEIGHT) as usize,
        );

        #[allow(clippy::cast_possible_truncation)]
        let (tile_x, tile_y) = (tile_x as i32, tile_y as i32);

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let tiles_down = ((Window::HEIGHT - pixel_y) / Tile::HEIGHT) as i32;

        let tiles_across = (Window::WIDTH - pixel_x) / Tile::WIDTH;
        let pixels_across = tiles_across * Tile::WIDTH;

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let tiles_across = tiles_across as i32;

        let buffer = window.buffer_mut();
        let mut buffer_index = pixel_x + pixel_y * Window::WIDTH;

        for tile_y in tile_y..tile_y + tiles_down {
            const ROW_OFFSET: usize = Tile::HEIGHT * Window::WIDTH;

            for tile_x in tile_x..tile_x + tiles_across {
                let texture = world.get_tile(tile_x, tile_y).texture();
                let mut texture_index = 0;

                for _ in 0..Tile::HEIGHT {
                    buffer[buffer_index..buffer_index + Tile::WIDTH]
                        .copy_from_slice(&texture[texture_index..texture_index + Tile::WIDTH]);

                    texture_index += Tile::WIDTH;
                    buffer_index += Window::WIDTH;
                }

                buffer_index = buffer_index - ROW_OFFSET + Tile::WIDTH;
            }

            buffer_index = buffer_index + ROW_OFFSET - pixels_across;
        }
    }
}
