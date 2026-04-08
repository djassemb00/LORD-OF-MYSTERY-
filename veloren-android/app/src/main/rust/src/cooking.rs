//! Cooking System
//!
//! Handles recipe management and food preparation.

use std::collections::HashMap;

// ========================
// Recipe
// ========================

/// A cooking recipe
#[derive(Clone, Debug)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub result_item_id: u32,
    pub result_count: u32,
    pub ingredients: Vec<(u32, u32)>, // (item_id, count)
    pub cooking_time: f32,
    pub experience: f32,
    pub required_level: u32,
    pub recipe_type: RecipeType,
}

impl Recipe {
    pub fn new(
        id: u32,
        name: &str,
        result_item_id: u32,
        result_count: u32,
        ingredients: Vec<(u32, u32)>,
        cooking_time: f32,
        experience: f32,
        required_level: u32,
        recipe_type: RecipeType,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            result_item_id,
            result_count,
            ingredients,
            cooking_time,
            experience,
            required_level,
            recipe_type,
        }
    }

    /// Check if player can craft this recipe
    pub fn can_craft(&self, player_level: u32) -> bool {
        player_level >= self.required_level
    }
}

// ========================
// Recipe Types
// ========================

/// Type of cooking recipe
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecipeType {
    Cooking,    // Basic cooking
    Baking,     // Oven baking
    Brewing,    // Brewing potions
    Alchemy,    // Alchemical recipes
    Smelting,   // Smelting ores
}

impl RecipeType {
    pub fn name(&self) -> &'static str {
        match self {
            RecipeType::Cooking => "Cooking",
            RecipeType::Baking => "Baking",
            RecipeType::Brewing => "Brewing",
            RecipeType::Alchemy => "Alchemy",
            RecipeType::Smelting => "Smelting",
        }
    }

    pub fn icon_id(&self) -> u32 {
        match self {
            RecipeType::Cooking => 1,
            RecipeType::Baking => 2,
            RecipeType::Brewing => 3,
            RecipeType::Alchemy => 4,
            RecipeType::Smelting => 5,
        }
    }
}

// ========================
// Cooking State
// ========================

/// Current cooking progress
pub struct CookingState {
    pub current_recipe: Option<u32>, // Recipe ID
    pub progress: f32,
    pub is_cooking: bool,
}

impl CookingState {
    pub fn new() -> Self {
        Self {
            current_recipe: None,
            progress: 0.0,
            is_cooking: false,
        }
    }

    /// Start cooking
    pub fn start(&mut self, recipe_id: u32) {
        self.current_recipe = Some(recipe_id);
        self.progress = 0.0;
        self.is_cooking = true;
    }

    /// Update cooking progress
    pub fn update(&mut self, delta_time: f32) -> bool {
        if !self.is_cooking {
            return false;
        }

        self.progress += delta_time;

        // Check if complete
        if let Some(recipe_id) = self.current_recipe {
            // Would get recipe from recipe manager
            let cooking_time = 5.0; // Placeholder
            if self.progress >= cooking_time {
                self.is_cooking = false;
                return true; // Complete
            }
        }

        false
    }

    /// Cancel cooking
    pub fn cancel(&mut self) {
        self.current_recipe = None;
        self.progress = 0.0;
        self.is_cooking = false;
    }

    /// Get progress (0-1)
    pub fn progress_percent(&self) -> f32 {
        if let Some(_) = self.current_recipe {
            let cooking_time = 5.0; // Placeholder
            (self.progress / cooking_time).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

// ========================
/// Recipe Manager
// ========================

/// Manages all recipes
pub struct RecipeManager {
    pub recipes: HashMap<u32, Recipe>,
    pub unlocked_recipes: Vec<u32>,
}

impl RecipeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            recipes: HashMap::new(),
            unlocked_recipes: Vec::new(),
        };

        // Add default recipes
        manager.add_default_recipes();

        manager
    }

