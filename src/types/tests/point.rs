use crate::types::Point;

#[test]
fn pnt_new() {
    let x = 0u32;
    let y = 0u32;

    let p1 = Point { x, y };

    let p2 = Point::new(x, y);

    assert_eq!(p1, p2);
}

#[test]
fn pnt_default() {
    let p1 = Point::new(0u32, 0u32);
    let p2 = Point::default();

    assert_eq!(p1, p2);
}
