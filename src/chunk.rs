use crate::tile::Tile;

/// A chunk of tiles in a world.
pub struct Chunk {
    /// The chunk's tiles.
    tiles: [Tile; Self::WIDTH * Self::HEIGHT],
}

impl Chunk {
    /// A chunk's width in tiles.
    const WIDTH: usize = 16;

    /// A chunk's height in tiles.
    const HEIGHT: usize = Self::WIDTH;

    /// Creates a new chunk.
    pub fn new() -> Self {
        let mut tiles = [Tile::Stone; Self::WIDTH * Self::HEIGHT];

        for x in 0..Self::WIDTH {
            tiles[x] = Tile::Grass;
            tiles[Self::WIDTH * Self::HEIGHT - 1 - x] = Tile::Grass;
        }

        for y in 1..Self::HEIGHT - 1 {
            tiles[y * Self::WIDTH] = Tile::Grass;
            tiles[y * Self::WIDTH + Self::WIDTH - 1] = Tile::Grass;
        }

        Self { tiles }
    }

    /// Returns the tile at an index in the chunk.
    pub fn get_tile(&self, index: usize) -> Tile {
        self.tiles[index]
    }
}

/// A chunk's position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    /// The chunk position's X position.
    x: i16,

    /// The chunk position's Y position.
    y: i16,
}

impl Position {
    /// Creates a new chunk position and index from a block position.
    pub fn with_index(x: i32, y: i32) -> (Self, usize) {
        // TODO: Implement this function. <krobbi>
        #[allow(clippy::cast_sign_loss)]
        (Self { x: 0, y: 0 }, ((x & 15) + (y & 15) * 16) as usize)
    }
}
