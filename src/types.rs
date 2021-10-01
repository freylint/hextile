//! Convenience types for hextile

/// Structure representing a point in 2D Cartesian space
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: impl Into<u32>, y: impl Into<u32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn pnt_new() {
        let x = 0u32;
        let y = 0u32;

        let p1 = Point { x, y };

        let p2 = Point::new(x, y);

        assert_eq!(p1, p2);
    }

    #[test]
    fn pnt_default() {
        let p1 = Point::new(0u32, 0u32);
        let p2 = Point::default();

        assert_eq!(p1, p2);
    }
}

/// Line in 2D cartesian space
///
/// _Note_ It is not recommended to construct this using the struct literal syntax.
/// This skips the validation done in the `Self::new()` constructor
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Line {
    /// Upper left point of Line
    ul: Point,
    /// Bottom right point of line
    br: Point,
}

#[allow(dead_code)]
impl Line {
    /// Returns a `Line`
    ///
    /// Validates that the upper left and bottom right points are valid
    pub fn new(ul: impl Into<Point>, br: impl Into<Point>) -> Self {
        // Cast generics into opaque types
        let ul = ul.into();
        let br = br.into();

        // Validate that ul is left of or at the same x coord as br
        assert!(ul.x <= br.x);

        // Validate that ul is above of or at the same y coord as br
        assert!(ul.y <= br.y);

        Self { ul, br }
    }

    /// Returns the top left `Point` of the `Line`
    pub fn upper_left(&self) -> Point {
        self.ul
    }

    /// Returns the bottom right `Point` of the `Line`
    pub fn bottom_right(&self) -> Point {
        self.br
    }
}

#[cfg(test)]
mod line_tests {
    use std::ops::Div;

    use rand::prelude::*;

    use super::*;

    #[test]
    fn line_new() {
        let l1 = Line {
            ul: Point::default(),
            br: Point::default(),
        };

        let l2 = Line::new(Point::default(), Point::default());

        assert_eq!(l1, l2);
    }

    #[test]
    fn line_default() {
        let l1 = Line::new(Point::default(), Point::default());

        let l2 = Line::default();

        assert_eq!(l1, l2);
    }

    /// Test for `Line::upper_left()`
    #[test]
    fn line_uleft() {
        let mut rng = ThreadRng::default();

        let b1 = u32::MAX / 2;

        let ul: u32 = rng.gen_range(0..b1);
        let br: u32 = rng.gen_range(b1 + 1..u32::MAX);

        let ul = Point::new(ul, ul);
        let br = Point::new(br, br);

        assert_eq!(Line::new(ul, br).upper_left(), ul);
    }
    /// Test for `Line::bottom_right()`
    #[test]
    fn line_bright() {
        let mut rng = ThreadRng::default();

        let b1 = u32::MAX / 2;

        //
        let ul: u32 = rng.gen_range(0..b1);
        let br: u32 = rng.gen_range(b1 + 1..u32::MAX);

        let ul = Point::new(ul, ul);
        let br = Point::new(br, br);

        assert_eq!(Line::new(ul, br).bottom_right(), br);
    }
}
