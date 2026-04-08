//! Character System for Android
//!
//! 3D character rendering using veloren-compatible Body types.
//! Supports humanoid characters with animated limbs.

use gl;
use vek::{Vec2, Vec3, Vec4};
use crate::veloren_types::Body;

// ========================
// Character Body Parts
// ========================

/// Body part for rendering
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyPart {
    Head,
    Chest,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
    LeftHand,
    RightHand,
    LeftFoot,
    RightFoot,
}

impl BodyPart {
    /// Get default dimensions for a body part [width, height, depth]
    pub fn default_dimensions(&self) -> Vec3<f32> {
        match self {
            BodyPart::Head => Vec3::new(0.5, 0.5, 0.5),
            BodyPart::Chest => Vec3::new(0.6, 0.7, 0.35),
            BodyPart::LeftArm | BodyPart::RightArm => Vec3::new(0.25, 0.7, 0.25),
            BodyPart::LeftLeg | BodyPart::RightLeg => Vec3::new(0.3, 0.75, 0.3),
            BodyPart::LeftHand | BodyPart::RightHand => Vec3::new(0.2, 0.25, 0.2),
            BodyPart::LeftFoot | BodyPart::RightFoot => Vec3::new(0.3, 0.2, 0.45),
        }
    }

    /// Get offset from body center
    pub fn offset_from_center(&self) -> Vec3<f32> {
        match self {
            BodyPart::Head => Vec3::new(0.0, 0.9, 0.0),
            BodyPart::Chest => Vec3::new(0.0, 0.35, 0.0),
            BodyPart::LeftArm => Vec3::new(-0.45, 0.4, 0.0),
            BodyPart::RightArm => Vec3::new(0.45, 0.4, 0.0),
            BodyPart::LeftLeg => Vec3::new(-0.2, -0.35, 0.0),
            BodyPart::RightLeg => Vec3::new(0.2, -0.35, 0.0),
            BodyPart::LeftHand => Vec3::new(-0.45, -0.1, 0.0),
            BodyPart::RightHand => Vec3::new(0.45, -0.1, 0.0),
            BodyPart::LeftFoot => Vec3::new(-0.2, -0.85, 0.075),
            BodyPart::RightFoot => Vec3::new(0.2, -0.85, 0.075),
        }
    }
}

// ========================
// Character Mesh
// ========================

/// Vertex for character rendering
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CharacterVertex {
    pub position: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub color: Vec3<f32>,
}

impl CharacterVertex {
    pub fn new(position: Vec3<f32>, normal: Vec3<f32>, color: Vec3<f32>) -> Self {
        Self {
            position,
            normal,
            color,
        }
    }
}

/// Mesh data for a character
pub struct CharacterMesh {
    pub vertices: Vec<CharacterVertex>,
    pub indices: Vec<u32>,
    pub part_transforms: Vec<Vec3<f32>>, // Per-part animation transforms
}

