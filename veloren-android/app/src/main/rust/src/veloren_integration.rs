//! Veloren Common Integration
//!
//! This module bridges our Android game with veloren-common,
//! using the official ECS, character system, and world generation.

// ========================
// ECS Integration
// ========================

use specs::{
    Builder, Component, Dispatcher, DispatcherBuilder, Entities, Entity, Join, Read, ReadStorage,
    System, VecStorage, World, Write, WriteStorage, DenseVecStorage,
};

use vek::{Vec2, Vec3, Vec4};

// Re-export veloren-common types we use
pub use veloren_common::comp;
pub use veloren_common::character::{Character, CharacterId};
pub use veloren_common::combat::{Damage, DamageSource, Knockback};

// ========================
// Android-Specific Components
// ========================

/// Component for Android touch input
#[derive(Debug, Clone, Copy)]
pub struct AndroidTouchInput {
    pub movement: Vec2<f32>,    // Left joystick (-1 to 1)
    pub camera: Vec2<f32>,      // Right joystick (-1 to 1)
    pub jump: bool,
    pub attack: bool,
    pub interact: bool,
}

impl Component for AndroidTouchInput {
    type Storage = VecStorage<Self>;
}

/// Component for Android rendering state
#[derive(Debug, Clone)]
pub struct AndroidRenderState {
    pub screen_width: u32,
    pub screen_height: u32,
    pub aspect_ratio: f32,
}

impl Component for AndroidRenderState {
    type Storage = VecStorage<Self>;
}

/// Component for player camera on Android
#[derive(Debug, Clone, Copy)]
pub struct AndroidCamera {
    pub position: Vec3<f32>,
    pub pitch: f32,    // Up/down angle
    pub yaw: f32,      // Left/right angle
    pub distance: f32, // Distance from character
    pub fov: f32,
}

impl AndroidCamera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 30.0, 30.0),
            pitch: -0.5,
            yaw: 0.0,
            distance: 30.0,
            fov: 70.0,
        }
    }

    /// Get view matrix (column-major for OpenGL)
    pub fn view_matrix(&self) -> [f32; 16] {
        let target = self.position - Vec3::new(0.0, self.distance * 0.5, 0.0);
        look_at(self.position, target, Vec3::unit_y())
    }

    /// Get projection matrix (column-major for OpenGL)
    pub fn projection_matrix(&self) -> [f32; 16] {
        perspective(self.fov, self.aspect_ratio(), 0.1, 1000.0)
    }

    fn aspect_ratio(&self) -> f32 {
        // Will be set from render state
        16.0 / 9.0
    }
}

impl Component for AndroidCamera {
    type Storage = VecStorage<Self>;
}

// ========================
// Simplified Player Component
// ========================

/// Simplified player component that wraps veloren-common types
#[derive(Clone, Debug)]
pub struct AndroidPlayer {
    pub position: Vec3<f32>,
    pub velocity: Vec3<f32>,
    pub orientation: Vec4<f32>,
    pub body: comp::Body,
    pub health: f32,
    pub max_health: f32,
    pub is_alive: bool,
    pub on_ground: bool,
}

impl Component for AndroidPlayer {
    type Storage = VecStorage<Self>;
}

impl AndroidPlayer {
    pub fn new(body: comp::Body) -> Self {
        Self {
            position: Vec3::new(0.0, 200.0, 0.0),
            velocity: Vec3::zero(),
            orientation: Vec4::unit_w(),
            body,
            health: 100.0,
            max_health: 100.0,
            is_alive: true,
            on_ground: false,
        }
    }
}

// ========================
// ECS World Setup
// ========================

/// Creates a Veloren ECS world with all necessary components
pub fn create_veloren_world() -> World {
    let mut world = World::new();

    // Register Android-specific components
    world.register::<AndroidTouchInput>();
    world.register::<AndroidRenderState>();
    world.register::<AndroidCamera>();
    world.register::<AndroidPlayer>();

    tracing::info!("Veloren ECS world created with Android components");
    world
}

/// Create a player entity with all necessary components
pub fn create_player_entity(world: &mut World, body: comp::Body) -> Entity {
    world
        .create_entity()
        .with(AndroidPlayer::new(body))
        .with(AndroidTouchInput {
            movement: Vec2::zero(),
            camera: Vec2::zero(),
            jump: false,
            attack: false,
            interact: false,
        })
        .with(AndroidCamera::new())
        .build()
}

// ========================
// ECS Systems
// ========================

/// System to process Android touch input
pub struct AndroidInputSystem;

