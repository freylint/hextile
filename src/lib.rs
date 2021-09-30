//!
//! # Overview
//! _EARLY WIP_
//! A specialized pure rust library for generating images containing hex tiels images containing hex tiels images containing hex tiels images containing hex tiels images containing hex tiels images containing hex tiels images containing hex tiels images containing hex tiles
//! with an efficient memory layout.
//!


#[cfg(test)]
mod tests_prelude {
    use image::Rgba;
    use rand::Rng;

    pub(crate) const ARRAY2_ZERO: [u32; 2] = [0; 2];
    pub(crate) const TEST_SIZE: [u32; 2] = [8, 8];
    pub(crate) const COLOR_WHITE_RGBA: Rgba<u8> = Rgba { 0: [255, 255, 255, 255] };
    pub(crate) const COLOR_BLACK_RGBA: Rgba<u8> = Rgba { 0: [0, 0, 0, 255] };
}

pub mod types {
    /// Structure representing a point in 2D Cartesian space
    #[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
    pub struct Point{pub x: u32, pub y: u32}

    impl Point {
        pub(crate) fn new(x: impl Into<u32>, y: impl Into<u32>) -> Self{
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

            let p1 = Point {
                x,
                y,
            };

            let p2 = Point::new(x,y);

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
        pub fn new(ul: impl Into<Point>, br: impl Into<Point>) -> Self{
            // Cast generics into opaque types
            let ul = ul.into();
            let br = br.into();

            // Validate that ul is left of or at the same x coord as br
            assert!(ul.x <= br.x);

            // Validate that ul is above of or at the same y coord as br
            assert!(ul.y <= br.y);

            Self {
                ul,
                br,
            }
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
        use super::*;
        use rand::prelude::*;
        use std::ops::Div;

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
            let br: u32 = rng.gen_range(b1+1..u32::MAX);

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
            let br: u32 = rng.gen_range(b1+1..u32::MAX);

            let ul = Point::new(ul, ul);
            let br = Point::new(br, br);

            assert_eq!(Line::new(ul, br).bottom_right(), br);
        }
    }
}

/// Rasterization module
pub(crate) mod rasterization {
    use image::{ImageBuffer, Pixel};
    use crate::types::Line;
    use num_traits::Bounded;

    /// Plots a line on an `ImageBuffer` using Bresenham's Line Algorithm.
    ///
    /// Works by stepping through an array of pixels and filling them
    /// it then calculates the error of the method
    /// and corrects for it
    pub(crate) fn plot_line_bres<P: Pixel + 'static>(
        buf: &mut ImageBuffer<P, Vec<P::Subpixel>>,
        color: &P,
        l: Line
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
        for x in ul.x .. br.x {
            // Plot Pixel
            buf.put_pixel(x, y_buf, *color);

            if d_buf > 0 {
                y_buf += 1;
                d_buf -= (2*dx) as i32;
            }
            d_buf += (2*dy) as i32;
        }
    }
}

pub mod api {
    use super::*;
    use image::{Pixel, ImageBuffer, Rgba};
    use crate::rasterization::plot_line_bres;
    use crate::types::{Line, Point};


    /// TODO Returns a fn pointer which returns an `ImageBuffer` containing a triangle
    pub fn gen_draw_fn<'f, P: Pixel + 'static>(
        _pos: impl Into<[u32; 2]>,
        size: impl Into<[u32; 2]>,
        color: &'static P,
    ) -> impl Fn() -> Result<ImageBuffer<P, Vec<P::Subpixel>>, Box<dyn std::error::Error>> + 'f
    {
        let size: [u32; 2] = size.into();

        // Assert the provided buffer size is square
        assert_eq!(size[0], size[1]);

        move || -> Result<ImageBuffer<P, Vec<P::Subpixel>>, Box<dyn std::error::Error>> {
            // Create buffer
            let mut buf: ImageBuffer<P, Vec<_>> =
                ImageBuffer::new(size[0], size[1]);

            // Draw shape on buffer
            plot_line_bres::<P>(&mut buf, color, Line::new(
                Point::new(0u32,0u32),
                Point::new(size[0], size[1])
            ));


            // Return buffer
            Ok(buf)
        }
    }

    /// Test module for the `gen_draw_fn` function.
    #[cfg(test)]
    mod gen_draw_fn_tests {
        use super::*;
        use image::{RgbaImage, Rgba, Pixels, ImageFormat};
        use crate::tests_prelude::*;

        #[test]
        fn returns() {
            let _ = gen_draw_fn(
                ARRAY2_ZERO,
                TEST_SIZE,
                &COLOR_WHITE_RGBA,
            );
        }

        #[test]
        fn returns_ok() {
            let res = gen_draw_fn(
                ARRAY2_ZERO,
                TEST_SIZE,
                &COLOR_WHITE_RGBA,
            );

            let _ = res().unwrap();
        }

        #[test]
        fn modifies_imgbuf() {
            let res = gen_draw_fn(
                ARRAY2_ZERO,
                TEST_SIZE,
                &COLOR_WHITE_RGBA,
            );

            assert_ne!(
                res().unwrap(),
                ImageBuffer::<Rgba<u8>, Vec<u8>>::new(TEST_SIZE[0], TEST_SIZE[1])
            );

        }#[test]
        fn draws_line() {
            // Predefine size separate from global test buf size
            // NOTE prevents changing the size from effecting this test
            // as it uses hardcoded mock data
            const size: [u32; 2] = [8,8];

            // COLOR Constants
            // White
            const W: [u8; 4] = [255, 255, 255, 255];
            // Alpha
            const A: [u8; 4] = [0, 0, 0, 0];

            // Manually computed image data
            #[rustfmt::skip]
            let data = Vec::from([
                W, A, A, A, A, A, A, A,
                A, W, A, A, A, A, A, A,
                A, A, W, A, A, A, A, A,
                A, A, A, W, A, A, A, A,
                A, A, A, A, W, A, A, A,
                A, A, A, A, A, W, A, A,
                A, A, A, A, A, A, W, A,
                A, A, A, A, A, A, A, W,
            ]);


            // Pack data into vector
            let mut data_stack: Vec<u8> = Vec::with_capacity((size[0] * size[1] * 4) as usize);
            for slice in data {
                for byte in slice {
                    data_stack.push(byte);
                }
            }


            let man_buf =
                ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
                    size[0], size[1], data_stack
                ).unwrap();

            // Algorithmically generate image buffer
            let buf = gen_draw_fn(
                ARRAY2_ZERO,
                TEST_SIZE,
                &COLOR_WHITE_RGBA,
            )().unwrap();

            // Check that the two buffers are the same
            assert_eq!(buf, man_buf);
        }
    }

    // Filters `Pixel`s , returning tru when the `Pixel`'s center point is part of a triangle
    fn tri_filter(size: &impl Into<[u32; 2]>, coord: &impl Into<[u32; 2]>) -> bool {
        true
    }

    /// Test module for the `tri_filter` function
    #[cfg(test)]
    mod tri_filter_tests {
        use super::*;
    }
}
