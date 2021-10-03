//! Convenience types for hextile

use image::{ImageBuffer, Pixel};

use crate::rasterization::plot_line_bres;

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

#[cfg(test)]
mod point_tests;

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

#[cfg(test)]
mod line_tests;

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

    /// Returns a fn pointer which returns an `ImageBuffer` with a triangle drawn on it
    pub fn draw_over_buf<P: Pixel + 'static>(
        &self,
        buf: &mut GenericImageBuf<P>,
        color: &'static P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Draw line on buffer
        plot_line_bres::<P>(buf, color, *self)
    }
}

/// Enumeration of a distance from an axis
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum AxisOffset {
    /// Horizontal/X Axis offset
    X(u32),
    /// Vertical/Y Axis offset
    Y(u32),
}

#[cfg(test)]
mod axis_offset_tests;

impl Default for AxisOffset {
    fn default() -> Self {
        Self::Y(u32::default())
    }
}

#[cfg(test)]
mod grid_line_tests;

/// Cartesian grid aligned Line
///
/// Given its own type as it is much faster to draw than lines which require rasterization.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GridLine {
    offset: AxisOffset,
    margin: u32,
}

impl GridLine {
    /// Accessor for the offset field of `GridLine`
    pub fn offset(&self) -> AxisOffset {
        self.offset
    }

    /// Mutable accessor for the offset field of `GridLine`
    pub fn offset_mut(&mut self) -> AxisOffset {
        self.offset
    }

    /// Accessor for the margin field of `GridLine`
    pub fn margin(&self) -> u32 {
        self.margin
    }

    /// Mutable Accessor for the margin field of `GridLine`
    pub fn margin_mut(&mut self) -> u32 {
        self.margin
    }
}
