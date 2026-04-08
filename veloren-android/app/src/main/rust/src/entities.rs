//! NPC and Monster System
//!
//! Handles NPC behavior, monster AI, and entity management.

use vek::{Vec2, Vec3};
use std::collections::HashMap;

// ========================
// Entity Types
// ========================

/// Type of entity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityType {
    // Passive NPCs
    Villager,
    Merchant,
    QuestGiver,
    
    // Hostile monsters
    Slime,
    Skeleton,
    Zombie,
    Spider,
    Wolf,
    Bear,
    Dragon,
    Golem,
    
    // Bosses
    BossDragon,
    BossLich,
    BossGiant,
}

impl EntityType {
    /// Check if hostile
    pub fn is_hostile(&self) -> bool {
        matches!(self,
            EntityType::Slime |
            EntityType::Skeleton |
            EntityType::Zombie |
            EntityType::Spider |
            EntityType::Wolf |
            EntityType::Bear |
            EntityType::Dragon |
            EntityType::Golem |
            EntityType::BossDragon |
            EntityType::BossLich |
            EntityType::BossGiant
        )
    }

    /// Check if boss
    pub fn is_boss(&self) -> bool {
        matches!(self,
            EntityType::BossDragon |
            EntityType::BossLich |
            EntityType::BossGiant
        )
    }

    /// Base health
    pub fn base_health(&self) -> f32 {
        match self {
            EntityType::Villager | EntityType::Merchant | EntityType::QuestGiver => 100.0,
            EntityType::Slime => 30.0,
            EntityType::Skeleton => 50.0,
            EntityType::Zombie => 60.0,
            EntityType::Spider => 40.0,
            EntityType::Wolf => 70.0,
            EntityType::Bear => 120.0,
            EntityType::Dragon => 200.0,
            EntityType::Golem => 150.0,
            EntityType::BossDragon => 1000.0,
            EntityType::BossLich => 800.0,
            EntityType::BossGiant => 1200.0,
        }
    }

    /// Attack damage
    pub fn attack_damage(&self) -> f32 {
        match self {
            EntityType::Villager | EntityType::Merchant | EntityType::QuestGiver => 0.0,
            EntityType::Slime => 5.0,
            EntityType::Skeleton => 12.0,
            EntityType::Zombie => 10.0,
            EntityType::Spider => 8.0,
            EntityType::Wolf => 15.0,
            EntityType::Bear => 25.0,
            EntityType::Dragon => 40.0,
            EntityType::Golem => 30.0,
            EntityType::BossDragon => 100.0,
            EntityType::BossLich => 80.0,
            EntityType::BossGiant => 120.0,
        }
    }

    /// Movement speed
    pub fn movement_speed(&self) -> f32 {
        match self {
            EntityType::Villager | EntityType::Merchant | EntityType::QuestGiver => 3.0,
            EntityType::Slime => 2.0,
            EntityType::Skeleton => 4.0,
            EntityType::Zombie => 2.5,
            EntityType::Spider => 5.0,
            EntityType::Wolf => 6.0,
            EntityType::Bear => 4.5,
            EntityType::Dragon => 7.0,
            EntityType::Golem => 2.0,
            EntityType::BossDragon => 5.0,
            EntityType::BossLich => 3.0,
            EntityType::BossGiant => 2.5,
        }
    }

    /// Aggro range
    pub fn aggro_range(&self) -> f32 {
        match self {
            EntityType::Villager | EntityType::Merchant | EntityType::QuestGiver => 0.0,
            EntityType::Slime => 5.0,
            EntityType::Skeleton => 10.0,
            EntityType::Zombie => 8.0,
            EntityType::Spider => 12.0,
            EntityType::Wolf => 15.0,
            EntityType::Bear => 12.0,
            EntityType::Dragon => 20.0,
            EntityType::Golem => 8.0,
            EntityType::BossDragon => 30.0,
            EntityType::BossLich => 25.0,
            EntityType::BossGiant => 20.0,
        }
    }

