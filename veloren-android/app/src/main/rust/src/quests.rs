//! Quest System
//!
//! Handles quest creation, tracking, and completion.

use std::collections::HashMap;

// ========================
// Quest Types
// ========================

/// Type of quest objective
#[derive(Clone, Debug)]
pub enum QuestObjective {
    /// Kill N entities of a type
    Kill { entity_type: String, current: u32, required: u32 },
    /// Collect N items
    Collect { item_id: u32, current: u32, required: u32 },
    /// Reach a location
    ReachLocation { location: String, completed: bool },
    /// Talk to an NPC
    TalkToNPC { npc_id: u64, completed: bool },
    /// Craft an item
    Craft { item_id: u32, current: u32, required: u32 },
    /// Complete a dungeon
    CompleteDungeon { dungeon_id: u32, completed: bool },
    /// Deliver an item
    Deliver { item_id: u32, target_npc: u64, completed: bool },
}

impl QuestObjective {
    /// Check if objective is complete
    pub fn is_complete(&self) -> bool {
        match self {
            QuestObjective::Kill { current, required, .. } => current >= required,
            QuestObjective::Collect { current, required, .. } => current >= required,
            QuestObjective::ReachLocation { completed, .. } => *completed,
            QuestObjective::TalkToNPC { completed, .. } => *completed,
            QuestObjective::Craft { current, required, .. } => current >= required,
            QuestObjective::CompleteDungeon { completed, .. } => *completed,
            QuestObjective::Deliver { completed, .. } => *completed,
        }
    }

    /// Progress text
    pub fn progress_text(&self) -> String {
        match self {
            QuestObjective::Kill { entity_type, current, required } => {
                format!("Kill {} ({}/{})", entity_type, current, required)
            },
            QuestObjective::Collect { item_id, current, required } => {
                format!("Collect item {} ({}/{})", item_id, current, required)
            },
            QuestObjective::ReachLocation { location, completed } => {
                if *completed {
                    format!("Reach {} ✓", location)
                } else {
                    format!("Reach {}", location)
                }
            },
            QuestObjective::TalkToNPC { npc_id, completed } => {
                if *completed {
                    format!("Talk to NPC {} ✓", npc_id)
                } else {
                    format!("Talk to NPC {}", npc_id)
                }
            },
            QuestObjective::Craft { item_id, current, required } => {
                format!("Craft item {} ({}/{})", item_id, current, required)
            },
            QuestObjective::CompleteDungeon { dungeon_id, completed } => {
                if *completed {
                    format!("Complete dungeon {} ✓", dungeon_id)
                } else {
                    format!("Complete dungeon {}", dungeon_id)
                }
            },
            QuestObjective::Deliver { item_id, target_npc, completed } => {
                if *completed {
                    format!("Deliver item {} to NPC {} ✓", item_id, target_npc)
                } else {
                    format!("Deliver item {} to NPC {}", item_id, target_npc)
                }
            },
        }
    }

    /// Update progress
    pub fn update(&mut self, amount: u32) {
        match self {
            QuestObjective::Kill { current, .. } |
            QuestObjective::Collect { current, .. } |
            QuestObjective::Craft { current, .. } => {
                *current += amount;
            },
            QuestObjective::ReachLocation { completed, .. } |
            QuestObjective::TalkToNPC { completed, .. } |
            QuestObjective::CompleteDungeon { completed, .. } |
            QuestObjective::Deliver { completed, .. } => {
                *completed = true;
            },
        }
    }
}

// ========================
// Quest
// ========================

/// Quest state
#[derive(Clone, Debug)]
pub enum QuestState {
    Available,
    InProgress,
    Completed,
    TurnedIn,
    Failed,
}

/// A quest
#[derive(Clone, Debug)]
pub struct Quest {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub giver_npc_id: u64,
    pub turn_in_npc_id: u64,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
    pub state: QuestState,
    pub level_requirement: u32,
}

impl Quest {
    /// Check if all objectives are complete
    pub fn all_objectives_complete(&self) -> bool {
        self.objectives.iter().all(|o| o.is_complete())
    }

    /// Check if quest can be accepted
    pub fn can_accept(&self, player_level: u32) -> bool {
        self.state == QuestState::Available && player_level >= self.level_requirement
    }

    /// Accept quest
    pub fn accept(&mut self) {
        self.state = QuestState::InProgress;
    }

    /// Complete quest
    pub fn complete(&mut self) {
        if self.all_objectives_complete() {
            self.state = QuestState::Completed;
        }
    }

    /// Turn in quest
    pub fn turn_in(&mut self) {
        self.state = QuestState::TurnedIn;
    }

    /// Get progress text
    pub fn progress_text(&self) -> String {
        let mut text = format!("{}\n", self.title);
        for objective in &self.objectives {
            text.push_str(&format!("  - {}\n", objective.progress_text()));
        }
        text
    }
}

