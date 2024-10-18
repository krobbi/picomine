use crate::tile::Tile;

/// A world containing tiles.
pub struct World;

impl World {
    /// Returns the tile at a position in the world.
    pub fn get_tile(x: i32, y: i32) -> Tile {
        const CHUNK_SIZE: i32 = 4;

        let corner_x = if x > 0 { 1 } else { CHUNK_SIZE - 1 };
        let corner_y = if y > 0 { 1 } else { CHUNK_SIZE - 1 };
        let (x, y) = (x & (CHUNK_SIZE - 1), y & (CHUNK_SIZE - 1));

        if x == 0 || y == 0 || (x == corner_x && y == corner_y) {
            Tile::Grass
        } else {
            Tile::Stone
        }
    }
}