    /// Experience reward
    pub fn xp_reward(&self) -> u32 {
        match self {
            EntityType::Villager | EntityType::Merchant | EntityType::QuestGiver => 0,
            EntityType::Slime => 10,
            EntityType::Skeleton => 25,
            EntityType::Zombie => 20,
            EntityType::Spider => 15,
            EntityType::Wolf => 35,
            EntityType::Bear => 60,
            EntityType::Dragon => 150,
            EntityType::Golem => 100,
            EntityType::BossDragon => 1000,
            EntityType::BossLich => 800,
            EntityType::BossGiant => 1200,
        }
    }
}

// ========================
// AI States
// ========================

/// AI behavior state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIState {
    Idle,
    Wandering,
    Chasing,
    Attacking,
    Fleeing,
    Dead,
}

// ========================
/// NPC/Monster Entity
// ========================

/// An NPC or monster entity
pub struct Entity {
    pub id: u64,
    pub entity_type: EntityType,
    pub position: Vec3<f32>,
    pub velocity: Vec3<f32>,
    pub health: f32,
    pub max_health: f32,
    pub is_alive: bool,
    pub ai_state: AIState,
    
    // AI
    pub target_position: Option<Vec3<f32>>,
    pub wander_timer: f32,
    pub attack_cooldown: f32,
    pub aggro_range: f32,
    
    // Drops
    pub drop_table: Vec<DropEntry>,
}

impl Entity {
    pub fn new(id: u64, entity_type: EntityType, position: Vec3<f32>) -> Self {
        let max_health = entity_type.base_health();
        Self {
            id,
            entity_type,
            position,
            velocity: Vec3::zero(),
            health: max_health,
            max_health,
            is_alive: true,
            ai_state: AIState::Idle,
            target_position: None,
            wander_timer: 0.0,
            attack_cooldown: 0.0,
            aggro_range: entity_type.aggro_range(),
            drop_table: Vec::new(),
        }
    }

    /// Update entity AI
    pub fn update_ai(&mut self, player_pos: Vec3<f32>, delta_time: f32) {
        if !self.is_alive {
            return;
        }

        // Update cooldowns
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= delta_time;
        }

        let distance_to_player = (self.position - player_pos).magnitude();

        match self.ai_state {
            AIState::Idle => {
                self.wander_timer -= delta_time;
                if self.wander_timer <= 0.0 {
                    self.ai_state = AIState::Wandering;
                    self.pick_wander_target();
                }
            },
            AIState::Wandering => {
                self.move_towards_target(delta_time);
                
                // Check if reached target
                if let Some(target) = self.target_position {
                    let dist = (self.position - target).magnitude();
                    if dist < 1.0 {
                        self.ai_state = AIState::Idle;
                        self.wander_timer = rand::random::<f32>() * 5.0 + 2.0;
                    }
                }
            },
            AIState::Chasing => {
                // Chase player
                if self.entity_type.is_hostile() {
                    let direction = (player_pos - self.position).normalized();
                    self.velocity = direction * self.entity_type.movement_speed();
                    
                    // Check if in attack range
                    if distance_to_player < 3.0 {
                        self.ai_state = AIState::Attacking;
                    }
                    
                    // Check if lost aggro
                    if distance_to_player > self.aggro_range * 1.5 {
                        self.ai_state = AIState::Wandering;
                    }
                }
            },
            AIState::Attacking => {
                // Face player
                let direction = (player_pos - self.position).normalized();
                self.velocity = Vec3::zero();
                
                // Attack if cooldown ready
                if self.attack_cooldown <= 0.0 {
                    self.attack_cooldown = 1.0;
                    // Attack would be handled by combat system
                }
                
                // Check if player escaped
                if distance_to_player > 4.0 {
                    self.ai_state = AIState::Chasing;
                }
            },
            AIState::Fleeing => {
                // Run away from player
                let direction = (self.position - player_pos).normalized();
                self.velocity = direction * self.entity_type.movement_speed() * 1.5;
                
                // Stop fleeing when far enough
                if distance_to_player > self.aggro_range * 2.0 {
                    self.ai_state = AIState::Wandering;
                }
            },
            AIState::Dead => {
                // Dead entities don't update
            },
        }

