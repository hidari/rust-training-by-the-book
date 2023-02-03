extern crate ch11_01_writing_tests;

#[test]
fn it_adds_two(){
    assert_eq!(4, ch11_01_writing_tests::add_two(2));
}
