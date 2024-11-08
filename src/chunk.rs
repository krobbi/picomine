use crate::{terrain::Terrain, tile::Tile};

/// A chunk of tiles in a world.
pub struct Chunk {
    /// The chunk's tiles.
    tiles: [Tile; Self::AREA],
}

impl Chunk {
    /// A chunk's width in tiles.
    const WIDTH: i32 = 32;

    /// A chunk's height in tiles.
    const HEIGHT: i32 = Self::WIDTH;

    /// A chunk's area in tiles.
    const AREA: usize = Self::WIDTH as usize * Self::HEIGHT as usize;

    /// Creates a new chunk from its terrain and position.
    pub fn new(terrain: &Terrain, position: Position) -> Self {
        let mut tiles = [Tile::default(); Self::AREA];
        let (base_x, base_y) = position.to_tile_position();

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let index = usize::try_from(x + y * Self::WIDTH).unwrap();
                let (x, y) = (base_x + x, base_y + y);
                tiles[index] = terrain.get_tile(x, y);
            }
        }

        Self { tiles }
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
        #[allow(clippy::cast_sign_loss)]
        let index =
            (x.rem_euclid(Chunk::WIDTH) + y.rem_euclid(Chunk::HEIGHT) * Chunk::WIDTH) as usize;

        #[allow(clippy::cast_possible_truncation)]
        let (x, y) = (
            x.div_euclid(Chunk::WIDTH) as i16,
            y.div_euclid(Chunk::HEIGHT) as i16,
        );

        (Self { x, y }, index)
    }

    /// Returns the chunk position as a tile position.
    fn to_tile_position(self) -> (i32, i32) {
        (
            i32::from(self.x) * Chunk::WIDTH,
            i32::from(self.y) * Chunk::HEIGHT,
        )
    }
}
