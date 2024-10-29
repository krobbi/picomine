use noise::{NoiseFn, Simplex};

use crate::tile::Tile;

/// Procedurally-generated terrain containing tiles.
pub struct Terrain {
    /// The terrain's simplex noise for generation.
    noise: Simplex,
}

impl Terrain {
    /// Creates new terrain from its seed.
    pub fn new(seed: u32) -> Self {
        Self {
            noise: Simplex::new(seed),
        }
    }

    /// Returns the tile at a tile position in the terrain.
    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        const SPAWN_RADIUS_SQUARED: i32 = 5i32.pow(2);
        const SCALE: f64 = 0.05;
        const OFFSET: f64 = 1.0 / 127.0;
        const SAND_THRESHOLD: f64 = -0.4;
        const STONE_THRESHOLD: f64 = 0.25;

        if x * x + y * y < SPAWN_RADIUS_SQUARED {
            return Tile::Sand;
        }

        let (x, y) = (f64::from(x) * SCALE + OFFSET, f64::from(y) * SCALE + OFFSET);
        let elevation = self.noise.get([x, y]);

        if elevation < SAND_THRESHOLD {
            Tile::Sand
        } else if elevation > STONE_THRESHOLD {
            Tile::Stone
        } else {
            Tile::Grass
        }
    }
}
