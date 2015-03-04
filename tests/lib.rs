///////////
//  
// INTEGRATION TESTS
//
///////////

// Integration Tests Directory is entirely separate Crate
// Importing of the hello_world Library is required
extern crate hello_world;

#[test]
fn it_works() {
    assert_eq!(4, hello_world::add_two(2));
}