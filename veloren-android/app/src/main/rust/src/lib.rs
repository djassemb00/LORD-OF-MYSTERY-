//! Veloren Android - Main Rust Library
//!
//! This library serves as the bridge between Android Java/Kotlin
//! and the Veloren game engine written in Rust.

use std::sync::Mutex;

mod render;
mod input;
mod assets;
mod world;
mod camera;
mod player;
mod particles;
mod veloren_integration;
mod terrain;
mod terrain_mesh;
mod terrain_renderer;
mod character;
mod character_renderer;
mod hud;
mod audio;
mod network;
mod menu;
mod combat;
mod inventory;
mod weather;
mod entities;
mod skills;
mod caves;
mod building;
mod quests;
mod gathering;
mod cooking;
mod veloren_types;

use render::GlRenderer;
use input::InputHandler;
use assets::AssetManager;
use world::WorldManager;
use camera::Camera;
use player::Player;
use particles::ParticleSystem;
use veloren_integration::VelorenGameState;
use terrain::TerrainWorld;
use terrain_renderer::TerrainRenderer;
use character_renderer::CharacterRenderer;
use hud::{HudState, HudRenderer};
use audio::AudioEngine;
use network::NetworkManager;
use menu::{MenuManager, MenuRenderer, MenuScreen, ButtonAction};
use combat::{CombatStats, CombatState, AttackType, DamageNumber};
use inventory::Inventory;
use weather::DayNightCycle;
use entities::EntityManager;
use skills::SkillSet;
use caves::CaveGenerator;
use building::BuildingManager;
use quests::QuestManager;
use gathering::GatheringManager;
use cooking::{RecipeManager, CookingState};

use vek::Vec2;

// Game state
static GAME_STATE: Mutex<Option<GameState>> = Mutex::new(None);

struct GameState {
    is_running: bool,
    screen_width: u32,
    screen_height: u32,
    delta_time: f32,
    frame_count: u64,
    renderer: GlRenderer,
    input_handler: InputHandler,
    asset_manager: AssetManager,
    world_manager: WorldManager,
    camera: Camera,
    player: Player,
    particle_system: ParticleSystem,
    
    // Veloren integration
    veloren_state: VelorenGameState,
    use_veloren: bool,
    
    // Terrain
    terrain_world: TerrainWorld,
    terrain_renderer: TerrainRenderer,
    terrain_mesh_dirty: bool,
    
    // Character
    character_renderer: CharacterRenderer,
    character_mesh_dirty: bool,
    
    // HUD
    hud_state: HudState,
    hud_renderer: HudRenderer,
    
    // Audio
    audio_engine: AudioEngine,
    
    // Network
    network_manager: NetworkManager,
    
    // Menu
    menu_manager: MenuManager,
    menu_renderer: MenuRenderer,
    
    // NEW: Combat
    combat_stats: CombatStats,
    combat_state: CombatState,
    damage_numbers: Vec<DamageNumber>,
    
    // NEW: Inventory
    inventory: Inventory,
    
    // NEW: Weather & Day/Night
    day_night_cycle: DayNightCycle,
    
    // NEW: Entities (NPCs/Monsters)
    entity_manager: EntityManager,
    
    // NEW: Skills
    skill_set: SkillSet,
    
    // NEW: Caves
    cave_generator: CaveGenerator,
    
    // NEW: Building
    building_manager: BuildingManager,
    
    // NEW: Quests
    quest_manager: QuestManager,
    
    // NEW: Gathering
    gathering_manager: GatheringManager,
    
    // NEW: Cooking
    recipe_manager: RecipeManager,
    cooking_state: CookingState,
}

