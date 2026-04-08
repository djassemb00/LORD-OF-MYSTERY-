//! Skills and Leveling System
//!
//! Handles player progression, skills, and leveling.

use std::collections::HashMap;

// ========================
// Skill Types
// ========================

/// Skill category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SkillType {
    // Combat skills
    Sword,
    Axe,
    Bow,
    Staff,
    Shield,
    
    // Gathering skills
    Mining,
    Woodcutting,
    Farming,
    Fishing,
    
    // Crafting skills
    Blacksmithing,
    Cooking,
    Alchemy,
    Tailoring,
    
    // General
    Agility,
    Strength,
    Vitality,
    Endurance,
}

impl SkillType {
    /// Display name
    pub fn name(&self) -> &'static str {
        match self {
            SkillType::Sword => "Sword",
            SkillType::Axe => "Axe",
            SkillType::Bow => "Bow",
            SkillType::Staff => "Staff",
            SkillType::Shield => "Shield",
            SkillType::Mining => "Mining",
            SkillType::Woodcutting => "Woodcutting",
            SkillType::Farming => "Farming",
            SkillType::Fishing => "Fishing",
            SkillType::Blacksmithing => "Blacksmithing",
            SkillType::Cooking => "Cooking",
            SkillType::Alchemy => "Alchemy",
            SkillType::Tailoring => "Tailoring",
            SkillType::Agility => "Agility",
            SkillType::Strength => "Strength",
            SkillType::Vitality => "Vitality",
            SkillType::Endurance => "Endurance",
        }
    }

    /// Icon ID
    pub fn icon_id(&self) -> u32 {
        match self {
            SkillType::Sword => 1,
            SkillType::Axe => 2,
            SkillType::Bow => 3,
            SkillType::Staff => 4,
            SkillType::Shield => 5,
            SkillType::Mining => 6,
            SkillType::Woodcutting => 7,
            SkillType::Farming => 8,
            SkillType::Fishing => 9,
            SkillType::Blacksmithing => 10,
            SkillType::Cooking => 11,
            SkillType::Alchemy => 12,
            SkillType::Tailoring => 13,
            SkillType::Agility => 14,
            SkillType::Strength => 15,
            SkillType::Vitality => 16,
            SkillType::Endurance => 17,
        }
    }
}

// ========================
// Skill
// ========================

/// Individual skill
#[derive(Clone, Debug)]
pub struct Skill {
    pub skill_type: SkillType,
    pub level: u32,
    pub experience: f32,
    pub experience_to_next: f32,
}

impl Skill {
    pub fn new(skill_type: SkillType) -> Self {
        Self {
            skill_type,
            level: 1,
            experience: 0.0,
            experience_to_next: 100.0,
        }
    }

    /// Add experience
    pub fn add_experience(&mut self, amount: f32) -> bool {
        self.experience += amount;
        
        // Check for level up
        while self.experience >= self.experience_to_next {
            self.experience -= self.experience_to_next;
            self.level += 1;
            self.experience_to_next = self.calculate_xp_for_level(self.level);
            return true; // Leveled up
        }
        
        false
    }

    /// Calculate XP needed for level
    fn calculate_xp_for_level(level: u32) -> f32 {
        100.0 * (level as f32).powf(1.5)
    }

    /// Get experience percentage (0-1)
    pub fn experience_percent(&self) -> f32 {
        self.experience / self.experience_to_next
    }

    /// Get skill bonus based on level
    pub fn get_bonus(&self) -> f32 {
        self.level as f32 * 0.05 // 5% per level
    }
}

// ========================
// Player Level
// ========================

/// Player overall level
pub struct PlayerLevel {
    pub level: u32,
    pub experience: f32,
    pub experience_to_next: f32,
    pub skill_points: u32,
    pub attribute_points: u32,
}

impl PlayerLevel {
    pub fn new() -> Self {
        Self {
            level: 1,
            experience: 0.0,
            experience_to_next: 500.0,
            skill_points: 0,
            attribute_points: 0,
        }
    }

