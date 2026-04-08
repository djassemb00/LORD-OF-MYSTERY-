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

use render::GlRenderer;
use input::InputHandler;
use assets::AssetManager;
use world::WorldManager;
use camera::Camera;
use player::Player;
use particles::ParticleSystem;

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
        }
    }

    fn update(&mut self) {
        if !self.is_running {
            return;
        }

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

        self.frame_count += 1;
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

            game_state.renderer.render(
                game_state.delta_time,
                &view_matrix,
                &projection_matrix,
            );
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
