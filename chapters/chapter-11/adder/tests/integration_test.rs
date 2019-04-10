use adder;

mod common;

#[test]
fn integration_it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
