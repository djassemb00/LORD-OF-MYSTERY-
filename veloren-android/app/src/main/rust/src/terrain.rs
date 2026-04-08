//! Veloren Terrain System for Android
//!
//! Simplified terrain generation using local Block types.
//! This avoids the complex veloren-common dependency chain.

use vek::{Vec2, Vec3};
use std::collections::HashMap;

// Re-export local block types
pub use crate::veloren_types::{Block, BlockKind};

// ========================
// Chunk Constants
// ========================

/// Chunk size matches veloren-common's TERRAIN_CHUNK_BLOCKS_LG
pub const CHUNK_SIZE_LG: u32 = 5;
pub const CHUNK_SIZE: u32 = 1 << CHUNK_SIZE_LG; // 32
pub const CHUNK_HEIGHT: u32 = 256;

/// Sea level
pub const WATER_LEVEL: i32 = 62;

// ========================
// Block Types (Android-friendly wrapper)
// ========================

/// Simplified block representation that can be converted to veloren-common Block
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AndroidBlock {
    Air,
    Water,
    Grass,
    Dirt,
    Stone,
    Sand,
    Snow,
    Wood,
    Leaves,
    Ice,
    Clay,
    Gravel,
}

impl AndroidBlock {
    /// Convert to veloren-compatible Block
    pub fn to_veloren_block(self) -> Block {
        use crate::veloren_types::SpriteKind;
        match self {
            AndroidBlock::Air => Block::air(SpriteKind::Empty),
            AndroidBlock::Water => Block::new(
                BlockKind::Water,
                SpriteKind::Empty,
            ),
            AndroidBlock::Grass => Block::new(
                BlockKind::Grass,
                SpriteKind::Empty,
            ),
            AndroidBlock::Dirt => Block::new(
                BlockKind::Dirt,
                SpriteKind::Empty,
            ),
            AndroidBlock::Stone => Block::new(
                BlockKind::Rock,
                SpriteKind::Empty,
            ),
            AndroidBlock::Sand => Block::new(
                BlockKind::Sand,
                SpriteKind::Empty,
            ),
            AndroidBlock::Snow => Block::new(
                BlockKind::Snow,
                SpriteKind::Empty,
            ),
            AndroidBlock::Wood => Block::new(
                BlockKind::Wood,
                SpriteKind::Empty,
            ),
            AndroidBlock::Leaves => Block::new(
                BlockKind::Leaves,
                SpriteKind::Empty,
            ),
            AndroidBlock::Ice => Block::new(
                BlockKind::Ice,
                SpriteKind::Empty,
            ),
            AndroidBlock::Clay => Block::new(
                BlockKind::Clay,
                SpriteKind::Empty,
            ),
            AndroidBlock::Gravel => Block::new(
                BlockKind::Gravel,
                SpriteKind::Empty,
            ),
        }
    }

    /// Check if this block is solid
    pub fn is_solid(self) -> bool {
        !matches!(self, AndroidBlock::Air | AndroidBlock::Water)
    }

    /// Check if this block is transparent
    pub fn is_transparent(self) -> bool {
        matches!(self, AndroidBlock::Air | AndroidBlock::Water | AndroidBlock::Leaves)
    }
}

// ========================
// Chunk System
// ========================

/// A chunk of terrain (32x32x256 blocks)
pub struct TerrainChunk {
    pub chunk_pos: Vec2<i32>,
    pub blocks: Vec<AndroidBlock>,
    pub is_loaded: bool,
    pub is_dirty: bool,
    pub height_map: Vec<u8>, // Quick height lookup
}

