//! Inventory System
//!
//! Handles items, equipment, and inventory management.

use std::collections::HashMap;

// ========================
// Item Types
// ========================

/// Item rarity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl ItemRarity {
    pub fn color(&self) -> [f32; 3] {
        match self {
            ItemRarity::Common => [0.7, 0.7, 0.7],
            ItemRarity::Uncommon => [0.2, 0.8, 0.2],
            ItemRarity::Rare => [0.2, 0.4, 1.0],
            ItemRarity::Epic => [0.6, 0.2, 0.8],
            ItemRarity::Legendary => [1.0, 0.5, 0.0],
        }
    }
}

/// Item type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    // Weapons
    Sword,
    Axe,
    Bow,
    Staff,
    Shield,
    
    // Armor
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Gloves,
    
    // Tools
    Pickaxe,
    Shovel,
    Hammer,
    
    // Consumables
    HealthPotion,
    EnergyPotion,
    Food,
    
    // Materials
    Wood,
    Stone,
    Iron,
    Gold,
    Diamond,
    
    // Misc
    QuestItem,
    Key,
}

// ========================
// Item
// ========================

/// An item in the inventory
#[derive(Clone, Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub icon_id: u32,
    pub stack_size: u32,
    pub max_stack: u32,
    pub description: String,
    
    // Stats
    pub attack_bonus: f32,
    pub defense_bonus: f32,
    pub speed_bonus: f32,
    pub health_bonus: f32,
    
    // Value
    pub value: u32,
}

impl Item {
    pub fn new(id: u32, name: &str, item_type: ItemType, rarity: ItemRarity) -> Self {
        Self {
            id,
            name: name.to_string(),
            item_type,
            rarity,
            icon_id: id,
            stack_size: 1,
            max_stack: match item_type {
                ItemType::HealthPotion | ItemType::EnergyPotion | ItemType::Food => 64,
                ItemType::Wood | ItemType::Stone | ItemType::Iron | ItemType::Gold => 99,
                _ => 1,
            },
            description: String::new(),
            attack_bonus: 0.0,
            defense_bonus: 0.0,
            speed_bonus: 0.0,
            health_bonus: 0.0,
            value: match rarity {
                ItemRarity::Common => 10,
                ItemRarity::Uncommon => 50,
                ItemRarity::Rare => 200,
                ItemRarity::Epic => 1000,
                ItemRarity::Legendary => 5000,
            },
        }
    }

    /// Check if item is stackable
    pub fn is_stackable(&self) -> bool {
        self.max_stack > 1
    }

    /// Check if stack is full
    pub fn is_stack_full(&self) -> bool {
        self.stack_size >= self.max_stack
    }
}

// ========================
// Inventory Slot
// ========================

/// A slot in the inventory
#[derive(Clone, Debug)]
pub struct InventorySlot {
    pub item: Option<Item>,
    pub is_equipped: bool,
}

impl InventorySlot {
    pub fn new() -> Self {
        Self {
            item: None,
            is_equipped: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.item.is_none()
    }

    pub fn item_type(&self) -> Option<ItemType> {
        self.item.as_ref().map(|i| i.item_type)
    }
}

// ========================
// Inventory
// ========================

/// Player inventory
pub struct Inventory {
    pub slots: Vec<InventorySlot>,
    pub max_slots: usize,
    pub coins: u32,
    
    // Equipment
    pub equipment: HashMap<ItemType, Option<Item>>,
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        let mut equipment = HashMap::new();
        equipment.insert(ItemType::Sword, None);
        equipment.insert(ItemType::Shield, None);
        equipment.insert(ItemType::Helmet, None);
        equipment.insert(ItemType::Chestplate, None);
        equipment.insert(ItemType::Leggings, None);
        equipment.insert(ItemType::Boots, None);
        equipment.insert(ItemType::Gloves, None);

        Self {
            slots: vec![InventorySlot::new(); max_slots],
            max_slots,
            coins: 0,
            equipment,
        }
    }

    /// Add item to inventory
    pub fn add_item(&mut self, item: Item) -> bool {
        // Try to stack first
        if item.is_stackable() {
            for slot in &mut self.slots {
                if let Some(ref mut existing) = slot.item {
                    if existing.id == item.id && !existing.is_stack_full() {
                        let space = existing.max_stack - existing.stack_size;
                        let to_add = item.stack_size.min(space);
                        existing.stack_size += to_add;
                        return true;
                    }
                }
            }
        }

        // Find empty slot
        for slot in &mut self.slots {
            if slot.is_empty() {
                slot.item = Some(item);
                return true;
            }
        }

        false // Inventory full
    }

    /// Remove item from slot
    pub fn remove_item(&mut self, slot_index: usize, count: u32) -> Option<Item> {
        if slot_index >= self.slots.len() {
            return None;
        }

        let slot = &mut self.slots[slot_index];
        if let Some(ref mut item) = slot.item {
            if item.stack_size <= count {
                let removed = slot.item.take();
                slot.is_equipped = false;
                return removed;
            } else {
                item.stack_size -= count;
                let mut removed = item.clone();
                removed.stack_size = count;
                return Some(removed);
            }
        }

        None
    }

    /// Equip item from slot
    pub fn equip_item(&mut self, slot_index: usize) -> bool {
        if slot_index >= self.slots.len() {
            return false;
        }

        let slot = &self.slots[slot_index];
        if let Some(ref item) = slot.item {
            let item_type = item.item_type;
            
            // Check if it's equippable
            if self.equipment.contains_key(&item_type) {
                // Unequip current
                self.equipment.insert(item_type, None);
                
                // Equip new
                self.equipment.insert(item_type, Some(item.clone()));
                self.slots[slot_index].is_equipped = true;
                return true;
            }
        }

        false
    }

    /// Get total attack bonus from equipment
    pub fn get_attack_bonus(&self) -> f32 {
        self.equipment.values()
            .filter_map(|e| e.as_ref())
            .map(|i| i.attack_bonus)
            .sum()
    }

    /// Get total defense bonus from equipment
    pub fn get_defense_bonus(&self) -> f32 {
        self.equipment.values()
            .filter_map(|e| e.as_ref())
            .map(|i| i.defense_bonus)
            .sum()
    }

    /// Get slot count
    pub fn used_slots(&self) -> usize {
        self.slots.iter().filter(|s| !s.is_empty()).count()
    }

    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.used_slots() >= self.max_slots
    }
}

// ========================
// Default Items
// ========================

/// Create default starter items
pub fn default_starter_items() -> Vec<Item> {
    vec![
        Item::new(1, "Wooden Sword", ItemType::Sword, ItemRarity::Common),
        Item::new(2, "Wooden Shield", ItemType::Shield, ItemRarity::Common),
        Item::new(3, "Health Potion", ItemType::HealthPotion, ItemRarity::Common),
        Item::new(4, "Bread", ItemType::Food, ItemRarity::Common),
        Item::new(5, "Wooden Pickaxe", ItemType::Pickaxe, ItemRarity::Common),
    ]
}
