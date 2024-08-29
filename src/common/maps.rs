//! Tilemaps for FOV Visualization - Rust (2D)

// TODO: finish TileMap

use super::math::Point;

/// 2D map coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Point> for Coords {
    fn from(p: Point) -> Self {
        Self {
            x: p.x.floor() as i32,
            y: p.y.floor() as i32,
        }
    }
}

/// 2D map of tiles with FOV obstructions.
///
/// Obstructions include:
/// - Body: entirety of the tile body
/// - Wall (N): north-facing wall
/// - Wall (W): west-facing wall
///
/// Obstructions are only set if the given part is _present_ and _opaque_.
/// Some FOV calculations, such as `simple`, may not use all obstructions.
pub struct TileMap {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_to_coords() {
        let actual: [Coords; 5] = [
            Point::new(0.5, 0.5).into(),
            Point::new(-1.1, 1.1).into(),
            Point::new(1.1, 1.1).into(),
            Point::new(1.1, -1.1).into(),
            Point::new(-1.1, -1.1).into(),
        ];
        let expected = [
            Coords::new(0, 0),
            Coords::new(-2, 1),
            Coords::new(1, 1),
            Coords::new(1, -2),
            Coords::new(-2, -2),
        ];

        assert_eq!(actual, expected);
    }
}
