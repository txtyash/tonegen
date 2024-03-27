#[test]
fn test_template() {
    let word = String::from("cavity");
    let result = cavity::cavity();
    assert_eq!(word, result);
}
