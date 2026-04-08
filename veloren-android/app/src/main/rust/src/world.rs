//! World generation and management

use vek::Vec3;
use std::collections::HashMap;

/// Chunk of the world (16x16x256 blocks)
pub struct Chunk {
    pub position: Vec3<i32>,
    pub blocks: Vec<BlockType>,
    pub is_loaded: bool,
    pub is_dirty: bool,
}

/// Type of block
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
    Water,
    Sand,
    Wood,
    Leaves,
    Snow,
}

impl Chunk {
    pub fn new(position: Vec3<i32>) -> Self {
        Self {
            position,
            blocks: vec![BlockType::Air; 16 * 16 * 256],
            is_loaded: false,
            is_dirty: true,
        }
    }

    /// Get block at local position
    pub fn get_block(&self, x: u32, y: u32, z: u32) -> BlockType {
        if x >= 16 || y >= 256 || z >= 16 {
            return BlockType::Air;
        }
        let index = (y * 16 * 16 + z * 16 + x) as usize;
        self.blocks[index]
    }

    /// Set block at local position
    pub fn set_block(&mut self, x: u32, y: u32, z: u32, block: BlockType) {
        if x >= 16 || y >= 256 || z >= 16 {
            return;
        }
        let index = (y * 16 * 16 + z * 16 + x) as usize;
        self.blocks[index] = block;
        self.is_dirty = true;
    }

    /// Generate terrain for this chunk using simple noise
    pub fn generate(&mut self, seed: u64) {
        let chunk_x = self.position.x;
        let chunk_z = self.position.z;

        for x in 0..16u32 {
            for z in 0..16u32 {
                // Simple height calculation with multiple octaves
                let world_x = chunk_x * 16 + x as i32;
                let world_z = chunk_z * 16 + z as i32;
                
                let height = self.calculate_height(world_x, world_z, seed);
                
                for y in 0..256u32 {
                    let block = if y == 0 {
                        BlockType::Stone
                    } else if y < (height - 5) as u32 {
                        BlockType::Stone
                    } else if y < height as u32 {
                        BlockType::Dirt
                    } else if y == height as u32 {
                        // Surface block based on height
                        if height > 120 {
                            BlockType::Snow
                        } else if height < 62 {
                            BlockType::Sand
                        } else {
                            BlockType::Grass
                        }
                    } else if y < 62 {
                        BlockType::Water
                    } else {
                        BlockType::Air
                    };
                    
                    self.set_block(x, y, z, block);
                }

                // Simple tree generation
                if height > 65 && height < 100 && x > 2 && x < 13 && z > 2 && z < 13 {
                    // Simple hash for random tree placement
                    let tree_hash = ((world_x.wrapping_mul(374761393) + world_z.wrapping_mul(668265263)) % 100) as i32;
                    if tree_hash == 0 {
                        self.generate_tree(x, height as u32, z);
                    }
                }
            }
        }
        
        self.is_loaded = true;
        self.is_dirty = true;
    }

    /// Calculate terrain height at position
    fn calculate_height(&self, x: i32, z: i32, seed: u64) -> i32 {
        // Simple multi-octave noise approximation
        let base_height = 64;
        
        // Large hills
        let hill_x = (x as f64 * 0.01).sin() * 20.0;
        let hill_z = (z as f64 * 0.01).cos() * 20.0;
        
        // Small bumps
        let bump_x = (x as f64 * 0.05).sin() * 5.0;
        let bump_z = (z as f64 * 0.05).cos() * 5.0;
        
        // Seed variation
        let seed_var = ((seed % 1000) as i32 - 500) / 50;
        
        base_height + hill_x as i32 + hill_z as i32 + bump_x as i32 + bump_z as i32 + seed_var
    }

    /// Generate a simple tree
    fn generate_tree(&mut self, x: u32, y: u32, z: u32) {
        // Trunk
        for i in 0..5 {
            if y + i < 256 {
                self.set_block(x, y + i, z, BlockType::Wood);
            }
        }
        
        // Leaves
        for dx in -2..=2 {
            for dz in -2..=2 {
                for dy in 3..=6 {
                    let nx = x as i32 + dx;
                    let nz = z as i32 + dz;
                    let ny = y as i32 + dy;
                    
                    if nx >= 0 && nx < 16 && nz >= 0 && nz < 16 && ny >= 0 && ny < 256 {
                        // Don't replace trunk
                        if dx == 0 && dz == 0 && dy < 5 {
                            continue;
                        }
                        
                        // Skip corners for rounder shape
                        if dx.abs() == 2 && dz.abs() == 2 && dy > 4 {
                            continue;
                        }
                        
                        self.set_block(nx as u32, ny as u32, nz as u32, BlockType::Leaves);
                    }
                }
            }
        }
    }
}

/// World manager
pub struct WorldManager {
    chunks: HashMap<(i32, i32), Chunk>,
    render_distance: i32,
    seed: u64,
}

impl WorldManager {
    pub fn new(seed: u64, render_distance: i32) -> Self {
        Self {
            chunks: HashMap::new(),
            render_distance,
            seed,
        }
    }

    /// Get or create chunk at world position
    pub fn get_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> &mut Chunk {
        self.chunks
            .entry((chunk_x, chunk_z))
            .or_insert_with(|| {
                let mut chunk = Chunk::new(Vec3::new(chunk_x * 16, 0, chunk_z * 16));
                chunk.generate(self.seed);
                chunk
            })
    }

    /// Update chunks around player position
    pub fn update_chunks(&mut self, player_x: i32, player_z: i32) {
        let player_chunk_x = player_x / 16;
        let player_chunk_z = player_z / 16;

        // Load chunks in render distance
        for dx in -self.render_distance..=self.render_distance {
            for dz in -self.render_distance..=self.render_distance {
                let cx = player_chunk_x + dx;
                let cz = player_chunk_z + dz;
                self.get_chunk(cx, cz);
            }
        }

        // TODO: Unload far chunks
    }

    /// Get block at world position
    pub fn get_block(&mut self, x: i32, y: i32, z: i32) -> BlockType {
        let chunk_x = x / 16;
        let chunk_z = z / 16;
        let local_x = ((x % 16) + 16) % 16;
        let local_z = ((z % 16) + 16) % 16;

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_z)) {
            chunk.get_block(local_x as u32, y as u32, local_z as u32)
        } else {
            BlockType::Air
        }
    }

    /// Set block at world position
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockType) {
        let chunk_x = x / 16;
        let chunk_z = z / 16;
        let local_x = ((x % 16) + 16) % 16;
        let local_z = ((z % 16) + 16) % 16;

        if let Some(chunk) = self.chunks.get_mut(&(chunk_x, chunk_z)) {
            chunk.set_block(local_x as u32, y as u32, local_z as u32, block);
        }
    }

    /// Get chunk count
    pub fn loaded_chunks(&self) -> usize {
        self.chunks.len()
    }
}