        // Update position
        self.position += self.velocity * delta_time;
    }

    /// Pick a random wander target
    fn pick_wander_target(&mut self) {
        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        let distance = rand::random::<f32>() * 10.0 + 5.0;
        
        self.target_position = Some(Vec3::new(
            self.position.x + angle.cos() * distance,
            self.position.y,
            self.position.z + angle.sin() * distance,
        ));
    }

    /// Move towards target position
    fn move_towards_target(&mut self, delta_time: f32) {
        if let Some(target) = self.target_position {
            let direction = (target - self.position).normalized();
            self.velocity = direction * self.entity_type.movement_speed() * 0.5;
        }
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: f32) -> bool {
        if !self.is_alive {
            return false;
        }

        self.health -= damage;
        
        if self.health <= 0.0 {
            self.health = 0.0;
            self.is_alive = false;
            self.ai_state = AIState::Dead;
            return true; // Died
        }

        // Start chasing attacker
        if self.entity_type.is_hostile() {
            self.ai_state = AIState::Chasing;
        }

        false
    }

    /// Get health percentage
    pub fn health_percent(&self) -> f32 {
        if self.max_health > 0.0 {
            self.health / self.max_health
        } else {
            0.0
        }
    }
}

/// Drop entry for entities
#[derive(Clone, Debug)]
pub struct DropEntry {
    pub item_id: u32,
    pub chance: f32,
    pub min_count: u32,
    pub max_count: u32,
}

impl DropEntry {
    pub fn new(item_id: u32, chance: f32, min_count: u32, max_count: u32) -> Self {
        Self {
            item_id,
            chance,
            min_count,
            max_count,
        }
    }

    /// Roll for drop
    pub fn roll(&self) -> Option<u32> {
        if rand::random::<f32>() < self.chance {
            let count = rand::random::<u32>() % (self.max_count - self.min_count + 1) + self.min_count;
            Some(count)
        } else {
            None
        }
    }
}

// ========================
/// Entity Manager
// ========================

/// Manages all entities in the world
pub struct EntityManager {
    pub entities: HashMap<u64, Entity>,
    pub next_id: u64,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 1,
        }
    }

    /// Spawn entity
    pub fn spawn(&mut self, entity_type: EntityType, position: Vec3<f32>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let mut entity = Entity::new(id, entity_type, position);
        
        // Add drops for hostile entities
        if entity_type.is_hostile() {
            entity.drop_table.push(DropEntry::new(100, 0.3, 1, 3)); // Common drop
            if entity_type.is_boss() {
                entity.drop_table.push(DropEntry::new(200, 1.0, 1, 1)); // Boss drop
            }
        }
        
        self.entities.insert(id, entity);
        id
    }

    /// Update all entities
    pub fn update(&mut self, player_pos: Vec3<f32>, delta_time: f32) {
        for entity in self.entities.values_mut() {
            entity.update_ai(player_pos, delta_time);
        }
        
        // Remove dead entities after delay (handled elsewhere)
    }

    /// Get nearby hostile entities
    pub fn get_nearby_hostile(&self, position: Vec3<f32>, radius: f32) -> Vec<&Entity> {
        self.entities.values()
            .filter(|e| e.is_alive && e.entity_type.is_hostile())
            .filter(|e| (e.position - position).magnitude() < radius)
            .collect()
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Get alive count
    pub fn alive_count(&self) -> usize {
        self.entities.values().filter(|e| e.is_alive).count()
    }
}
