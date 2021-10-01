use super::*;
use crate::rasterization::plot_line_bres;
use crate::types::{Line, Point};
use image::{ImageBuffer, Pixel, Rgba};

/// Test module for the `gen_draw_fn` function.
#[cfg(test)]
mod gen_draw_fn_tests;

/// Returns a fn pointer which returns an `ImageBuffer` with a triangle drawn on it
pub fn gen_draw_fn<'f, P: Pixel + 'static>(
    _pos: impl Into<[u32; 2]>,
    size: impl Into<[u32; 2]>,
    color: &'static P,
) -> impl Fn() -> Result<ImageBuffer<P, Vec<P::Subpixel>>, Box<dyn std::error::Error>> + 'f {
    let size: [u32; 2] = size.into();

    // Assert the provided buffer size is square
    assert_eq!(size[0], size[1]);

    move || -> Result<ImageBuffer<P, Vec<P::Subpixel>>, Box<dyn std::error::Error>> {
        // Create buffer
        let mut buf: ImageBuffer<P, Vec<_>> = ImageBuffer::new(size[0], size[1]);

        // Draw shape on buffer
        plot_line_bres::<P>(
            &mut buf,
            color,
            Line::new(Point::new(0u32, 0u32), Point::new(size[0], size[1])),
        );

        // Return buffer
        Ok(buf)
    }
}
