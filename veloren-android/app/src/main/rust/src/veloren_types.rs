//! Local stubs for veloren-common types
//!
//! These types mirror the veloren-common types but are defined locally
//! to avoid the complex veloren-common dependency chain that doesn't
//! compile for Android.

// ========================
// Body Types
// ========================

/// Character body type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Body {
    Humanoid(Humanoid),
    Dwarf(Dwarf),
    Orc(Orc),
}

impl Default for Body {
    fn default() -> Self {
        Body::Humanoid(Humanoid::default())
    }
}

/// Humanoid body type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Humanoid {
    pub gender: Gender,
}

/// Dwarf body type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dwarf {
    pub gender: Gender,
}

/// Orc body type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Orc {
    pub gender: Gender,
}

/// Gender
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

// ========================
// Block Types
// ========================

/// Block kind for terrain
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    Air,
    Water,
    Grass,
    Dirt,
    Rock,
    Sand,
    Snow,
    Wood,
    Leaves,
    Ice,
    Clay,
    Gravel,
}

impl Default for BlockKind {
    fn default() -> Self {
        BlockKind::Air
    }
}

/// Sprite kind for blocks
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpriteKind {
    #[default]
    Empty,
}

/// A terrain block
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub kind: BlockKind,
    pub sprite: SpriteKind,
}

impl Block {
    /// Create a new block
    pub fn new(kind: BlockKind, sprite: SpriteKind) -> Self {
        Self { kind, sprite }
    }

    /// Create an air block
    pub fn air(sprite: SpriteKind) -> Self {
        Self {
            kind: BlockKind::Air,
            sprite,
        }
    }

    /// Check if block is solid
    pub fn is_solid(&self) -> bool {
        self.kind != BlockKind::Air && self.kind != BlockKind::Water
    }

    /// Check if block is transparent
    pub fn is_transparent(&self) -> bool {
        matches!(self.kind, BlockKind::Air | BlockKind::Water | BlockKind::Leaves)
    }

    /// Get solid height
    pub fn solid_height(&self) -> f32 {
        1.0
    }

    /// Convert to vacant block
    pub fn into_vacant(self) -> Self {
        Self::air(SpriteKind::Empty)
    }

    /// Get sprite
    pub fn get_sprite(&self) -> SpriteKind {
        self.sprite
    }

    /// Get block kind
    pub fn get_kind(&self) -> BlockKind {
        self.kind
    }
}
