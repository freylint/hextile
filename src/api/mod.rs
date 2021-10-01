//! External API

use image::{ImageBuffer, Pixel};

use crate::rasterization::plot_line_bres;
use crate::types::{GenImageBuf, Line};

#[cfg(test)]
mod tests;

/// Returns a fn pointer which returns an `ImageBuffer` with a triangle drawn on it
pub fn line_draw_fn<'f, P: Pixel + 'static>(
    line: Line,
    size: impl Into<[u32; 2]>,
    color: &'static P,
) -> impl Fn() -> Result<ImageBuffer<P, Vec<P::Subpixel>>, Box<dyn std::error::Error>> + 'f {
    let size: [u32; 2] = size.into();

    // Assert the provided buffer size is square
    assert_eq!(size[0], size[1]);

    move || -> Result<GenImageBuf<P>, Box<dyn std::error::Error>> {
        // Create buffer
        let mut buf: ImageBuffer<P, Vec<_>> = ImageBuffer::new(size[0], size[1]);

        // Draw shape on buffer
        plot_line_bres::<P>(&mut buf, color, line);

        // Return buffer
        Ok(buf)
    }
}
