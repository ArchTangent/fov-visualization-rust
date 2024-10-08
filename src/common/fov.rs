//! Common FOV types for FOV Visualization - Rust (2D).

use super::math::{Delta, Line, Point};

/// Data for a visible tile and its subparts.
/// 
/// Subparts include:
/// - `body`: the main tile body.
/// - `wall_n`: the north wall (`Standard` calc only).
/// - `wall_w`: the west wall (`Standard` calc only).
#[derive(Debug)]
pub struct VisibleTile {
    id: usize,
    body: bool,
    wall_n: bool,
    wall_w: bool,
}

/// FOV radius used in calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FovRadius {
    R16,
    R32,
    R64,
    R128,
}

impl FovRadius {
    /// Converts `FovRadius` into integer `u8` form.
    pub fn to_int(&self) -> u8 {
        match self {
            FovRadius::R16 => 16,
            FovRadius::R32 => 32,
            FovRadius::R64 => 64,
            FovRadius::R128 => 128,
        }
    }
    /// Converts `FovRadius` into float `u64` form.
    pub fn to_flt(&self) -> f64 {
        match self {
            FovRadius::R16 => 16.0,
            FovRadius::R32 => 32.0,
            FovRadius::R64 => 64.0,
            FovRadius::R128 => 128.0,
        }
    }
}

/// The eight primary subdivisions of an FOV map.
/// 
/// Visualized:
/// ```text
///  
///    3 3 3  2 2 2   
///  4   3 3  2 2   1
///  4 4   3  2   1 1
///  4 4 4      1 1 1
///         +
///  5 5 5      8 8 8
///  5 5   6  7   8 8
///  5   6 6  7 7   8    
///    6 6 6  7 7 7  
/// ```
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
    pub fn dpds_to_dxdy(&self, dpri: u16, dsec: u16) -> (i16, i16) {
        let dp = dpri as i16;
        let ds = dsec as i16;
        
        match self {
            Octant::O1 => (dp, ds),
            Octant::O2 => (ds, dp),
            Octant::O3 => (-ds, dp),
            Octant::O4 => (-dp, ds),
            Octant::O5 => (-dp, -ds),
            Octant::O6 => (-ds, -dp),
            Octant::O7 => (ds, -dp),
            Octant::O8 => (dp, -ds),
        }
    }    
    // TODO: erase
    // pub fn dpds_to_dxdy(&self, dpri: u8, dsec: u8) -> Delta {
    //     let dp = dpri as i32;
    //     let ds = dsec as i32;

    //     match self {
    //         Octant::O1 => Delta::new(dp, ds),
    //         Octant::O2 => Delta::new(ds, dp),
    //         Octant::O3 => Delta::new(-ds, dp),
    //         Octant::O4 => Delta::new(-dp, ds),
    //         Octant::O5 => Delta::new(-dp, -ds),
    //         Octant::O6 => Delta::new(-ds, -dp),
    //         Octant::O7 => Delta::new(ds, -dp),
    //         Octant::O8 => Delta::new(dp, -ds),
    //     }
    // }
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QFactor {
    Single,
    Double,
}

/// A list of FOV lines.
pub struct FovLines {
    pub radius: FovRadius,
    pub qfactor: QFactor,
    inner: Vec<Line>,
}

