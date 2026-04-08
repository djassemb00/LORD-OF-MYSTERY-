//! Mining and Gathering System
//!
//! Handles resource gathering from the environment.

use vek::Vec3;
use crate::terrain::AndroidBlock;

// ========================
// Resource Node
// ========================

/// A mineable resource node
#[derive(Clone, Debug)]
pub struct ResourceNode {
    pub position: Vec3<i32>,
    pub resource_type: ResourceType,
    pub current_amount: u32,
    pub max_amount: u32,
    pub hardness: f32, // Time to mine
    pub tool_required: Option<ToolType>,
}

impl ResourceNode {
    pub fn new(position: Vec3<i32>, resource_type: ResourceType) -> Self {
        let (max_amount, hardness, tool_required) = match resource_type {
            ResourceType::Coal => (20, 1.0, Some(ToolType::Pickaxe)),
            ResourceType::Iron => (15, 2.0, Some(ToolType::Pickaxe)),
            ResourceType::Gold => (10, 3.0, Some(ToolType::Pickaxe)),
            ResourceType::Diamond => (5, 5.0, Some(ToolType::Pickaxe)),
            ResourceType::Stone => (50, 0.5, Some(ToolType::Pickaxe)),
            ResourceType::Wood => (30, 0.8, Some(ToolType::Axe)),
            ResourceType::Herb => (10, 0.3, None),
            ResourceType::Fish => (1, 2.0, Some(ToolType::FishingRod)),
        };

        Self {
            position,
            resource_type,
            current_amount: max_amount,
            max_amount,
            hardness,
            tool_required,
        }
    }

    /// Mine the node
    pub fn mine(&mut self, tool_level: u32, has_required_tool: bool) -> Option<u32> {
        if self.current_amount == 0 {
            return None;
        }

        // Check tool requirement
        if let Some(required_tool) = &self.tool_required {
            if !has_required_tool {
                // Can still mine but much slower
                return None;
            }
        }

        // Calculate yield based on tool level
        let base_yield = 1;
        let bonus_yield = if tool_level > 5 { 1 } else { 0 };
        let total_yield = base_yield + bonus_yield;

        self.current_amount = self.current_amount.saturating_sub(1);

        if total_yield > 0 {
            Some(total_yield as u32)
        } else {
            None
        }
    }

    /// Check if node is depleted
    pub fn is_depleted(&self) -> bool {
        self.current_amount == 0
    }

    /// Get item ID for this resource
    pub fn get_item_id(&self) -> u32 {
        match self.resource_type {
            ResourceType::Coal => 101,
            ResourceType::Iron => 102,
            ResourceType::Gold => 103,
            ResourceType::Diamond => 104,
            ResourceType::Stone => 105,
            ResourceType::Wood => 106,
            ResourceType::Herb => 107,
            ResourceType::Fish => 108,
        }
    }
}

// ========================
// Resource Types
// ========================

/// Type of gatherable resource
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceType {
    // Mining
    Coal,
    Iron,
    Gold,
    Diamond,
    Stone,
    
    // Woodcutting
    Wood,
    
    // Farming
    Herb,
    
    // Fishing
    Fish,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::Coal => "Coal",
            ResourceType::Iron => "Iron",
            ResourceType::Gold => "Gold",
            ResourceType::Diamond => "Diamond",
            ResourceType::Stone => "Stone",
            ResourceType::Wood => "Wood",
            ResourceType::Herb => "Herb",
            ResourceType::Fish => "Fish",
        }
    }

    pub fn icon_id(&self) -> u32 {
        match self {
            ResourceType::Coal => 1,
            ResourceType::Iron => 2,
            ResourceType::Gold => 3,
            ResourceType::Diamond => 4,
            ResourceType::Stone => 5,
            ResourceType::Wood => 6,
            ResourceType::Herb => 7,
            ResourceType::Fish => 8,
        }
    }
}

// ========================
// Tool Types
// ========================

/// Tool required for gathering
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    Pickaxe,
    Axe,
    Shovel,
    FishingRod,
    Sickle,
}

impl ToolType {
    pub fn name(&self) -> &'static str {
        match self {
            ToolType::Pickaxe => "Pickaxe",
            ToolType::Axe => "Axe",
            ToolType::Shovel => "Shovel",
            ToolType::FishingRod => "Fishing Rod",
            ToolType::Sickle => "Sickle",
        }
    }
}

// ========================
// Gathering Manager
// ========================

/// Manages resource nodes and gathering
pub struct GatheringManager {
    pub resource_nodes: Vec<ResourceNode>,
    pub respawn_timer: f32,
    pub respawn_cooldown: f32,
}

impl GatheringManager {
    pub fn new() -> Self {
        Self {
            resource_nodes: Vec::new(),
            respawn_timer: 300.0, // 5 minutes
            respawn_cooldown: 0.0,
        }
    }

    /// Spawn a resource node
    pub fn spawn_node(&mut self, position: Vec3<i32>, resource_type: ResourceType) {
        self.resource_nodes.push(ResourceNode::new(position, resource_type));
    }

    /// Update gathering system
    pub fn update(&mut self, delta_time: f32) {
        // Respawn depleted nodes
        self.respawn_cooldown -= delta_time;
        if self.respawn_cooldown <= 0.0 {
            self.respawn_cooldown = self.respawn_timer;
            self.respawn_nodes();
        }
    }

    /// Respawn depleted nodes
    fn respawn_nodes(&mut self) {
        for node in &mut self.resource_nodes {
            if node.is_depleted() {
                node.current_amount = node.max_amount;
            }
        }
    }

    /// Get nearby resource nodes
    pub fn get_nearby_nodes(&self, position: Vec3<f32>, radius: f32) -> Vec<&ResourceNode> {
        self.resource_nodes.iter()
            .filter(|node| {
                let dx = (node.position.x as f32 - position.x).abs();
                let dy = (node.position.y as f32 - position.y).abs();
                let dz = (node.position.z as f32 - position.z).abs();
                dx <= radius && dy <= radius && dz <= radius
            })
            .filter(|node| !node.is_depleted())
            .collect()
    }

    /// Mine a node
    pub fn mine_node(
        &mut self,
        node_index: usize,
        tool_level: u32,
        has_required_tool: bool,
    ) -> Option<(u32, u32)> {
        if node_index >= self.resource_nodes.len() {
            return None;
        }

        let node = &mut self.resource_nodes[node_index];
        if let Some(amount) = node.mine(tool_level, has_required_tool) {
            let item_id = node.get_item_id();
            Some((item_id, amount))
        } else {
            None
        }
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.resource_nodes.len()
    }

    /// Get active node count (not depleted)
    pub fn active_node_count(&self) -> usize {
        self.resource_nodes.iter().filter(|n| !n.is_depleted()).count()
    }
}

// ========================
// Block to Resource Mapping
// ========================

/// Get resource type from block
pub fn block_to_resource(block: AndroidBlock) -> Option<ResourceType> {
    match block {
        AndroidBlock::Stone => Some(ResourceType::Stone),
        AndroidBlock::Wood => Some(ResourceType::Wood),
        _ => None,
    }
}
