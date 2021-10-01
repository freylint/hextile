//! External API

use image::Pixel;

use crate::rasterization::plot_line_bres;
use crate::types::{GenericImageBuf, Line};

#[cfg(test)]
mod tests;

/// Returns a fn pointer which returns an `ImageBuffer` with a triangle drawn on it
pub fn draw_line<P: Pixel + 'static>(
    buf: &mut GenericImageBuf<P>,
    line: Line,
    color: &'static P,
) -> Result<(), Box<dyn std::error::Error>> {
    // Draw line on buffer
    plot_line_bres::<P>(buf, color, line)
}