impl GameState {
    fn new() -> Self {
        Self {
            is_running: false,
            screen_width: 0,
            screen_height: 0,
            delta_time: 0.0,
            frame_count: 0,
            renderer: GlRenderer::new(),
            input_handler: InputHandler::new(),
            asset_manager: AssetManager::new("assets"),
            world_manager: WorldManager::new(12345, 4),
            camera: Camera::new(),
            player: Player::new(),
            particle_system: ParticleSystem::new(1000),
            
            // Veloren integration
            veloren_state: VelorenGameState::new(),
            use_veloren: true,
            
            // Terrain
            terrain_world: TerrainWorld::new(12345, 4),
            terrain_renderer: TerrainRenderer::new(),
            terrain_mesh_dirty: true,
            
            // Character
            character_renderer: CharacterRenderer::new(),
            character_mesh_dirty: true,
            
            // HUD
            hud_state: HudState::new(),
            hud_renderer: HudRenderer::new(),
            
            // Audio
            audio_engine: AudioEngine::new(),
            
            // Network
            network_manager: NetworkManager::new(),
            
            // Menu
            menu_manager: MenuManager::new(),
            menu_renderer: MenuRenderer::new(),
            
            // Combat
            combat_stats: CombatStats::new(),
            combat_state: CombatState::new(),
            damage_numbers: Vec::new(),
            
            // Inventory
            inventory: Inventory::new(36), // 6x6 grid
            
            // Weather & Day/Night
            day_night_cycle: DayNightCycle::new(),
            
            // Entities
            entity_manager: EntityManager::new(),
            
            // Skills
            skill_set: SkillSet::new(),
            
            // Caves
            cave_generator: CaveGenerator::new(12345),
            
            // Building
            building_manager: BuildingManager::new(),
            
            // Quests
            quest_manager: {
                let mut qm = QuestManager::new();
                for quest in quests::default_starter_quests() {
                    qm.add_available_quest(quest);
                }
                qm
            },
            
            // Gathering
            gathering_manager: GatheringManager::new(),
            
            // Cooking
            recipe_manager: RecipeManager::new(),
            cooking_state: CookingState::new(),
        }
    }

    fn update(&mut self) {
        if !self.is_running {
            return;
        }

        if self.use_veloren {
            // Check if menu is visible
            if self.menu_manager.is_visible && self.menu_manager.current_screen != MenuScreen::Loading {
                // Menu is visible, don't update game
                return;
            }
            
            // NEW: Use veloren-common ECS system
            self.update_veloren();
        } else {
            // OLD: Use simple system
            self.update_legacy();
        }

        self.frame_count += 1;
    }
    
