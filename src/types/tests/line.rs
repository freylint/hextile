use crate::types::{Line, Point};
use rand::prelude::*;

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