impl FovLines {
    /// Creates a new `FovLines` instance.
    pub fn new(rfov: FovRadius, qfactor: QFactor) -> Self {
        Self {
            radius: rfov,
            qfactor,
            inner: get_fov_lines(rfov, qfactor) 
        }
    }
    /// Returns an iterator over the struct's FOV lines.
    pub fn iter(&self) -> std::slice::Iter<Line> {
        self.inner.iter()
    }
    /// Returns the number of FOV Nodes in the struct.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Returns a list of FOV lines with specified radius and Q-value.
/// 
/// FOV lines are defined in terms of `(pri, sec)` coordinates, as 
/// opposed to `(x, y)` coordinates. All lines are the same due to 
/// symmetry between octants.
pub fn get_fov_lines(rfov: FovRadius, qfactor: QFactor) -> Vec<Line> {
    match qfactor {
        QFactor::Single => get_fov_lines_single(rfov),
        QFactor::Double => get_fov_lines_double(rfov),
    }
}

/// Returns a list of `Radius * Q-value` FOV lines.
fn get_fov_lines_single(rfov: FovRadius) -> Vec<Line> {
    // Lines and origin
    let mut lines = Vec::new();
    let radius = rfov.to_flt();
    let p0pri: f64 = 0.5;
    let p0sec: f64 = 0.5;

    // FOV points with secondary delta just into neighboring tile
    for n in 0..rfov.to_int() {
        let dpri = radius;
        let dsec = n as f64 + 0.51;

        // One FOV point per tile along edge
        let pfpri = p0pri + dpri;
        let pfsec = p0sec + dsec;

        let line_n = Line::new(p0pri, p0sec, pfpri, pfsec);
        lines.push(line_n);
    }

    lines
}

/// Returns a list of `2 * Radius * Q-value` FOV lines.
fn get_fov_lines_double(rfov: FovRadius) -> Vec<Line> {
    // Lines and origin
    let mut lines = Vec::new();
    let radius = rfov.to_flt();
    let p0pri: f64 = 0.5;
    let p0sec: f64 = 0.5;

    // First FOV point delta from origin (pri/sec)
    let pipri = p0pri + radius;
    let pisec = p0sec + 0.25;

    let line_i = Line::new(p0pri, p0sec, pipri, pisec);
    lines.push(line_i);

    // FOV lines between first and last, depending on Q-factor
    for n in 1..rfov.to_int() {
        let nf = n as f64;

        // Two FOV points per tile along edge
        let pnx = p0pri + radius;
        let pny = p0sec + nf - 0.25;

        let line_n1 = Line::new(p0pri, p0sec, pnx, pny);
        lines.push(line_n1);

        let pnx = p0pri + radius;
        let pny = p0sec + nf + 0.25;

        let line_n2 = Line::new(p0pri, p0sec, pnx, pny);
        lines.push(line_n2);
    }

    // Final FOV point delta from origin (pri/sec)
    let pfpri = p0pri + radius;
    let pfsec = p0sec + radius - 0.25;

    let line_f = Line::new(p0pri, p0sec, pfpri, pfsec);
    lines.push(line_f);

    lines
}

/// Generates FOV lines for the `body` of an FOV Node, same for all octants.
///
/// These lines are offset by `dpri`, `dsec` of each Node in the FOV octant, 
/// and checked against FOV lines.
/// 
/// _Note:_ in this context, `Line.x` and `Line.y` refer to `pri` 
/// `sec`, respectively.
pub fn body_lines() -> (Line, Line) {
    ( 
        Line { x1: 0.0, y1: 0.0, x2: 0.0, y2: 1.0, },
        Line { x1: 0.0, y1: 0.0, x2: 1.0, y2: 0.0, }, 
    )
}

/// Generates an FOV Node's North wall FOV line (`wall_n`) based on octant.
/// 
/// These lines are offset by `dpri`, `dsec` of each Node in the FOV octant, 
/// and checked against FOV lines.
/// 
/// Octants (1 and 4), (2 and 3), (5 and 8), and (6 and 7) should have the 
/// same values.
/// 
/// _Note:_ in this context, `Line.x` and `Line.y` refer to `pri` 
/// `sec`, respectively.
pub fn wall_n_line(octant: Octant) -> Line {
    match octant {
        Octant::O1 => Line { x1: 0.0, y1: 1.0, x2: 1.0, y2: 1.0, },
        Octant::O2 => Line { x1: 1.0, y1: 0.0, x2: 1.0, y2: 1.0, },
        Octant::O3 => Line { x1: 1.0, y1: 0.0, x2: 1.0, y2: 1.0, },
        Octant::O4 => Line { x1: 0.0, y1: 1.0, x2: 1.0, y2: 1.0, },
        Octant::O5 => Line { x1: 0.0, y1: 0.0, x2: 1.0, y2: 0.0, },
        Octant::O6 => Line { x1: 0.0, y1: 0.0, x2: 0.0, y2: 1.0, },
        Octant::O7 => Line { x1: 0.0, y1: 0.0, x2: 0.0, y2: 1.0, },
        Octant::O8 => Line { x1: 0.0, y1: 0.0, x2: 1.0, y2: 0.0, },
    }
}

/// Generates an FOV Node's West wall FOV line (`wall_w`) based on octant.
///
/// These lines are offset by `dpri`, `dsec` of each Node in the FOV octant, 
/// and checked against FOV lines.
/// 
/// Octants (1 and 8), (2 and 7), (3 and 6), and (4 and 5) should have the 
/// same values.
/// 
/// _Note:_ in this context, `Line.x` and `Line.y` refer to `pri` 
/// `sec`, respectively.
pub fn wall_w_line(octant: Octant) -> Line {
    match octant {
        Octant::O1 => Line { x1: 0.0, y1: 0.0, x2: 0.0, y2: 1.0, },
        Octant::O2 => Line { x1: 0.0, y1: 0.0, x2: 1.0, y2: 0.0, },
        Octant::O3 => Line { x1: 0.0, y1: 1.0, x2: 1.0, y2: 1.0, },
        Octant::O4 => Line { x1: 1.0, y1: 0.0, x2: 1.0, y2: 1.0, },
        Octant::O5 => Line { x1: 1.0, y1: 0.0, x2: 1.0, y2: 1.0, },
        Octant::O6 => Line { x1: 0.0, y1: 1.0, x2: 1.0, y2: 1.0, },
        Octant::O7 => Line { x1: 0.0, y1: 0.0, x2: 1.0, y2: 0.0, },
        Octant::O8 => Line { x1: 0.0, y1: 0.0, x2: 0.0, y2: 1.0, },
    }
}

//  ########  ########   ######   ########
//     ##     ##        ##           ##
//     ##     ######     ######      ##
//     ##     ##              ##     ##
//     ##     ########  #######      ##

#[cfg(test)]
mod tests {
    use super::*;

