use std::collections::HashMap;

use crate::{
    chunk::{self, Chunk},
    terrain::Terrain,
    tile::Tile,
};

/// A world containing chunks of tiles.
pub struct World {
    /// The world's terrain.
    terrain: Terrain,

    /// The world's chunks.
    chunks: HashMap<chunk::Position, Chunk>,
}

impl World {
    /// Creates a new world from its seed.
    pub fn new(seed: u32) -> Self {
        Self {
            terrain: Terrain::new(seed),
            chunks: HashMap::new(),
        }
    }

    /// Sets the tile at a tile position in the world.
    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        let (position, index) = chunk::Position::with_index(x, y);
        self.get_chunk(position).set_tile(index, tile);
    }

    /// Returns the tile at a tile position in the world.
    pub fn get_tile(&mut self, x: i32, y: i32) -> Tile {
        let (position, index) = chunk::Position::with_index(x, y);
        self.get_chunk(position).get_tile(index)
    }

    /// Attempts to break the tile at a tile position in the world.
    pub fn break_tile(&mut self, x: i32, y: i32) {
        let (position, index) = chunk::Position::with_index(x, y);
        let chunk = self.get_chunk(position);
        let tile = chunk.get_tile(index);

        if let Some(tile) = tile.get_break_replacement_tile() {
            chunk.set_tile(index, tile);
        }
    }

    /// Returns or inserts the chunk at a chunk position in the world.
    fn get_chunk(&mut self, position: chunk::Position) -> &mut Chunk {
        self.chunks
            .entry(position)
            .or_insert_with_key(|&p| Chunk::new(&self.terrain, p))
    }
}
