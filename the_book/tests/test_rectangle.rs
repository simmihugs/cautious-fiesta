use the_book::lib::rectangle::Rectangle;

#[test]
fn test_rectangle_can_hold() {
    assert!(Rectangle::from(32, 32).can_hold(&Rectangle::from(16, 16)));
}
