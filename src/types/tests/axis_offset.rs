use crate::types::AxisOffset;

#[test]
fn default() {
    assert_eq!(AxisOffset::Y(0u32), AxisOffset::default())
}
