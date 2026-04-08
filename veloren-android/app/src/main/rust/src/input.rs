//! Input handling for Android touch events

use std::collections::HashMap;

/// Touch input state
#[derive(Debug, Clone)]
pub struct TouchInput {
    pub pointer_id: i32,
    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
}

/// Virtual joystick
#[derive(Debug, Clone)]
pub struct VirtualJoystick {
    pub base_x: f32,
    pub base_y: f32,
    pub current_x: f32,
    pub current_y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub active: bool,
    pub pointer_id: Option<i32>,
}

impl VirtualJoystick {
    pub fn new() -> Self {
        Self {
            base_x: 0.0,
            base_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            active: false,
            pointer_id: None,
        }
    }

    pub fn touch_down(&mut self, pointer_id: i32, x: f32, y: f32) {
        if !self.active {
            self.base_x = x;
            self.base_y = y;
            self.current_x = x;
            self.current_y = y;
            self.active = true;
            self.pointer_id = Some(pointer_id);
        }
    }

    pub fn touch_move(&mut self, pointer_id: i32, x: f32, y: f32) {
        if self.pointer_id == Some(pointer_id) {
            self.current_x = x;
            self.current_y = y;
            self.delta_x = (x - self.base_x) / 100.0;
            self.delta_y = (y - self.base_y) / 100.0;

            // Clamp to -1.0 to 1.0
            self.delta_x = self.delta_x.clamp(-1.0, 1.0);
            self.delta_y = self.delta_y.clamp(-1.0, 1.0);
        }
    }

    pub fn touch_up(&mut self, pointer_id: i32) {
        if self.pointer_id == Some(pointer_id) {
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.base_x = 0.0;
        self.base_y = 0.0;
        self.current_x = 0.0;
        self.current_y = 0.0;
        self.delta_x = 0.0;
        self.delta_y = 0.0;
        self.active = false;
        self.pointer_id = None;
    }
}

/// Input handler for the game
pub struct InputHandler {
    pub left_joystick: VirtualJoystick,
    pub right_joystick: VirtualJoystick,
    pub jump_pressed: bool,
    pub attack_pressed: bool,
    pub interact_pressed: bool,
    pub screen_width: f32,
    pub screen_height: f32,
    active_touches: HashMap<i32, (f32, f32)>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            left_joystick: VirtualJoystick::new(),
            right_joystick: VirtualJoystick::new(),
            jump_pressed: false,
            attack_pressed: false,
            interact_pressed: false,
            screen_width: 0.0,
            screen_height: 0.0,
            active_touches: HashMap::new(),
        }
    }

    pub fn set_screen_size(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    pub fn touch_down(&mut self, pointer_id: i32, x: f32, y: f32) {
        self.active_touches.insert(pointer_id, (x, y));

        // Left half = movement joystick
        if x < self.screen_width / 2.0 && !self.left_joystick.active {
            self.left_joystick.touch_down(pointer_id, x, y);
        }
        // Right half = camera joystick
        else if x >= self.screen_width / 2.0 && !self.right_joystick.active {
            self.right_joystick.touch_down(pointer_id, x, y);
        }
    }

    pub fn touch_move(&mut self, pointer_id: i32, x: f32, y: f32) {
        self.active_touches.insert(pointer_id, (x, y));
        self.left_joystick.touch_move(pointer_id, x, y);
        self.right_joystick.touch_move(pointer_id, x, y);
    }

    pub fn touch_up(&mut self, pointer_id: i32) {
        self.active_touches.remove(&pointer_id);
        self.left_joystick.touch_up(pointer_id);
        self.right_joystick.touch_up(pointer_id);
    }

    /// Get movement input (-1.0 to 1.0 for x and y)
    pub fn get_movement(&self) -> (f32, f32) {
        (self.left_joystick.delta_x, self.left_joystick.delta_y)
    }

    /// Get camera look input (-1.0 to 1.0 for x and y)
    pub fn get_camera_look(&self) -> (f32, f32) {
        (self.right_joystick.delta_x, self.right_joystick.delta_y)
    }
}
