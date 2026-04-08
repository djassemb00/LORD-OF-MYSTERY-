//! Cave Generation System
//!
//! Generates underground cave networks using 3D noise.

use vek::Vec3;
use crate::terrain::{AndroidBlock, CHUNK_SIZE, CHUNK_HEIGHT};

// ========================
// Cave Constants
// ========================

/// Cave generation constants
pub const CAVE_THRESHOLD: f64 = 0.15;
pub const CAVE_SCALE: f64 = 0.05;
pub const CAVE_OCTAVES: usize = 4;

// ========================
// Cave Generator
// ========================

/// Generates cave systems within terrain
pub struct CaveGenerator {
    pub seed: u64,
}

impl CaveGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate caves for a chunk
    /// Modifies the blocks array in place
    pub fn generate_caves(&self, chunk_x: i32, chunk_z: i32, blocks: &mut [AndroidBlock]) {
        for y in 1..(CHUNK_HEIGHT - 1) {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let world_x = chunk_x * CHUNK_SIZE as i32 + x as i32;
                    let world_y = y as i32;
                    let world_z = chunk_z * CHUNK_SIZE as i32 + z as i32;

                    // Only carve caves underground (below surface)
                    let surface_height = self.get_surface_height(world_x, world_z);
                    if world_y >= surface_height - 5 {
                        continue;
                    }

                    // Don't carve below bedrock
                    if world_y <= 5 {
                        continue;
                    }

                    // Sample 3D noise
                    let noise = self.cave_noise(world_x as f64, world_y as f64, world_z as f64);

                    // Carve cave if noise exceeds threshold
                    if noise > CAVE_THRESHOLD {
                        let index = (y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x) as usize;
                        if index < blocks.len() {
                            blocks[index] = AndroidBlock::Air;
                        }
                    }
                }
            }
        }
    }

    /// 3D cave noise using multiple octaves
    fn cave_noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;

        for _ in 0..CAVE_OCTAVES {
            value += self.noise_3d(
                x * CAVE_SCALE * frequency,
                y * CAVE_SCALE * frequency,
                z * CAVE_SCALE * frequency,
                self.seed
            ) * amplitude;

            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value / max_value
    }

    /// 3D value noise
    fn noise_3d(&self, x: f64, y: f64, z: f64, seed: u64) -> f64 {
        let ix = x.floor() as i64;
        let iy = y.floor() as i64;
        let iz = z.floor() as i64;
        let fx = x - ix as f64;
        let fy = y - iy as f64;
        let fz = z - iz as f64;

        // Smooth interpolation
        let sx = fx * fx * (3.0 - 2.0 * fx);
        let sy = fy * fy * (3.0 - 2.0 * fy);
        let sz = fz * fz * (3.0 - 2.0 * fz);

        // Hash corners
        let n000 = self.hash3(ix, iy, iz, seed) / 255.0;
        let n100 = self.hash3(ix + 1, iy, iz, seed) / 255.0;
        let n010 = self.hash3(ix, iy + 1, iz, seed) / 255.0;
        let n110 = self.hash3(ix + 1, iy + 1, iz, seed) / 255.0;
        let n001 = self.hash3(ix, iy, iz + 1, seed) / 255.0;
        let n101 = self.hash3(ix + 1, iy, iz + 1, seed) / 255.0;
        let n011 = self.hash3(ix, iy + 1, iz + 1, seed) / 255.0;
        let n111 = self.hash3(ix + 1, iy + 1, iz + 1, seed) / 255.0;

        // Interpolate
        let nx00 = n000 * (1.0 - sx) + n100 * sx;
        let nx10 = n010 * (1.0 - sx) + n110 * sx;
        let nx01 = n001 * (1.0 - sx) + n101 * sx;
        let nx11 = n011 * (1.0 - sx) + n111 * sx;

        let nxy0 = nx00 * (1.0 - sy) + nx10 * sy;
        let nxy1 = nx01 * (1.0 - sy) + nx11 * sy;

        nxy0 * (1.0 - sz) + nxy1 * sz - 0.5
    }

    /// 3D hash function
    fn hash3(&self, x: i64, y: i64, z: i64, seed: u64) -> f64 {
        let mut h = seed.wrapping_add(
            (x.wrapping_mul(374761393) ^ 
             y.wrapping_mul(668265263) ^ 
             z.wrapping_mul(1274126177)) as u64
        );
        h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        h = h ^ (h >> 16);
        (h & 0xFF) as f64
    }

    /// Get surface height at position
    fn get_surface_height(&self, x: i32, z: i32) -> i32 {
        let base_height = 64;
        let seed = self.seed;
        let seed_offset = (seed % 2000) as i32 - 1000;

        let continental = self.noise_2d(x as f64 * 0.002, z as f64 * 0.002, seed) * 40.0;
        let hills = self.noise_2d(x as f64 * 0.01, z as f64 * 0.01, seed + 1) * 20.0;
        let bumps = self.noise_2d(x as f64 * 0.05, z as f64 * 0.05, seed + 2) * 5.0;
        let mountains = (self.noise_2d(x as f64 * 0.001, z as f64 * 0.001, seed + 3) * 80.0).abs();

        (base_height + seed_offset / 2 + continental as i32 + hills as i32
            + bumps as i32 + mountains as i32).max(10).min(200)
    }

    /// 2D noise
    fn noise_2d(&self, x: f64, z: f64, seed: u64) -> f64 {
        let ix = x.floor() as i64;
        let iz = z.floor() as i64;
        let fx = x - ix as f64;
        let fz = z - iz as f64;

        let sx = fx * fx * (3.0 - 2.0 * fx);
        let sz = fz * fz * (3.0 - 2.0 * fz);

        let n00 = self.hash(ix, iz, seed) / 255.0;
        let n10 = self.hash(ix + 1, iz, seed) / 255.0;
        let n01 = self.hash(ix, iz + 1, seed) / 255.0;
        let n11 = self.hash(ix + 1, iz + 1, seed) / 255.0;

        let nx0 = n00 * (1.0 - sx) + n10 * sx;
        let nx1 = n01 * (1.0 - sx) + n11 * sx;

        nx0 * (1.0 - sz) + nx1 * sz - 0.5
    }

    /// 2D hash
    fn hash(&self, x: i64, z: i64, seed: u64) -> f64 {
        let mut h = seed.wrapping_add((x.wrapping_mul(374761393) ^ z.wrapping_mul(668265263)) as u64);
        h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        h = h ^ (h >> 16);
        (h & 0xFF) as f64
    }

    /// Check if position is inside a cave
    pub fn is_in_cave(&self, world_x: i32, world_y: i32, world_z: i32) -> bool {
        let surface_height = self.get_surface_height(world_x, world_z);
        
        if world_y >= surface_height - 5 || world_y <= 5 {
            return false;
        }

        let noise = self.cave_noise(world_x as f64, world_y as f64, world_z as f64);
        noise > CAVE_THRESHOLD
    }

    /// Get ore distribution based on depth
    pub fn get_ore_at_depth(&self, depth: i32, noise: f64) -> Option<AndroidBlock> {
        // Only place ores in cave walls
        if noise <= CAVE_THRESHOLD {
            return None;
        }

        // Ore distribution based on depth
        let ore_chance = match depth {
            0..=20 => 0.02,    // Deep: Diamond, Gold
            21..=40 => 0.03,   // Mid-deep: Gold, Iron
            41..=60 => 0.04,   // Mid: Iron, Coal
            _ => 0.01,         // Shallow: Coal
        };

        if rand::random::<f64>() < ore_chance as f64 {
            Some(match depth {
                0..=10 => {
                    if rand::random::<f64>() < 0.1 {
                        AndroidBlock::Stone // Diamond placeholder
                    } else {
                        AndroidBlock::Stone // Gold placeholder
                    }
                },
                11..=30 => AndroidBlock::Stone, // Iron placeholder
                _ => AndroidBlock::Stone,       // Coal placeholder
            })
        } else {
            None
        }
    }
}

