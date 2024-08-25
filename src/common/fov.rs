//! Common FOV types for FOV Visualization - Rust (2D).

/// FOV radius used in calculations.
pub enum FovRadius {
    R32,
    R64,
    R128,
}

/// The eight primary subdivisions of an FOV map.
pub enum Octant {
    /// Octant ENE of origin.
    O1,
    /// Octant NNE of origin.
    O2,
    /// Octant NNW of origin.
    O3,
    /// Octant WNW of origin.
    O4,
    /// Octant WSW of origin.
    O5,
    /// Octant SSW of origin.
    O6,
    /// Octant SSE of origin.
    O7,
    /// Octant ESE of origin.
    O8,
}

/// Quantizing factor, multiplied by FOV radius to set FOV granularity.
pub enum QFactor {
    Single,
    Double,
}