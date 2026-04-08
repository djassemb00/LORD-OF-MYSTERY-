//! OpenGL ES Mesh utilities for rendering 3D objects

use gl;
use std::mem;

/// Vertex with position, normal, and texture coordinates
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

/// A renderable mesh
pub struct Mesh {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub indices_count: u32,
}

impl Mesh {
    /// Create a new mesh from vertices and indices
    pub fn new(vertices: &[Vertex], indices: &[u32]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            // Create and bind VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create and bind VBO
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Create and bind EBO
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Set vertex attribute pointers
            let stride = mem::size_of::<Vertex>() as i32;

            // Position attribute (location = 0)
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                0 as *const _,
            );

            // Normal attribute (location = 1)
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<f32>()) as *const _,
            );

            // Texture coordinates attribute (location = 2)
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * mem::size_of::<f32>()) as *const _,
            );

            // Unbind VAO
            gl::BindVertexArray(0);
        }

        Self {
            vao,
            vbo,
            ebo,
            indices_count: indices.len() as u32,
        }
    }

    /// Render the mesh
    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices_count as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

/// Create a simple cube mesh
pub fn create_cube() -> Mesh {
    let vertices = vec![
        // Front face
        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },
        // Back face
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
        // Left face
        Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coords: [0.0, 1.0] },
        // Right face
        Vertex { position: [ 0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], normal: [1.0, 0.0, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], normal: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
        // Top face
        Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },
        // Bottom face
        Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    ];

    let indices = vec![
        0,  1,  2,  2,  3,  0,   // Front
        4,  5,  6,  6,  7,  4,   // Back
        8,  9,  10, 10, 11, 8,   // Left
        12, 13, 14, 14, 15, 12,  // Right
        16, 17, 18, 18, 19, 16,  // Top
        20, 21, 22, 22, 23, 20,  // Bottom
    ];

    Mesh::new(&vertices, &indices)
}
