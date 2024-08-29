//! FOV Visualization - Rust (2D)


fn main() {
    println!("===== FOV VISUALIZATION - RUST (2D) =====\n");
    
    use fov2d::fov::*;
    use fov2d::math::Line;
    
    let rfov = FovRadius::R8;
    let qfactor = QFactor::Single;
    let octant = Octant::O1;

    let lines1 = get_fov_lines(rfov, qfactor, octant);

    println!("FOV Lines {rfov:?} {qfactor:?} {octant:?}:");
    for fov_line in lines1.iter() {
        let l = fov_line.length();
        println!("  {fov_line:?}: len: {l}");
    }

    // Check Line 1 vs far tile, 8 tiles away
    let line_dp8 = Line::new(8.0, 0.0, 8.0, 1.0);
    let isect_dp8 = lines1[0].clone().intersection(line_dp8);
    println!("Intersection: {isect_dp8:?}");

    // --- Mini Bench for Intersect --- //
    use std::time::Instant;

    let suite_a: Vec<_> = (-1000..=1000).map(|i| Line::new(0.0, 0.0, 15.0, (i as f64) * 0.015)).collect();

    let suite_b = [
        Line::new(5.0, 10.0, 10.0, 10.0),
        Line::new(10.0, -5.0, 10.0, 5.0),
        Line::new(5.0, -10.0, 10.0, -10.0),
    ];
}
