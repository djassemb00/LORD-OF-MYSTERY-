//! Asset loading utilities for Android

use std::collections::HashMap;
use std::sync::Mutex;

/// Asset manager for loading files from Android assets folder
pub struct AssetManager {
    loaded_textures: HashMap<String, TextureData>,
    loaded_models: HashMap<String, ModelData>,
    loaded_sounds: HashMap<String, SoundData>,
    assets_path: String,
}

/// Raw texture data (RGBA8)
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// Model data (vertices + indices)
pub struct ModelData {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

/// Sound data (PCM)
pub struct SoundData {
    pub sample_rate: u32,
    pub channels: u16,
    pub data: Vec<f32>,
}

impl AssetManager {
    pub fn new(assets_path: &str) -> Self {
        Self {
            loaded_textures: HashMap::new(),
            loaded_models: HashMap::new(),
            loaded_sounds: HashMap::new(),
            assets_path: assets_path.to_string(),
        }
    }

    /// Load a texture from assets
    pub fn load_texture(&mut self, path: &str) -> Result<&TextureData, String> {
        if self.loaded_textures.contains_key(path) {
            return Ok(self.loaded_textures.get(path).unwrap());
        }

        // TODO: Load from Android assets
        // For now, create a placeholder
        let texture = TextureData {
            width: 64,
            height: 64,
            data: vec![255; 64 * 64 * 4], // White texture
        };

        self.loaded_textures.insert(path.to_string(), texture);
        Ok(self.loaded_textures.get(path).unwrap())
    }

    /// Load a model from assets
    pub fn load_model(&mut self, path: &str) -> Result<&ModelData, String> {
        if self.loaded_models.contains_key(path) {
            return Ok(self.loaded_models.get(path).unwrap());
        }

        // TODO: Load from Android assets
        // For now, create a placeholder cube
        let model = ModelData {
            vertices: vec![
                // Position + Normal + TexCoords
                -0.5, -0.5,  0.5,  0.0, 0.0, 1.0,  0.0, 0.0,
                 0.5, -0.5,  0.5,  0.0, 0.0, 1.0,  1.0, 0.0,
                 0.5,  0.5,  0.5,  0.0, 0.0, 1.0,  1.0, 1.0,
                -0.5,  0.5,  0.5,  0.0, 0.0, 1.0,  0.0, 1.0,
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
        };

        self.loaded_models.insert(path.to_string(), model);
        Ok(self.loaded_models.get(path).unwrap())
    }

    /// Load a sound from assets
    pub fn load_sound(&mut self, path: &str) -> Result<&SoundData, String> {
        if self.loaded_sounds.contains_key(path) {
            return Ok(self.loaded_sounds.get(path).unwrap());
        }

        // TODO: Load from Android assets
        // For now, create a placeholder
        let sound = SoundData {
            sample_rate: 44100,
            channels: 2,
            data: vec![0.0; 44100], // 1 second of silence
        };

        self.loaded_sounds.insert(path.to_string(), sound);
        Ok(self.loaded_sounds.get(path).unwrap())
    }

    /// Clear all loaded assets
    pub fn clear_assets(&mut self) {
        self.loaded_textures.clear();
        self.loaded_models.clear();
        self.loaded_sounds.clear();
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> usize {
        let textures_size: usize = self.loaded_textures.values()
            .map(|t| t.data.len())
            .sum();
        
        let models_size: usize = self.loaded_models.values()
            .map(|m| m.vertices.len() * 4 + m.indices.len() * 4)
            .sum();
        
        let sounds_size: usize = self.loaded_sounds.values()
            .map(|s| s.data.len() * 4)
            .sum();

        textures_size + models_size + sounds_size
    }
}

/// Global asset manager instance
static ASSET_MANAGER: Mutex<Option<AssetManager>> = Mutex::new(None);

/// Initialize the asset manager
pub fn init_asset_manager(assets_path: &str) {
    let mut manager = ASSET_MANAGER.lock().unwrap();
    *manager = Some(AssetManager::new(assets_path));
    tracing::info!("Asset manager initialized: {}", assets_path);
}

/// Get reference to asset manager
pub fn get_asset_manager() -> Result<&'static mut AssetManager, String> {
    // Safe because ASSET_MANAGER is static and initialized once
    unsafe {
        ASSET_MANAGER
            .lock()
            .unwrap()
            .as_mut()
            .map(|m| &mut *(m as *mut _))
            .ok_or("Asset manager not initialized".to_string())
    }
}