impl TerrainChunk {
    pub fn new(chunk_pos: Vec2<i32>) -> Self {
        Self {
            chunk_pos,
            blocks: vec![AndroidBlock::Air; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT) as usize],
            is_loaded: false,
            is_dirty: true,
            height_map: vec![0; (CHUNK_SIZE * CHUNK_SIZE) as usize],
        }
    }

    /// Get block at local position
    pub fn get_block(&self, x: u32, y: u32, z: u32) -> AndroidBlock {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return AndroidBlock::Air;
        }
        let index = (y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x) as usize;
        self.blocks[index]
    }

    /// Set block at local position
    pub fn set_block(&mut self, x: u32, y: u32, z: u32, block: AndroidBlock) {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return;
        }
        let index = (y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x) as usize;
        self.blocks[index] = block;
        self.is_dirty = true;

        // Update height map if this is the highest solid block at this x,z
        if block.is_solid() && y >= self.height_map[(z * CHUNK_SIZE + x) as usize] as u32 {
            self.height_map[(z * CHUNK_SIZE + x) as usize] = y as u8;
        }
    }

    /// Get height at local x,z position
    pub fn get_height(&self, x: u32, z: u32) -> u32 {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return 0;
        }
        self.height_map[(z * CHUNK_SIZE + x) as usize] as u32
    }

    /// Generate terrain using multi-octave noise (similar to veloren-world)
    pub fn generate(&mut self, seed: u64) {
        let chunk_x = self.chunk_pos.x;
        let chunk_z = self.chunk_pos.z;

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_x * CHUNK_SIZE as i32 + x as i32;
                let world_z = chunk_z * CHUNK_SIZE as i32 + z as i32;

                // Calculate terrain height using multi-octave noise
                let height = self.calculate_height(world_x, world_z, seed);

                // Fill column
                for y in 0..CHUNK_HEIGHT {
                    let block = self.determine_block(y as i32, height, seed);
                    self.set_block(x, y, z, block);
                }

                // Generate trees
                if self.should_place_tree(world_x, world_z, height, seed) {
                    let tree_base_y = height as u32;
                    if tree_base_y + 10 < CHUNK_HEIGHT {
                        self.generate_tree(x, tree_base_y, z);
                    }
                }
            }
        }

        self.is_loaded = true;
        self.is_dirty = true;
    }

    /// Calculate terrain height using multi-octave noise
    /// Similar to veloren-world's approach but simplified
    fn calculate_height(&self, x: i32, z: i32, seed: u64) -> i32 {
        let base_height = 64;
        let seed_offset = (seed % 2000) as i32 - 1000;

        // Large-scale terrain features (continental scale)
        let continental = self.noise_2d(x as f64 * 0.002, z as f64 * 0.002, seed) * 40.0;

        // Hills (regional scale)
        let hills = self.noise_2d(x as f64 * 0.01, z as f64 * 0.01, seed + 1) * 20.0;

        // Small bumps (local scale)
        let bumps = self.noise_2d(x as f64 * 0.05, z as f64 * 0.05, seed + 2) * 5.0;

        // Mountains (very large scale)
        let mountains = self.noise_2d(x as f64 * 0.001, z as f64 * 0.001, seed + 3) * 80.0;
        let mountains = mountains.abs(); // Mountains only go up

        // Combine all octaves
        let height = base_height + seed_offset / 2 + continental as i32 + hills as i32
            + bumps as i32 + mountains as i32;

        // Clamp to reasonable range
        height.max(10).min(200)
    }

    /// Simple 2D noise function (value noise with interpolation)
    fn noise_2d(&self, x: f64, z: f64, seed: u64) -> f64 {
        // Simple hash-based value noise
        let ix = x.floor() as i64;
        let iz = z.floor() as i64;
        let fx = x - ix as f64;
        let fz = z - iz as f64;

        // Smooth interpolation
        let sx = fx * fx * (3.0 - 2.0 * fx);
        let sz = fz * fz * (3.0 - 2.0 * fz);

        // Hash corners
        let n00 = self.hash(ix, iz, seed) / 255.0;
        let n10 = self.hash(ix + 1, iz, seed) / 255.0;
        let n01 = self.hash(ix, iz + 1, seed) / 255.0;
        let n11 = self.hash(ix + 1, iz + 1, seed) / 255.0;

        // Interpolate
        let nx0 = n00 * (1.0 - sx) + n10 * sx;
        let nx1 = n01 * (1.0 - sx) + n11 * sx;

        nx0 * (1.0 - sz) + nx1 * sz - 0.5 // Center around 0
    }

    /// Simple hash function for noise
    fn hash(&self, x: i64, z: i64, seed: u64) -> f64 {
        let mut h = seed.wrapping_add((x.wrapping_mul(374761393) ^ z.wrapping_mul(668265263)) as u64);
        h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        h = h ^ (h >> 16);
        (h & 0xFF) as f64
    }

    /// Determine block type based on height and position
    fn determine_block(&self, y: i32, height: i32, _seed: u64) -> AndroidBlock {
        if y == 0 {
            return AndroidBlock::Stone; // Bedrock
        }

        if y < height - 4 {
            // Deep underground
            AndroidBlock::Stone
        } else if y < height - 1 {
            // Sub-surface
            if height < WATER_LEVEL + 5 {
                AndroidBlock::Sand
            } else {
                AndroidBlock::Dirt
            }
        } else if y == height {
            // Surface block
            if height < WATER_LEVEL - 2 {
                AndroidBlock::Sand // Beach
            } else if height > 140 {
                AndroidBlock::Snow // Mountain peaks
            } else if height > 120 {
                AndroidBlock::Stone // High mountains
            } else {
                AndroidBlock::Grass // Normal ground
            }
        } else if y < height {
            // Just below surface
            AndroidBlock::Dirt
        } else if y <= WATER_LEVEL {
            // Water
            AndroidBlock::Water
        } else {
            // Air
            AndroidBlock::Air
        }
    }

    /// Determine if a tree should be placed
    fn should_place_tree(&self, x: i32, z: i32, height: i32, seed: u64) -> bool {
        // Only place trees on grass
        if height < WATER_LEVEL + 5 || height > 110 {
            return false;
        }

        // Simple hash for random placement
        let hash = self.hash(x as i64, z as i64, seed + 100);
        hash > 245.0 // ~4% chance
    }

    /// Generate a simple tree
    fn generate_tree(&mut self, x: u32, base_y: u32, z: u32) {
        // Tree height
        let trunk_height = 5 + (base_y % 3);

        // Trunk
        for i in 0..trunk_height {
            if base_y + i < CHUNK_HEIGHT {
                self.set_block(x, base_y + i, z, AndroidBlock::Wood);
            }
        }

        // Leaves (sphere-ish shape)
        let leaf_start = base_y + trunk_height - 2;
        let leaf_end = base_y + trunk_height + 2;

        for dy in leaf_start..=leaf_end.min(CHUNK_HEIGHT - 1) {
            let radius = if dy < leaf_start + 2 { 2 } else { 1 };

            for dx in -radius..=radius {
                for dz in -radius..=radius {
                    // Skip corners for rounder shape
                    if dx.abs() == radius && dz.abs() == radius {
                        continue;
                    }

                    let nx = x as i32 + dx;
                    let nz = z as i32 + dz;

                    if nx >= 0 && nx < CHUNK_SIZE as i32 && nz >= 0 && nz < CHUNK_SIZE as i32 {
                        // Don't replace trunk
                        if dx == 0 && dz == 0 && dy < base_y + trunk_height {
                            continue;
                        }

                        self.set_block(nx as u32, dy, nz as u32, AndroidBlock::Leaves);
                    }
                }
            }
        }
    }

    /// Get all blocks as veloren-common Blocks
    pub fn get_veloren_blocks(&self) -> Vec<Block> {
        self.blocks.iter().map(|b| b.to_veloren_block()).collect()
    }
}

