//! Simple FOV calculation for FOV Visualization - Rust (2D).
//!
//! _Simple_ FOV determines visiblity for the tile `body` subpart only.

use crate::{fov::VisibleTile, FovRadius, Octant, QFactor};
use super::FovMap16;

/// Returns visible tile IDs (and their constitutent subnodes) for all FOV octants.
pub fn get_visible_tiles(fovmap: &FovMap16, r: usize) -> Vec<VisibleTile> {
    // Set capacity to max number of visible tiles. 
    let mut tiles = Vec::with_capacity(fovmap.capacity());
    // TODO: octant 1
    // TODO: octant 2
    // TODO: octant 3
    // TODO: octant 4
    // TODO: octant 5
    // TODO: octant 6
    // TODO: octant 7
    // TODO: octant 8
    tiles
}

/// Returns visible tile IDs (and their constitutent subnodes) in a given FOV octant.
pub fn fov_calc(octant: Octant) -> Vec<VisibleTile> {
    todo!();
}
