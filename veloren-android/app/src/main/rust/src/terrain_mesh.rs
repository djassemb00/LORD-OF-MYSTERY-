//! Terrain Mesh Generation for Android
//!
//! Converts block data into OpenGL meshes with greedy meshing optimization.
//! Uses veloren-common Block types for consistency.

use vek::{Vec2, Vec3, Vec4};
use crate::terrain::{AndroidBlock, TerrainChunk, CHUNK_SIZE, CHUNK_HEIGHT};

// ========================
// Vertex Format
// ========================

/// Vertex for terrain rendering
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TerrainVertex {
    pub position: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub color: Vec3<f32>,
    pub tex_coords: Vec2<f32>,
}

impl TerrainVertex {
    pub fn new(position: Vec3<f32>, normal: Vec3<f32>, color: Vec3<f32>) -> Self {
        Self {
            position,
            normal,
            color,
            tex_coords: Vec2::zero(),
        }
    }
}

// ========================
// Face Definitions
// ========================

/// Face directions for block faces
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceDirection {
    Top,
    Bottom,
    North,
    South,
    East,
    West,
}

impl FaceDirection {
    /// Get the normal vector for this face
    pub fn normal(&self) -> Vec3<f32> {
        match self {
            FaceDirection::Top => Vec3::unit_y(),
            FaceDirection::Bottom => -Vec3::unit_y(),
            FaceDirection::North => -Vec3::unit_z(),
            FaceDirection::South => Vec3::unit_z(),
            FaceDirection::East => Vec3::unit_x(),
            FaceDirection::West => -Vec3::unit_x(),
        }
    }

    /// Get the offset to check for neighbor in this direction
    pub fn neighbor_offset(&self) -> Vec3<i32> {
        match self {
            FaceDirection::Top => Vec3::new(0, 1, 0),
            FaceDirection::Bottom => Vec3::new(0, -1, 0),
            FaceDirection::North => Vec3::new(0, 0, -1),
            FaceDirection::South => Vec3::new(0, 0, 1),
            FaceDirection::East => Vec3::new(1, 0, 0),
            FaceDirection::West => Vec3::new(-1, 0, 0),
        }
    }

    /// Get vertices for a quad at the given position (counter-clockwise)
    pub fn quad_vertices(&self, x: f32, y: f32, z: f32) -> [Vec3<f32>; 4] {
        match self {
            FaceDirection::Top => [
                Vec3::new(x, y + 1.0, z),
                Vec3::new(x + 1.0, y + 1.0, z),
                Vec3::new(x + 1.0, y + 1.0, z + 1.0),
                Vec3::new(x, y + 1.0, z + 1.0),
            ],
            FaceDirection::Bottom => [
                Vec3::new(x, y, z + 1.0),
                Vec3::new(x + 1.0, y, z + 1.0),
                Vec3::new(x + 1.0, y, z),
                Vec3::new(x, y, z),
            ],
            FaceDirection::North => [
                Vec3::new(x, y, z),
                Vec3::new(x, y + 1.0, z),
                Vec3::new(x + 1.0, y + 1.0, z),
                Vec3::new(x + 1.0, y, z),
            ],
            FaceDirection::South => [
                Vec3::new(x + 1.0, y, z + 1.0),
                Vec3::new(x + 1.0, y + 1.0, z + 1.0),
                Vec3::new(x, y + 1.0, z + 1.0),
                Vec3::new(x, y, z + 1.0),
            ],
            FaceDirection::East => [
                Vec3::new(x + 1.0, y, z),
                Vec3::new(x + 1.0, y + 1.0, z),
                Vec3::new(x + 1.0, y + 1.0, z + 1.0),
                Vec3::new(x + 1.0, y, z + 1.0),
            ],
            FaceDirection::West => [
                Vec3::new(x, y, z + 1.0),
                Vec3::new(x, y + 1.0, z + 1.0),
                Vec3::new(x, y + 1.0, z),
                Vec3::new(x, y, z),
            ],
        }
    }
}