// ========================
// World Manager
// ========================

/// Manages terrain chunks around the player
pub struct TerrainWorld {
    chunks: HashMap<(i32, i32), TerrainChunk>,
    render_distance: i32,
    seed: u64,
}

impl TerrainWorld {
    pub fn new(seed: u64, render_distance: i32) -> Self {
        Self {
            chunks: HashMap::new(),
            render_distance,
            seed,
        }
    }

    /// Get or create chunk at chunk coordinates
    pub fn get_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> &mut TerrainChunk {
        self.chunks
            .entry((chunk_x, chunk_z))
            .or_insert_with(|| {
                let mut chunk = TerrainChunk::new(Vec2::new(chunk_x, chunk_z));
                chunk.generate(self.seed);
                chunk
            })
    }

    /// Update chunks around player world position
    pub fn update_around(&mut self, player_wpos: Vec3<i32>) {
        let player_chunk_x = player_wpos.x / CHUNK_SIZE as i32;
        let player_chunk_z = player_wpos.z / CHUNK_SIZE as i32;

        // Load chunks in render distance
        for dx in -self.render_distance..=self.render_distance {
            for dz in -self.render_distance..=self.render_distance {
                // Skip corners for circular render distance
                if dx * dx + dz * dz > self.render_distance * self.render_distance {
                    continue;
                }

                let cx = player_chunk_x + dx;
                let cz = player_chunk_z + dz;
                self.get_chunk(cx, cz);
            }
        }

        // Unload far chunks
        self.chunks.retain(|&(cx, cz), _| {
            let dx = cx - player_chunk_x;
            let dz = cz - player_chunk_z;
            dx * dx + dz * dz <= (self.render_distance + 2) * (self.render_distance + 2)
        });
    }

