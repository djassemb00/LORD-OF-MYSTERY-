//! Building System
//!
//! Allows players to place and remove blocks for building.

use vek::Vec3;
use std::collections::HashMap;
use crate::terrain::AndroidBlock;

// ========================
// Building Block
// ========================

/// A player-placed block
#[derive(Clone, Debug)]
pub struct BuildingBlock {
    pub position: Vec3<i32>,
    pub block_type: AndroidBlock,
    pub placed_by: u64, // Player ID
    pub timestamp: u64,
}

// ========================
// Building Manager
// ========================

/// Manages player-built structures
pub struct BuildingManager {
    pub placed_blocks: HashMap<Vec3<i32>, BuildingBlock>,
    pub max_blocks_per_player: u32,
    pub build_radius: i32,
}

impl BuildingManager {
    pub fn new() -> Self {
        Self {
            placed_blocks: HashMap::new(),
            max_blocks_per_player: 1000,
            build_radius: 100,
        }
    }

    /// Place a block
    pub fn place_block(&mut self, position: Vec3<i32>, block_type: AndroidBlock, player_id: u64) -> bool {
        // Check if position is valid
        if position.y < 0 || position.y >= 256 {
            return false;
        }

        // Check if block already exists
        if self.placed_blocks.contains_key(&position) {
            return false;
        }

        // Check player block limit
        let player_block_count = self.placed_blocks.values()
            .filter(|b| b.placed_by == player_id)
            .count() as u32;
        
        if player_block_count >= self.max_blocks_per_player {
            return false;
        }

        // Place block
        self.placed_blocks.insert(position, BuildingBlock {
            position,
            block_type,
            placed_by: player_id,
            timestamp: 0, // Would use actual timestamp
        });

        true
    }

    /// Remove a block
    pub fn remove_block(&mut self, position: Vec3<i32>, player_id: u64) -> Option<AndroidBlock> {
        if let Some(block) = self.placed_blocks.get(&position) {
            // Only allow removal by owner (or admin)
            if block.placed_by == player_id {
                return self.placed_blocks.remove(&position).map(|b| b.block_type);
            }
        }
        None
    }

    /// Get block at position
    pub fn get_block(&self, position: Vec3<i32>) -> Option<&BuildingBlock> {
        self.placed_blocks.get(&position)
    }

    /// Get blocks in radius
    pub fn get_blocks_in_radius(&self, center: Vec3<i32>, radius: i32) -> Vec<&BuildingBlock> {
        self.placed_blocks.values()
            .filter(|b| {
                let dx = (b.position.x - center.x).abs();
                let dy = (b.position.y - center.y).abs();
                let dz = (b.position.z - center.z).abs();
                dx <= radius && dy <= radius && dz <= radius
            })
            .collect()
    }

    /// Get block count for player
    pub fn get_player_block_count(&self, player_id: u64) -> u32 {
        self.placed_blocks.values()
            .filter(|b| b.placed_by == player_id)
            .count() as u32
    }

    /// Clear all blocks by player
    pub fn clear_player_blocks(&mut self, player_id: u64) {
        self.placed_blocks.retain(|_, b| b.placed_by != player_id);
    }

    /// Get total block count
    pub fn total_blocks(&self) -> usize {
        self.placed_blocks.len()
    }
}

// ========================
// Building Presets
// ========================

/// Pre-built structure templates
pub enum BuildingPreset {
    House,
    Tower,
    Wall,
    Bridge,
    Stairs,
}

impl BuildingPreset {
    /// Get blocks for preset
    pub fn get_blocks(&self, origin: Vec3<i32>) -> Vec<(Vec3<i32>, AndroidBlock)> {
        match self {
            BuildingPreset::House => self.house(origin),
            BuildingPreset::Tower => self.tower(origin),
            BuildingPreset::Wall => self.wall(origin),
            BuildingPreset::Bridge => self.bridge(origin),
            BuildingPreset::Stairs => self.stairs(origin),
        }
    }

    fn house(&self, origin: Vec3<i32>) -> Vec<(Vec3<i32>, AndroidBlock)> {
        let mut blocks = Vec::new();
        let width = 7;
        let height = 5;
        let depth = 7;

        // Floor
        for x in 0..width {
            for z in 0..depth {
                blocks.push((
                    Vec3::new(origin.x + x, origin.y, origin.z + z),
                    AndroidBlock::Wood,
                ));
            }
        }

        // Walls
        for y in 1..height {
            for x in 0..width {
                // Front and back walls
                blocks.push((
                    Vec3::new(origin.x + x, origin.y + y, origin.z),
                    AndroidBlock::Wood,
                ));
                blocks.push((
                    Vec3::new(origin.x + x, origin.y + y, origin.z + depth - 1),
                    AndroidBlock::Wood,
                ));
            }
            for z in 1..(depth - 1) {
                // Side walls
                blocks.push((
                    Vec3::new(origin.x, origin.y + y, origin.z + z),
                    AndroidBlock::Wood,
                ));
                blocks.push((
                    Vec3::new(origin.x + width - 1, origin.y + y, origin.z + z),
                    AndroidBlock::Wood,
                ));
            }
        }

        // Roof
        for x in 0..width {
            for z in 0..depth {
                blocks.push((
                    Vec3::new(origin.x + x, origin.y + height, origin.z + z),
                    AndroidBlock::Wood,
                ));
            }
        }

        blocks
    }

    fn tower(&self, origin: Vec3<i32>) -> Vec<(Vec3<i32>, AndroidBlock)> {
        let mut blocks = Vec::new();
        let size = 5;
        let height = 15;

        for y in 0..height {
            for x in 0..size {
                for z in 0..size {
                    // Walls only (hollow tower)
                    if x == 0 || x == size - 1 || z == 0 || z == size - 1 {
                        blocks.push((
                            Vec3::new(origin.x + x, origin.y + y, origin.z + z),
                            AndroidBlock::Stone,
                        ));
                    }
                    // Floor every 3 blocks
                    else if y % 3 == 0 {
                        blocks.push((
                            Vec3::new(origin.x + x, origin.y + y, origin.z + z),
                            AndroidBlock::Wood,
                        ));
                    }
                }
            }
        }

        blocks
    }

    fn wall(&self, origin: Vec3<i32>) -> Vec<(Vec3<i32>, AndroidBlock)> {
        let mut blocks = Vec::new();
        let length = 10;
        let height = 4;

        for x in 0..length {
            for y in 0..height {
                blocks.push((
                    Vec3::new(origin.x + x, origin.y + y, origin.z),
                    AndroidBlock::Stone,
                ));
            }
        }

        blocks
    }

    fn bridge(&self, origin: Vec3<i32>) -> Vec<(Vec3<i32>, AndroidBlock)> {
        let mut blocks = Vec::new();
        let length = 10;
        let width = 3;

        for x in 0..length {
            for z in 0..width {
                blocks.push((
                    Vec3::new(origin.x + x, origin.y, origin.z + z),
                    AndroidBlock::Wood,
                ));
            }
        }

        blocks
    }

    fn stairs(&self, origin: Vec3<i32>) -> Vec<(Vec3<i32>, AndroidBlock)> {
        let mut blocks = Vec::new();
        let steps = 10;

        for i in 0..steps {
            blocks.push((
                Vec3::new(origin.x + i, origin.y + i, origin.z),
                AndroidBlock::Stone,
            ));
        }

        blocks
    }
}