impl<'s> System<'s> for AndroidInputSystem {
    type SystemData = (
        ReadStorage<'s, AndroidTouchInput>,
        WriteStorage<'s, AndroidPlayer>,
    );

    fn run(&mut self, (input, mut player): Self::SystemData) {
        for (input, player) in (&input, &mut player).join() {
            if !player.is_alive {
                continue;
            }

            // Convert 2D input to 3D movement
            let move_dir = Vec3::new(input.movement.x, 0.0, input.movement.y);
            let speed = 8.0; // meters per second

            if move_dir.magnitude_squared() > 0.001 {
                // Update velocity
                player.velocity.x = move_dir.x * speed;
                player.velocity.z = move_dir.z * speed;

                // Update orientation to face movement direction
                let look_dir = Vec2::new(move_dir.x, move_dir.z);
                if look_dir.magnitude_squared() > 0.001 {
                    player.orientation = Vec4::look(
                        Vec3::zero(),
                        Vec3::new(move_dir.x, 0.0, move_dir.z).normalized(),
                        Vec3::unit_y(),
                    );
                }
            } else {
                // Slow down when no input
                player.velocity.x *= 0.8;
                player.velocity.z *= 0.8;
            }

            // Jump (check if on ground - will be updated with terrain height externally)
            if input.jump && player.on_ground {
                player.velocity.y = 10.0;
                player.on_ground = false;
            }

            // Update position
            player.position.x += player.velocity.x * 0.016; // Assume 60 FPS
            player.position.z += player.velocity.z * 0.016;

            // Gravity (ground height will be corrected externally)
            player.velocity.y -= 0.3;
            player.position.y += player.velocity.y * 0.016;
        }
    }
}

/// System to update Android camera to follow player
pub struct AndroidCameraSystem;

impl<'s> System<'s> for AndroidCameraSystem {
    type SystemData = (
        ReadStorage<'s, AndroidPlayer>,
        WriteStorage<'s, AndroidCamera>,
        ReadStorage<'s, AndroidTouchInput>,
    );

    fn run(&mut self, (player, mut camera, input): Self::SystemData) {
        for (player_pos, cam, touch) in (&player, &mut camera, &input).join() {
            // Camera follows player from behind and above
            let offset = Vec3::new(0.0, 15.0, 20.0);
            cam.position = player_pos.position + offset;

            // Adjust pitch based on camera input
            cam.pitch -= touch.camera.y * 0.02;
            cam.pitch = cam.pitch.clamp(-1.5, -0.1);

            // Adjust yaw based on camera input
            cam.yaw -= touch.camera.x * 0.02;
        }
    }
}

// ========================
// Dispatcher
// ========================

/// Creates a dispatcher with all systems
pub fn create_dispatcher<'a>() -> DispatcherBuilder<'a, 'a> {
    let mut builder = DispatcherBuilder::new();
    builder.add(AndroidInputSystem, "android_input", &[]);
    builder.add(AndroidCameraSystem, "android_camera", &["android_input"]);
    builder
}

// ========================
// Matrix Helpers
// ========================

