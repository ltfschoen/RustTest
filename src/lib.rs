// lib.rs is a Library Crate

///////////
//  
// MODULES
//
///////////

// Crate Root Module containing Sub-Modules
// Build with 'cargo build' to produce 
// compiled .rlib in 'target' directory
//  ls target
mod english {
      // Sub-Modules Declared
      // english::us_english
      mod greetings {
            // Sub-Module Contents
            fn hello() -> String {
              "Hello.".to_string()
            }
      }

      // Export a Public Interface (Private is Default)
      mod farewells {
            fn goodbye() -> String {
              "Goodbye.".to_string()
            }
      }
}

// Module Declaration for Multiple Files
// Rust expects a 'german.rs' or
// 'german/mod.rs' file with Module contents
pub mod german;

///////////
//  
// HELPER METHODS
//
///////////

pub fn add_two(a: i32) -> i32 {
    a + 2
}

///////////
//  
// UNIT TESTING MODULE
//
///////////

// cfg Attribute allows grouping of Unit Tests separate from rest of Crate
// cfg Attribute only compiles containing code when running tests saving compile time

#[cfg(test)]
mod tests {

      // use Declaration (glob Feature) required to bring Helper Functions 
      // (i.e. it_adds_two() ) from the parent directory into scope
      use super::*;

      // Tests (run with 'cargo test')
      // Note that the '#[test]' prefix indicates a Test Function
      // Note: 'echo $?' returns non-zero status code if any tests fail

      #[test]
      fn it_works() {
        assert!(true);
      }

      // Invert Test Failure with 'should_fail'

      #[test]
      #[should_fail]
      fn it_fails() {
        assert!(false);
      }

      #[test]
      #[should_fail(expected = "english")]
      fn it_fails_equality_comparison() {
        // assert!(false);
        assert_eq!("english", "german");
        // assert_eq!("english", "english");
        // panic!("PLEASE PANIC WHEN PANIC! USED");
      }

      #[test]
      fn it_adds_two() {
          assert_eq!(4, add_two(2));
      }

}
