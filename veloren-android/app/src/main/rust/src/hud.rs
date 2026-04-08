//! HUD (Heads-Up Display) for Android
//!
//! Renders health bar, stamina, inventory slots, and other UI elements.

use gl;
use vek::Vec2;

// ========================
// HUD Elements
// ========================

/// HUD state
pub struct HudState {
    pub health: f32,
    pub max_health: f32,
    pub energy: f32,
    pub max_energy: f32,
    pub position: Vec2<f32>,  // Screen position
    pub is_visible: bool,
}

impl HudState {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            energy: 100.0,
            max_energy: 100.0,
            position: Vec2::zero(),
            is_visible: true,
        }
    }

    /// Update health
    pub fn set_health(&mut self, current: f32, max: f32) {
        self.health = current;
        self.max_health = max;
    }

    /// Update energy
    pub fn set_energy(&mut self, current: f32, max: f32) {
        self.energy = current;
        self.max_energy = max;
    }

    /// Get health percentage
    pub fn health_percent(&self) -> f32 {
        if self.max_health > 0.0 {
            self.health / self.max_health
        } else {
            0.0
        }
    }

    /// Get energy percentage
    pub fn energy_percent(&self) -> f32 {
        if self.max_energy > 0.0 {
            self.energy / self.max_energy
        } else {
            0.0
        }
    }
}

// ========================
// HUD Renderer (2D Overlay)
// ========================

/// Simple 2D HUD renderer using OpenGL
pub struct HudRenderer {
    pub vao: u32,
    pub vbo: u32,
    pub shader_program: u32,
    pub is_initialized: bool,
}

impl HudRenderer {
    pub fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            shader_program: 0,
            is_initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        // Create simple 2D shader
        let vertex_shader = r#"
            #version 300 es
            layout(location = 0) in vec2 a_position;
            layout(location = 1) in vec3 a_color;

            uniform vec2 u_resolution;

            out vec3 v_color;

            void main() {
                // Convert pixel coordinates to NDC
                vec2 ndc = (a_position / u_resolution) * 2.0 - 1.0;
                gl_Position = vec4(ndc.x, -ndc.y, 0.0, 1.0);
                v_color = a_color;
            }
        "#;

        let fragment_shader = r#"
            #version 300 es
            precision mediump float;

            in vec3 v_color;
            out vec4 frag_color;

            void main() {
                frag_color = vec4(v_color, 1.0);
            }
        "#;

        let mut success: i32 = 0;

        let vs = unsafe {
            let shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str = std::ffi::CString::new(vertex_shader).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut _);
                tracing::error!("HUD vertex shader failed: {}", String::from_utf8_lossy(&buf));
                return;
            }
            shader
        };

        let fs = unsafe {
            let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str = std::ffi::CString::new(fragment_shader).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0u8; len as usize];
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut _);
                tracing::error!("HUD fragment shader failed: {}", String::from_utf8_lossy(&buf));
                return;
            }
            shader
        };

        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                tracing::error!("HUD shader link failed");
                return;
            }
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
            program
        };

        // Create VAO and VBO
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            // Position attribute
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 5 * 4, std::ptr::null());

            // Color attribute
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * 4, (2 * 4) as *const _);

            gl::BindVertexArray(0);
        }

        self.shader_program = program;
        self.is_initialized = true;

        tracing::info!("HUD renderer initialized");
    }

    /// Draw a filled rectangle
    pub fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32, color: [f32; 3]) {
        if !self.is_initialized {
            return;
        }

        let vertices: [f32; 15] = [
            // pos (x, y)       // color (r, g, b)
            x, y,               color[0], color[1], color[2],
            x + width, y,       color[0], color[1], color[2],
            x + width, y + height, color[0], color[1], color[2],
            x, y,               color[0], color[1], color[2],
            x + width, y + height, color[0], color[1], color[2],
            x, y + height,      color[0], color[1], color[2],
        ];

        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * 4) as isize,
                vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    /// Draw health bar
    pub fn draw_health_bar(&self, x: f32, y: f32, width: f32, height: f32, percent: f32) {
        // Background (dark red)
        self.draw_rect(x, y, width, height, [0.3, 0.0, 0.0]);

        // Foreground (bright red) based on percent
        let fill_width = width * percent.clamp(0.0, 1.0);
        let color = if percent > 0.5 {
            [0.9, 0.1, 0.1]  // Healthy - bright red
        } else if percent > 0.25 {
            [0.9, 0.5, 0.0]  // Warning - orange
        } else {
            [0.9, 0.0, 0.0]  // Critical - dark red
        };
        self.draw_rect(x, y, fill_width, height, color);

        // Border (white)
        self.draw_rect(x, y, width, 2.0, [1.0, 1.0, 1.0]);
        self.draw_rect(x, y, 2.0, height, [1.0, 1.0, 1.0]);
        self.draw_rect(x + width - 2.0, y, 2.0, height, [1.0, 1.0, 1.0]);
        self.draw_rect(x, y + height - 2.0, width, 2.0, [1.0, 1.0, 1.0]);
    }

    /// Draw energy/stamina bar
    pub fn draw_energy_bar(&self, x: f32, y: f32, width: f32, height: f32, percent: f32) {
        // Background (dark yellow)
        self.draw_rect(x, y, width, height, [0.3, 0.3, 0.0]);

        // Foreground (bright yellow)
        let fill_width = width * percent.clamp(0.0, 1.0);
        self.draw_rect(x, y, fill_width, height, [1.0, 0.9, 0.0]);

        // Border
        self.draw_rect(x, y, width, 2.0, [1.0, 1.0, 1.0]);
        self.draw_rect(x, y, 2.0, height, [1.0, 1.0, 1.0]);
        self.draw_rect(x + width - 2.0, y, 2.0, height, [1.0, 1.0, 1.0]);
        self.draw_rect(x, y + height - 2.0, width, 2.0, [1.0, 1.0, 1.0]);
    }

    /// Draw full HUD
    pub fn render_hud(&self, state: &HudState, screen_width: f32, screen_height: f32) {
        if !state.is_visible || !self.is_initialized {
            return;
        }

        let bar_width = 200.0;
        let bar_height = 20.0;
        let margin = 20.0;

        // Health bar (top-left)
        let health_x = margin;
        let health_y = margin;
        self.draw_health_bar(health_x, health_y, bar_width, bar_height, state.health_percent());

        // Energy bar (below health)
        let energy_y = health_y + bar_height + 10.0;
        self.draw_energy_bar(health_x, energy_y, bar_width, bar_height, state.energy_percent());
    }

    /// Cleanup
    pub fn cleanup(&mut self) {
        if self.is_initialized {
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao);
                gl::DeleteBuffers(1, &self.vbo);
                gl::DeleteProgram(self.shader_program);
            }
            self.is_initialized = false;
        }
    }
}
