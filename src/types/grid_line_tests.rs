use crate::types::{AxisOffset, GridLine};

#[test]
fn new() {
    let l1 = GridLine {
        offset: Default::default(),
        margin: 0,
    };

    let l2 = GridLine::new(Default::default(), 0);

    assert_eq!(l2, l1);
}

#[test]
fn offset() {
    let off1 = AxisOffset::X(1);

    let l = GridLine::new(off1, 0);

    let off1 = if let AxisOffset::X(off) = off1 {
        off
    } else {
        panic!("Control offset is not the expected enum variant");
    };

    let off2 = if let AxisOffset::X(off) = l.offset() {
        *off
    } else {
        panic!("Comparison offset is not the expected enum variant.")
    };

    assert_eq!(off2, off1);
}

#[test]
fn offset_mut() {
    // Control offset
    let off = AxisOffset::X(1);

    // Create Line
    let mut l = GridLine::new(AxisOffset::X(2), 2);

    // Mutate offset
    *l.offset_mut() = off;

    let off1 = if let AxisOffset::X(off) = off {
        off
    } else {
        panic!("Control offset is not the expected enum variant");
    };

    let off2 = if let AxisOffset::X(off) = l.offset() {
        *off
    } else {
        panic!("Comparison offset is not the expected enum variant.")
    };

    assert_eq!(off2, off1);
}

#[test]
fn margin() {
    let control = 42;

    let l = GridLine::new(Default::default(), control);

    assert_eq!(*l.margin(), control);
}

#[test]
fn margin_mut() {
    let control = 42;

    let mut l = GridLine::new(Default::default(), 0);

    *l.margin_mut() = control;
    assert_eq!(*l.margin(), control);
}
