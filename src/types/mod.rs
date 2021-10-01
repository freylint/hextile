//! Convenience types for hextile

use image::{ImageBuffer, Pixel};

#[cfg(test)]
mod tests;

/// Generic Image Buffer
pub type GenericImageBuf<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;
/// Result returning a `GenericImageBuf` or `Box<dyn std::error::Error`
pub type GenericImageBufResult<P> = Result<GenericImageBuf<P>, Box<dyn std::error::Error>>;

/// Structure representing a point in 2D Cartesian space
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Point {
    /// X Coordinate in cartesian space
    pub x: u32,
    /// Y Coordinate in cartesian space
    pub y: u32,
}

impl Point {
    /// Returns `Point` from raw fields
    pub fn new(x: impl Into<u32>, y: impl Into<u32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

/// Line in 2D cartesian space
///
/// _Note_ It is not recommended to construct this using the struct literal syntax.
/// This skips the validation done in the `Self::new()` constructor
#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
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
