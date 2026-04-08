//! 3D Camera system for the game

use vek::{Vec3, Mat4};

/// Camera types
#[derive(Clone, Copy, PartialEq)]
pub enum CameraMode {
    ThirdPerson,
    FirstPerson,
    Orbit,
}

/// 3D Camera
pub struct Camera {
    pub position: Vec3<f32>,
    pub target: Vec3<f32>,
    pub yaw: f32,      // Horizontal rotation
    pub pitch: f32,    // Vertical rotation
    pub distance: f32, // Distance from target (third person)
    pub fov: f32,      // Field of view
    pub near_plane: f32,
    pub far_plane: f32,
    pub aspect_ratio: f32,
    pub mode: CameraMode,
    
    // Cached matrices
    view_matrix: Mat4<f32>,
    projection_matrix: Mat4<f32>,
    dirty: bool,
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 10.0, 20.0),
            target: Vec3::new(0.0, 5.0, 0.0),
            yaw: 0.0,
            pitch: -30.0_f32.to_radians(),
            distance: 20.0,
            fov: 60.0_f32.to_radians(),
            near_plane: 0.1,
            far_plane: 500.0,
            aspect_ratio: 16.0 / 9.0,
            mode: CameraMode::ThirdPerson,
            view_matrix: Mat4::identity(),
            projection_matrix: Mat4::identity(),
            dirty: true,
        };
        
        camera.update_matrices();
        camera
    }
    
    /// Update camera from input
    pub fn update(&mut self, look_x: f32, look_y: f32, delta_time: f32) {
        let sensitivity = 2.0;
        
        // Update yaw and pitch
        self.yaw += look_x * sensitivity * delta_time;
        self.pitch += look_y * sensitivity * delta_time;
        
        // Clamp pitch
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
        
        self.dirty = true;
    }
    
    /// Set camera position
    pub fn set_position(&mut self, position: Vec3<f32>) {
        self.target = position;
        self.dirty = true;
    }
    
    /// Set aspect ratio
    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
        self.dirty = true;
    }
    
    /// Set FOV
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov.to_radians();
        self.dirty = true;
    }
    
    /// Toggle camera mode
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            CameraMode::ThirdPerson => CameraMode::FirstPerson,
            CameraMode::FirstPerson => CameraMode::Orbit,
            CameraMode::Orbit => CameraMode::ThirdPerson,
        };
        self.dirty = true;
    }
    
    /// Get view matrix
    pub fn get_view_matrix(&mut self) -> [f32; 16] {
        if self.dirty {
            self.update_matrices();
        }
        self.view_matrix.into_col_array()
    }

    /// Get projection matrix
    pub fn get_projection_matrix(&mut self) -> [f32; 16] {
        if self.dirty {
            self.update_matrices();
        }
        self.projection_matrix.into_col_array()
    }
    
    /// Update view and projection matrices
    fn update_matrices(&mut self) {
        // Calculate camera position based on yaw, pitch, and distance
        let yaw_cos = self.yaw.cos();
        let yaw_sin = self.yaw.sin();
        let pitch_cos = self.pitch.cos();
        let pitch_sin = self.pitch.sin();
        
        match self.mode {
            CameraMode::ThirdPerson => {
                // Third person: behind and above the target
                let offset = Vec3::new(
                    yaw_cos * pitch_cos * self.distance,
                    pitch_sin * self.distance,
                    yaw_sin * pitch_cos * self.distance,
                );
                
                self.position = self.target + offset;
            }
            CameraMode::FirstPerson => {
                // First person: at the target
                self.position = self.target;
            }
            CameraMode::Orbit => {
                // Orbit: around the target
                let offset = Vec3::new(
                    yaw_cos * pitch_cos * self.distance,
                    pitch_sin * self.distance,
                    yaw_sin * pitch_cos * self.distance,
                );
                
                self.position = self.target + offset;
            }
        }
        
        // Create view matrix (look_at)
        self.view_matrix = Self::look_at(self.position, self.target, Vec3::unit_y());
        
        // Create projection matrix (perspective)
        self.projection_matrix = Self::perspective(
            self.fov,
            self.aspect_ratio,
            self.near_plane,
            self.far_plane,
        );
        
        self.dirty = false;
    }
    
    /// Create look-at matrix
    fn look_at(eye: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32> {
        let f = (target - eye).normalized();
        let s = f.cross(up).normalized();
        let u = s.cross(f);
        
        Mat4::new(
            s.x, u.x, -f.x, 0.0,
            s.y, u.y, -f.y, 0.0,
            s.z, u.z, -f.z, 0.0,
            -s.dot(eye), -u.dot(eye), f.dot(eye), 1.0,
        )
    }
    
    /// Create perspective projection matrix
    fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4<f32> {
        let f = 1.0 / (fov / 2.0).tan();
        let nf = 1.0 / (near - far);
        
        Mat4::new(
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (far + near) * nf, -1.0,
            0.0, 0.0, 2.0 * far * near * nf, 0.0,
        )
    }
    
    /// Get forward direction
    pub fn get_forward(&self) -> Vec3<f32> {
        let yaw_cos = self.yaw.cos();
        let yaw_sin = self.yaw.sin();
        let pitch_cos = self.pitch.cos();
        
        Vec3::new(yaw_sin * pitch_cos, self.pitch.sin(), yaw_cos * pitch_cos).normalized()
    }
    
    /// Get right direction
    pub fn get_right(&self) -> Vec3<f32> {
        self.get_forward().cross(Vec3::unit_y()).normalized()
    }
}