impl CharacterMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            part_transforms: vec![Vec3::zero(); 10], // One per BodyPart
        }
    }

    /// Build mesh from body type
    pub fn from_body(body: &Body, animation_state: &CharacterAnimation) -> Self {
        let mut mesh = Self::new();

        // Determine body color based on type
        let body_color = match body {
            Body::Humanoid(_) => Vec3::new(0.85, 0.72, 0.6), // Skin tone
            Body::Dwarf(_) => Vec3::new(0.75, 0.65, 0.55),
            Body::Orc(_) => Vec3::new(0.4, 0.6, 0.35),
            _ => Vec3::new(0.7, 0.7, 0.7),
        };

        // Generate mesh for each body part
        for part in &[
            BodyPart::Head,
            BodyPart::Chest,
            BodyPart::LeftArm,
            BodyPart::RightArm,
            BodyPart::LeftLeg,
            BodyPart::RightLeg,
            BodyPart::LeftHand,
            BodyPart::RightHand,
            BodyPart::LeftFoot,
            BodyPart::RightFoot,
        ] {
            let dimensions = part.default_dimensions();
            let offset = part.offset_from_center();

            // Apply animation transform
            let anim_offset = animation_state.get_part_offset(*part);
            let final_offset = offset + anim_offset;

            // Create box mesh for this part
            mesh.add_box(final_offset, dimensions, body_color);
        }

        mesh
    }

    /// Add a box to the mesh
    fn add_box(&mut self, center: Vec3<f32>, size: Vec3<f32>, color: Vec3<f32>) {
        let half = size / 2.0;

        // Front face
        self.add_quad(
            [
                Vec3::new(center.x - half.x, center.y - half.y, center.z + half.z),
                Vec3::new(center.x + half.x, center.y - half.y, center.z + half.z),
                Vec3::new(center.x + half.x, center.y + half.y, center.z + half.z),
                Vec3::new(center.x - half.x, center.y + half.y, center.z + half.z),
            ],
            Vec3::unit_z(),
            color,
        );

        // Back face
        self.add_quad(
            [
                Vec3::new(center.x + half.x, center.y - half.y, center.z - half.z),
                Vec3::new(center.x - half.x, center.y - half.y, center.z - half.z),
                Vec3::new(center.x - half.x, center.y + half.y, center.z - half.z),
                Vec3::new(center.x + half.x, center.y + half.y, center.z - half.z),
            ],
            -Vec3::unit_z(),
            color,
        );

        // Left face
        self.add_quad(
            [
                Vec3::new(center.x - half.x, center.y - half.y, center.z - half.z),
                Vec3::new(center.x - half.x, center.y - half.y, center.z + half.z),
                Vec3::new(center.x - half.x, center.y + half.y, center.z + half.z),
                Vec3::new(center.x - half.x, center.y + half.y, center.z - half.z),
            ],
            -Vec3::unit_x(),
            color,
        );

        // Right face
        self.add_quad(
            [
                Vec3::new(center.x + half.x, center.y - half.y, center.z + half.z),
                Vec3::new(center.x + half.x, center.y - half.y, center.z - half.z),
                Vec3::new(center.x + half.x, center.y + half.y, center.z - half.z),
                Vec3::new(center.x + half.x, center.y + half.y, center.z + half.z),
            ],
            Vec3::unit_x(),
            color,
        );

        // Top face
        self.add_quad(
            [
                Vec3::new(center.x - half.x, center.y + half.y, center.z + half.z),
                Vec3::new(center.x + half.x, center.y + half.y, center.z + half.z),
                Vec3::new(center.x + half.x, center.y + half.y, center.z - half.z),
                Vec3::new(center.x - half.x, center.y + half.y, center.z - half.z),
            ],
            Vec3::unit_y(),
            color,
        );

        // Bottom face
        self.add_quad(
            [
                Vec3::new(center.x - half.x, center.y - half.y, center.z - half.z),
                Vec3::new(center.x + half.x, center.y - half.y, center.z - half.z),
                Vec3::new(center.x + half.x, center.y - half.y, center.z + half.z),
                Vec3::new(center.x - half.x, center.y - half.y, center.z + half.z),
            ],
            -Vec3::unit_y(),
            color,
        );
    }

    /// Add a quad (4 vertices, 6 indices)
    fn add_quad(&mut self, vertices: [Vec3<f32>; 4], normal: Vec3<f32>, color: Vec3<f32>) {
        let base = self.vertices.len() as u32;

        for v in &vertices {
            self.vertices.push(CharacterVertex::new(*v, normal, color));
        }

        // Two triangles
        self.indices.push(base);
        self.indices.push(base + 1);
        self.indices.push(base + 2);
        self.indices.push(base);
        self.indices.push(base + 2);
        self.indices.push(base + 3);
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get index count
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

// ========================
// Character Animation
// ========================

/// Character animation state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterAnimation {
    Idle,
    Walking(f32),  // Walk cycle progress (0-1)
    Running(f32),  // Run cycle progress (0-1)
    Jumping,
    Falling,
    Swimming(f32),
    Attacking(f32),
    Dead,
}

