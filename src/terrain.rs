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
        const SCALE: f64 = 0.04;
        const OFFSET: f64 = 1.0 / 127.0;
        const STONE_HEIGHT: f64 = 0.65;
        const GRASS_HEIGHT: f64 = 0.25;
        const SAND_HEIGHT: f64 = 0.05;

        let is_in_spawn_radius = x * x + y * y < SPAWN_RADIUS_SQUARED;
        let (x, y) = (f64::from(x) * SCALE + OFFSET, f64::from(y) * SCALE + OFFSET);
        let height = self.noise.get([x, y]);

        if is_in_spawn_radius {
            return if height >= GRASS_HEIGHT {
                Tile::Grass
            } else {
                Tile::Sand
            };
        }

        if height >= STONE_HEIGHT {
            Tile::Stone
        } else if height >= GRASS_HEIGHT {
            Tile::Grass
        } else if height >= SAND_HEIGHT {
            Tile::Sand
        } else {
            Tile::Water
        }
    }
}
