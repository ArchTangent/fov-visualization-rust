//! Simple FOV data builder for FOV Visualization - Rust (2D).
//! 
//! Simple FOV uses one tile part as an obstruction: the tile `body`.

use crate::{math::Line, FovRadius, Octant, QFactor};


// TODO: Fov8 for rFOV up to 8, with Q8 and Q16
// TODO: Fov16 for rFOV up to 16, with Q16 and Q32
// TODO: Fov32 for rFOV up to 32, with Q32 and Q64
// TODO: Fov64 for rFOV up to 64, with Q64 and Q128
// TODO: Fov128 for rFOV up to 128, with Q128 and Q256

// TODO: circular FOV culling 

/// Returns a list of FOV lines for a given radius, octant, and Q-value.
pub fn get_fov_lines(radius: FovRadius, qfactor: QFactor, octant: Octant) -> Vec<Line> {
    todo!()
}