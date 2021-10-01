//! Tests module for the `line_draw_fn` function

use image::{ImageBuffer, Rgba};

use crate::api::line_draw_fn;
use crate::tests_prelude::*;
use crate::types::{Line, Point};

#[test]
fn returns() {
    let _ = line_draw_fn(Line::default(), TEST_SIZE, &COLOR_WHITE_RGBA);
}

#[test]
fn returns_ok() {
    let res = line_draw_fn(Line::default(), TEST_SIZE, &COLOR_WHITE_RGBA);

    let _ = res().unwrap();
}

#[test]
fn modifies_imgbuf() {
    let res = line_draw_fn(
        Line::new(Point::new(0u32, 0u32), Point::new(1u32, 1u32)),
        TEST_SIZE,
        &COLOR_WHITE_RGBA,
    );

    assert_ne!(
        res().unwrap(),
        ImageBuffer::<Rgba<u8>, Vec<u8>>::new(TEST_SIZE[0], TEST_SIZE[1])
    );
}

#[test]
fn draws_line() {
    // Predefine SIZE separate from global test buf SIZE
    // NOTE prevents changing the SIZE from effecting this test
    // as it uses hardcoded mock data
    const SIZE: [u32; 2] = [8, 8];

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

    let line = Line::new(Point::new(0u32, 0u32), Point::new(SIZE[0], SIZE[1]));

    // Pack data into vector
    let mut data_stack: Vec<u8> = Vec::with_capacity((SIZE[0] * SIZE[1] * 4) as usize);
    for slice in data {
        for byte in slice {
            data_stack.push(byte);
        }
    }

    let man_buf = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(SIZE[0], SIZE[1], data_stack).unwrap();

    // Algorithmically generate image buffer
    let buf = line_draw_fn(line, TEST_SIZE, &COLOR_WHITE_RGBA)().unwrap();

    // Check that the two buffers are the same
    assert_eq!(buf, man_buf);
}