// ========================
// Block Colors
// ========================

/// Get the color for a block type
pub fn get_block_color(block: AndroidBlock, face: FaceDirection) -> Vec3<f32> {
    // Base colors for each block type
    let base_color = match block {
        AndroidBlock::Air => Vec3::new(0.0, 0.0, 0.0),
        AndroidBlock::Water => Vec3::new(0.1, 0.3, 0.8),
        AndroidBlock::Grass => match face {
            FaceDirection::Top => Vec3::new(0.25, 0.7, 0.15),
            FaceDirection::Bottom => Vec3::new(0.4, 0.25, 0.1),
            _ => Vec3::new(0.3, 0.5, 0.15),
        },
        AndroidBlock::Dirt => Vec3::new(0.4, 0.25, 0.1),
        AndroidBlock::Stone => {
            let variation = 0.45;
            Vec3::new(variation, variation, variation)
        },
        AndroidBlock::Sand => Vec3::new(0.85, 0.75, 0.5),
        AndroidBlock::Snow => Vec3::new(0.95, 0.95, 0.98),
        AndroidBlock::Wood => match face {
            FaceDirection::Top | FaceDirection::Bottom => Vec3::new(0.45, 0.3, 0.15),
            _ => Vec3::new(0.35, 0.2, 0.1),
        },
        AndroidBlock::Leaves => Vec3::new(0.15, 0.55, 0.1),
        AndroidBlock::Ice => Vec3::new(0.6, 0.8, 0.95),
        AndroidBlock::Clay => Vec3::new(0.5, 0.45, 0.4),
        AndroidBlock::Gravel => Vec3::new(0.4, 0.38, 0.35),
    };

    // Apply simple lighting based on face direction
    let light_factor = match face {
        FaceDirection::Top => 1.0,
        FaceDirection::Bottom => 0.5,
        FaceDirection::North | FaceDirection::South => 0.7,
        FaceDirection::East | FaceDirection::West => 0.8,
    };

    base_color * light_factor
}

// ========================
// Mesh Data
// ========================

/// Complete mesh data for a chunk
pub struct ChunkMesh {
    pub vertices: Vec<TerrainVertex>,
    pub indices: Vec<u32>,
    pub is_dirty: bool,
    pub vertex_count: usize,
    pub index_count: usize,
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            is_dirty: true,
            vertex_count: 0,
            index_count: 0,
        }
    }

    /// Clear mesh data for rebuild
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.is_dirty = true;
    }

    /// Add a quad to the mesh
    pub fn add_quad(&mut self, vertices: [Vec3<f32>; 4], normal: Vec3<f32>, color: Vec3<f32>) {
        let base_index = self.vertices.len() as u32;

        // Add vertices
        for pos in &vertices {
            self.vertices.push(TerrainVertex::new(*pos, normal, color));
        }

        // Add indices (two triangles)
        self.indices.push(base_index);
        self.indices.push(base_index + 1);
        self.indices.push(base_index + 2);
        self.indices.push(base_index);
        self.indices.push(base_index + 2);
        self.indices.push(base_index + 3);

        self.vertex_count = self.vertices.len();
        self.index_count = self.indices.len();
    }
}

// ========================
// Mesh Generation
// ========================

