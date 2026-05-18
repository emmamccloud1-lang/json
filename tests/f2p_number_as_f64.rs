use serde_json::Number;

#[test]
fn f2p_as_f64_basic() {
    let n = Number::from(10);
    assert_eq!(n.as_f64(), Some(10.0));
}
