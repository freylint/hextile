//! Internal rasterization logic

use contracts::debug_requires;
use image::Pixel;

use crate::types::{GenericImageBuf, Point};

/// Plots a line on an `ImageBuffer` using Bresenham's Line Algorithm.
///
/// Works by stepping through an array of pixels and filling them
/// it then calculates the error of the method
/// and corrects for it
#[inline]
#[debug_requires((ul.x <= br.x) && (ul.y <= br.y))]
#[debug_requires(buf.width() >= 1)]
#[debug_requires(buf.height() >= 1)]
pub(crate) fn plot_line_bres<P: Pixel + 'static>(
    buf: &mut GenericImageBuf<P>,
    color: &P,
    ul: &Point,
    br: &Point,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "intrinsics")]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    unsafe {
        use std::intrinsics::prefetch_read_data;

        prefetch_read_data(ul, 3);
        prefetch_read_data(br, 3);
    }
    // Precalc range while cache is hot
    let line_x_coords = ul.x..br.x;

    // Calculate delta of points
    let dx = br.x - ul.x;
    let dy = br.y - ul.y;

    // Create information buffers
    // d - Delta error
    // y - y coordinate of graph
    let mut d_buf: i32 = (2u32 * dy - dx) as i32;
    let mut y_buf: u32 = ul.y;

    // Apply bresenham's algorithm
    for x in line_x_coords {
        // Plot Pixel
        #[cfg(feature = "intrinsics")]
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        unsafe {
            use std::intrinsics::{prefetch_read_data, prefetch_write_data};

            prefetch_read_data(color, 3);
            prefetch_write_data(buf, 2);
        }
        buf.put_pixel(x, y_buf, *color);

        if d_buf > 0 {
            y_buf += 1;
            d_buf -= (2 * dx) as i32;
        }
        d_buf += (2 * dy) as i32;
    }
    Ok(())
}
