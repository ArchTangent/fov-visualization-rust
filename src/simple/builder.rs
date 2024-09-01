//! Simple FOV data builder for FOV Visualization - Rust (2D).
//!
//! Simple FOV uses one tile part as an obstruction: the tile `body`.
//!
//! Building an Octant:
//! - Create preparatory nodes (`FovPrepNode`) that are the same for all octants.
//! - Create 8 FOV octant (`FovOctant`) instances consisting of FOV nodes (`FovNode`)
//!   with `(dx, dy)` set according to `octant` and `(dpri, dsec)` of preparatory nodes.

use crate::{
    fov::{body_lines, FovLines},
    math::{dist_u16, dist_u8, Line},
    FovRadius, Octant, QFactor,
};

// TODO: Fov16 for rFOV up to 16, with Q16 and Q32
// TODO: Fov32 for rFOV up to 32, with Q32 and Q64
// TODO: Fov64 for rFOV up to 64, with Q64 and Q128
// TODO: Fov128 for rFOV up to 128, with Q128 and Q256

/// Node in an FOV map representing a single tile with 16 FOV bits (`Q=16`).
#[derive(Debug, Clone)]
pub struct FovNode16 {
    pub body: u16,
    pub dpri: u8,
    pub dsec: u8,
}

/// One of eight FOV octants, comprised of 16-bit FOV nodes.
///
/// Notes:
/// - for Simple FOV, octants differ only in dx/dy values. The content of each 
///   FOV node is the same.
/// - `node_indexes` holds the highest node index for a given radius (`r=0` to `r=16`).
pub struct FovOctant16 {
    rfov: FovRadius,
    octant: Octant,
    nodes: Vec<FovNode16>,
    node_indexes: Vec<usize>,
}

impl FovOctant16 {
    /// Builds a new `FovOctant`.
    pub fn new(nodes: &Vec<FovNode16>, rfov: FovRadius, octant: Octant) -> Self {
        println!("[FovOctant16] building node indexes...");
        let max_r = rfov.to_int() as usize;
        let mut node_indexes = Vec::with_capacity(max_r + 1);
        let mut r = 0;

        for (i, node) in nodes.iter().enumerate() {
            if node.dpri > r {
                println!("  r: {} i: {}", r, i - 1);
                node_indexes.push(i - 1);
                r += 1;
            }
        }

        // Highest node index for max radius is always the last node
        node_indexes.push(nodes.len() - 1);
        println!("...node_indexes: {:?}", node_indexes);

        Self {
            rfov,
            octant,
            nodes: nodes.clone(),
            node_indexes,
        }
    }
    pub fn iter(&self) -> std::slice::Iter<FovNode16> {
        self.nodes.iter()
    }
}

/// Builds nodes for a _Simple_ FOV octant with Q-value `16`.
/// 
/// Notes:
/// - `circ` is the circular culling value used to define FOV shape.
/// - for Simple FOV, the first node `(0,0)` is always visible (all bits set).
pub fn build_fov_nodes_q16(rfov: FovRadius, fov_lines: &FovLines, circ: f64) -> Vec<FovNode16> {
    assert!(rfov == FovRadius::R16 && fov_lines.qfactor == QFactor::Single);

    let n_total = (0..rfov.to_int() as u32 + 2).sum::<u32>() - 1;
    let radius = rfov.to_flt() + circ;
    let mut nodes = vec![FovNode16 {
        body: u16::MAX,
        dpri: 0,
        dsec: 0,
    }];

    // Baseline FOV node lines that define the `body`. Offset by `(dpri, dsec)`.
    let (body_base_1, body_base_2) = body_lines();

    // Octant traversal values
    let mut dpri: u8 = 0;
    let mut dsec: u8 = 0;
    let mut dsec_target: u8 = 0;

    // Get (ds,dp), perform circular culling, and generate FOV bits
    for _ in 0..n_total {
        let sec_eq = dsec == dsec_target;
        dpri += sec_eq as u8;
        dsec = dsec * !sec_eq as u8 + !sec_eq as u8;
        dsec_target += sec_eq as u8;

        if dist_u8(dpri, dsec) > radius {
            continue;
        }

        let body_line_1 = body_base_1.shifted_by(dpri as f64, dsec as f64);
        let body_line_2 = body_base_2.shifted_by(dpri as f64, dsec as f64);
        let mut body = 0u16;

        for (bit_ix, fov_line) in fov_lines.iter().enumerate() {
            let to_set = 1u16 << bit_ix;

            body |= to_set * fov_line.intersects(body_line_1) as u16;
            body |= to_set * fov_line.intersects(body_line_2) as u16;
        }

        nodes.push(FovNode16 { body, dpri, dsec })
    }

    nodes
}

//  ########  ########   ######   ########
//     ##     ##        ##           ##
//     ##     ######     ######      ##
//     ##     ##              ##     ##
//     ##     ########  #######      ##

#[cfg(test)]
mod tests {
    use super::*;

    // FOV Node sanity check:
    // - All FOV lines should pass through the 0th FOV Node.
    // - For Single Q-Factor:
    //   - FOV Node at `(dpri, dsec)` = `(rFOV, 0)` has one FOV bit set.
    //   - FOV Nodes at `(dpri, dsec)` = `(rFOV, >0)` have two FOV bits set.
    // - For Double Q-Factor:
    //   - FOV Node at `(dpri, dsec)` = `(rFOV, 0)` has one FOV bit set.
    //   - FOV Nodes at `(dpri, dsec)` = `(rFOV, >0)` have _at least two_ FOV bits set.
    #[test]
    fn fov_octant_bits_set_q16() {
        let rfov = FovRadius::R16;
        let qsingle = QFactor::Single;
        let fov_lines_16 = FovLines::new(rfov, qsingle);
        let fov_octant_16 = build_fov_nodes_q16(FovRadius::R16, &fov_lines_16, 0.50);

        for fov_node in fov_octant_16.iter() {
            if fov_node.dpri == 16 {
                let body_ct = fov_node.body.count_ones();
                if fov_node.dsec == 0 {
                    assert_eq!(body_ct, 1);
                } else {
                    assert!(body_ct > 1);
                }
            }
        }
    }
}
