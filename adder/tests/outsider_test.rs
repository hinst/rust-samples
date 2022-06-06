use adder;

#[test]
fn test_get_area() {
    assert_eq!(
        10 * 12,
        adder::Rectangle::new(10, 12).get_area()
    );
}