/// Generate mesh from chunk data (naive approach - no greedy meshing)
pub fn generate_chunk_mesh(chunk: &TerrainChunk) -> ChunkMesh {
    let mut mesh = ChunkMesh::new();

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let block = chunk.get_block(x, y, z);

                // Skip air blocks
                if block == AndroidBlock::Air {
                    continue;
                }

                // Skip transparent blocks for simplicity (can be added later)
                if block.is_transparent() && block != AndroidBlock::Water {
                    continue;
                }

                let world_x = chunk.chunk_pos.x * CHUNK_SIZE as i32 + x as i32;
                let world_z = chunk.chunk_pos.z * CHUNK_SIZE as i32 + z as i32;
                let fx = world_x as f32;
                let fy = y as f32;
                let fz = world_z as f32;

                // Check each face
                for face in &[
                    FaceDirection::Top,
                    FaceDirection::Bottom,
                    FaceDirection::North,
                    FaceDirection::South,
                    FaceDirection::East,
                    FaceDirection::West,
                ] {
                    // Check if neighbor is transparent
                    let offset = face.neighbor_offset();
                    let neighbor_x = x as i32 + offset.x;
                    let neighbor_y = y as i32 + offset.y;
                    let neighbor_z = z as i32 + offset.z;

                    let neighbor_transparent = if neighbor_x >= 0
                        && neighbor_x < CHUNK_SIZE as i32
                        && neighbor_y >= 0
                        && neighbor_y < CHUNK_HEIGHT as i32
                        && neighbor_z >= 0
                        && neighbor_z < CHUNK_SIZE as i32
                    {
                        let neighbor = chunk.get_block(neighbor_x as u32, neighbor_y as u32, neighbor_z as u32);
                        neighbor.is_transparent()
                    } else {
                        true // Outside chunk is transparent
                    };

                    // Only render face if neighbor is transparent
                    if neighbor_transparent {
                        let color = get_block_color(block, *face);
                        let vertices = face.quad_vertices(fx, fy, fz);
                        mesh.add_quad(vertices, face.normal(), color);
                    }
                }
            }
        }
    }

    mesh.is_dirty = false;
    mesh
}

// ========================
// Greedy Meshing
// ========================

/// Generate mesh with greedy meshing optimization
/// This merges adjacent coplanar faces into larger quads
pub fn generate_chunk_mesh_greedy(chunk: &TerrainChunk) -> ChunkMesh {
    let mut mesh = ChunkMesh::new();

    // Track which faces have been merged
    let mut visited = vec![false; (CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE) as usize];

    for face_dir in &[
        FaceDirection::Top,
        FaceDirection::Bottom,
        FaceDirection::North,
        FaceDirection::South,
        FaceDirection::East,
        FaceDirection::West,
    ] {
        greedy_mesh_face(chunk, &mut mesh, &mut visited, *face_dir);
    }

    mesh.is_dirty = false;
    mesh
}

