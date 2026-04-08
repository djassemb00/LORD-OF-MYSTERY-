//! Audio System for Android
//!
//! Simple audio engine using OpenSL ES for Android.
//! Supports background music, sound effects, and ambient sounds.

use std::collections::HashMap;

// ========================
// Sound Types
// ========================

/// Type of sound
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoundType {
    // Ambient sounds
    Wind,
    Water,
    Birds,
    Forest,
    
    // Player sounds
    Footstep,
    Jump,
    Attack,
    Hurt,
    Death,
    
    // UI sounds
    Click,
    MenuOpen,
    MenuClose,
    
    // Music
    MainMenu,
    Exploration,
    Combat,
    Boss,
}

impl SoundType {
    /// Get the asset path for this sound
    pub fn asset_path(&self) -> &'static str {
        match self {
            SoundType::Wind => "audio/ambient/wind.ogg",
            SoundType::Water => "audio/ambient/water.ogg",
            SoundType::Birds => "audio/ambient/birds.ogg",
            SoundType::Forest => "audio/ambient/forest.ogg",
            SoundType::Footstep => "audio/player/footstep.ogg",
            SoundType::Jump => "audio/player/jump.ogg",
            SoundType::Attack => "audio/player/attack.ogg",
            SoundType::Hurt => "audio/player/hurt.ogg",
            SoundType::Death => "audio/player/death.ogg",
            SoundType::Click => "audio/ui/click.ogg",
            SoundType::MenuOpen => "audio/ui/menu_open.ogg",
            SoundType::MenuClose => "audio/ui/menu_close.ogg",
            SoundType::MainMenu => "audio/music/main_menu.ogg",
            SoundType::Exploration => "audio/music/exploration.ogg",
            SoundType::Combat => "audio/music/combat.ogg",
            SoundType::Boss => "audio/music/boss.ogg",
        }
    }

    /// Check if this sound should loop
    pub fn should_loop(&self) -> bool {
        matches!(self, 
            SoundType::Wind | 
            SoundType::Water | 
            SoundType::Birds | 
            SoundType::Forest |
            SoundType::MainMenu |
            SoundType::Exploration |
            SoundType::Combat |
            SoundType::Boss
        )
    }

    /// Check if this is a music sound
    pub fn is_music(&self) -> bool {
        matches!(self,
            SoundType::MainMenu |
            SoundType::Exploration |
            SoundType::Combat |
            SoundType::Boss
        )
    }
}

// ========================
// Audio Engine
// ========================

/// Audio engine state
pub struct AudioEngine {
    pub is_initialized: bool,
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ambient_volume: f32,
    pub current_music: Option<SoundType>,
    pub loaded_sounds: HashMap<SoundType, bool>, // true if loaded
    pub playing_sounds: HashMap<SoundType, bool>, // true if playing
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 1.0,
            ambient_volume: 0.5,
            current_music: None,
            loaded_sounds: HashMap::new(),
            playing_sounds: HashMap::new(),
        }
    }

    /// Initialize audio engine
    pub fn initialize(&mut self) {
        // Note: Actual OpenSL ES initialization would happen here
        // For now, we simulate the audio system
        self.is_initialized = true;
        tracing::info!("Audio engine initialized (simulated)");
    }

    /// Set master volume (0.0 - 1.0)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    /// Set music volume (0.0 - 1.0)
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }

    /// Set SFX volume (0.0 - 1.0)
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    /// Set ambient volume (0.0 - 1.0)
    pub fn set_ambient_volume(&mut self, volume: f32) {
        self.ambient_volume = volume.clamp(0.0, 1.0);
    }

    /// Play a sound effect
    pub fn play_sound(&mut self, sound: SoundType) {
        if !self.is_initialized {
            return;
        }

        // Mark as playing
        self.playing_sounds.insert(sound, true);
        
        tracing::debug!("Playing sound: {:?}", sound);
    }

    /// Stop a sound
    pub fn stop_sound(&mut self, sound: SoundType) {
        self.playing_sounds.remove(&sound);
    }

    /// Play background music
    pub fn play_music(&mut self, music: SoundType) {
        if !self.is_initialized || !music.is_music() {
            return;
        }

        // Stop current music if playing
        if let Some(current) = self.current_music {
            if current != music {
                self.stop_music();
            }
        }

        self.current_music = Some(music);
        self.playing_sounds.insert(music, true);
        
        tracing::info!("Playing music: {:?}", music);
    }

    /// Stop background music
    pub fn stop_music(&mut self) {
        if let Some(music) = self.current_music {
            self.playing_sounds.remove(&music);
            self.current_music = None;
        }
    }

    /// Update ambient sounds based on environment
    pub fn update_ambient_sounds(&mut self, is_near_water: bool, is_in_forest: bool, is_windy: bool) {
        if !self.is_initialized {
            return;
        }

        // Play/stop ambient sounds based on environment
        if is_near_water {
            self.play_sound(SoundType::Water);
        } else {
            self.stop_sound(SoundType::Water);
        }

        if is_in_forest {
            self.play_sound(SoundType::Birds);
            self.play_sound(SoundType::Forest);
        } else {
            self.stop_sound(SoundType::Birds);
            self.stop_sound(SoundType::Forest);
        }

        if is_windy {
            self.play_sound(SoundType::Wind);
        } else {
            self.stop_sound(SoundType::Wind);
        }
    }

    /// Play player action sound
    pub fn play_player_sound(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Footstep => self.play_sound(SoundType::Footstep),
            PlayerAction::Jump => self.play_sound(SoundType::Jump),
            PlayerAction::Attack => self.play_sound(SoundType::Attack),
            PlayerAction::Hurt => self.play_sound(SoundType::Hurt),
            PlayerAction::Death => self.play_sound(SoundType::Death),
        }
    }

    /// Check if a sound is currently playing
    pub fn is_playing(&self, sound: SoundType) -> bool {
        self.playing_sounds.get(&sound).copied().unwrap_or(false)
    }

    /// Pause all sounds
    pub fn pause_all(&mut self) {
        self.playing_sounds.clear();
    }

    /// Resume audio (after pause)
    pub fn resume(&mut self) {
        // Audio will resume automatically on next play call
    }

    /// Cleanup audio engine
    pub fn cleanup(&mut self) {
        self.stop_music();
        self.playing_sounds.clear();
        self.loaded_sounds.clear();
        self.is_initialized = false;
        tracing::info!("Audio engine cleaned up");
    }
}

/// Player action that triggers sound
pub enum PlayerAction {
    Footstep,
    Jump,
    Attack,
    Hurt,
    Death,
}

// ========================
/// Audio Manager for JNI
// ========================

/// Global audio manager (accessible from JNI)
static mut AUDIO_ENGINE: Option<AudioEngine> = None;

/// Get mutable reference to audio engine
pub fn get_audio_engine_mut() -> Option<&'static mut AudioEngine> {
    unsafe { AUDIO_ENGINE.as_mut() }
}

/// Initialize global audio engine
pub fn init_audio_engine() {
    unsafe {
        if AUDIO_ENGINE.is_none() {
            AUDIO_ENGINE = Some(AudioEngine::new());
        }
        if let Some(engine) = AUDIO_ENGINE.as_mut() {
            engine.initialize();
        }
    }
}
