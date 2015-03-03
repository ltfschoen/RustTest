// lib.rs is a Library Crate

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

mod tests {

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

      pub fn add_two(a: i32) -> i32 {
          a + 2
      }

      #[test]
      fn it_adds_two() {
          assert_eq!(4, add_two(2));
      }

}
