//! Network/Multiplayer System for Android
//!
//! Handles connection to Veloren servers, player synchronization,
//! and world updates.

use std::collections::HashMap;
use vek::{Vec2, Vec3, Vec4};

// ========================
// Network State
// ========================

/// Connection state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticating,
    Playing,
    Disconnecting,
    Error(String),
}

/// Server information
#[derive(Clone, Debug)]
pub struct ServerInfo {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub player_count: u32,
    pub max_players: u32,
    pub description: String,
    pub ping: Option<u32>,
}

impl ServerInfo {
    pub fn new(name: &str, address: &str, port: u16) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
            port,
            player_count: 0,
            max_players: 100,
            description: String::new(),
            ping: None,
        }
    }
}

// ========================
/// Remote Player
// ========================

/// Remote player data from server
#[derive(Clone, Debug)]
pub struct RemotePlayer {
    pub entity_id: u64,
    pub alias: String,
    pub position: Vec3<f32>,
    pub velocity: Vec3<f32>,
    pub orientation: Vec4<f32>,
    pub health: f32,
    pub max_health: f32,
    pub is_alive: bool,
    pub last_update: u64, // Timestamp
}

impl RemotePlayer {
    pub fn new(entity_id: u64, alias: &str) -> Self {
        Self {
            entity_id,
            alias: alias.to_string(),
            position: Vec3::zero(),
            velocity: Vec3::zero(),
            orientation: Vec4::unit_w(),
            health: 100.0,
            max_health: 100.0,
            is_alive: true,
            last_update: 0,
        }
    }

    /// Interpolate position for smooth rendering
    pub fn interpolated_position(&self, target_time: u64) -> Vec3<f32> {
        // Simple linear interpolation
        let time_diff = target_time.saturating_sub(self.last_update) as f32;
        self.position + self.velocity * time_diff * 0.001
    }
}

// ========================
/// Network Manager
// ========================

/// Network manager handles all multiplayer functionality
pub struct NetworkManager {
    pub state: ConnectionState,
    pub server: Option<ServerInfo>,
    pub remote_players: HashMap<u64, RemotePlayer>,
    pub local_player_id: Option<u64>,
    pub chat_messages: Vec<ChatMessage>,
    pub tick_count: u64,
    pub last_ping: u64,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            server: None,
            remote_players: HashMap::new(),
            local_player_id: None,
            chat_messages: Vec::new(),
            tick_count: 0,
            last_ping: 0,
        }
    }

    /// Connect to a server
    pub fn connect(&mut self, server: ServerInfo) {
        self.state = ConnectionState::Connecting;
        self.server = Some(server);
        tracing::info!("Connecting to server: {:?}", self.server.as_ref().map(|s| &s.name));
    }

    /// Disconnect from server
    pub fn disconnect(&mut self) {
        self.state = ConnectionState::Disconnecting;
        self.remote_players.clear();
        self.local_player_id = None;
        self.chat_messages.clear();
        tracing::info!("Disconnected from server");
    }

    /// Handle connection success
    pub fn on_connected(&mut self, player_id: u64) {
        self.state = ConnectionState::Connected;
        self.local_player_id = Some(player_id);
        tracing::info!("Connected to server, player ID: {}", player_id);
    }

    /// Handle authentication success
    pub fn on_authenticated(&mut self) {
        self.state = ConnectionState::Playing;
        tracing::info!("Authentication successful");
    }

    /// Update remote player positions
    pub fn update_remote_player(&mut self, entity_id: u64, position: Vec3<f32>, velocity: Vec3<f32>) {
        if let Some(player) = self.remote_players.get_mut(&entity_id) {
            player.position = position;
            player.velocity = velocity;
            player.last_update = self.tick_count;
        }
    }

    /// Add a new remote player
    pub fn add_remote_player(&mut self, entity_id: u64, alias: &str) {
        self.remote_players.insert(entity_id, RemotePlayer::new(entity_id, alias));
        tracing::info!("New player joined: {}", alias);
    }

    /// Remove a remote player
    pub fn remove_remote_player(&mut self, entity_id: u64) {
        if let Some(player) = self.remote_players.remove(&entity_id) {
            tracing::info!("Player left: {}", player.alias);
        }
    }

    /// Add chat message
    pub fn add_chat_message(&mut self, sender: &str, message: &str) {
        self.chat_messages.push(ChatMessage {
            sender: sender.to_string(),
            message: message.to_string(),
            timestamp: self.tick_count,
        });

        // Keep only last 100 messages
        if self.chat_messages.len() > 100 {
            self.chat_messages.remove(0);
        }
    }

    /// Get remote players as a list
    pub fn get_remote_players(&self) -> Vec<&RemotePlayer> {
        self.remote_players.values().collect()
    }

    /// Get player count
    pub fn player_count(&self) -> u32 {
        self.remote_players.len() as u32 + 1 // +1 for local player
    }

    /// Update tick
    pub fn tick(&mut self) {
        self.tick_count += 1;
    }

    /// Check if connected and playing
    pub fn is_playing(&self) -> bool {
        self.state == ConnectionState::Playing
    }
}

// ========================
/// Chat Message
// ========================

/// Chat message from server
#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub sender: String,
    pub message: String,
    pub timestamp: u64,
}

impl ChatMessage {
    /// Check if this is a system message
    pub fn is_system(&self) -> bool {
        self.sender.is_empty() || self.sender == "System"
    }
}

// ========================
/// Server List
// ========================

/// Default server list
pub fn default_servers() -> Vec<ServerInfo> {
    vec![
        ServerInfo::new("Veloren Official", "play.veloren.net", 14004),
        ServerInfo::new("EU Server", "eu.veloren.net", 14004),
        ServerInfo::new("US Server", "us.veloren.net", 14004),
        ServerInfo::new("Test Server", "test.veloren.net", 14004),
    ]
}

// ========================
/// Network JNI Bridge
// ========================

/// Global network manager
static mut NETWORK_MANAGER: Option<NetworkManager> = None;

/// Get mutable reference to network manager
pub fn get_network_manager_mut() -> Option<&'static mut NetworkManager> {
    unsafe { NETWORK_MANAGER.as_mut() }
}

/// Initialize network manager
pub fn init_network_manager() {
    unsafe {
        if NETWORK_MANAGER.is_none() {
            NETWORK_MANAGER = Some(NetworkManager::new());
        }
    }
}
