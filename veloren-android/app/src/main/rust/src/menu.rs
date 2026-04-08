//! Main Menu System for Android
//!
//! Handles main menu, settings, server selection, and character selection screens.

use gl;
use vek::Vec2;

// ========================
/// Menu Screens
// ========================

/// Current menu screen
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuScreen {
    MainMenu,
    Settings,
    ServerList,
    CharacterSelect,
    Loading,
    InGameMenu,
}

// ========================
/// Menu Button
// ========================

/// A clickable button
pub struct MenuButton {
    pub text: String,
    pub position: Vec2<f32>,
    pub size: Vec2<f32>,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub action: ButtonAction,
}

impl MenuButton {
    pub fn new(text: &str, x: f32, y: f32, width: f32, height: f32, action: ButtonAction) -> Self {
        Self {
            text: text.to_string(),
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
            is_hovered: false,
            is_pressed: false,
            action,
        }
    }

    /// Check if point is inside button
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.x && 
        x <= self.position.x + self.size.x &&
        y >= self.position.y && 
        y <= self.position.y + self.size.y
    }

    /// Handle touch down
    pub fn on_touch_down(&mut self, x: f32, y: f32) -> bool {
        if self.contains_point(x, y) {
            self.is_pressed = true;
            true
        } else {
            false
        }
    }

    /// Handle touch up
    pub fn on_touch_up(&mut self, x: f32, y: f32) -> Option<ButtonAction> {
        if self.is_pressed && self.contains_point(x, y) {
            self.is_pressed = false;
            Some(self.action)
        } else {
            self.is_pressed = false;
            None
        }
    }
}

/// Button action
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAction {
    Play,
    Settings,
    Servers,
    Characters,
    Quit,
    Back,
    Connect,
    CreateCharacter,
    DeleteCharacter,
    StartGame,
    Resume,
    Disconnect,
}

// ========================
/// Menu State
// ========================

/// Menu manager state
pub struct MenuManager {
    pub current_screen: MenuScreen,
    pub buttons: Vec<MenuButton>,
    pub is_visible: bool,
    pub title_text: String,
    
    // Settings
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub render_distance: i32,
    pub graphics_quality: GraphicsQuality,
    
    // Server list
    pub selected_server: Option<usize>,
    pub server_count: usize,
    
    // Character select
    pub selected_character: Option<usize>,
    pub character_count: usize,
}

impl MenuManager {
    pub fn new() -> Self {
        Self {
            current_screen: MenuScreen::MainMenu,
            buttons: Vec::new(),
            is_visible: true,
            title_text: String::new(),
            music_volume: 0.7,
            sfx_volume: 1.0,
            render_distance: 4,
            graphics_quality: GraphicsQuality::Medium,
            selected_server: None,
            server_count: 0,
            selected_character: None,
            character_count: 0,
        }
    }

    /// Setup main menu buttons
    pub fn setup_main_menu(&mut self, screen_width: f32, screen_height: f32) {
        self.current_screen = MenuScreen::MainMenu;
        self.title_text = "Veloren Android".to_string();
        self.buttons.clear();

        let button_width = 300.0;
        let button_height = 60.0;
        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height * 0.3;
        let spacing = 80.0;

        self.buttons.push(MenuButton::new(
            "Play",
            start_x, start_y,
            button_width, button_height,
            ButtonAction::Play,
        ));

        self.buttons.push(MenuButton::new(
            "Servers",
            start_x, start_y + spacing,
            button_width, button_height,
            ButtonAction::Servers,
        ));

        self.buttons.push(MenuButton::new(
            "Settings",
            start_x, start_y + spacing * 2,
            button_width, button_height,
            ButtonAction::Settings,
        ));

        self.buttons.push(MenuButton::new(
            "Quit",
            start_x, start_y + spacing * 3,
            button_width, button_height,
            ButtonAction::Quit,
        ));
    }

    /// Setup settings screen
    pub fn setup_settings(&mut self, screen_width: f32, screen_height: f32) {
        self.current_screen = MenuScreen::Settings;
        self.title_text = "Settings".to_string();
        self.buttons.clear();

        let button_width = 300.0;
        let button_height = 60.0;
        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height * 0.7;

        self.buttons.push(MenuButton::new(
            "Back",
            start_x, start_y,
            button_width, button_height,
            ButtonAction::Back,
        ));
    }

    /// Setup server list
    pub fn setup_server_list(&mut self, screen_width: f32, screen_height: f32) {
        self.current_screen = MenuScreen::ServerList;
        self.title_text = "Select Server".to_string();
        self.buttons.clear();

        let button_width = 300.0;
        let button_height = 60.0;
        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height * 0.7;

        self.buttons.push(MenuButton::new(
            "Connect",
            start_x, start_y,
            button_width, button_height,
            ButtonAction::Connect,
        ));

        self.buttons.push(MenuButton::new(
            "Back",
            start_x, start_y + 80.0,
            button_width, button_height,
            ButtonAction::Back,
        ));
    }

    /// Setup character select
    pub fn setup_character_select(&mut self, screen_width: f32, screen_height: f32) {
        self.current_screen = MenuScreen::CharacterSelect;
        self.title_text = "Select Character".to_string();
        self.buttons.clear();

        let button_width = 300.0;
        let button_height = 60.0;
        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height * 0.7;

        self.buttons.push(MenuButton::new(
            "Start Game",
            start_x, start_y,
            button_width, button_height,
            ButtonAction::StartGame,
        ));

        self.buttons.push(MenuButton::new(
            "Create Character",
            start_x, start_y + 80.0,
            button_width, button_height,
            ButtonAction::CreateCharacter,
        ));

        self.buttons.push(MenuButton::new(
            "Back",
            start_x, start_y + 160.0,
            button_width, button_height,
            ButtonAction::Back,
        ));
    }