    // FOV line sanity check: proper number of lines.
    #[test]
    fn fov_line_count() {
        let suite = [
            get_fov_lines(FovRadius::R16, QFactor::Single),
            get_fov_lines(FovRadius::R16, QFactor::Double),
            get_fov_lines(FovRadius::R32, QFactor::Single),
            get_fov_lines(FovRadius::R32, QFactor::Double),
            get_fov_lines(FovRadius::R64, QFactor::Single),
            get_fov_lines(FovRadius::R64, QFactor::Double),
            get_fov_lines(FovRadius::R128, QFactor::Single),
            get_fov_lines(FovRadius::R128, QFactor::Double),
        ];
        let actual: Vec<_> = suite.iter().map(|lines| lines.len()).collect();

        let expected = [16, 32, 32, 64, 64, 128, 128, 256];

        assert_eq!(actual, expected);
    }

    // FOV node line sanity check: lines in some octant pairs should be identical.
    #[test]
    fn fov_node_line_match() {
        let north_pairs = [
            (wall_n_line(Octant::O1), wall_n_line(Octant::O4)),
            (wall_n_line(Octant::O2), wall_n_line(Octant::O3)),
            (wall_n_line(Octant::O5), wall_n_line(Octant::O8)),
            (wall_n_line(Octant::O6), wall_n_line(Octant::O7)),
        ];
        let west_pairs = [
            (wall_w_line(Octant::O1), wall_w_line(Octant::O8)),
            (wall_w_line(Octant::O2), wall_w_line(Octant::O7)),
            (wall_w_line(Octant::O3), wall_w_line(Octant::O6)),
            (wall_w_line(Octant::O4), wall_w_line(Octant::O5)),
        ];
        
        for pair in north_pairs.iter() {
            assert_eq!(pair.0, pair.1);
        }

        for pair in west_pairs.iter() {
            assert_eq!(pair.0, pair.1);
        }
    }
}
