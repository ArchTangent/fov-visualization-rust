//! Common FOV types for FOV Visualization - Rust (2D).

use super::math::{Delta, Line, Point};

// TODO: resolve proper number of FOV lines, equal to the rFOV

/// FOV radius used in calculations.
#[derive(Debug, Clone, Copy)]
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
    /// Converts `FovRadius` into float `u64` form.
    pub fn to_flt(&self) -> f64 {
        match self {
            FovRadius::R8 => 8.0,
            FovRadius::R16 => 16.0,
            FovRadius::R32 => 32.0,
            FovRadius::R64 => 64.0,
            FovRadius::R128 => 128.0,
        }
    }
}

/// The eight primary subdivisions of an FOV map.
#[derive(Debug, Clone, Copy)]
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
    /// Converts pri/sec `i32` deltas (`dp`, `ds`) to x/y deltas (`dx`, `dy`).
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
    /// Converts pri/sec `f64` deltas (`dp`, `ds`) to x/y deltas (`dx`, `dy`).
    pub fn dpds_to_dxdy_flt(&self, dp: f64, ds: f64) -> Point {
        match self {
            Octant::O1 => Point::new(dp, ds),
            Octant::O2 => Point::new(ds, dp),
            Octant::O3 => Point::new(-ds, dp),
            Octant::O4 => Point::new(-dp, ds),
            Octant::O5 => Point::new(-dp, -ds),
            Octant::O6 => Point::new(-ds, -dp),
            Octant::O7 => Point::new(ds, -dp),
            Octant::O8 => Point::new(dp, -ds),
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
#[derive(Debug, Clone, Copy)]
pub enum QFactor {
    Single,
    Double,
}

/// Returns a list of FOV lines with specified radius, octant, and Q-value.
pub fn get_fov_lines(rfov: FovRadius, qfactor: QFactor, octant: Octant) -> Vec<Line> {
    match qfactor {
        QFactor::Single => get_fov_lines_single(rfov, octant),
        QFactor::Double => get_fov_lines_single(rfov, octant),
    }
}

/// Returns a list of `Radius * Q-value` FOV lines.
fn get_fov_lines_single(rfov: FovRadius, octant: Octant) -> Vec<Line> {
    // Lines and origin
    let mut lines = Vec::new();
    let radius = rfov.to_flt();
    let p0x: f64 = 0.5;
    let p0y: f64 = 0.5;

    // FOV points with secondary delta of +0.75
    for n in 0..rfov.to_int() {
        let ds = n as f64 + 0.75;

        // One FOV point per tile along edge
        let delta_n = octant.dpds_to_dxdy_flt(radius, ds);
        let pnx = p0x + delta_n.x;
        let pny = p0y + delta_n.y;

        let line_n = Line::new(p0x, p0y, pnx, pny);
        lines.push(line_n);
    }

    lines
}

/// Returns a list of `2 * Radius * Q-value` FOV lines.
fn get_fov_lines_double(rfov: FovRadius, octant: Octant) -> Vec<Line> {
    // Lines and origin
    let mut lines = Vec::new();
    let radius = rfov.to_flt();
    let p0x: f64 = 0.5;
    let p0y: f64 = 0.5;

    // First FOV point delta from origin (pri/sec)
    let delta_i = octant.dpds_to_dxdy_flt(radius, 0.25);
    let pix = p0x + delta_i.x;
    let piy = p0y + delta_i.y;

    let line_i = Line::new(p0x, p0y, pix, piy);
    lines.push(line_i);

    // FOV lines between first and last, depending on Q-factor
    for n in 1..rfov.to_int() {
        let nf = n as f64;

        // Two FOV points per tile along edge
        let delta_n = octant.dpds_to_dxdy_flt(radius, nf - 0.25);
        let pnx = p0x + delta_n.x;
        let pny = p0y + delta_n.y;

        let line_n1 = Line::new(p0x, p0y, pnx, pny);
        lines.push(line_n1);

        let delta_n = octant.dpds_to_dxdy_flt(radius, nf + 0.25);
        let pnx = p0x + delta_n.x;
        let pny = p0y + delta_n.y;

        let line_n2 = Line::new(p0x, p0y, pnx, pny);
        lines.push(line_n2);
    }

    // Final FOV point delta from origin (pri/sec)
    let delta_f = octant.dpds_to_dxdy_flt(radius, radius - 0.25);
    let pfx = p0x + delta_f.x;
    let pfy = p0y + delta_f.y;

    let line_f = Line::new(p0x, p0y, pfx, pfy);
    lines.push(line_f);

    lines
}