    /// Add experience
    pub fn add_experience(&mut self, amount: f32) -> Vec<SkillType> {
        self.experience += amount;
        let mut leveled_skills = Vec::new();
        
        while self.experience >= self.experience_to_next {
            self.experience -= self.experience_to_next;
            self.level += 1;
            self.experience_to_next = self.calculate_xp_for_level(self.level);
            self.skill_points += 2;
            self.attribute_points += 3;
        }
        
        leveled_skills
    }

    /// Calculate XP needed for level
    fn calculate_xp_for_level(level: u32) -> f32 {
        500.0 * (level as f32).powf(1.8)
    }

    /// Get experience percentage
    pub fn experience_percent(&self) -> f32 {
        self.experience / self.experience_to_next
    }
}

// ========================
// Skill Set
// ========================

/// All player skills
pub struct SkillSet {
    pub skills: HashMap<SkillType, Skill>,
    pub player_level: PlayerLevel,
}

impl SkillSet {
    pub fn new() -> Self {
        let mut skills = HashMap::new();
        
        // Initialize all skills at level 1
        for skill_type in &[
            SkillType::Sword,
            SkillType::Axe,
            SkillType::Bow,
            SkillType::Staff,
            SkillType::Shield,
            SkillType::Mining,
            SkillType::Woodcutting,
            SkillType::Farming,
            SkillType::Fishing,
            SkillType::Blacksmithing,
            SkillType::Cooking,
            SkillType::Alchemy,
            SkillType::Tailoring,
            SkillType::Agility,
            SkillType::Strength,
            SkillType::Vitality,
            SkillType::Endurance,
        ] {
            skills.insert(*skill_type, Skill::new(*skill_type));
        }
        
        Self {
            skills,
            player_level: PlayerLevel::new(),
        }
    }

    /// Add skill experience
    pub fn add_skill_xp(&mut self, skill_type: SkillType, amount: f32) -> bool {
        // Also add to player level
        self.player_level.add_experience(amount * 0.5);
        
        // Add to specific skill
        if let Some(skill) = self.skills.get_mut(&skill_type) {
            skill.add_experience(amount)
        } else {
            false
        }
    }

    /// Get skill level
    pub fn get_skill_level(&self, skill_type: SkillType) -> u32 {
        self.skills.get(&skill_type).map(|s| s.level).unwrap_or(1)
    }

    /// Get skill bonus
    pub fn get_skill_bonus(&self, skill_type: SkillType) -> f32 {
        self.skills.get(&skill_type).map(|s| s.get_bonus()).unwrap_or(0.0)
    }

    /// Get total level (sum of all skill levels)
    pub fn total_level(&self) -> u32 {
        self.skills.values().map(|s| s.level).sum()
    }

    /// Get highest skill
    pub fn highest_skill(&self) -> Option<(&SkillType, &Skill)> {
        self.skills.iter().max_by_key(|(_, s)| s.level)
    }
}

// ========================
// Attributes
// ========================

/// Player attributes
#[derive(Clone, Debug)]
pub struct Attributes {
    pub strength: u32,
    pub agility: u32,
    pub vitality: u32,
    pub endurance: u32,
    pub intelligence: u32,
    pub luck: u32,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            strength: 5,
            agility: 5,
            vitality: 5,
            endurance: 5,
            intelligence: 5,
            luck: 5,
        }
    }

    /// Get attack power from strength
    pub fn attack_power(&self) -> f32 {
        self.strength as f32 * 2.0
    }

    /// Get defense from vitality
    pub fn defense(&self) -> f32 {
        self.vitality as f32 * 1.5
    }

    /// Get speed from agility
    pub fn movement_speed(&self) -> f32 {
        5.0 + self.agility as f32 * 0.2
    }

    /// Get max health from vitality
    pub fn max_health(&self) -> f32 {
        100.0 + self.vitality as f32 * 10.0
    }

    /// Get max energy from endurance
    pub fn max_energy(&self) -> f32 {
        100.0 + self.endurance as f32 * 8.0
    }

    /// Get critical chance from agility and luck
    pub fn critical_chance(&self) -> f32 {
        0.05 + (self.agility as f32 + self.luck as f32) * 0.002
    }
}
