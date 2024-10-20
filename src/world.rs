use std::collections::HashMap;

use crate::{
    chunk::{self, Chunk},
    tile::Tile,
};

/// A world containing chunks of tiles.
pub struct World {
    /// The world's chunks.
    chunks: HashMap<chunk::Position, Chunk>,
}

impl World {
    /// Creates a new world.
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    /// Returns the tile at a position in the world.
    pub fn get_tile(&mut self, x: i32, y: i32) -> Tile {
        let (position, index) = chunk::Position::with_index(x, y);
        self.get_chunk(position).get_tile(index)
    }

    /// Returns or inserts the chunk at a chunk position in the world.
    fn get_chunk(&mut self, position: chunk::Position) -> &Chunk {
        self.chunks.entry(position).or_insert_with_key(|k| {
            #[cfg(debug_assertions)]
            eprintln!("loaded chunk: {k:?}");

            Chunk::new()
        })
    }
}
