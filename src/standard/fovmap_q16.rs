//! Standard FOV Maps for FOV Visualization - Rust (2D).
//!
//! Notes:
//! - The `FovData` struct contains one or more `FovMap` structs, each of which contains eight `FovOctant`s of `FovNode`s.
//! - Standard FOV uses three tile parts as obstructions: the tile `body`, west-facing wall `wall_w`, and north-facing wall `wall_n`.
//! 
//! Building an FOV map:
//! - Create a list of FOV Nodes (`Vec<FovNode>`) specific to each octant (wall position varies).
//! - Create 8 FOV octant (`FovOctant`) instances from FOV nodes.
//! - Create an FOV map (`FovMap`) from the 8 octants.
