//! Terrain Renderer for Android
//!
//! Renders terrain meshes using OpenGL ES with proper shaders.

use gl;
use std::ffi::CStr;
use vek::{Vec2, Vec3, Vec4};
use crate::terrain_mesh::{ChunkMesh, TerrainVertex};

// ========================
// OpenGL Buffers
// ========================

/// OpenGL buffer storage for a terrain chunk
pub struct TerrainRenderData {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub index_count: u32,
    pub is_initialized: bool,
}

impl TerrainRenderData {
    pub fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            index_count: 0,
            is_initialized: false,
        }
    }

    /// Initialize OpenGL buffers
    pub fn initialize(&mut self) {
        unsafe {
            // Generate VAO
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            // Generate VBO
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            // Generate EBO
            gl::GenBuffers(1, &mut self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            // Setup vertex attributes
            // Position (location 0)
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<TerrainVertex>() as i32,
                std::mem::offset_of!(TerrainVertex, position) as *const _,
            );

            // Normal (location 1)
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<TerrainVertex>() as i32,
                std::mem::offset_of!(TerrainVertex, normal) as *const _,
            );

            // Color (location 2)
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<TerrainVertex>() as i32,
                std::mem::offset_of!(TerrainVertex, color) as *const _,
            );

            // TexCoords (location 3)
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<TerrainVertex>() as i32,
                std::mem::offset_of!(TerrainVertex, tex_coords) as *const _,
            );

            gl::BindVertexArray(0);
        }

        self.is_initialized = true;
    }

    /// Update buffer data
    pub fn update(&self, mesh: &ChunkMesh) {
        if !self.is_initialized {
            return;
        }

        unsafe {
            // Update VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<TerrainVertex>()) as isize,
                mesh.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Update EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
                mesh.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            self.index_count = mesh.index_count as u32;
        }
    }

    /// Render the mesh
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

    /// Cleanup OpenGL resources
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

// ========================
// Terrain Shader
// ========================

/// Shader program for terrain rendering
pub struct TerrainShader {
    pub program: u32,
    pub view_loc: i32,
    pub projection_loc: i32,
    pub model_loc: i32,
    pub light_dir_loc: i32,
    pub camera_pos_loc: i32,
    pub is_initialized: bool,
}

impl TerrainShader {
    pub fn new() -> Self {
        let vertex_shader = r#"
            #version 300 es
            layout(location = 0) in vec3 a_position;
            layout(location = 1) in vec3 a_normal;
            layout(location = 2) in vec3 a_color;
            layout(location = 3) in vec2 a_tex_coords;

            uniform mat4 u_view;
            uniform mat4 u_projection;
            uniform mat4 u_model;

            out vec3 v_normal;
            out vec3 v_color;
            out vec3 v_world_pos;

            void main() {
                vec4 world_pos = u_model * vec4(a_position, 1.0);
                v_world_pos = world_pos.xyz;
                v_normal = mat3(u_model) * a_normal;
                v_color = a_color;
                gl_Position = u_projection * u_view * world_pos;
            }
        "#;

        let fragment_shader = r#"
            #version 300 es
            precision mediump float;

            in vec3 v_normal;
            in vec3 v_color;
            in vec3 v_world_pos;

            uniform vec3 u_light_dir;
            uniform vec3 u_camera_pos;

            out vec4 frag_color;

            void main() {
                // Normalize inputs
                vec3 normal = normalize(v_normal);
                vec3 light_dir = normalize(u_light_dir);

                // Ambient + Diffuse lighting
                float ambient = 0.35;
                float diff = max(dot(normal, light_dir), 0.0);
                float lighting = ambient + diff * 0.65;

                // Simple fog based on distance
                float dist = length(v_world_pos - u_camera_pos);
                float fog_factor = clamp(dist / 200.0, 0.0, 1.0);
                vec3 fog_color = vec3(0.6, 0.7, 0.9);

                // Apply lighting
                vec3 final_color = v_color * lighting;

                // Apply fog
                final_color = mix(final_color, fog_color, fog_factor * 0.5);

                frag_color = vec4(final_color, 1.0);
            }
        "#;

        let program = compile_shaders(vertex_shader, fragment_shader);

        let view_loc = unsafe {
            let c_str = std::ffi::CString::new("u_view").unwrap();
            gl::GetUniformLocation(program, c_str.as_ptr())
        };

        let projection_loc = unsafe {
            let c_str = std::ffi::CString::new("u_projection").unwrap();
            gl::GetUniformLocation(program, c_str.as_ptr())
        };

        let model_loc = unsafe {
            let c_str = std::ffi::CString::new("u_model").unwrap();
            gl::GetUniformLocation(program, c_str.as_ptr())
        };

        let light_dir_loc = unsafe {
            let c_str = std::ffi::CString::new("u_light_dir").unwrap();
            gl::GetUniformLocation(program, c_str.as_ptr())
        };

        let camera_pos_loc = unsafe {
            let c_str = std::ffi::CString::new("u_camera_pos").unwrap();
            gl::GetUniformLocation(program, c_str.as_ptr())
        };

        Self {
            program,
            view_loc,
            projection_loc,
            model_loc,
            light_dir_loc,
            camera_pos_loc,
            is_initialized: program != 0,
        }
    }

