extern crate ch11_01_writing_tests;

mod common;

#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, ch11_01_writing_tests::add_two(2));
}
