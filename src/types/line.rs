use image::{ImageBuffer, Pixel, Rgba};
use rand::prelude::*;

use crate::prelude::*;
use crate::types::{GenericImageBuf, Line, Point};

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

fn test_buf<P: Pixel + 'static>(s: u32) -> GenericImageBuf<P> {
    ImageBuffer::new(s, s)
}

#[test]
fn returns() {
    let _ = Line::default().draw_over_buf(&mut test_buf(TEST_SIZE), &COLOR_WHITE_RGBA);
}

#[test]
fn returns_ok() {
    let res = Line::default().draw_over_buf(&mut test_buf(TEST_SIZE), &COLOR_WHITE_RGBA);
    let _ = res.unwrap();
}

#[test]
fn modifies_imgbuf() {
    let mut buf = test_buf(TEST_SIZE);

    let _ = Line::new(Point::default(), Point::new(TEST_SIZE, TEST_SIZE))
        .draw_over_buf(&mut buf, &COLOR_WHITE_RGBA)
        .unwrap();

    assert_ne!(
        buf,
        ImageBuffer::<Rgba<u8>, Vec<u8>>::new(TEST_SIZE, TEST_SIZE)
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

    // Allocate buffers
    let mut buf = ImageBuffer::new(SIZE[0], SIZE[1]);
    let man_buf = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(SIZE[0], SIZE[1], data_stack).unwrap();

    // Algorithmically draw line on image buffer
    line.draw_over_buf(&mut buf, &COLOR_WHITE_RGBA).unwrap();

    // Check that the two buffers are the same
    assert_eq!(man_buf, buf);
}