    /// Use this shader program
    pub fn use_program(&self) {
        if self.is_initialized {
            unsafe {
                gl::UseProgram(self.program);
            }
        }
    }

    /// Set uniform mat4
    pub fn set_uniform_mat4(&self, location: i32, matrix: &[f32; 16]) {
        if location >= 0 {
            unsafe {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
            }
        }
    }

    /// Set uniform vec3
    pub fn set_uniform_vec3(&self, location: i32, vector: &[f32; 3]) {
        if location >= 0 {
            unsafe {
                gl::Uniform3fv(location, 1, vector.as_ptr());
            }
        }
    }
}

/// Compile vertex and fragment shaders into a program
fn compile_shaders(vertex_src: &str, fragment_src: &str) -> u32 {
    let mut success: i32 = 0;

    // Compile vertex shader
    let vertex_shader = unsafe {
        let shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str = std::ffi::CString::new(vertex_src).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut _);
            let msg = String::from_utf8_lossy(&buf);
            tracing::error!("Vertex shader compilation failed: {}", msg);
            return 0;
        }

        shader
    };

    // Compile fragment shader
    let fragment_shader = unsafe {
        let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str = std::ffi::CString::new(fragment_src).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut _);
            let msg = String::from_utf8_lossy(&buf);
            tracing::error!("Fragment shader compilation failed: {}", msg);
            return 0;
        }

        shader
    };

    // Link program
    let program = unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut _);
            let msg = String::from_utf8_lossy(&buf);
            tracing::error!("Shader program linking failed: {}", msg);
            return 0;
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    };

    tracing::info!("Terrain shader compiled successfully");
    program
}

// ========================
// Terrain Renderer
// ========================

/// Main terrain renderer
pub struct TerrainRenderer {
    pub shader: TerrainShader,
    pub render_data: TerrainRenderData,
}

impl TerrainRenderer {
    pub fn new() -> Self {
        Self {
            shader: TerrainShader::new(),
            render_data: TerrainRenderData::new(),
        }
    }

    /// Initialize the renderer
    pub fn initialize(&mut self) {
        self.render_data.initialize();
        if self.shader.is_initialized {
            tracing::info!("Terrain renderer initialized");
        }
    }

    /// Update mesh data
    pub fn update_mesh(&mut self, mesh: &ChunkMesh) {
        self.render_data.update(mesh);
    }

    /// Render the terrain
    pub fn render(&self, view_matrix: &[f32; 16], projection_matrix: &[f32; 16], camera_pos: Vec3<f32>) {
        if !self.shader.is_initialized || !self.render_data.is_initialized {
            return;
        }

        // Use shader
        self.shader.use_program();

        // Set uniforms
        self.shader.set_uniform_mat4(self.shader.view_loc, view_matrix);
        self.shader.set_uniform_mat4(self.shader.projection_loc, projection_matrix);

        // Identity model matrix
        let model = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        self.shader.set_uniform_mat4(self.shader.model_loc, &model);

        // Light direction (from above and slightly to the side)
        let light_dir = [0.5, 1.0, 0.3];
        self.shader.set_uniform_vec3(self.shader.light_dir_loc, &light_dir);

        // Camera position
        let cam_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
        self.shader.set_uniform_vec3(self.shader.camera_pos_loc, &cam_pos);

        // Render
        self.render_data.render();
    }
}
