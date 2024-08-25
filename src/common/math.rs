//! Math functionality for FOV Visualization - Rust (2D)

/// 3D point.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    /// Creates a new `Point` instance.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /// Creates a new `Point` displaced by `Vector` `v`.
    pub fn shifted_by(&self, v: Vector) -> Self {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
    /// Displaced current `Point` by `Vector` `v`, _in-place_.
    pub fn shift_by(&mut self, v: Vector) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

/// 3D line used for FOV, LOS, and intersections.
#[derive(Debug, Clone)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
}

impl Line {
    /// Creates a new line.
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Self {
        Self {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        }
    }
    /// Creates a new line of specified `length` from given `ray`.
    pub fn from_ray(ray: Ray, length: f64) -> Self {
        let v = Vector::normalized(ray.r0.x, ray.r0.y, ray.r0.z);
        let x1 = ray.r0.x;
        let y1 = ray.r0.y;
        let z1 = ray.r0.z;
        let x2 = x1 + v.x * length;
        let y2 = y1 + v.y * length;
        let z2 = z1 + v.z * length;

        Self {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        }
    }
    pub fn len(&self) -> f64 {
        let dx = (self.x1 - self.x2).abs();
        let dy = (self.y1 - self.y2).abs();
        let dz = (self.z1 - self.z2).abs();

        return (dx * dx + dy * dy + dz * dz).sqrt();
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
    pub fn new(x0: f64, y0: f64, z0: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            r0: Point::new(x0, y0, z0),
            rv: Vector::new(vx, vy, vz),
        }
    }
    /// Creates a new ray with normalized vector..
    pub fn normalized(x: f64, y: f64, z: f64) -> Self {
        let v = Vector::normalized(x, y, z);
        Self {
            r0: Point { x, y, z },
            rv: v,
        }
    }
    /// Normalizes the vector component of the ray.
    pub fn normalize(&mut self) {
        self.rv.normalize();
    }
}

/// 3D Vector.
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    /// Creates a new vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /// Creates a new normalized vector where unit vector `u = v/|v|`.
    pub fn normalized(x: f64, y: f64, z: f64) -> Self {
        let mut v = Vector::new(x, y, z);
        v.normalize();
        v
    }
    /// Returns the magnitude of the vector.
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    /// Normalizes a vector, unit vector `u = v/|v|`.
    pub fn normalize(&mut self) {
        let mag = self.magnitude();

        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }
}

impl std::ops::Add<Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

// TODO: continue FovRect; add Ray-Rect intersection
impl FovRect {
    pub fn new(p0: Point, s1: Vector, s2: Vector, s1_abs_mag: f64, s2_abs_mag: f64, normal: Vector) -> Self {
        Self { p0, s1, s2, s1_abs_mag, s2_abs_mag, normal }
    }
}

// pub struct Ray {
//     pub x1: f64,
//     pub y1: f64,
//     pub z1: f64,
//     pub dx: f64,
//     pub dy: f64,
//     pub dz: f64,
// }

// class Coords:
//     """2D map integer coordinates."""

//     __slots__ = "x", "y"

//     def __init__(self, x: int, y: int) -> None:
//         self.x = x
//         self.y = y

//     def __iter__(self):
//         return iter((self.x, self.y))

//     def __repr__(self) -> str:
//         return f"{self.x, self.y}"

//     def as_tuple(self):
//         return (self.x, self.y)

// class Line:
//     """2D line segment."""

//     __slots__ = "x1", "y1", "x2", "y2"

//     def __init__(
//         self, x1: int | float, y1: int | float, x2: int | float, y2: int | float
//     ) -> None:
//         self.x1 = x1
//         self.y1 = y1
//         self.x2 = x2
//         self.y2 = y2

//     def __iter__(self):
//         return iter((self.x1, self.y1, self.x2, self.y2))

//     def __repr__(self) -> str:
//         return f"Line {self.x1, self.y1, self.x2, self.y2}"

//     def as_tuple(self):
//         return (self.x1, self.y1, self.x2, self.y2)

//     def to_dict(self):
//         """Converts `Line` to dictionary for serialization."""
//         return {
//             "x1": self.x1,
//             "y1": self.y1,
//             "x2": self.x1,
//             "y2": self.y2,
//         }

//     def intersects(self, other: Self) -> bool:
//         """Returns `True` if this line intersects `other` line.

//         Segment 1 is from (x1, y1) to (x2, y2), along `t`.
//         Segment 2 is from (x3, y3) to (x4, y4), along `u`.
//         """
//         x1, y1, x2, y2 = self
//         x3, y3, x4, y4 = other
//         denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)
//         if denom == 0:
//             return False

//         # Intersection point must be along `t` and `u`
//         t_num = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)
//         if (t_num > 0 and t_num > denom) or (t_num < 0 and t_num < denom):
//             return False

//         u_num = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)
//         if (u_num > 0 and u_num > denom) or (u_num < 0 and u_num < denom):
//             return False

//         return True

//     def intersection(self, other: Self):
//         """Returns intersection point of self and `other` line, else `None`.

//         Segment 1 is from (x1, y1) to (x2, y2), along `t`.
//         Segment 2 is from (x3, y3) to (x4, y4), along `u`.
//         """
//         x1, y1, x2, y2 = self
//         x3, y3, x4, y4 = other
//         denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)
//         if denom == 0:
//             return None

//         # Intersection point must be along `t` and `u`
//         t_num = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)
//         if (t_num > 0 and t_num > denom) or (t_num < 0 and t_num < denom):
//             return None

//         u_num = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)
//         if (u_num > 0 and u_num > denom) or (u_num < 0 and u_num < denom):
//             return None

//         # Choose either `t` or `u` intersection point (`t` chosen)
//         t = t_num / denom
//         return (x1 + t * (x2 - x1), y1 + t * (y2 - y1))

// class Point:
//     """2D map floating point coordinates."""

//     __slots__ = "x", "y"

//     def __init__(self, x: float, y: float) -> None:
//         self.x = x
//         self.y = y

//     def __iter__(self):
//         return iter((self.x, self.y))

//     def __repr__(self) -> str:
//         return f"P{self.x, self.y}"

//     def as_tuple(self):
//         return (self.x, self.y)

//     def distance(self, other: Self) -> float:
//         """Returns distance between self and other."""
//         dx_abs = (other.x - self.x) ** 2
//         dy_abs = (other.y - self.y) ** 2

//         return math.sqrt(dx_abs + dy_abs)

//     def rounded(self) -> Self:
//         return Point(round(self.x, 3), round(self.y, 3))

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
        let v1 = Vector::new(3.0, 4.0, 0.0);
        assert_eq!(v1.magnitude(), 5.0);
    }
}
