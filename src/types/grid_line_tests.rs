use crate::types::GridLine;

#[test]
fn new() {
    let l1 = GridLine {
        offset: Default::default(),
        margin: 0,
    };

    let l2 = GridLine::new(Default::default(), 0);

    assert_eq!(l2, l1);
}