impl CharacterAnimation {
    /// Get animation offset for a body part
    pub fn get_part_offset(&self, part: BodyPart) -> Vec3<f32> {
        match self {
            CharacterAnimation::Idle => self.idle_offset(part),
            CharacterAnimation::Walking(progress) => self.walk_offset(part, progress),
            CharacterAnimation::Running(progress) => self.run_offset(part, progress),
            CharacterAnimation::Jumping => self.jump_offset(part),
            CharacterAnimation::Falling => self.fall_offset(part),
            CharacterAnimation::Swimming(progress) => self.swim_offset(part, progress),
            CharacterAnimation::Attacking(progress) => self.attack_offset(part, progress),
            CharacterAnimation::Dead => self.dead_offset(part),
        }
    }

    fn idle_offset(&self, part: BodyPart) -> Vec3<f32> {
        // Subtle breathing animation
        match part {
            BodyPart::Chest => Vec3::new(0.0, 0.01, 0.01),
            BodyPart::Head => Vec3::new(0.0, 0.015, 0.0),
            _ => Vec3::zero(),
        }
    }

    fn walk_offset(&self, part: BodyPart, progress: f32) -> Vec3<f32> {
        let cycle = (progress * std::f32::consts::TAU).sin();
        let cycle_cos = (progress * std::f32::consts::TAU).cos();

        match part {
            BodyPart::LeftLeg => Vec3::new(0.0, cycle * 0.15, cycle * 0.2),
            BodyPart::RightLeg => Vec3::new(0.0, -cycle * 0.15, -cycle * 0.2),
            BodyPart::LeftArm => Vec3::new(0.0, -cycle * 0.1, -cycle * 0.15),
            BodyPart::RightArm => Vec3::new(0.0, cycle * 0.1, cycle * 0.15),
            BodyPart::Chest => Vec3::new(0.0, cycle_cos * 0.02, 0.0),
            BodyPart::Head => Vec3::new(0.0, cycle_cos * 0.03, 0.0),
            _ => Vec3::zero(),
        }
    }

    fn run_offset(&self, part: BodyPart, progress: f32) -> Vec3<f32> {
        let cycle = (progress * std::f32::consts::TAU).sin();
        let cycle_cos = (progress * std::f32::consts::TAU).cos();

        match part {
            BodyPart::LeftLeg => Vec3::new(0.0, cycle * 0.25, cycle * 0.35),
            BodyPart::RightLeg => Vec3::new(0.0, -cycle * 0.25, -cycle * 0.35),
            BodyPart::LeftArm => Vec3::new(0.0, -cycle * 0.2, -cycle * 0.25),
            BodyPart::RightArm => Vec3::new(0.0, cycle * 0.2, cycle * 0.25),
            BodyPart::Chest => Vec3::new(0.0, cycle_cos * 0.04, cycle_cos * 0.02),
            BodyPart::Head => Vec3::new(0.0, cycle_cos * 0.05, 0.0),
            _ => Vec3::zero(),
        }
    }

    fn jump_offset(&self, part: BodyPart) -> Vec3<f32> {
        match part {
            BodyPart::LeftArm => Vec3::new(0.0, -0.2, -0.1),
            BodyPart::RightArm => Vec3::new(0.0, -0.2, -0.1),
            BodyPart::LeftLeg => Vec3::new(0.0, 0.1, 0.05),
            BodyPart::RightLeg => Vec3::new(0.0, 0.1, 0.05),
            _ => Vec3::zero(),
        }
    }