    /// Get block at world position
    pub fn get_block(&mut self, wpos: Vec3<i32>) -> AndroidBlock {
        let chunk_x = wpos.x / CHUNK_SIZE as i32;
        let chunk_z = wpos.z / CHUNK_SIZE as i32;
        let local_x = ((wpos.x % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let local_z = ((wpos.z % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32;

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_z)) {
            chunk.get_block(local_x as u32, wpos.y as u32, local_z as u32)
        } else {
            AndroidBlock::Air
        }
    }

    /// Set block at world position
    pub fn set_block(&mut self, wpos: Vec3<i32>, block: AndroidBlock) {
        let chunk_x = wpos.x / CHUNK_SIZE as i32;
        let chunk_z = wpos.z / CHUNK_SIZE as i32;
        let local_x = ((wpos.x % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let local_z = ((wpos.z % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32;

        if let Some(chunk) = self.chunks.get_mut(&(chunk_x, chunk_z)) {
            chunk.set_block(local_x as u32, wpos.y as u32, local_z as u32, block);
        }
    }

    /// Get terrain height at world x,z position
    pub fn get_height(&mut self, wx: i32, wz: i32) -> i32 {
        let chunk_x = wx / CHUNK_SIZE as i32;
        let chunk_z = wz / CHUNK_SIZE as i32;
        let local_x = ((wx % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let local_z = ((wz % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32;

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_z)) {
            chunk.get_height(local_x as u32, local_z as u32) as i32
        } else {
            // Generate temporary chunk for height lookup
            let mut temp_chunk = TerrainChunk::new(Vec2::new(chunk_x, chunk_z));
            temp_chunk.generate(self.seed);
            temp_chunk.get_height(local_x as u32, local_z as u32) as i32
        }
    }

    /// Get loaded chunk count
    pub fn loaded_chunks(&self) -> usize {
        self.chunks.len()
    }

    /// Get all dirty chunks (need mesh rebuild)
    pub fn get_dirty_chunks(&mut self) -> Vec<((i32, i32), &TerrainChunk)> {
        self.chunks
            .iter()
            .filter(|(_, chunk)| chunk.is_dirty)
            .map(|(pos, chunk)| (*pos, chunk))
            .collect()
    }

    /// Mark chunk as clean after mesh rebuild
    pub fn mark_chunk_clean(&mut self, chunk_pos: (i32, i32)) {
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.is_dirty = false;
        }
    }
}
