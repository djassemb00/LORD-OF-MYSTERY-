//! Character Renderer
//!
//! Renders 3D characters with animations using OpenGL ES.

use gl;
use vek::{Vec3, Vec4};
use crate::character::{CharacterMesh, CharacterRenderData, CharacterVertex, CharacterAnimation};

// ========================
// Character Shader
// ========================

pub struct CharacterShader {
    pub program: u32,
    pub view_loc: i32,
    pub projection_loc: i32,
    pub model_loc: i32,
    pub light_dir_loc: i32,
    pub camera_pos_loc: i32,
    pub is_initialized: bool,
}

impl CharacterShader {
    pub fn new() -> Self {
        let vertex_shader = r#"
            #version 300 es
            layout(location = 0) in vec3 a_position;
            layout(location = 1) in vec3 a_normal;
            layout(location = 2) in vec3 a_color;

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
                vec3 normal = normalize(v_normal);
                vec3 light_dir = normalize(u_light_dir);

                // Ambient + Diffuse + Specular
                float ambient = 0.3;
                float diff = max(dot(normal, light_dir), 0.0);
                
                // Specular (Blinn-Phong)
                vec3 view_dir = normalize(u_camera_pos - v_world_pos);
                vec3 half_dir = normalize(light_dir + view_dir);
                float spec = pow(max(dot(normal, half_dir), 0.0), 32.0) * 0.3;

                float lighting = ambient + diff * 0.6 + spec;

                // Simple fog
                float dist = length(v_world_pos - u_camera_pos);
                float fog_factor = clamp(dist / 150.0, 0.0, 1.0);
                vec3 fog_color = vec3(0.6, 0.7, 0.9);

                vec3 final_color = v_color * lighting;
                final_color = mix(final_color, fog_color, fog_factor * 0.4);

                frag_color = vec4(final_color, 1.0);
            }
        "#;

        let program = compile_shaders(vertex_shader, fragment_shader);

        Self {
            program,
            view_loc: get_uniform_location(program, "u_view"),
            projection_loc: get_uniform_location(program, "u_projection"),
            model_loc: get_uniform_location(program, "u_model"),
            light_dir_loc: get_uniform_location(program, "u_light_dir"),
            camera_pos_loc: get_uniform_location(program, "u_camera_pos"),
            is_initialized: program != 0,
        }
    }

    pub fn use_program(&self) {
        if self.is_initialized {
            unsafe { gl::UseProgram(self.program); }
        }
    }

    pub fn set_uniform_mat4(&self, location: i32, matrix: &[f32; 16]) {
        if location >= 0 {
            unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr()); }
        }
    }

    pub fn set_uniform_vec3(&self, location: i32, vector: &[f32; 3]) {
        if location >= 0 {
            unsafe { gl::Uniform3fv(location, 1, vector.as_ptr()); }
        }
    }
}

fn compile_shaders(vertex_src: &str, fragment_src: &str) -> u32 {
    let mut success: i32 = 0;

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
            tracing::error!("Character vertex shader failed: {}", String::from_utf8_lossy(&buf));
            return 0;
        }
        shader
    };

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
            tracing::error!("Character fragment shader failed: {}", String::from_utf8_lossy(&buf));
            return 0;
        }
        shader
    };

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
            tracing::error!("Character shader link failed: {}", String::from_utf8_lossy(&buf));
            return 0;
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        program
    };

    tracing::info!("Character shader compiled successfully");
    program
}

fn get_uniform_location(program: u32, name: &str) -> i32 {
    unsafe {
        let c_str = std::ffi::CString::new(name).unwrap();
        gl::GetUniformLocation(program, c_str.as_ptr())
    }
}

// ========================
// Character Renderer Manager
// ========================

pub struct CharacterRenderer {
    pub shader: CharacterShader,
    pub render_data: CharacterRenderData,
}

impl CharacterRenderer {
    pub fn new() -> Self {
        Self {
            shader: CharacterShader::new(),
            render_data: CharacterRenderData::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.render_data.initialize();
        if self.shader.is_initialized {
            tracing::info!("Character renderer initialized");
        }
    }

    /// Update character mesh
    pub fn update_character(&mut self, mesh: &CharacterMesh, position: Vec3<f32>, orientation: Vec4<f32>) {
        self.render_data.update(mesh);
        self.render_data.position = position;
        self.render_data.orientation = orientation;
    }

    /// Render character with model matrix
    pub fn render(&self, view_matrix: &[f32; 16], projection_matrix: &[f32; 16], camera_pos: Vec3<f32>) {
        if !self.shader.is_initialized || !self.render_data.is_initialized {
            return;
        }

        self.shader.use_program();

        // Set view and projection
        self.shader.set_uniform_mat4(self.shader.view_loc, view_matrix);
        self.shader.set_uniform_mat4(self.shader.projection_loc, projection_matrix);

        // Build model matrix from position and orientation
        let model = build_model_matrix(self.render_data.position, self.render_data.orientation);
        self.shader.set_uniform_mat4(self.shader.model_loc, &model);

        // Lighting
        let light_dir = [0.5, 1.0, 0.3];
        self.shader.set_uniform_vec3(self.shader.light_dir_loc, &light_dir);

        let cam_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
        self.shader.set_uniform_vec3(self.shader.camera_pos_loc, &cam_pos);

        // Render
        self.render_data.render();
    }
}

// ========================
// Matrix Helpers
// ========================

/// Build a 4x4 model matrix from position and quaternion orientation
fn build_model_matrix(position: Vec3<f32>, orientation: Vec4<f32>) -> [f32; 16] {
    // Convert quaternion to rotation matrix
    let x = orientation.x;
    let y = orientation.y;
    let z = orientation.z;
    let w = orientation.w;

    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;

    [
        1.0 - 2.0 * (yy + zz),  2.0 * (xy + wz),        2.0 * (xz - wy),        0.0,
        2.0 * (xy - wz),        1.0 - 2.0 * (xx + zz),  2.0 * (yz + wx),        0.0,
        2.0 * (xz + wy),        2.0 * (yz - wx),        1.0 - 2.0 * (xx + yy),  0.0,
        position.x,              position.y,              position.z,              1.0,
    ]
}
