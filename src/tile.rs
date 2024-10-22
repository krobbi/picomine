/// The textures for all tiles.
static mut TEXTURES: [u32; Tile::COUNT * Tile::AREA] = [0; Tile::COUNT * Tile::AREA];

/// A tile of a world.
#[derive(Clone, Copy)]
pub enum Tile {
    /// A grass tile.
    Grass,

    /// A stone tile.
    Stone,
}

impl Tile {
    /// A tile's width in pixels.
    pub const WIDTH: usize = 16;

    /// A tile's height in pixels.
    pub const HEIGHT: usize = Self::WIDTH;

    /// A tile's area in pixels.
    const AREA: usize = Self::WIDTH * Self::HEIGHT;

    /// The number of tiles.
    const COUNT: usize = 2;

    /// Loads textures for all tiles.
    pub fn load_textures() {
        Self::Grass.load_texture();
        Self::Stone.load_texture();
    }

    /// Returns the tile's texture as a slice.
    pub fn texture(self) -> &'static [u32] {
        self.texture_mut()
    }

    /// Returns the tile's color.
    fn get_color(self) -> u32 {
        match self {
            Self::Grass => 0x16_9e_26,
            Self::Stone => 0xa9_b0_b0,
        }
    }

    /// Returns the tile's texture as a mutable slice.
    fn texture_mut(self) -> &'static mut [u32] {
        let index = self as usize * Self::AREA;

        // SAFETY: Mutable statics are unsafe because they may be accessed by
        // multiple threads and cause a data race. PicoMine is currently
        // single-threaded so this should be impossible. The textures array
        // consists only of primitive integers, which are always in a valid
        // state. If a data race did occur, the only effect should be tile
        // textures briefly appearing visually corrupted.
        unsafe { &mut TEXTURES[index..index + Self::AREA] }
    }

    /// Loads the tile's texture.
    fn load_texture(self) {
        /// Multiplies a color by a factor.
        fn multiply_color(color: u32, factor: f32) -> u32 {
            multiply_channel(color, 16, factor)
                | multiply_channel(color, 8, factor)
                | multiply_channel(color, 0, factor)
        }

        /// Multiplies a color channel by a factor.
        fn multiply_channel(color: u32, shift: u32, factor: f32) -> u32 {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_precision_loss,
                clippy::cast_sign_loss
            )]
            {
                u32::clamp(
                    (((color >> shift) & 0xff) as f32 * factor) as u32,
                    0x00,
                    0xff,
                ) << shift
            }
        }

        /// Returns a pseudo-random number from a seed.
        fn randomize(seed: usize) -> f32 {
            const MAGIC: usize = 877;
            const MASK: usize = 0xff;

            #[allow(clippy::cast_precision_loss)]
            {
                (seed.wrapping_mul(MAGIC) & MASK) as f32 / MASK as f32
            }
        }

        let texture = self.texture_mut();
        let color = self.get_color();
        let light_color = multiply_color(color, 1.25);
        let dark_color = multiply_color(color, 0.75);

        for x in 1..Self::WIDTH - 1 {
            texture[x] = light_color;
            texture[Self::AREA - x - 1] = dark_color;
        }

        for y in 1..Self::HEIGHT - 1 {
            texture[y * Self::WIDTH] = light_color;
            texture[y * Self::WIDTH + Self::WIDTH - 1] = dark_color;

            for x in 1..Self::WIDTH - 1 {
                let index = x + y * Self::WIDTH;
                let factor = 1.0 + (randomize(index) - 0.5) * 0.1;
                texture[index] = multiply_color(color, factor);
            }
        }

        texture[0] = multiply_color(color, 1.5);
        texture[Self::WIDTH - 1] = color;
        texture[Self::AREA - Self::WIDTH] = color;
        texture[Self::AREA - 1] = multiply_color(color, 0.5);
    }
}
