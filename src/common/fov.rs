//! Common FOV types for FOV Visualization - Rust (2D).

use super::maps::Coords;
use super::math::{Delta, Point, Line};

/// FOV radius used in calculations.
pub enum FovRadius {
    R8,
    R16,
    R32,
    R64,
    R128,
}

impl FovRadius {
    /// Converts `FovRadius` into integer `u8` form.
    pub fn to_int(&self) -> u8 {
        match self {
            FovRadius::R8 => 8,
            FovRadius::R16 => 16,
            FovRadius::R32 => 32,
            FovRadius::R64 => 64,
            FovRadius::R128 => 128,
        }
    }
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

impl Octant {
    /// Converts primary/secondary deltas (`dp`, `ds`) to x/y deltas (`dx`, `dy`).
    /// 
    /// Table:
    /// ```text
    /// Octant 1:   dx = (dpri *  1) + (dsec *  0)
	/// 		    dy = (dpri *  0) + (dsec *  1)
    ///
    /// Octant 2:   dx = (dpri *  0) + (dsec *  1)
    ///             dy = (dpri *  1) + (dsec *  0)
    ///
    /// Octant 3:   dx = (dpri *  0) + (dsec * -1)
    ///             dy = (dpri *  1) + (dsec *  0)
    ///
    /// Octant 4:   dx = (dpri * -1) + (dsec *  0)
    ///             dy = (dpri *  0) + (dsec *  1)
    ///
    /// Octant 5:   dx = (dpri * -1) + (dsec *  0)
    ///             dy = (dpri *  0) + (dsec * -1)
    ///
    /// Octant 6:   dx = (dpri *  0) + (dsec * -1)
    ///             dy = (dpri * -1) + (dsec *  0)
    ///
    /// Octant 7:   dx = (dpri *  0) + (dsec *  1)
    ///             dy = (dpri * -1) + (dsec *  0)
    ///
    /// Octant 8:   dx = (dpri *  1) + (dsec *  0)
    /// 			dy = (dpri *  0) + (dsec * -1)	
    /// ```
    pub fn dpds_to_dxdy(&self, dp: i32, ds: i32) -> Delta {
        match self {
            Octant::O1 => Delta::new(dp, ds),
            Octant::O2 => Delta::new(ds, dp),
            Octant::O3 => Delta::new(-ds, dp),
            Octant::O4 => Delta::new(-dp, ds),
            Octant::O5 => Delta::new(-dp, -ds),
            Octant::O6 => Delta::new(-ds, -dp),
            Octant::O7 => Delta::new(ds, -dp),
            Octant::O8 => Delta::new(dp, -ds),
        }
    }
    /// Converts `Octant` to floating point `(dx, dy)` deltas.
    pub fn deltas_f(&self) -> Point {
        match self {
            Octant::O1 => Point::new(1.0, 1.0),
            Octant::O2 => Point::new(1.0, 1.0),
            Octant::O3 => Point::new(-1.0, 1.0),
            Octant::O4 => Point::new(-1.0, 1.0),
            Octant::O5 => Point::new(-1.0, -1.0),
            Octant::O6 => Point::new(-1.0, -1.0),
            Octant::O7 => Point::new(1.0, -1.0),
            Octant::O8 => Point::new(1.0, -1.0),
        }
    }
    /// Converts `Octant` to integer `(dx, dy)` deltas.
    pub fn deltas_i(&self) -> Delta {
        match self {
            Octant::O1 => Delta::new(1, 1),
            Octant::O2 => Delta::new(1, 1),
            Octant::O3 => Delta::new(-1, 1),
            Octant::O4 => Delta::new(-1, 1),
            Octant::O5 => Delta::new(-1, -1),
            Octant::O6 => Delta::new(-1, -1),
            Octant::O7 => Delta::new(1, -1),
            Octant::O8 => Delta::new(1, -1),
        }
    }
}

/// Quantizing factor, multiplied by FOV radius to set FOV granularity.
pub enum QFactor {
    Single,
    Double,
}

// TODO: FOV lines should be in dx/dy terms
// TODO: Octant.to_deltaFOV lines should be in dx/dy terms
/// Returns a list of FOV lines for a given radius, octant, and Q-value.
pub fn get_fov_lines(radius: FovRadius, qfactor: QFactor, octant: Octant) -> Vec<Line> {
    // let deltas xmult, ymult
    // match (radius, qfactor) {
    //     FovRadius::R8, QFactor::Single
    // }
    todo!()
}
