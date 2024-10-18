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

    /// Returns the tile's color.
    pub fn get_color(self) -> u32 {
        match self {
            Self::Grass => 0x16_9e_26,
            Self::Stone => 0xa9_b0_b0,
        }
    }
}
