//! Player system

use vek::Vec3;

/// Player state
pub struct Player {
    pub position: Vec3<f32>,
    pub velocity: Vec3<f32>,
    pub rotation: f32,
    pub on_ground: bool,
    pub health: f32,
    pub max_health: f32,
    pub is_alive: bool,
    
    // Physics
    pub gravity: f32,
    pub jump_force: f32,
    pub move_speed: f32,
    
    // Animation state
    pub anim_state: AnimationState,
    pub anim_time: f32,
}

/// Animation states
#[derive(Clone, Copy, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
    Jumping,
    Falling,
    Swimming,
    Attacking,
    Dead,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 80.0, 0.0), // Start high above terrain
            velocity: Vec3::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            on_ground: false,
            health: 100.0,
            max_health: 100.0,
            is_alive: true,

            gravity: -25.0,
            jump_force: 9.0,
            move_speed: 8.0,

            anim_state: AnimationState::Idle,
            anim_time: 0.0,
        }
    }
    
    /// Update player state
    pub fn update(&mut self, move_x: f32, move_y: f32, delta_time: f32) {
        if !self.is_alive {
            return;
        }
        
        // Apply gravity
        if !self.on_ground {
            self.velocity.y += self.gravity * delta_time;
        }
        
        // Calculate movement direction
        let move_dir = Vec3::new(move_x, 0.0, move_y);
        
        if move_dir.magnitude_squared() > 0.001 {
            // Normalize and apply speed
            let normalized = move_dir.normalized();
            self.velocity.x = normalized.x * self.move_speed;
            self.velocity.z = normalized.z * self.move_speed;

            // Update rotation to face movement direction
            self.rotation = move_dir.z.atan2(move_dir.x);

            // Update animation state
            let speed = move_dir.magnitude();
            if speed > 0.7 {
                self.anim_state = AnimationState::Running;
            } else {
                self.anim_state = AnimationState::Walking;
            }
        } else {
            // No input - slow down
            self.velocity.x *= 0.85;
            self.velocity.z *= 0.85;
            
            if self.on_ground {
                self.anim_state = AnimationState::Idle;
            }
        }
        
        // Apply velocity
        self.position += self.velocity * delta_time;
        
        // Simple ground collision
        let ground_height = 64.0; // TODO: Get from world
        if self.position.y < ground_height {
            self.position.y = ground_height;
            self.velocity.y = 0.0;
            self.on_ground = true;
            
            if self.anim_state == AnimationState::Falling {
                self.anim_state = AnimationState::Idle;
            }
        } else {
            self.on_ground = false;
            
            if self.velocity.y < 0.0 {
                self.anim_state = AnimationState::Falling;
            }
        }
        
        // Update animation time
        self.anim_time += delta_time;
    }
    
    /// Jump
    pub fn jump(&mut self) {
        if self.on_ground && self.is_alive {
            self.velocity.y = self.jump_force;
            self.on_ground = false;
            self.anim_state = AnimationState::Jumping;
        }
    }
    
    /// Take damage
    pub fn take_damage(&mut self, amount: f32) {
        if !self.is_alive {
            return;
        }
        
        self.health -= amount;
        
        if self.health <= 0.0 {
            self.health = 0.0;
            self.is_alive = false;
            self.anim_state = AnimationState::Dead;
        }
    }
    
    /// Heal
    pub fn heal(&mut self, amount: f32) {
        if !self.is_alive {
            return;
        }
        
        self.health = (self.health + amount).min(self.max_health);
    }
    
    /// Respawn
    pub fn respawn(&mut self) {
        self.position = Vec3::new(0.0, 80.0, 0.0);
        self.velocity = Vec3::zero();
        self.health = self.max_health;
        self.is_alive = true;
        self.anim_state = AnimationState::Idle;
        self.on_ground = false;
    }
    
    /// Attack
    pub fn attack(&mut self) {
        if self.is_alive && self.anim_state != AnimationState::Attacking {
            self.anim_state = AnimationState::Attacking;
            self.anim_time = 0.0;
        }
    }

    /// Check if currently jumping
    pub fn is_jumping(&self) -> bool {
        self.anim_state == AnimationState::Jumping
    }

    /// Check if currently attacking
    pub fn is_attacking(&self) -> bool {
        self.anim_state == AnimationState::Attacking
    }

    /// Check if animation is complete
    pub fn is_anim_complete(&self) -> bool {
        match self.anim_state {
            AnimationState::Attacking => self.anim_time > 0.5,
            _ => true,
        }
    }
}
