//! Simple FOV data builder for FOV Visualization - Rust (2D).
//! 
//! Simple FOV uses one tile part as an obstruction: the tile `body`.

use crate::{fov::body_lines, math::{Line, Vector}, FovRadius, Octant, QFactor};

// TODO: Fov8 for rFOV up to 8, with Q8 and Q16
// TODO: Fov16 for rFOV up to 16, with Q16 and Q32
// TODO: Fov32 for rFOV up to 32, with Q32 and Q64
// TODO: Fov64 for rFOV up to 64, with Q64 and Q128
// TODO: Fov128 for rFOV up to 128, with Q128 and Q256


/// Node in an FOV map representing a single tile with 8 FOV bits (`Q=8`).
#[derive(Debug)]
pub struct FovNode8 {
    body: u8,
    dpri: u8,
    dsec: u8,
}

// TODO: set bits for node 0 based on octant
// TODO: circular FOV culling 
/// Builds a _simple_ FOV octant with Q-value of `8`.
pub fn build_fov_octant_q8(rfov: FovRadius, octant: Octant) -> Vec<FovNode8> {
    // For Simple FOV, the first node `(0,0)` is always visible.
    let mut nodes = vec![FovNode8 { body: 0, dpri: 0, dsec: 0 }];
    let n_total = (0..rfov.to_int() as u32 + 2).sum::<u32>() - 1;

    // Baseline FOV node lines that define the `body`. Offset by `(dpri, dsec)`.
    let (body_base_1, body_base_2) = body_lines();

    // Octant traversal values
    let mut dpri: u8 = 0;
    let mut dsec: u8 = 0;
    let mut dsec_target: u8 = 0;  

    // Get (ds,dp), perform radius check, and generate FOV bits
    for _ in 0..n_total {
        let sec_eq = dsec == dsec_target;
        dpri += sec_eq as u8;
        dsec = dsec * !sec_eq as u8 + !sec_eq as u8;
        dsec_target += sec_eq as u8;

        // Test FOV lines against FOV Node lines and set bits
        let body_line_1 = body_base_1.shifted_by(dpri as f64, dsec as f64);
        let body_line_2 = body_base_2.shifted_by(dpri as f64, dsec as f64);

        

        nodes.push(FovNode8 { body: 0, dpri, dsec })
    }

    nodes
}

/// Sets FOV bits for a Tile obstruction (`body`, `wall`)
/// 
/// FOV lines are in 
pub fn set_fov_bits_8(fov_line: Line, ) -> u8 {
    todo!()
}
// /// Node in an FOV map representing a single tile with 8 FOV bits (`Q=8`).
// #[derive(Debug)]
// pub struct FovNode8 {
//     body: u8,
//     dpri: i16,
//     dsec: i16,
// }

// /// Builds a _simple_ FOV octant with Q-value of `8`.
// pub fn build_fov_octant_q8(rfov: FovRadius, octant: Octant) -> Vec<FovNode8> {
//     // First node `(0,0)` is always visible.
//     let mut nodes = vec![FovNode8 { body: 0, dx: 0, dy: 0 }];
//     let n_total = (0..rfov.to_int() as u32 + 2).sum::<u32>() - 1;

//     let mut pri: i32 = 0;
//     let mut sec: i32 = 0;
//     let mut sec_target: i32 = 0;    

//     // Get (ds,dp), perform radius check, and generate FOV bits
//     for _ in 0..n_total {
//         let sec_eq = sec == sec_target;
//         pri += sec_eq as i32;
//         sec = sec * !sec_eq as i32 + !sec_eq as i32;
//         sec_target += sec_eq as i32;

//         let d = octant.dpds_to_dxdy(pri, sec);



//         nodes.push(FovNode8 { body: 0, dx: d.dx, dy: d.dy })
//     }

//     nodes
// }
