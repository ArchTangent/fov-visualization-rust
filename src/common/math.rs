//! Math functionality for FOV Visualization - Rust (2D)

// TODO: continue FovRect; add Ray-Rect intersection

use super::maps::Coords;

/// 2D integer deltas.
#[derive(Debug, Clone, Copy)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Delta {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }
}

/// 2D floating point coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new `Point` instance.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    /// Returns the distance between `self` and `other`.
    pub fn distance(&self, other: Point) -> f64 {
        let dx_abs = (other.x - self.x).powi(2);
        let dy_abs = (other.y - self.y).powi(2);

        (dx_abs + dy_abs).sqrt()
    }
    /// Creates a new `Point` displaced by `Vector` `v`.
    pub fn shifted_by(&self, v: Vector) -> Self {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
    /// Displaces current `Point` by `Vector` `v`, _in-place_.
    pub fn shift_by(&mut self, v: Vector) {
        self.x += v.x;
        self.y += v.y;
    }
    /// Converts current `Point` into `Coords` using `floor()`.
    pub fn to_coords(&self) -> Coords {
        Coords::from(*self)
    }
}

/// 2D line used for FOV, LOS, and intersections.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Line {
    /// Creates a new line.
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }
    /// Creates a new line of specified `length` from given `ray`.
    pub fn from_ray(ray: Ray, length: f64) -> Self {
        let v = Vector::normalized(ray.r0.x, ray.r0.y);
        let x1 = ray.r0.x;
        let y1 = ray.r0.y;
        let x2 = x1 + v.x * length;
        let y2 = y1 + v.y * length;

        Self { x1, y1, x2, y2 }
    }
    /// Returns the length of the line.
    pub fn length(&self) -> f64 {
        let dx = (self.x1 - self.x2).abs();
        let dy = (self.y1 - self.y2).abs();

        return (dx * dx + dy * dy).sqrt();
    }
    /// Returns `true` if `self` intersects `other` line, else `false`.
    ///
    /// - Segment 1 is from `(x1, y1)` to `(x2, y2)`, along `t`.
    /// - Segment 2 is from `(x3, y3)` to `(x4, y4)`, along `u`.
    pub fn intersects(self, other: Self) -> bool {
        let (x1, y1, x2, y2) = (self.x1, self.y1, self.x2, self.y2);
        let (x3, y3, x4, y4) = (other.x1, other.y1, other.x2, other.y2);

        // Intersection point must be along `t` and `u`
        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let t_num = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let u_num = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);

        !(denom == 0.0
            || (t_num > 0.0 && t_num > denom)
            || (t_num < 0.0 && t_num < denom)
            || (u_num > 0.0 && u_num > denom)
            || (u_num < 0.0 && u_num < denom))
    }
    /// Returns intersection point of `self` and `other` line, else `None`.
    ///
    /// - Segment 1 is from `(x1, y1)` to `(x2, y2)`, along `t`.
    /// - Segment 2 is from `(x3, y3)` to `(x4, y4)`, along `u`.
    pub fn intersection(self, other: Self) -> Option<Point> {
        let (x1, y1, x2, y2) = (self.x1, self.y1, self.x2, self.y2);
        let (x3, y3, x4, y4) = (other.x1, other.y1, other.x2, other.y2);

        // Intersection point must be along `t` and `u`
        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let t_num = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let u_num = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);

        if denom == 0.0
            || (t_num > 0.0 && t_num > denom)
            || (t_num < 0.0 && t_num < denom)
            || (u_num > 0.0 && u_num > denom)
            || (u_num < 0.0 && u_num < denom)
        {
            return None;
        }

        // Choose either `t` or `u` intersection point (`t` chosen)
        let t = t_num / denom;

        Some(Point::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
    }
    /// Creates a new `Line` displaced by `x` and `y`.
    pub fn shifted_by(&self, x: f64, y: f64) -> Self {
        Line {
            x1: self.x1 + x,
            y1: self.y1 + y,
            x2: self.x2 + x,
            y2: self.y2 + y,
        }
    }
}

/// 3D ray used for FOV, LOS, and intersections.
#[derive(Debug, Clone)]
pub struct Ray {
    r0: Point,
    rv: Vector,
}

impl Ray {
    /// Creates a new ray.
    pub fn new(x0: f64, y0: f64, vx: f64, vy: f64) -> Self {
        Self {
            r0: Point::new(x0, y0),
            rv: Vector::new(vx, vy),
        }
    }
    /// Creates a new ray with normalized vector..
    pub fn normalized(x: f64, y: f64) -> Self {
        let v = Vector::normalized(x, y);
        Self {
            r0: Point { x, y },
            rv: v,
        }
    }
    /// Normalizes the vector component of the ray.
    pub fn normalize(&mut self) {
        self.rv.normalize();
    }
}

/// 2D Vector.
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    /// Creates a new vector.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    /// Creates a new normalized vector where unit vector `u = v/|v|`.
    pub fn normalized(x: f64, y: f64) -> Self {
        let mut v = Vector::new(x, y);
        v.normalize();
        v
    }
    /// Returns the magnitude of the vector.
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    /// Normalizes a vector, unit vector `u = v/|v|`.
    pub fn normalize(&mut self) {
        let mag = self.magnitude();

        self.x /= mag;
        self.y /= mag;
    }
}

impl std::ops::Add<Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// 3D axis-aligned rectangle specifically made for FOV calculations.
/// Reference point is closest to origin `(0,0)` - width and height are added to it.
/// Side vector `s1` is from `p0` to `p1` (width); side vector `s2` is from `p0` to `p2` (height).
///
/// Width and height are in cell distance (`0.0` to `1.0`).
///
/// - `p0`: reference point. Always closest to origin.
/// - `s1`, `s2`: Side vectors defining width and height. Needed for intersections.
/// - `s1_abs_mag`, `s2_abs_mag`: absolute magnitude (no square root) of side vectors `s1` and `s2`.
///    Effectively width squared or height squared.
/// - `normal`: defines normal vector to the rectangle plane. Always points toward origin. For
///    side `A`, normal points toward `x=0`. For side `B`, it points toward `y=0`. For
///    side `C`, it points toward `z=0`. Will be normalized (in unit form), but does
///    not need to be.
pub struct FovRect {
    pub p0: Point,
    pub s1: Vector,
    pub s2: Vector,
    pub s1_abs_mag: f64,
    pub s2_abs_mag: f64,
    pub normal: Vector,
}

impl FovRect {
    pub fn new(
        p0: Point,
        s1: Vector,
        s2: Vector,
        s1_abs_mag: f64,
        s2_abs_mag: f64,
        normal: Vector,
    ) -> Self {
        Self {
            p0,
            s1,
            s2,
            s1_abs_mag,
            s2_abs_mag,
            normal,
        }
    }
}

/// Convenience function to calculate distance between two `u8` values.
pub fn dist_u8(a: u8, b: u8) -> f64 {
    ((a as f64).powi(2) + (b as f64).powi(2)).sqrt()
}

/// Convenience function to calculate distance between two `u16` values.
pub fn dist_u16(a: u16, b: u16) -> f64 {
    ((a as f64).powi(2) + (b as f64).powi(2)).sqrt()
}

//  ########  ########   ######   ########
//     ##     ##        ##           ##
//     ##     ######     ######      ##
//     ##     ##              ##     ##
//     ##     ########  #######      ##

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectors() {
        let v1 = Vector::new(3.0, 4.0);
        assert_eq!(v1.magnitude(), 5.0);
    }
}
