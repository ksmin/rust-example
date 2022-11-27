extern crate test_adder;

#[test]
fn it_adds_two() {
	assert_eq!(4, test_adder::add_two(2));
}