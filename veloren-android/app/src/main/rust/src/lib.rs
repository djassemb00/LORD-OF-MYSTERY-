//! Veloren Android - JNI Bridge
//!
//! This module provides the interface between Java/Kotlin (Android) and Rust (Native)

use std::sync::Mutex;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jint, jfloat};

// Import our modules
mod input;
mod render;
mod camera;

// Game systems (declared but not all actively used yet)
mod assets;
mod audio;
mod building;
mod caves;
mod character;
mod character_renderer;
mod combat;
mod cooking;
mod entities;
mod gathering;
mod hud;
mod inventory;
mod menu;
mod network;
mod particles;
mod player;
mod quests;
mod skills;
mod terrain;
mod terrain_mesh;
mod terrain_renderer;
mod veloren_integration;
mod veloren_types;
mod weather;
mod world;

use input::InputHandler;
use render::renderer::GlRenderer;
use camera::Camera;

/// Global game state
struct GameState {
    input_handler: InputHandler,
    renderer: GlRenderer,
    camera: Camera,
    is_initialized: bool,
    is_running: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            input_handler: InputHandler::new(),
            renderer: GlRenderer::new(),
            camera: Camera::new(),
            is_initialized: false,
            is_running: false,
        }
    }
}

lazy_static::lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
}

// ========================
// JNI Functions
// ========================

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_GameActivity_nativeInit(
    mut env: JNIEnv,
    class: JClass,
    screen_width: jint,
    screen_height: jint,
) {
    let mut state = GAME_STATE.lock().unwrap();
    
    tracing::info!("Native init: {}x{}", screen_width, screen_height);
    
    // Initialize input
    state.input_handler.set_screen_size(
        screen_width as f32,
        screen_height as f32,
    );
    
    // Initialize renderer
    if let Err(e) = state.renderer.initialize(
        screen_width as u32,
        screen_height as u32,
    ) {
        tracing::error!("Failed to initialize renderer: {}", e);
    }
    
    state.is_initialized = true;
    state.is_running = true;
    
    tracing::info!("Game initialized successfully");
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_GameActivity_nativeOnResume(
    mut env: JNIEnv,
    class: JClass,
) {
    let mut state = GAME_STATE.lock().unwrap();
    state.is_running = true;
    tracing::info!("Game resumed");
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_GameActivity_nativeOnPause(
    mut env: JNIEnv,
    class: JClass,
) {
    let mut state = GAME_STATE.lock().unwrap();
    state.is_running = false;
    tracing::info!("Game paused");
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_GameActivity_nativeOnDestroy(
    mut env: JNIEnv,
    class: JClass,
) {
    let mut state = GAME_STATE.lock().unwrap();
    state.is_running = false;
    state.is_initialized = false;
    tracing::info!("Game destroyed");
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_GameActivity_nativeUpdate(
    mut env: JNIEnv,
    class: JClass,
    delta_time: jfloat,
) {
    let mut state = GAME_STATE.lock().unwrap();
    
    if !state.is_running {
        return;
    }
    
    // Get input
    let (move_x, move_y) = state.input_handler.get_movement();
    let (look_x, look_y) = state.input_handler.get_camera_look();
    
    // Update camera
    state.camera.update(look_x, look_y, delta_time);
    
    // TODO: Update player, entities, physics, etc.
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_GameActivity_nativeJump(
    mut env: JNIEnv,
    class: JClass,
) {
    let state = GAME_STATE.lock().unwrap();
    // TODO: Implement jump logic
    tracing::debug!("Jump pressed");
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_GameActivity_nativeAttack(
    mut env: JNIEnv,
    class: JClass,
) {
    let state = GAME_STATE.lock().unwrap();
    // TODO: Implement attack logic
    tracing::debug!("Attack pressed");
}

// ========================
// Renderer JNI Functions
// ========================

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_VelorenRenderer_nativeInitRenderer(
    mut env: JNIEnv,
    class: JClass,
) {
    // Renderer is already initialized in nativeInit
    tracing::debug!("Native renderer initialized");
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_VelorenRenderer_nativeOnResize(
    mut env: JNIEnv,
    class: JClass,
    width: jint,
    height: jint,
) {
    let mut state = GAME_STATE.lock().unwrap();
    state.renderer.resize(width as u32, height as u32);
    state.input_handler.set_screen_size(width as f32, height as f32);
}

#[no_mangle]
pub extern "C" fn Java_djb1_com_veloren_VelorenRenderer_nativeRenderFrame(
    mut env: JNIEnv,
    class: JClass,
) {
    let mut state = GAME_STATE.lock().unwrap();
    
    if !state.is_running {
        return;
    }
    
    // Get view and projection matrices from camera
    let view_matrix = state.camera.get_view_matrix();
    let projection_matrix = state.camera.get_projection_matrix();
    
    // Render
    state.renderer.render(0.016, &view_matrix, &projection_matrix);
}