    /// Add default recipes
    fn add_default_recipes(&mut self) {
        // Basic cooking recipes
        self.add_recipe(Recipe::new(
            1,
            "Cooked Meat",
            201,
            1,
            vec![(301, 1)], // Raw meat
            3.0,
            25.0,
            1,
            RecipeType::Cooking,
        ));

        self.add_recipe(Recipe::new(
            2,
            "Bread",
            202,
            1,
            vec![(302, 2)], // Wheat
            5.0,
            30.0,
            1,
            RecipeType::Baking,
        ));

        self.add_recipe(Recipe::new(
            3,
            "Vegetable Soup",
            203,
            1,
            vec![(303, 2), (304, 1)], // Vegetables, Water
            8.0,
            50.0,
            5,
            RecipeType::Cooking,
        ));

        self.add_recipe(Recipe::new(
            4,
            "Grilled Fish",
            204,
            1,
            vec![(305, 1)], // Raw fish
            4.0,
            35.0,
            3,
            RecipeType::Cooking,
        ));

        self.add_recipe(Recipe::new(
            5,
            "Health Potion",
            205,
            1,
            vec![(306, 2), (307, 1)], // Herbs, Water
            10.0,
            75.0,
            10,
            RecipeType::Alchemy,
        ));

        // Smelting recipes
        self.add_recipe(Recipe::new(
            6,
            "Iron Ingot",
            206,
            1,
            vec![(102, 3), (101, 1)], // Iron ore, Coal
            15.0,
            50.0,
            5,
            RecipeType::Smelting,
        ));

        self.add_recipe(Recipe::new(
            7,
            "Gold Ingot",
            207,
            1,
            vec![(103, 3), (101, 1)], // Gold ore, Coal
            20.0,
            75.0,
            10,
            RecipeType::Smelting,
        ));

        // Advanced recipes
        self.add_recipe(Recipe::new(
            8,
            "Steak Dinner",
            208,
            1,
            vec![(301, 2), (303, 1), (304, 1)],
            12.0,
            100.0,
            15,
            RecipeType::Cooking,
        ));

        self.add_recipe(Recipe::new(
            9,
            "Cake",
            209,
            1,
            vec![(302, 3), (308, 2), (309, 1)], // Wheat, Eggs, Milk
            20.0,
            150.0,
            20,
            RecipeType::Baking,
        ));

        self.add_recipe(Recipe::new(
            10,
            "Energy Potion",
            210,
            1,
            vec![(306, 3), (310, 1)], // Herbs, Crystal
            15.0,
            100.0,
            15,
            RecipeType::Alchemy,
        ));
    }

    /// Add a recipe
    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.id, recipe);
        self.unlocked_recipes.push(recipe.id);
    }

    /// Get recipe by ID
    pub fn get_recipe(&self, recipe_id: u32) -> Option<&Recipe> {
        self.recipes.get(&recipe_id)
    }

    /// Get all recipes
    pub fn get_all_recipes(&self) -> Vec<&Recipe> {
        self.recipes.values().collect()
    }

    /// Get recipes by type
    pub fn get_recipes_by_type(&self, recipe_type: RecipeType) -> Vec<&Recipe> {
        self.recipes.values()
            .filter(|r| r.recipe_type == recipe_type)
            .collect()
    }

    /// Get unlocked recipes
    pub fn get_unlocked_recipes(&self) -> Vec<&Recipe> {
        self.unlocked_recipes.iter()
            .filter_map(|id| self.recipes.get(id))
            .collect()
    }

    /// Unlock a recipe
    pub fn unlock_recipe(&mut self, recipe_id: u32) {
        if self.recipes.contains_key(&recipe_id) {
            if !self.unlocked_recipes.contains(&recipe_id) {
                self.unlocked_recipes.push(recipe_id);
            }
        }
    }

    /// Check if recipe is unlocked
    pub fn is_recipe_unlocked(&self, recipe_id: u32) -> bool {
        self.unlocked_recipes.contains(&recipe_id)
    }

    /// Get recipe count
    pub fn recipe_count(&self) -> usize {
        self.recipes.len()
    }

    /// Get unlocked recipe count
    pub fn unlocked_recipe_count(&self) -> usize {
        self.unlocked_recipes.len()
    }
}

// ========================
// Food Effects
// ========================

/// Effect of consuming food
#[derive(Clone, Debug)]
pub struct FoodEffect {
    pub effect_type: FoodEffectType,
    pub amount: f32,
    pub duration: f32, // 0 for instant
}

impl FoodEffect {
    pub fn heal(amount: f32) -> Self {
        Self {
            effect_type: FoodEffectType::Heal,
            amount,
            duration: 0.0,
        }
    }

    pub fn buff(stat: StatType, amount: f32, duration: f32) -> Self {
        Self {
            effect_type: FoodEffectType::Buff(stat),
            amount,
            duration,
        }
    }
}

/// Type of food effect
#[derive(Clone, Debug)]
pub enum FoodEffectType {
    Heal,
    Energy,
    Buff(StatType),
}

/// Stat type for buffs
#[derive(Clone, Copy, Debug)]
pub enum StatType {
    Attack,
    Defense,
    Speed,
    Luck,
}