    /// Update using veloren-common ECS
    fn update_veloren(&mut self) {
        // Get input from touch handler
        let (move_x, move_y) = self.input_handler.get_movement();
        let (look_x, look_y) = self.input_handler.get_camera_look();
        
        // Pass to veloren ECS
        self.veloren_state.set_player_input(
            Vec2::new(move_x, move_y),
            Vec2::new(look_x, look_y),
            self.player.is_jumping(),
            self.player.is_attacking(),
        );
        
        // Update ECS systems
        self.veloren_state.update(self.delta_time);
        
        // Correct player Y position based on terrain height
        let player_pos = self.veloren_state.get_player_position();
        let terrain_height = self.veloren_state.get_terrain_height(
            player_pos.x as i32,
            player_pos.z as i32,
        ) as f32;
        
        // Ground collision correction
        if player_pos.y < terrain_height + 1.0 {
            // Snap player to ground
            if let Some(entity) = self.veloren_state.player_entity {
                if let Some(mut player) = self.veloren_state.world
                    .write_storage::<veloren_integration::AndroidPlayer>()
                    .get_mut(entity) 
                {
                    player.position.y = terrain_height + 1.0;
                    player.velocity.y = 0.0;
                    player.on_ground = true;
                }
            }
        } else {
            // Mark as not on ground
            if let Some(entity) = self.veloren_state.player_entity {
                if let Some(mut player) = self.veloren_state.world
                    .write_storage::<veloren_integration::AndroidPlayer>()
                    .get_mut(entity) 
                {
                    player.on_ground = false;
                }
            }
        }
        
        // Update terrain around player
        let player_wpos = vek::Vec3::new(
            player_pos.x as i32,
            player_pos.y as i32,
            player_pos.z as i32,
        );
        self.terrain_world.update_around(player_wpos);
        
        // Check if terrain mesh needs rebuild
        let dirty_chunks = self.terrain_world.get_dirty_chunks();
        if !dirty_chunks.is_empty() {
            self.terrain_mesh_dirty = true;
        }
        
        // Mark character mesh as dirty (animation changed)
        self.character_mesh_dirty = true;
        
        // Update HUD state from ECS
        if let Some(entity) = self.veloren_state.player_entity {
            if let Some(player) = self.veloren_state.world
                .read_storage::<veloren_integration::AndroidPlayer>()
                .get(entity) 
            {
                self.hud_state.set_health(player.health, player.max_health);
            }
        }
        
        // NEW: Update combat
        self.combat_state.update(self.delta_time);
        
        // Handle attack input
        if self.player.is_attacking() && !self.combat_state.is_attacking() {
            let current_time = self.frame_count as f32 * self.delta_time;
            if self.combat_state.start_attack(AttackType::Light, current_time) {
                self.audio_engine.play_sound(audio::SoundType::Attack);
                self.skill_set.add_skill_xp(skills::SkillType::Sword, 5.0);
            }
        }
        
        // Update damage numbers
        self.damage_numbers.retain_mut(|dn| {
            dn.update(self.delta_time);
            !dn.is_expired()
        });
        
        // NEW: Update day/night cycle
        self.day_night_cycle.update(self.delta_time);
        
        // NEW: Update entities
        self.entity_manager.update(player_pos, self.delta_time);
        
        // Spawn entities if needed (demo)
        if self.entity_manager.alive_count() < 5 && self.frame_count % 300 == 0 {
            use entities::{EntityType, EntityManager};
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let distance = 30.0 + rand::random::<f32>() * 20.0;
            let spawn_pos = Vec3::new(
                player_pos.x + angle.cos() * distance,
                terrain_height + 2.0,
                player_pos.z + angle.sin() * distance,
            );
            
            let entity_types = [
                EntityType::Slime,
                EntityType::Skeleton,
                EntityType::Zombie,
                EntityType::Spider,
            ];
            let entity_type = entity_types[rand::random::<usize>() % entity_types.len()];
            self.entity_manager.spawn(entity_type, spawn_pos);
        }
        
        // NEW: Update gathering
        self.gathering_manager.update(self.delta_time);
        
        // NEW: Update cooking
        if self.cooking_state.update(self.delta_time) {
            // Cooking complete
            if let Some(recipe_id) = self.cooking_state.current_recipe {
                if let Some(recipe) = self.recipe_manager.get_recipe(recipe_id) {
                    // Add cooking XP
                    self.skill_set.add_skill_xp(skills::SkillType::Cooking, recipe.experience);
                    tracing::info!("Cooked: {}", recipe.name);
                }
            }
            self.cooking_state.cancel();
        }
        
        // NEW: Spawn resource nodes periodically
        if self.frame_count % 600 == 0 && self.gathering_manager.node_count() < 20 {
            use gathering::ResourceType;
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let distance = 20.0 + rand::random::<f32>() * 30.0;
            let spawn_pos = Vec3::new(
                player_pos.x as i32 + (angle.cos() * distance) as i32,
                terrain_height as i32,
                player_pos.z as i32 + (angle.sin() * distance) as i32,
            );
            
            let resource_types = [
                ResourceType::Stone,
                ResourceType::Wood,
                ResourceType::Herb,
            ];
            let resource_type = resource_types[rand::random::<usize>() % resource_types.len()];
            self.gathering_manager.spawn_node(spawn_pos, resource_type);
        }
        
        // Update camera from ECS
        if let Some(view) = self.veloren_state.get_camera_view_matrix() {
            let _ = view;
        }
    }
    
    /// Update using legacy simple system
    fn update_legacy(&mut self) {
        // Get input
        let (move_x, move_y) = self.input_handler.get_movement();
        let (look_x, look_y) = self.input_handler.get_camera_look();

        // Update player
        self.player.update(move_x, move_y, self.delta_time);

        // Update camera
        self.camera.update(look_x, look_y, self.delta_time);
        self.camera.set_position(self.player.position);

        // Update world around player
        let px = self.player.position.x as i32;
        let pz = self.player.position.z as i32;
        self.world_manager.update_chunks(px, pz);
    }

    /// Handle menu button action
    fn handle_menu_action(&mut self, action: ButtonAction) {
        match action {
            ButtonAction::Play => {
                self.menu_manager.setup_character_select(
                    self.screen_width as f32,
                    self.screen_height as f32,
                );
                self.audio_engine.play_sound(audio::SoundType::Click);
            },
            ButtonAction::Settings => {
                self.menu_manager.setup_settings(
                    self.screen_width as f32,
                    self.screen_height as f32,
                );
                self.audio_engine.play_sound(audio::SoundType::Click);
            },
            ButtonAction::Servers => {
                self.menu_manager.setup_server_list(
                    self.screen_width as f32,
                    self.screen_height as f32,
                );
                self.audio_engine.play_sound(audio::SoundType::Click);
            },
            ButtonAction::StartGame => {
                // Hide menu and start game
                self.menu_manager.hide();
                self.audio_engine.play_music(audio::SoundType::Exploration);
                tracing::info!("Game started!");
            },
            ButtonAction::Resume => {
                self.menu_manager.hide();
                self.audio_engine.play_sound(audio::SoundType::Click);
            },
            ButtonAction::Back => {
                self.menu_manager.setup_main_menu(
                    self.screen_width as f32,
                    self.screen_height as f32,
                );
                self.audio_engine.play_sound(audio::SoundType::Click);
            },
            ButtonAction::Quit => {
                tracing::info!("Quit requested");
                // Would exit the app
            },
            ButtonAction::Connect => {
                // Connect to selected server
                if let Some(idx) = self.menu_manager.selected_server {
                    let servers = network::default_servers();
                    if let Some(server) = servers.get(idx) {
                        self.network_manager.connect(server.clone());
                        self.menu_manager.current_screen = MenuScreen::Loading;
                    }
                }
            },
            ButtonAction::Disconnect => {
                self.network_manager.disconnect();
                self.menu_manager.setup_main_menu(
                    self.screen_width as f32,
                    self.screen_height as f32,
                );
            },
            _ => {
                tracing::debug!("Unhandled menu action: {:?}", action);
            },
        }
    }
}