    /// Setup in-game menu
    pub fn setup_ingame_menu(&mut self, screen_width: f32, screen_height: f32) {
        self.current_screen = MenuScreen::InGameMenu;
        self.title_text = "Paused".to_string();
        self.buttons.clear();

        let button_width = 300.0;
        let button_height = 60.0;
        let start_x = (screen_width - button_width) / 2.0;
        let start_y = screen_height * 0.3;

        self.buttons.push(MenuButton::new(
            "Resume",
            start_x, start_y,
            button_width, button_height,
            ButtonAction::Resume,
        ));

        self.buttons.push(MenuButton::new(
            "Settings",
            start_x, start_y + 80.0,
            button_width, button_height,
            ButtonAction::Settings,
        ));

        self.buttons.push(MenuButton::new(
            "Disconnect",
            start_x, start_y + 160.0,
            button_width, button_height,
            ButtonAction::Disconnect,
        ));
    }

    /// Handle touch down
    pub fn on_touch_down(&mut self, x: f32, y: f32) {
        for button in &mut self.buttons {
            button.on_touch_down(x, y);
        }
    }

    /// Handle touch up
    pub fn on_touch_up(&mut self, x: f32, y: f32) -> Option<ButtonAction> {
        for button in &mut self.buttons {
            if let Some(action) = button.on_touch_up(x, y) {
                return Some(action);
            }
        }
        None
    }

    /// Hide menu
    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    /// Show menu
    pub fn show(&mut self) {
        self.is_visible = true;
    }
}

// ========================
/// Graphics Quality
// ========================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

impl GraphicsQuality {
    pub fn name(&self) -> &'static str {
        match self {
            GraphicsQuality::Low => "Low",
            GraphicsQuality::Medium => "Medium",
            GraphicsQuality::High => "High",
            GraphicsQuality::Ultra => "Ultra",
        }
    }
}

// ========================
/// Menu Renderer
// ========================

/// Renders menu screens using OpenGL
pub struct MenuRenderer {
    pub vao: u32,
    pub vbo: u32,
    pub shader_program: u32,
    pub is_initialized: bool,
}

impl MenuRenderer {
    pub fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            shader_program: 0,
            is_initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        // Simple 2D shader for menus
        let vertex_shader = r#"
            #version 300 es
            layout(location = 0) in vec2 a_position;
            layout(location = 1) in vec3 a_color;

            uniform vec2 u_resolution;

            out vec3 v_color;

            void main() {
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
                tracing::error!("Menu vertex shader failed");
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
                tracing::error!("Menu fragment shader failed");
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
                tracing::error!("Menu shader link failed");
                return;
            }
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
            program
        };

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 5 * 4, std::ptr::null());

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * 4, (2 * 4) as *const _);

            gl::BindVertexArray(0);
        }

        self.shader_program = program;
        self.is_initialized = true;

        tracing::info!("Menu renderer initialized");
    }

    /// Draw a filled rectangle
    pub fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32, color: [f32; 3], resolution: Vec2<f32>) {
        if !self.is_initialized {
            return;
        }

        let vertices: [f32; 15] = [
            x, y,               color[0], color[1], color[2],
            x + width, y,       color[0], color[1], color[2],
            x + width, y + height, color[0], color[1], color[2],
            x, y,               color[0], color[1], color[2],
            x + width, y + height, color[0], color[1], color[2],
            x, y + height,      color[0], color[1], color[2],
        ];

        unsafe {
            gl::UseProgram(self.shader_program);
            
            let res_loc = gl::GetUniformLocation(self.shader_program, std::ffi::CString::new("u_resolution").unwrap().as_ptr());
            if res_loc >= 0 {
                gl::Uniform2f(res_loc, resolution.x, resolution.y);
            }

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

    /// Render full menu
    pub fn render_menu(&self, menu: &MenuManager, screen_width: f32, screen_height: f32) {
        if !menu.is_visible || !self.is_initialized {
            return;
        }

        // Background
        self.draw_rect(0.0, 0.0, screen_width, screen_height, [0.1, 0.1, 0.15], Vec2::new(screen_width, screen_height));

        // Title background
        let title_width = 600.0;
        let title_height = 80.0;
        let title_x = (screen_width - title_width) / 2.0;
        let title_y = 50.0;
        self.draw_rect(title_x, title_y, title_width, title_height, [0.2, 0.2, 0.3], Vec2::new(screen_width, screen_height));

        // Buttons
        for button in &menu.buttons {
            let color = if button.is_pressed {
                [0.4, 0.4, 0.5]  // Pressed
            } else if button.is_hovered {
                [0.35, 0.35, 0.45]  // Hovered
            } else {
                [0.25, 0.25, 0.35]  // Normal
            };

            self.draw_rect(
                button.position.x,
                button.position.y,
                button.size.x,
                button.size.y,
                color,
                Vec2::new(screen_width, screen_height),
            );

            // Button border
            self.draw_rect(
                button.position.x,
                button.position.y,
                button.size.x,
                2.0,
                [0.5, 0.5, 0.6],
                Vec2::new(screen_width, screen_height),
            );
        }
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
