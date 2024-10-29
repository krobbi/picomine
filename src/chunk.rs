use crate::tile::Tile;

/// A chunk of tiles in a world.
pub struct Chunk {
    /// The chunk's tiles.
    tiles: [Tile; Self::WIDTH * Self::HEIGHT],
}

impl Chunk {
    /// A chunk's width in tiles.
    const WIDTH: usize = 32;

    /// A chunk's height in tiles.
    const HEIGHT: usize = Self::WIDTH;

    /// Creates a new chunk.
    pub fn new() -> Self {
        Self {
            tiles: [Tile::Grass; Self::WIDTH * Self::HEIGHT],
        }
    }

    /// Sets the tile at an index in the chunk.
    pub fn set_tile(&mut self, index: usize, tile: Tile) {
        self.tiles[index] = tile;
    }

    /// Returns the tile at an index in the chunk.
    pub fn get_tile(&self, index: usize) -> Tile {
        self.tiles[index]
    }
}

/// A chunk's position.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// The chunk position's X position.
    x: i16,

    /// The chunk position's Y position.
    y: i16,
}

impl Position {
    /// Creates a new chunk position and chunk index from a tile position.
    pub fn with_index(x: i32, y: i32) -> (Self, usize) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        const CHUNK_WIDTH: i32 = Chunk::WIDTH as i32;

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        const CHUNK_HEIGHT: i32 = Chunk::HEIGHT as i32;

        let (chunk_x, chunk_y) = (x.div_euclid(CHUNK_WIDTH), y.div_euclid(CHUNK_HEIGHT));

        #[allow(clippy::cast_sign_loss)]
        let index =
            (x - chunk_x * CHUNK_WIDTH + (y - chunk_y * CHUNK_HEIGHT) * CHUNK_WIDTH) as usize;

        #[allow(clippy::cast_possible_truncation)]
        let (x, y) = (chunk_x as i16, chunk_y as i16);

        (Self { x, y }, index)
    }
}