/// Quest rewards
#[derive(Clone, Debug)]
pub struct QuestRewards {
    pub experience: u32,
    pub coins: u32,
    pub items: Vec<(u32, u32)>, // (item_id, count)
    pub reputation: i32,
}

impl QuestRewards {
    pub fn new(experience: u32, coins: u32) -> Self {
        Self {
            experience,
            coins,
            items: Vec::new(),
            reputation: 0,
        }
    }

    pub fn with_item(mut self, item_id: u32, count: u32) -> Self {
        self.items.push((item_id, count));
        self
    }
}

// ========================
// Quest Manager
// ========================

/// Manages all quests
pub struct QuestManager {
    pub available_quests: Vec<Quest>,
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<u32>, // Quest IDs
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            available_quests: Vec::new(),
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
        }
    }

    /// Add available quest
    pub fn add_available_quest(&mut self, quest: Quest) {
        self.available_quests.push(quest);
    }

    /// Accept a quest
    pub fn accept_quest(&mut self, quest_id: u32, player_level: u32) -> bool {
        if let Some(idx) = self.available_quests.iter().position(|q| q.id == quest_id) {
            let quest = &self.available_quests[idx];
            if quest.can_accept(player_level) {
                let mut quest = self.available_quests.remove(idx);
                quest.accept();
                self.active_quests.push(quest);
                return true;
            }
        }
        false
    }

    /// Update quest objective
    pub fn update_objective(&mut self, objective_type: &str, amount: u32) {
        for quest in &mut self.active_quests {
            for objective in &mut quest.objectives {
                match (objective, objective_type) {
                    (QuestObjective::Kill { entity_type, .. }, "kill") => {
                        if !objective.is_complete() {
                            objective.update(amount);
                        }
                    },
                    (QuestObjective::Collect { .. }, "collect") => {
                        if !objective.is_complete() {
                            objective.update(amount);
                        }
                    },
                    (QuestObjective::Craft { .. }, "craft") => {
                        if !objective.is_complete() {
                            objective.update(amount);
                        }
                    },
                    _ => {},
                }
            }

            // Check if quest is complete
            if quest.all_objectives_complete() {
                quest.complete();
            }
        }
    }

    /// Turn in completed quest
    pub fn turn_in_quest(&mut self, quest_id: u32) -> Option<QuestRewards> {
        if let Some(idx) = self.active_quests.iter().position(|q| q.id == quest_id) {
            let quest = &self.active_quests[idx];
            if quest.state == QuestState::Completed {
                let quest = self.active_quests.remove(idx);
                let rewards = quest.rewards.clone();
                self.completed_quests.push(quest.id);
                return Some(rewards);
            }
        }
        None
    }

    /// Get active quests
    pub fn get_active_quests(&self) -> Vec<&Quest> {
        self.active_quests.iter().collect()
    }

    /// Get available quests for NPC
    pub fn get_available_quests_for_npc(&self, npc_id: u64) -> Vec<&Quest> {
        self.available_quests.iter()
            .filter(|q| q.giver_npc_id == npc_id)
            .collect()
    }

    /// Get active quest count
    pub fn active_quest_count(&self) -> usize {
        self.active_quests.len()
    }

    /// Check if quest is completed
    pub fn is_quest_completed(&self, quest_id: u32) -> bool {
        self.completed_quests.contains(&quest_id)
    }
}

// ========================
// Default Quests
// ========================

/// Create default starter quests
pub fn default_starter_quests() -> Vec<Quest> {
    vec![
        Quest {
            id: 1,
            title: "First Steps".to_string(),
            description: "Kill 5 slimes to prove your worth.".to_string(),
            giver_npc_id: 1,
            turn_in_npc_id: 1,
            objectives: vec![
                QuestObjective::Kill {
                    entity_type: "Slime".to_string(),
                    current: 0,
                    required: 5,
                },
            ],
            rewards: QuestRewards::new(100, 50),
            state: QuestState::Available,
            level_requirement: 1,
        },
        Quest {
            id: 2,
            title: "Gathering Materials".to_string(),
            description: "Collect 10 wood and 10 stone.".to_string(),
            giver_npc_id: 2,
            turn_in_npc_id: 2,
            objectives: vec![
                QuestObjective::Collect { item_id: 101, current: 0, required: 10 },
                QuestObjective::Collect { item_id: 102, current: 0, required: 10 },
            ],
            rewards: QuestRewards::new(150, 75),
            state: QuestState::Available,
            level_requirement: 1,
        },
        Quest {
            id: 3,
            title: "The Dark Cave".to_string(),
            description: "Explore and clear the dark cave.".to_string(),
            giver_npc_id: 3,
            turn_in_npc_id: 3,
            objectives: vec![
                QuestObjective::Kill {
                    entity_type: "Skeleton".to_string(),
                    current: 0,
                    required: 10,
                },
                QuestObjective::CompleteDungeon { dungeon_id: 1, completed: false },
            ],
            rewards: QuestRewards::new(500, 200).with_item(200, 1),
            state: QuestState::Available,
            level_requirement: 5,
        },
    ]
}
