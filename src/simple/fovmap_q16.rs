//! Simple FOV Maps for FOV Visualization - Rust (2D).
//!
//! Notes:
//! - The `FovData` struct contains one or more `FovMap` structs, each of which contains eight `FovOctant`s of `FovNode`s.
//! - Simple FOV uses one tile part as an obstruction: the tile `body`.
//!
//! Building an FO Map:
//! - Create a list of FOV Nodes (`Vec<FovNode>`), same for each octant.
//! - Create 8 FOV octant (`FovOctant`) instances from FOV nodes.
//! - Create an FOV map (`FovMap`) from the 8 octants.

use crate::{
    fov::{body_lines, FovLines},
    math::dist_u8,
    FovRadius, QFactor,
};

/// FOV map of eight FOV octants, each comprised of 16-bit FOV nodes.
pub struct FovMap16 {
    rfov: FovRadius,
    capacity: usize,
    octant_1: FovOctant16,
    octant_2: FovOctant16,
    octant_3: FovOctant16,
    octant_4: FovOctant16,
    octant_5: FovOctant16,
    octant_6: FovOctant16,
    octant_7: FovOctant16,
    octant_8: FovOctant16,
}

impl FovMap16 {
    /// Creates a new _Simple_ `FovMap` with Q-value `16`.
    ///
    /// Note: `circ_adj` is the circular culling adjustment used to define FOV shape.
    pub fn new(rfov: FovRadius, qfactor: QFactor, circ_adj: f64) -> Self {
        println!("[FovMap16] building FOV map...");
        assert!(rfov == FovRadius::R16, "FovMap16 requires FOV radius of 16!");
        assert!(qfactor == QFactor::Single, "FovMap16 requires Q-Factor of 1!");

        let fov_lines = FovLines::new(rfov, qfactor);
        let nodes = build_fov_nodes_q16(rfov, &fov_lines, circ_adj);
        let capacity = nodes.len() * 8;

        Self {
            rfov,
            capacity,
            octant_1: FovOctant16::new(&nodes, rfov),
            octant_2: FovOctant16::new(&nodes, rfov),
            octant_3: FovOctant16::new(&nodes, rfov),
            octant_4: FovOctant16::new(&nodes, rfov),
            octant_5: FovOctant16::new(&nodes, rfov),
            octant_6: FovOctant16::new(&nodes, rfov),
            octant_7: FovOctant16::new(&nodes, rfov),
            octant_8: FovOctant16::new(&nodes, rfov),
        }
    }
    /// Prints a summary of `FovMap` data.
    pub fn summarize(&self) {
        println!("[FovMap16] Summary:");
        println!("  radius:    {}", self.rfov.to_int());
        println!("  octant 1:  {} nodes", self.octant_1.len());
        println!("  octant 2:  {} nodes", self.octant_2.len());
        println!("  octant 3:  {} nodes", self.octant_3.len());
        println!("  octant 4:  {} nodes", self.octant_4.len());
        println!("  octant 5:  {} nodes", self.octant_5.len());
        println!("  octant 6:  {} nodes", self.octant_6.len());
        println!("  octant 7:  {} nodes", self.octant_7.len());
        println!("  octant 8:  {} nodes", self.octant_8.len());
        println!("  total:     {} nodes", self.capacity);
        println!("  size:      {} bytes", size_of::<Self>());
        println!("  size mem:  {} bytes", self.capacity * size_of::<FovNode16>());
    }
    /// Returns the maxiumum number of FOV nodes in the FOV map.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// One of eight FOV octants, comprised of 16-bit FOV nodes.
///
/// Notes:
/// - for Simple FOV, octants differ only in dx/dy values. The content of each
///   FOV node is the same.
/// - `node_indexes` holds the highest node index for a given radius (`r=0` to `r=16`).
#[derive(Debug)]
pub struct FovOctant16 {
    nodes: Vec<FovNode16>,
    node_indexes: Vec<usize>,
}

impl FovOctant16 {
    /// Creates a new `FovOctant`.
    pub fn new(nodes: &Vec<FovNode16>, rfov: FovRadius) -> Self {
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
            nodes: nodes.clone(),
            node_indexes,
        }
    }
    /// Returns an iterator over the FOV nodes in the octant.
    pub fn iter(&self) -> std::slice::Iter<FovNode16> {
        self.nodes.iter()
    }
    /// Returns the number of nodes in the octant.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Returns the maximum FOV node index for a given radius.
    pub fn max_node_index(&self, radius: usize) -> usize {
        assert!(radius < 17, "radius must be <= 16!");
        self.node_indexes[radius]
    }
}

/// Node in an FOV map representing a single tile with 16 FOV bits (`Q=16`).
#[derive(Debug, Clone)]
pub struct FovNode16 {
    pub body: u16,
    pub dpri: u8,
    pub dsec: u8,
}

/// Creates nodes for a _Simple_ FOV octant with Q-value `16`.
///
/// Note: for Simple FOV, the first node `(0,0)` is always visible (all bits set).
pub fn build_fov_nodes_q16(rfov: FovRadius, fov_lines: &FovLines, circ_adj: f64) -> Vec<FovNode16> {
    let n_total = (0..rfov.to_int() as u32 + 2).sum::<u32>() - 1;
    let radius = rfov.to_flt() + circ_adj;
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
    fn fov_nodes_bits_set_q16() {
        let rfov = FovRadius::R16;
        let qsingle = QFactor::Single;
        let fov_lines_16s = FovLines::new(rfov, qsingle);
        let fov_octant_16s = build_fov_nodes_q16(FovRadius::R16, &fov_lines_16s, 0.50);

        for fov_node in fov_octant_16s.iter() {
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
