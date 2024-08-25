//! Tilemaps for FOV Visualization - Rust (2D)

/// 2D map coordinates.
#[derive(Debug, Clone, Copy)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// 2D map of tiles with FOV obstructions.
/// 
/// Obstructions include:
/// - Body: entirety of the tile body
/// - Wall (N): north-facing wall
/// - Wall (W): west-facing wall
/// 
/// Obstructions are only set if the given part is _present_ and _opaque_. 
/// Some FOV calculations, such as `simple`, may not use all obstructions.
pub struct TileMap {

}

