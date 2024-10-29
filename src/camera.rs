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
        let buffer = window.buffer_mut();

        let mut draw_tile =
            |x, y, mut clip_index, clip_width, clip_height, buffer_index: &mut usize| {
                let texture = world.get_tile(x, y).texture();

                for _ in 0..clip_height {
                    buffer[*buffer_index..*buffer_index + clip_width]
                        .copy_from_slice(&texture[clip_index..clip_index + clip_width]);

                    clip_index += Tile::WIDTH;
                    *buffer_index += Window::WIDTH;
                }
            };

        let mut buffer_index = 0;
        let (left_x, top_y) = (self.x.floor(), self.y.floor());

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (left_clip_x, top_clip_y) = (
            ((self.x - left_x) * Self::TILE_WIDTH) as usize,
            ((self.y - top_y) * Self::TILE_HEIGHT) as usize,
        );

        let (left_clip_width, top_clip_height) =
            (Tile::WIDTH - left_clip_x, Tile::HEIGHT - top_clip_y);

        let (center_width, center_height) = (
            (Window::WIDTH - left_clip_width) / Tile::WIDTH,
            (Window::HEIGHT - top_clip_height) / Tile::HEIGHT,
        );

        let (right_clip_width, bottom_clip_height) = (
            Window::WIDTH - left_clip_width - center_width * Tile::WIDTH,
            Window::HEIGHT - top_clip_height - center_height * Tile::HEIGHT,
        );

        #[allow(clippy::cast_possible_truncation)]
        let (left_x, top_y) = (left_x as i32, top_y as i32);

        let (center_left_x, center_top_y) = (left_x + 1, top_y + 1);

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let (right_x, bottom_y) = (
            center_left_x + center_width as i32,
            center_top_y + center_height as i32,
        );

        let top_clip_index = top_clip_y * Tile::WIDTH;

        let mut draw_strip = |x, clip_x, clip_width| {
            const COLUMN_OFFSET: usize = Window::HEIGHT * Window::WIDTH;

            draw_tile(
                x,
                top_y,
                top_clip_index + clip_x,
                clip_width,
                top_clip_height,
                &mut buffer_index,
            );

            for y in center_top_y..bottom_y {
                draw_tile(x, y, clip_x, clip_width, Tile::HEIGHT, &mut buffer_index);
            }

            draw_tile(
                x,
                bottom_y,
                clip_x,
                clip_width,
                bottom_clip_height,
                &mut buffer_index,
            );

            buffer_index -= COLUMN_OFFSET - clip_width;
        };

        draw_strip(left_x, left_clip_x, left_clip_width);

        for x in center_left_x..right_x {
            draw_strip(x, 0, Tile::WIDTH);
        }

        draw_strip(right_x, 0, right_clip_width);
    }
}