// ========================
// Dungeon Generator
// ========================

/// Simple dungeon room generator
pub struct DungeonGenerator {
    pub seed: u64,
}

impl DungeonGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate a simple dungeon room
    pub fn generate_room(
        &self,
        center_x: i32,
        center_y: i32,
        center_z: i32,
        width: i32,
        height: i32,
        depth: i32,
    ) -> Vec<(i32, i32, i32)> {
        let mut blocks = Vec::new();

        // Carve out room
        for y in 0..height {
            for z in 0..depth {
                for x in 0..width {
                    let wx = center_x - width / 2 + x;
                    let wy = center_y + y;
                    let wz = center_z - depth / 2 + z;

                    // Floor
                    if y == 0 {
                        blocks.push((wx, wy, wz, AndroidBlock::Stone));
                    }
                    // Ceiling
                    else if y == height - 1 {
                        blocks.push((wx, wy, wz, AndroidBlock::Stone));
                    }
                    // Walls
                    else if x == 0 || x == width - 1 || z == 0 || z == depth - 1 {
                        blocks.push((wx, wy, wz, AndroidBlock::Stone));
                    }
                    // Air inside
                    else {
                        blocks.push((wx, wy, wz, AndroidBlock::Air));
                    }
                }
            }
        }

        blocks
    }

    /// Generate corridor between two points
    pub fn generate_corridor(
        &self,
        start: (i32, i32, i32),
        end: (i32, i32, i32),
        width: i32,
    ) -> Vec<(i32, i32, i32, AndroidBlock)> {
        let mut blocks = Vec::new();

        let (x1, y1, z1) = start;
        let (x2, y2, z2) = end;

        // L-shaped corridor
        let mid_x = x2;
        let mid_z = z1;

        // First segment (x direction)
        let x_start = x1.min(mid_x);
        let x_end = x1.max(mid_x);
        for x in x_start..=x_end {
            for w in 0..width {
                for h in 0..3 {
                    blocks.push((x, y1 + h, z1 + w, AndroidBlock::Air));
                }
            }
        }

        // Second segment (z direction)
        let z_start = z1.min(z2);
        let z_end = z1.max(z2);
        for z in z_start..=z_end {
            for w in 0..width {
                for h in 0..3 {
                    blocks.push((mid_x, y2 + h, z + w, AndroidBlock::Air));
                }
            }
        }

        blocks
    }
}