/// Create a look-at view matrix (column-major for OpenGL)
fn look_at(eye: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> [f32; 16] {
    let f = (target - eye).normalized();
    let s = f.cross(up).normalized();
    let u = s.cross(f);

    [
        s.x, u.x, -f.x, 0.0, s.y, u.y, -f.y, 0.0, s.z, u.z, -f.z, 0.0,
        -s.dot(eye),
        -u.dot(eye),
        f.dot(eye),
        1.0,
    ]
}

/// Create a perspective projection matrix (column-major for OpenGL)
fn perspective(fov_degrees: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let fov_rad = fov_degrees.to_radians();
    let f = 1.0 / (fov_rad / 2.0).tan();
    let nf = 1.0 / (near - far);

    [
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (far + near) * nf,
        -1.0,
        0.0,
        0.0,
        2.0 * far * near * nf,
        0.0,
    ]
}

// ========================
// Game State Manager
// ========================

/// High-level game state using veloren-common
pub struct VelorenGameState {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>,
    pub player_entity: Option<Entity>,
    pub tick_count: u64,
    /// Terrain seed for height calculations
    pub terrain_seed: u64,
}

impl VelorenGameState {
    pub fn new() -> Self {
        let mut world = create_veloren_world();

        // Create player with default humanoid body
        let player = create_player_entity(&mut world, comp::Body::Humanoid(comp::body::Humanoid::default()));
        let player_entity = Some(player);

        // Build dispatcher
        let mut builder = create_dispatcher();
        let dispatcher = builder.build();

        tracing::info!("Veloren game state initialized");

        Self {
            world,
            dispatcher,
            player_entity,
            tick_count: 0,
            terrain_seed: 12345,
        }
    }

    /// Calculate terrain height at world position (simple noise-based)
    pub fn get_terrain_height(&self, wx: i32, wz: i32) -> i32 {
        // Simple multi-octave noise (same as terrain.rs)
        let base_height = 64;
        let seed = self.terrain_seed;
        let seed_offset = (seed % 2000) as i32 - 1000;

        let continental = self.noise_2d(wx as f64 * 0.002, wz as f64 * 0.002, seed) * 40.0;
        let hills = self.noise_2d(wx as f64 * 0.01, wz as f64 * 0.01, seed + 1) * 20.0;
        let bumps = self.noise_2d(wx as f64 * 0.05, wz as f64 * 0.05, seed + 2) * 5.0;
        let mountains = (self.noise_2d(wx as f64 * 0.001, wz as f64 * 0.001, seed + 3) * 80.0).abs();

        (base_height + seed_offset / 2 + continental as i32 + hills as i32
            + bumps as i32 + mountains as i32).max(10).min(200)
    }

    /// Simple 2D noise function
    fn noise_2d(&self, x: f64, z: f64, seed: u64) -> f64 {
        let ix = x.floor() as i64;
        let iz = z.floor() as i64;
        let fx = x - ix as f64;
        let fz = z - iz as f64;

        let sx = fx * fx * (3.0 - 2.0 * fx);
        let sz = fz * fz * (3.0 - 2.0 * fz);

        let n00 = self.hash(ix, iz, seed) / 255.0;
        let n10 = self.hash(ix + 1, iz, seed) / 255.0;
        let n01 = self.hash(ix, iz + 1, seed) / 255.0;
        let n11 = self.hash(ix + 1, iz + 1, seed) / 255.0;

        let nx0 = n00 * (1.0 - sx) + n10 * sx;
        let nx1 = n01 * (1.0 - sx) + n11 * sx;

        nx0 * (1.0 - sz) + nx1 * sz - 0.5
    }

    /// Simple hash function
    fn hash(&self, x: i64, z: i64, seed: u64) -> f64 {
        let mut h = seed.wrapping_add((x.wrapping_mul(374761393) ^ z.wrapping_mul(668265263)) as u64);
        h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        h = h ^ (h >> 16);
        (h & 0xFF) as f64
    }

    /// Update game state
    pub fn update(&mut self, delta_time: f32) {
        // Dispatch all systems
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
        self.tick_count += 1;
    }

    /// Get player position
    pub fn get_player_position(&self) -> Vec3<f32> {
        if let Some(entity) = self.player_entity {
            if let Some(player) = self.world.read_storage::<AndroidPlayer>().get(entity) {
                return player.position;
            }
        }
        Vec3::zero()
    }

    /// Get player health
    pub fn get_player_health(&self) -> f32 {
        if let Some(entity) = self.player_entity {
            if let Some(player) = self.world.read_storage::<AndroidPlayer>().get(entity) {
                return player.health;
            }
        }
        0.0
    }

    /// Get player body type
    pub fn get_player_body(&self) -> Option<comp::Body> {
        if let Some(entity) = self.player_entity {
            if let Some(player) = self.world.read_storage::<AndroidPlayer>().get(entity) {
                return Some(player.body.clone());
            }
        }
        None
    }

    /// Get tick count for debugging
    pub fn get_tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Set touch input for player
    pub fn set_player_input(&mut self, movement: Vec2<f32>, camera: Vec2<f32>, jump: bool, attack: bool) {
        if let Some(entity) = self.player_entity {
            if let Some(input) = self.world.write_storage::<AndroidTouchInput>().get_mut(entity) {
                input.movement = movement;
                input.camera = camera;
                input.jump = jump;
                input.attack = attack;
            }
        }
    }

    /// Get camera view matrix
    pub fn get_camera_view_matrix(&self) -> Option<[f32; 16]> {
        if let Some(entity) = self.player_entity {
            if let Some(camera) = self.world.read_storage::<AndroidCamera>().get(entity) {
                return Some(camera.view_matrix());
            }
        }
        None
    }

    /// Get camera projection matrix
    pub fn get_camera_projection_matrix(&self) -> Option<[f32; 16]> {
        if let Some(entity) = self.player_entity {
            if let Some(camera) = self.world.read_storage::<AndroidCamera>().get(entity) {
                return Some(camera.projection_matrix());
            }
        }
        None
    }
}
