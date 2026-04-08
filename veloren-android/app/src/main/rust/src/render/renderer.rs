//! Main OpenGL ES Renderer

use gl;
use std::ffi::CStr;

/// OpenGL ES Renderer
pub struct GlRenderer {
    width: u32,
    height: u32,
    is_initialized: bool,
    test_cube: Option<super::mesh::Mesh>,
    shader: Option<super::shader::ShaderProgram>,
}

impl GlRenderer {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            is_initialized: false,
            test_cube: None,
            shader: None,
        }
    }

    /// Initialize the renderer
    pub fn initialize(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.width = width;
        self.height = height;

        // Load OpenGL ES functions
        gl::load_with(|name| {
            std::ffi::CString::new(name)
                .unwrap()
                .as_ptr() as *const _
        });

        // Print GL info
        let vendor = unsafe { CStr::from_ptr(gl::GetString(gl::VENDOR) as *const _) };
        let renderer = unsafe { CStr::from_ptr(gl::GetString(gl::RENDERER) as *const _) };
        let version = unsafe { CStr::from_ptr(gl::GetString(gl::VERSION) as *const _) };

        tracing::info!(
            "GL Vendor: {}",
            vendor.to_str().unwrap_or("unknown")
        );
        tracing::info!(
            "GL Renderer: {}",
            renderer.to_str().unwrap_or("unknown")
        );
        tracing::info!(
            "GL Version: {}",
            version.to_str().unwrap_or("unknown")
        );

        // Set clear color (dark blue - Veloren theme)
        unsafe {
            gl::ClearColor(0.05, 0.05, 0.15, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }

        // Create test shader
        let vertex_shader = r#"
            #version 300 es
            layout(location = 0) in vec3 a_position;
            layout(location = 1) in vec3 a_normal;
            layout(location = 2) in vec2 a_tex_coords;
            
            uniform mat4 u_view;
            uniform mat4 u_projection;
            uniform mat4 u_model;
            
            out vec3 v_normal;
            out vec2 v_tex_coords;
            out vec3 v_position;
            
            void main() {
                vec4 world_pos = u_model * vec4(a_position, 1.0);
                v_position = world_pos.xyz;
                v_normal = mat3(u_model) * a_normal;
                v_tex_coords = a_tex_coords;
                gl_Position = u_projection * u_view * world_pos;
            }
        "#;
        
        let fragment_shader = r#"
            #version 300 es
            precision mediump float;
            
            in vec3 v_normal;
            in vec2 v_tex_coords;
            in vec3 v_position;
            
            uniform vec3 u_light_dir;
            uniform vec3 u_camera_pos;
            
            out vec4 frag_color;
            
            void main() {
                // Simple lighting
                vec3 normal = normalize(v_normal);
                vec3 light_dir = normalize(u_light_dir);
                
                float diff = max(dot(normal, light_dir), 0.0);
                float ambient = 0.3;
                float lighting = ambient + diff * 0.7;
                
                // Grass-like color
                vec3 color = vec3(0.2, 0.6, 0.2);
                
                // Add some variation based on position
                color += vec3(
                    sin(v_position.x * 0.5) * 0.1,
                    0.0,
                    sin(v_position.z * 0.5) * 0.1
                );
                
                frag_color = vec4(color * lighting, 1.0);
            }
        "#;
        
        match super::shader::ShaderProgram::new(vertex_shader, fragment_shader) {
            Ok(shader) => {
                self.shader = Some(shader);
                tracing::info!("Shader program created successfully");
            }
            Err(e) => {
                tracing::error!("Failed to create shader: {}", e);
            }
        }

        // Create test cube
        self.test_cube = Some(super::mesh::create_cube());

        self.is_initialized = true;
        tracing::info!("OpenGL ES renderer initialized");

        Ok(())
    }

    /// Handle resize
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        tracing::info!("Renderer resized: {}x{}", width, height);
    }

    /// Render a frame
    pub fn render(&mut self, delta_time: f32, view_matrix: &[f32; 16], projection_matrix: &[f32; 16]) {
        if !self.is_initialized {
            return;
        }

        // Clear screen
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Use shader
        if let Some(ref shader) = self.shader {
            shader.use_program();
            
            // Set uniforms
            if let Some(loc) = shader.get_uniform_location("u_view") {
                shader.set_uniform_mat4(loc, view_matrix);
            }
            if let Some(loc) = shader.get_uniform_location("u_projection") {
                shader.set_uniform_mat4(loc, projection_matrix);
            }
            
            // Light direction (from above and slightly to the side)
            let light_dir = [0.5, 1.0, 0.3];
            if let Some(loc) = shader.get_uniform_location("u_light_dir") {
                shader.set_uniform_vec3(loc, &light_dir);
            }
            
            // Camera position (placeholder)
            let camera_pos = [0.0, 10.0, 20.0];
            if let Some(loc) = shader.get_uniform_location("u_camera_pos") {
                shader.set_uniform_vec3(loc, &camera_pos);
            }
            
            // Render test cubes in a grid pattern
            if let Some(ref cube) = self.test_cube {
                for x in -3..=3 {
                    for z in -3..=3 {
                        // Simple model matrix (translation only)
                        let model = [
                            1.0, 0.0, 0.0, 0.0,
                            0.0, 1.0, 0.0, 0.0,
                            0.0, 0.0, 1.0, 0.0,
                            x as f32 * 3.0, 64.0, z as f32 * 3.0, 1.0,
                        ];
                        
                        if let Some(loc) = shader.get_uniform_location("u_model") {
                            shader.set_uniform_mat4(loc, &model);
                        }
                        
                        cube.render();
                    }
                }
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