/// Greedy mesh a single face direction
fn greedy_mesh_face(
    chunk: &TerrainChunk,
    mesh: &mut ChunkMesh,
    visited: &mut Vec<bool>,
    face_dir: FaceDirection,
) {
    // Determine the iteration order based on face direction
    let (axis1, axis2, axis_fixed) = match face_dir {
        FaceDirection::Top | FaceDirection::Bottom => (0, 2, 1), // X, Z fixed Y
        FaceDirection::North | FaceDirection::South => (0, 1, 2), // X, Y fixed Z
        FaceDirection::East | FaceDirection::West => (2, 1, 0),  // Z, Y fixed X
    };

    let size1 = if axis1 == 0 { CHUNK_SIZE } else if axis1 == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };
    let size2 = if axis2 == 0 { CHUNK_SIZE } else if axis2 == 1 { CHUNK_HEIGHT } else { CHUNK_SIZE };
    let fixed_size = CHUNK_HEIGHT;

    // For each position on the fixed axis
    for fixed in 0..fixed_size {
        // Reset visited for this face
        for i in visited.iter_mut() {
            *i = false;
        }

        // Iterate through all positions on this face
        for pos2 in 0..size2 {
            for pos1 in 0..size1 {
                // Get block coordinates
                let (x, y, z) = match face_dir {
                    FaceDirection::Top | FaceDirection::Bottom => (pos1, fixed, pos2),
                    FaceDirection::North | FaceDirection::South => (pos1, pos2, fixed),
                    FaceDirection::East | FaceDirection::West => (fixed, pos2, pos1),
                };

                // Skip if already visited
                let idx = (y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x) as usize;
                if visited[idx] {
                    continue;
                }

                let block = chunk.get_block(x, y, z);

                // Skip air and transparent blocks
                if block == AndroidBlock::Air || block.is_transparent() {
                    continue;
                }

                // Check if this face should be rendered
                let offset = face_dir.neighbor_offset();
                let nx = x as i32 + offset.x;
                let ny = y as i32 + offset.y;
                let nz = z as i32 + offset.z;

                let neighbor_transparent = if nx >= 0 && nx < CHUNK_SIZE as i32
                    && ny >= 0 && ny < CHUNK_HEIGHT as i32
                    && nz >= 0 && nz < CHUNK_SIZE as i32
                {
                    let neighbor = chunk.get_block(nx as u32, ny as u32, nz as u32);
                    neighbor.is_transparent()
                } else {
                    true
                };

                if !neighbor_transparent {
                    continue;
                }

                // Find the largest quad we can make
                let mut width = 1;
                let mut height = 1;

                // Expand in axis1 direction
                while pos1 + width < size1 {
                    let (tx, ty, tz) = match face_dir {
                        FaceDirection::Top | FaceDirection::Bottom => (pos1 + width, fixed, pos2),
                        FaceDirection::North | FaceDirection::South => (pos1 + width, pos2, fixed),
                        FaceDirection::East | FaceDirection::West => (fixed, pos2, pos1 + width),
                    };

                    let tidx = (ty * CHUNK_SIZE * CHUNK_SIZE + tz * CHUNK_SIZE + tx) as usize;
                    if visited[tidx] {
                        break;
                    }

                    let test_block = chunk.get_block(tx, ty, tz);
                    if test_block != block {
                        break;
                    }

                    // Check neighbor
                    let tnx = tx as i32 + offset.x;
                    let tny = ty as i32 + offset.y;
                    let tnz = tz as i32 + offset.z;

                    let t_neighbor_transparent = if tnx >= 0 && tnx < CHUNK_SIZE as i32
                        && tny >= 0 && tny < CHUNK_HEIGHT as i32
                        && tnz >= 0 && tnz < CHUNK_SIZE as i32
                    {
                        let neighbor = chunk.get_block(tnx as u32, tny as u32, tnz as u32);
                        neighbor.is_transparent()
                    } else {
                        true
                    };

                    if !t_neighbor_transparent {
                        break;
                    }

                    width += 1;
                }

                // Expand in axis2 direction
                'outer: while pos2 + height < size2 {
                    for w in 0..width {
                        let (tx, ty, tz) = match face_dir {
                            FaceDirection::Top | FaceDirection::Bottom => {
                                (pos1 + w, fixed, pos2 + height)
                            },
                            FaceDirection::North | FaceDirection::South => {
                                (pos1 + w, pos2 + height, fixed)
                            },
                            FaceDirection::East | FaceDirection::West => {
                                (fixed, pos2 + height, pos1 + w)
                            },
                        };

                        let tidx = (ty * CHUNK_SIZE * CHUNK_SIZE + tz * CHUNK_SIZE + tx) as usize;
                        if visited[tidx] {
                            break 'outer;
                        }

                        let test_block = chunk.get_block(tx, ty, tz);
                        if test_block != block {
                            break 'outer;
                        }

                        // Check neighbor
                        let tnx = tx as i32 + offset.x;
                        let tny = ty as i32 + offset.y;
                        let tnz = tz as i32 + offset.z;

                        let t_neighbor_transparent = if tnx >= 0 && tnx < CHUNK_SIZE as i32
                            && tny >= 0 && tny < CHUNK_HEIGHT as i32
                            && tnz >= 0 && tnz < CHUNK_SIZE as i32
                        {
                            let neighbor = chunk.get_block(tnx as u32, tny as u32, tnz as u32);
                            neighbor.is_transparent()
                        } else {
                            true
                        };

                        if !t_neighbor_transparent {
                            break 'outer;
                        }
                    }
                    height += 1;
                }

                // Mark blocks as visited
                for h in 0..height {
                    for w in 0..width {
                        let (tx, ty, tz) = match face_dir {
                            FaceDirection::Top | FaceDirection::Bottom => {
                                (pos1 + w, fixed, pos2 + h)
                            },
                            FaceDirection::North | FaceDirection::South => {
                                (pos1 + w, pos2 + h, fixed)
                            },
                            FaceDirection::East | FaceDirection::West => {
                                (fixed, pos2 + h, pos1 + w)
                            },
                        };

                        let tidx = (ty * CHUNK_SIZE * CHUNK_SIZE + tz * CHUNK_SIZE + tx) as usize;
                        visited[tidx] = true;
                    }
                }

                // Add the merged quad
                let world_x = chunk.chunk_pos.x * CHUNK_SIZE as i32 + pos1 as i32;
                let world_z = chunk.chunk_pos.z * CHUNK_SIZE as i32 + pos2 as i32;

                let (fx, fy, fz) = match face_dir {
                    FaceDirection::Top | FaceDirection::Bottom => {
                        (world_x as f32, fixed as f32, world_z as f32)
                    },
                    FaceDirection::North | FaceDirection::South => {
                        (world_x as f32, pos2 as f32, fixed as f32)
                    },
                    FaceDirection::East | FaceDirection::West => {
                        (fixed as f32, pos2 as f32, world_z as f32)
                    },
                };

                // Create quad vertices with the merged size
                let quad_verts = create_greedy_quad(face_dir, fx, fy, fz, width as f32, height as f32, axis1, axis2);
                let color = get_block_color(block, face_dir);
                mesh.add_quad(quad_verts, face_dir.normal(), color);
            }
        }
    }
}