    fn fall_offset(&self, part: BodyPart) -> Vec3<f32> {
        match part {
            BodyPart::LeftArm => Vec3::new(-0.1, 0.1, 0.0),
            BodyPart::RightArm => Vec3::new(0.1, 0.1, 0.0),
            BodyPart::LeftLeg => Vec3::new(0.0, 0.05, -0.05),
            BodyPart::RightLeg => Vec3::new(0.0, 0.05, -0.05),
            _ => Vec3::zero(),
        }
    }

    fn swim_offset(&self, part: BodyPart, progress: f32) -> Vec3<f32> {
        let cycle = (progress * std::f32::consts::TAU).sin();

        match part {
            BodyPart::LeftArm => Vec3::new(0.0, cycle * 0.15, cycle * 0.1),
            BodyPart::RightArm => Vec3::new(0.0, cycle * 0.15, cycle * 0.1),
            BodyPart::LeftLeg => Vec3::new(0.0, cycle * 0.1, 0.0),
            BodyPart::RightLeg => Vec3::new(0.0, cycle * 0.1, 0.0),
            _ => Vec3::zero(),
        }
    }

    fn attack_offset(&self, part: BodyPart, progress: f32) -> Vec3<f32> {
        let swing = (progress * std::f32::consts::PI).sin().max(0.0);

        match part {
            BodyPart::RightArm => Vec3::new(-swing * 0.5, -swing * 0.3, -swing * 0.4),
            BodyPart::Chest => Vec3::new(-swing * 0.1, 0.0, -swing * 0.05),
            _ => Vec3::zero(),
        }
    }

    fn dead_offset(&self, part: BodyPart) -> Vec3<f32> {
        match part {
            BodyPart::Chest => Vec3::new(0.0, -0.3, 0.3),
            BodyPart::Head => Vec3::new(0.0, -0.5, 0.2),
            BodyPart::LeftArm => Vec3::new(-0.2, -0.2, 0.1),
            BodyPart::RightArm => Vec3::new(0.2, -0.2, 0.1),
            BodyPart::LeftLeg => Vec3::new(-0.1, -0.1, 0.2),
            BodyPart::RightLeg => Vec3::new(0.1, -0.1, 0.2),
            _ => Vec3::zero(),
        }
    }
}

// ========================
/// Character Renderer Data
// ========================

/// OpenGL render data for a character
pub struct CharacterRenderData {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub index_count: u32,
    pub position: Vec3<f32>,
    pub orientation: Vec4<f32>,
    pub is_initialized: bool,
}

impl CharacterRenderData {
    pub fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            index_count: 0,
            position: Vec3::zero(),
            orientation: Vec4::unit_w(),
            is_initialized: false,
        }
    }

    /// Initialize OpenGL buffers
    pub fn initialize(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::GenBuffers(1, &mut self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            // Position (location 0)
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<CharacterVertex>() as i32,
                std::mem::offset_of!(CharacterVertex, position) as *const _,
            );

            // Normal (location 1)
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<CharacterVertex>() as i32,
                std::mem::offset_of!(CharacterVertex, normal) as *const _,
            );

            // Color (location 2)
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<CharacterVertex>() as i32,
                std::mem::offset_of!(CharacterVertex, color) as *const _,
            );

            gl::BindVertexArray(0);
        }

        self.is_initialized = true;
    }

    /// Update mesh data
    pub fn update(&self, mesh: &CharacterMesh) {
        if !self.is_initialized {
            return;
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<CharacterVertex>()) as isize,
                mesh.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
                mesh.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            self.index_count = mesh.index_count() as u32;
        }
    }

    /// Render the character
    pub fn render(&self) {
        if !self.is_initialized || self.index_count == 0 {
            return;
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }

    /// Cleanup
    pub fn cleanup(&mut self) {
        if self.is_initialized {
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao);
                gl::DeleteBuffers(1, &self.vbo);
                gl::DeleteBuffers(1, &self.ebo);
            }
            self.is_initialized = false;
        }
    }
}
