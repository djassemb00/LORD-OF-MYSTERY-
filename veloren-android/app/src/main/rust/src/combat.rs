//! Combat System
//!
//! Handles attacks, damage, defense, and combat states.

use vek::Vec3;

// ========================
// Combat Stats
// ========================

/// Combat statistics for an entity
#[derive(Clone, Debug)]
pub struct CombatStats {
    pub attack_power: f32,
    pub defense: f32,
    pub attack_speed: f32,
    pub attack_range: f32,
    pub critical_chance: f32,
    pub critical_multiplier: f32,
    pub dodge_chance: f32,
    pub lifesteal: f32,
}

impl CombatStats {
    pub fn new() -> Self {
        Self {
            attack_power: 10.0,
            defense: 5.0,
            attack_speed: 1.0,
            attack_range: 3.0,
            critical_chance: 0.05,
            critical_multiplier: 2.0,
            dodge_chance: 0.02,
            lifesteal: 0.0,
        }
    }

    /// Calculate damage dealt to target
    pub fn calculate_damage(&self, target_defense: f32) -> DamageResult {
        let is_critical = rand::random::<f32>() < self.critical_chance;
        let is_dodged = rand::random::<f32>() < target_defense / 100.0;

        let base_damage = self.attack_power.max(1.0);
        let critical_mult = if is_critical { self.critical_multiplier } else { 1.0 };
        let defense_reduction = target_defense / (target_defense + 50.0);
        
        let final_damage = if is_dodged {
            0.0
        } else {
            (base_damage * critical_mult * (1.0 - defense_reduction * 0.5)).max(1.0)
        };

        DamageResult {
            damage: final_damage,
            is_critical,
            is_dodged,
            lifesteal_heal: final_damage * self.lifesteal,
        }
    }
}

/// Result of a damage calculation
#[derive(Clone, Debug)]
pub struct DamageResult {
    pub damage: f32,
    pub is_critical: bool,
    pub is_dodged: bool,
    pub lifesteal_heal: f32,
}

// ========================
// Attack Types
// ========================

/// Type of attack
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackType {
    Light,
    Heavy,
    Charged,
    Special,
}

impl AttackType {
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            AttackType::Light => 0.8,
            AttackType::Heavy => 1.5,
            AttackType::Charged => 2.0,
            AttackType::Special => 2.5,
        }
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            AttackType::Light => 0.3,
            AttackType::Heavy => 0.8,
            AttackType::Charged => 1.5,
            AttackType::Special => 3.0,
        }
    }
}

// ========================
// Combat State
// ========================

/// Current combat state
#[derive(Clone, Debug)]
pub struct CombatState {
    pub is_in_combat: bool,
    pub current_attack: Option<ActiveAttack>,
    pub last_attack_time: f32,
    pub combo_count: u32,
    pub combo_timer: f32,
    pub invulnerable_timer: f32,
}

impl CombatState {
    pub fn new() -> Self {
        Self {
            is_in_combat: false,
            current_attack: None,
            last_attack_time: -999.0,
            combo_count: 0,
            combo_timer: 0.0,
            invulnerable_timer: 0.0,
        }
    }

    /// Start an attack
    pub fn start_attack(&mut self, attack_type: AttackType, current_time: f32) -> bool {
        let attack_cooldown = attack_type.cooldown();
        
        // Check if we can attack (cooldown)
        if current_time - self.last_attack_time < attack_cooldown {
            return false;
        }

        // Update combo
        if self.combo_timer > 0.0 {
            self.combo_count += 1;
        } else {
            self.combo_count = 1;
        }
        self.combo_timer = 2.0; // Combo window

        self.current_attack = Some(ActiveAttack {
            attack_type,
            start_time: current_time,
            duration: 0.5,
            has_hit: false,
        });
        self.last_attack_time = current_time;
        self.is_in_combat = true;

        true
    }

    /// Update combat state
    pub fn update(&mut self, delta_time: f32) {
        // Update combo timer
        if self.combo_timer > 0.0 {
            self.combo_timer -= delta_time;
        } else {
            self.combo_count = 0;
        }

        // Update invulnerability
        if self.invulnerable_timer > 0.0 {
            self.invulnerable_timer -= delta_time;
        }

        // Update current attack
        if let Some(ref mut attack) = self.current_attack {
            let elapsed = attack.start_time + attack.duration - self.last_attack_time;
            if elapsed <= 0.0 {
                self.current_attack = None;
            }
        }

        // Check if still in combat
        if self.current_attack.is_none() && self.combo_count == 0 {
            self.is_in_combat = false;
        }
    }

    /// Check if currently attacking
    pub fn is_attacking(&self) -> bool {
        self.current_attack.is_some()
    }

    /// Get attack progress (0-1)
    pub fn attack_progress(&self, current_time: f32) -> f32 {
        if let Some(ref attack) = self.current_attack {
            let elapsed = current_time - attack.start_time;
            (elapsed / attack.duration).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Active attack in progress
#[derive(Clone, Debug)]
pub struct ActiveAttack {
    pub attack_type: AttackType,
    pub start_time: f32,
    pub duration: f32,
    pub has_hit: bool,
}

// ========================
// Damage Numbers
// ========================

/// Floating damage number for display
#[derive(Clone, Debug)]
pub struct DamageNumber {
    pub position: Vec3<f32>,
    pub damage: f32,
    pub is_critical: bool,
    pub is_dodged: bool,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl DamageNumber {
    pub fn new(position: Vec3<f32>, damage: f32, is_critical: bool, is_dodged: bool) -> Self {
        Self {
            position,
            damage,
            is_critical,
            is_dodged,
            lifetime: 1.5,
            max_lifetime: 1.5,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.lifetime -= delta_time;
        self.position.y += delta_time * 2.0; // Float up
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime <= 0.0
    }

    pub fn alpha(&self) -> f32 {
        (self.lifetime / self.max_lifetime).clamp(0.0, 1.0)
    }
}