/// Initialize the game
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeInit(
    _env: *mut (),
    _class: *mut (),
    screen_width: i32,
    screen_height: i32,
) {
    // Initialize tracing subscriber for logging
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init();

    tracing::info!("=== Veloren Android ===");
    tracing::info!("Version: 0.1.0");
    tracing::info!("Screen: {}x{}", screen_width, screen_height);

    // Create game state
    let mut state = GAME_STATE.lock().unwrap();
    *state = Some(GameState {
        screen_width: screen_width as u32,
        screen_height: screen_height as u32,
        ..GameState::new()
    });

    // Setup main menu
    if let Some(ref mut game_state) = state.as_mut() {
        game_state.menu_manager.setup_main_menu(
            screen_width as f32,
            screen_height as f32,
        );
    }

    tracing::info!("Game state initialized successfully");
}

/// Resume the game
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeOnResume(
    _env: *mut (),
    _class: *mut (),
) {
    tracing::info!("Game resumed");

    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        game_state.is_running = true;
    }
}

/// Pause the game
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeOnPause(
    _env: *mut (),
    _class: *mut (),
) {
    tracing::info!("Game paused");

    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        game_state.is_running = false;
    }
}

/// Destroy the game
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeOnDestroy(
    _env: *mut (),
    _class: *mut (),
) {
    tracing::info!("Game destroyed");

    let mut state = GAME_STATE.lock().unwrap();
    *state = None;
}

/// Update game state
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeUpdate(
    _env: *mut (),
    _class: *mut (),
    delta_time: f32,
) {
    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        game_state.delta_time = delta_time;
        game_state.update();
    }
}

/// Jump action
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeJump(
    _env: *mut (),
    _class: *mut (),
) {
    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        game_state.player.jump();
        tracing::debug!("Player jump");
    }
}

/// Attack action
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeAttack(
    _env: *mut (),
    _class: *mut (),
) {
    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        game_state.player.attack();
        tracing::debug!("Player attack");
    }
}

/// Initialize the OpenGL renderer
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_VelorenRenderer_nativeInitRenderer(
    _env: *mut (),
    _class: *mut (),
) {
    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        if let Err(e) = game_state.renderer.initialize(
            game_state.screen_width,
            game_state.screen_height,
        ) {
            tracing::error!("Failed to initialize renderer: {}", e);
        } else {
            tracing::info!("Renderer initialized successfully");
        }

        // Initialize terrain renderer
        game_state.terrain_renderer.initialize();

        // Initialize character renderer
        game_state.character_renderer.initialize();

        // Initialize HUD renderer
        game_state.hud_renderer.initialize();
        
        // Initialize menu renderer
        game_state.menu_renderer.initialize();
        
        // Initialize audio engine
        game_state.audio_engine.initialize();
        
        // Play main menu music
        game_state.audio_engine.play_music(audio::SoundType::MainMenu);
    }
}

/// Handle surface resize
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_VelorenRenderer_nativeOnResize(
    _env: *mut (),
    _class: *mut (),
    width: i32,
    height: i32,
) {
    tracing::info!("Surface resized: {}x{}", width, height);

    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        game_state.screen_width = width as u32;
        game_state.screen_height = height as u32;
        game_state.renderer.resize(width as u32, height as u32);
        game_state.input_handler.set_screen_size(width as f32, height as f32);
        game_state.camera.set_aspect_ratio(width as f32 / height as f32);
    }
}

