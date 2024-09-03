//! FOV Visualization - Rust (2D)
//! 
//! FOV Structure:
//! - Field of View (FOV) data is stored in an _FOV data_ struct.
//! - FOV data is broken up into _FOV sets_ of eight _FOV octants_.
//! - Octants are comprised of _FOV nodes_.
//! - FOV nodes are used to reference _FOV tiles_ in _FOV maps_, which are derived from _Tile maps_.
//! - FOV nodes relate obstacles in FOV tiles (e.g. walls) to _quantized_ bits.
//! - Quantized bits are used to determine tile visiblity.
//! - Quantized values range from `16` to `128` depending on max _FOV radius_ (`rFOV`).
//! - FOV calculations depend on radius and _Q-value_ (number of quantized bits).
//! - The larger the radius and Q-value, the longer the FOV calculation will take.
//! 
//! 
//! FOV 
//! 
//! Ray-Line, Line-Line Intersections:
//! - https://stackoverflow.com/questions/14307158
//! - https://stackoverflow.com/questions/563198
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

fn main() {
    println!("===== FOV VISUALIZATION - RUST (2D) =====\n");
    
    use fov2d::fov::*;
    use fov2d::math::Line;    
    
    let rfov = FovRadius::R16;
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

    // --- Node Check --- //
    use fov2d::simple::*;

    let fov_lines = FovLines::new(rfov, qfactor);
    let nodes_o1 = build_fov_nodes_q16(rfov, &fov_lines, 0.5);

    println!("nodes O1, Q8, rFOV = 8:");
    for node in nodes_o1.iter() {
        println!("  {node:?}");
    }
    println!("{} nodes: in total", nodes_o1.len());

    // --- Octant Check --- //
    let nodes_q16 = build_fov_nodes_q16(rfov, &fov_lines, 0.5);
    for fov_node in nodes_q16.iter() {
        println!("{fov_node:?}");
    }

    // let octant_q16 = FovOctant16::new(&nodes_q16, rfov, octant);

    let fov_map_q16 = FovSet16::new(rfov, qfactor, 0.50);
    fov_map_q16.summarize();

    // TODO: redo octant ordering with Y=0 at top of screen?
    println!("size of FovSet16: {}", size_of_val(&fov_map_q16));
    println!("size of FovOctant6: {}", size_of::<FovOctant16>());

}
