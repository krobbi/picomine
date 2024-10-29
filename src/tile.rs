use std::path::PathBuf;

use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::resources;

/// The total number of pixels for all tile textures.
const TEXTURE_PIXEL_COUNT: usize = Tile::COUNT * Tile::AREA;

/// The textures for all tiles.
static mut TEXTURES: [u32; TEXTURE_PIXEL_COUNT] = [0; TEXTURE_PIXEL_COUNT];

/// A tile of a world.
#[derive(Clone, Copy, Default, EnumCount, EnumIter)]
#[repr(u8)]
pub enum Tile {
    /// A grass tile.
    #[default]
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

    /// Loads textures for all tiles.
    pub fn load_textures() {
        for tile in Self::iter() {
            tile.load_texture();
        }
    }

    /// Returns the tile's texture as a slice.
    pub fn texture(self) -> &'static [u32] {
        self.texture_mut()
    }

    /// Returns the tile's identifying name.
    fn get_id_name(self) -> &'static str {
        match self {
            Self::Grass => "grass",
            Self::Stone => "stone",
        }
    }

    /// Returns the tile's texture as a mutable slice.
    fn texture_mut(self) -> &'static mut [u32] {
        let index = usize::from(self as u8) * Self::AREA;

        // SAFETY: Mutable statics are unsafe because they may be accessed by
        // multiple threads and cause a data race. PicoMine is currently
        // single-threaded so this should be impossible. The textures array
        // contains primitive integers, which always have valid data. If a data
        // race did occur, the only effect should be tile textures briefly
        // appearing visually corrupted.
        unsafe { &mut TEXTURES[index..index + Self::AREA] }
    }

    /// Loads the tile's texture.
    fn load_texture(self) {
        let mut path = PathBuf::from("res");
        path.push(self.get_id_name());
        path.set_extension("png");
        resources::load_texture(&path, Self::WIDTH, Self::HEIGHT, self.texture_mut());
    }
}