/// Create a quad with custom width and height for greedy meshing
fn create_greedy_quad(
    face_dir: FaceDirection,
    x: f32,
    y: f32,
    z: f32,
    width: f32,
    height: f32,
    axis1: u8,
    axis2: u8,
) -> [Vec3<f32>; 4] {
    // Create quad vertices scaled by width and height
    match face_dir {
        FaceDirection::Top => [
            Vec3::new(x, y + 1.0, z),
            Vec3::new(x + width, y + 1.0, z),
            Vec3::new(x + width, y + 1.0, z + height),
            Vec3::new(x, y + 1.0, z + height),
        ],
        FaceDirection::Bottom => [
            Vec3::new(x, y, z + height),
            Vec3::new(x + width, y, z + height),
            Vec3::new(x + width, y, z),
            Vec3::new(x, y, z),
        ],
        FaceDirection::North => [
            Vec3::new(x, y, z),
            Vec3::new(x, y + height, z),
            Vec3::new(x + width, y + height, z),
            Vec3::new(x + width, y, z),
        ],
        FaceDirection::South => [
            Vec3::new(x + width, y, z + 1.0),
            Vec3::new(x + width, y + height, z + 1.0),
            Vec3::new(x, y + height, z + 1.0),
            Vec3::new(x, y, z + 1.0),
        ],
        FaceDirection::East => [
            Vec3::new(x + 1.0, y, z),
            Vec3::new(x + 1.0, y + height, z),
            Vec3::new(x + 1.0, y + height, z + width),
            Vec3::new(x + 1.0, y, z + width),
        ],
        FaceDirection::West => [
            Vec3::new(x, y, z + width),
            Vec3::new(x, y + height, z + width),
            Vec3::new(x, y + height, z),
            Vec3::new(x, y, z),
        ],
    }
}

// ========================
// Mesh Statistics
// ========================

/// Statistics about a mesh
pub struct MeshStats {
    pub vertex_count: usize,
    pub index_count: usize,
    pub quad_count: usize,
    pub reduction_ratio: f32, // Compared to naive meshing
}

impl MeshStats {
    pub fn from_mesh(mesh: &ChunkMesh, naive_vertex_count: usize) -> Self {
        let quad_count = mesh.indices.len() / 6;
        let reduction = if naive_vertex_count > 0 {
            1.0 - (mesh.vertex_count as f32 / naive_vertex_count as f32)
        } else {
            0.0
        };

        Self {
            vertex_count: mesh.vertex_count,
            index_count: mesh.index_count,
            quad_count,
            reduction_ratio: reduction,
        }
    }
}
