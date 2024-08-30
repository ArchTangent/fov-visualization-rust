//! FOV Visualization - Rust (2D)
//! 
//! FOV Octants:
//! 
//! ```text
//!  
//!    3 3 3  2 2 2   
//!  4   3 3  2 2   1
//!  4 4   3  2   1 1
//!  4 4 4      1 1 1
//!         +
//!  5 5 5      8 8 8
//!  5 5   6  7   8 8
//!  5   6 6  7 7   8    
//!    6 6 6  7 7 7  
//! ```
//! 


fn main() {
    println!("===== FOV VISUALIZATION - RUST (2D) =====\n");
    
    use fov2d::fov::*;
    use fov2d::math::Line;    
    
    let rfov = FovRadius::R8;
    let qfactor = QFactor::Single;
    let octant = Octant::O1;

    let lines1 = get_fov_lines(rfov, qfactor);

    println!("FOV Lines {rfov:?} {qfactor:?} {octant:?}:");
    for fov_line in lines1.iter() {
        let l = fov_line.length();
        println!("  {fov_line:?}: len: {l}");
    }

    // Check Line 1 vs far tile, 8 tiles away
    let line_dp8 = Line::new(8.0, 0.0, 8.0, 1.0);
    let isect_dp8 = lines1[0].clone().intersection(line_dp8);
    println!("Intersection: {isect_dp8:?}");

    // --- Node Check ---
    use fov2d::simple::*;

    let nodes_o1 = build_fov_octant_q8(rfov, octant);

    println!("nodes O1, Q8, rFOV = 8:");
    for node in nodes_o1.iter() {
        println!("  {node:?}");
    }
    println!("{} nodes: in total", nodes_o1.len());

}
