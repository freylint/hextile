//! Rasterization module for hextile

#[cfg(test)]
mod tests;

use crate::types::Line;
use image::{ImageBuffer, Pixel};

/// Plots a line on an `ImageBuffer` using Bresenham's Line Algorithm.
///
/// Works by stepping through an array of pixels and filling them
/// it then calculates the error of the method
/// and corrects for it
pub(crate) fn plot_line_bres<P: Pixel + 'static>(
    buf: &mut ImageBuffer<P, Vec<P::Subpixel>>,
    color: &P,
    l: Line,
) {
    // Get points in line
    let ul = l.upper_left();
    let br = l.bottom_right();

    // Calculate delta of points
    let dx = br.x - ul.x;
    let dy = br.y - ul.y;

    // Create information buffers
    // d - Delta error
    // y - y coordinate of graph
    let mut d_buf: i32 = (2 * dy - dx) as i32;
    let mut y_buf: u32 = ul.y;

    // Apply bresenham's algorithm
    for x in ul.x..br.x {
        // Plot Pixel
        buf.put_pixel(x, y_buf, *color);

        if d_buf > 0 {
            y_buf += 1;
            d_buf -= (2 * dx) as i32;
        }
        d_buf += (2 * dy) as i32;
    }
}