/// Render a frame
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_VelorenRenderer_nativeRenderFrame(
    _env: *mut (),
    _class: *mut (),
) {
    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        if game_state.is_running {
            // Update game logic
            game_state.update();

            // Render with camera view
            let view_matrix = game_state.camera.get_view_matrix();
            let projection_matrix = game_state.camera.get_projection_matrix();

            // If terrain mesh is dirty, rebuild it
            if game_state.terrain_mesh_dirty {
                // Get first chunk mesh (simplified - should iterate all chunks)
                let dirty_chunks = game_state.terrain_world.get_dirty_chunks();
                if let Some((_pos, chunk)) = dirty_chunks.first() {
                    // Generate mesh with greedy meshing
                    let mesh = terrain_mesh::generate_chunk_mesh_greedy(chunk);
                    
                    // Update terrain renderer
                    game_state.terrain_renderer.update_mesh(&mesh);
                    
                    // Mark chunk as clean
                    game_state.terrain_world.mark_chunk_clean(*_pos);
                }
                
                // If no dirty chunks, mark as clean
                if dirty_chunks.is_empty() {
                    game_state.terrain_mesh_dirty = false;
                }
            }

            // Rebuild character mesh if dirty
            if game_state.character_mesh_dirty {
                if let Some(entity) = game_state.veloren_state.player_entity {
                    if let Some(player) = game_state.veloren_state.world
                        .read_storage::<veloren_integration::AndroidPlayer>()
                        .get(entity) 
                    {
                        // Determine animation state
                        let anim_state = if player.velocity.y > 0.5 {
                            character::CharacterAnimation::Jumping
                        } else if player.velocity.y < -0.5 {
                            character::CharacterAnimation::Falling
                        } else if (player.velocity.x.abs() + player.velocity.z.abs()) > 5.0 {
                            character::CharacterAnimation::Running(
                                game_state.frame_count as f32 * 0.1
                            )
                        } else if (player.velocity.x.abs() + player.velocity.z.abs()) > 0.5 {
                            character::CharacterAnimation::Walking(
                                game_state.frame_count as f32 * 0.08
                            )
                        } else {
                            character::CharacterAnimation::Idle
                        };

                        // Build character mesh
                        let mesh = character::CharacterMesh::from_body(&player.body, &anim_state);
                        
                        // Update character renderer
                        game_state.character_renderer.update_character(
                            &mesh,
                            player.position,
                            player.orientation,
                        );
                        
                        game_state.character_mesh_dirty = false;
                    }
                }
            }

            // Render terrain if available
            if game_state.use_veloren && game_state.terrain_renderer.shader.is_initialized {
                let player_pos = game_state.veloren_state.get_player_position();
                
                // First render terrain
                game_state.terrain_renderer.render(
                    &view_matrix,
                    &projection_matrix,
                    player_pos,
                );
                
                // Then render character
                game_state.character_renderer.render(
                    &view_matrix,
                    &projection_matrix,
                    player_pos + vek::Vec3::new(0.0, 10.0, 0.0),
                );
            } else {
                // Fallback to old renderer
                game_state.renderer.render(
                    game_state.delta_time,
                    &view_matrix,
                    &projection_matrix,
                );
            }
            
            // Render HUD (2D overlay)
            game_state.hud_renderer.render_hud(
                &game_state.hud_state,
                game_state.screen_width as f32,
                game_state.screen_height as f32,
            );
            
            // Render menu if visible
            if game_state.menu_manager.is_visible {
                game_state.menu_renderer.render_menu(
                    &game_state.menu_manager,
                    game_state.screen_width as f32,
                    game_state.screen_height as f32,
                );
            }
        }
    }
}

/// Get game stats for debugging
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeGetStats(
    _env: *mut (),
    _class: *mut (),
) -> i64 {
    let state = GAME_STATE.lock().unwrap();
    if let Some(ref game_state) = state.as_ref() {
        return game_state.frame_count as i64;
    }
    0
}

/// Handle menu touch
#[no_mangle]
pub extern "system" fn Java_djb1_com_veloren_GameActivity_nativeMenuTouch(
    _env: *mut (),
    _class: *mut (),
    x: f32,
    y: f32,
    is_down: bool,
) {
    let mut state = GAME_STATE.lock().unwrap();
    if let Some(ref mut game_state) = state.as_mut() {
        if is_down {
            game_state.menu_manager.on_touch_down(x, y);
        } else {
            if let Some(action) = game_state.menu_manager.on_touch_up(x, y) {
                game_state.handle_menu_action(action);
            }
        }
    }
}
