//! FOV Visualization - Rust (2D)

fn main() {
    println!("===== FOV VISUALIZATION - RUST (2D) =====\n");

    use fov2d::fov::*;

    let rfov = FovRadius::R8;
    let qfactor = QFactor::Single;
    let octant = Octant::O1;

    let lines1 = get_fov_lines(rfov, qfactor, octant);

    println!("FOV Lines {rfov:?} {qfactor:?} {octant:?}:");
    for fov_line in lines1.iter() {
        let l = fov_line.length();
        println!("  {fov_line:?}: len: {l}");
    }
}
