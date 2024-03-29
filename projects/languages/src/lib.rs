// lib.rs is a Library Crate

// Module-level documentation uses triple graves and exclamation mark
// Note: When 'cargo run' is performed, _0 is generated as the name of the
//       documentation module tests. They auto-increment as more tests added.

//! The `greetings` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, greetings::add_two(2));
//! ```

#![feature(test)]
extern crate test;

///////////
//  
// HELPER METHODS
//
///////////

// Function-level documentation uses triple graves

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use greetings::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

///////////
//  
// MODULES
//
///////////

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

    // Import the Test Crate containing Benchmarking and Optimisation support
    use test::Bencher;
    use test::black_box;

    // Tests (run with 'cargo test')
    // Note that the '#[test]' prefix indicates a Test Function
    // Note: 'echo $?' returns non-zero status code if any tests fail

    ///////////
    //  
    // UNIT TESTS
    //
    ///////////

    #[test]
    fn it_works() {
        assert!(true);
    }

    // Invert Test Failure with 'should_fail'

    #[test]
    #[should_panic]
    fn it_fails() {
        assert!(false);
    }

    #[test]
    #[should_panic(expected = "english")]
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

    ///////////
    //  
    // BENCHMARK TESTS
    //
    ///////////

    // bench Attribute and associated Benchmark Tests take an
    // argument '&mut Bencher' into their functions. 'Bencher'
    // provides the 'iter' method and a closure containing
    // code to be benchmarked

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    // Benchmarks may be unexpectantly changed when compiled with
    // Optimisations activated causing the compiler to remove calculations that
    // it thinks have no external effects.
    // Overcome by prefixing with 'return' in the the iter() block
    // or by using the generic test::black_box function (an opaque
    // black box to the optimising that forces it to consider any arg
    // as used)

    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000); // test::black_box
            (0..n).fold(0, |old, new| old ^ new);
        });
    }

}

///////////
//  
// ROOT MODULE AND SUB-MODULES
//
///////////

// Crate Root Module containing Sub-Modules
// Build with 'cargo build' to produce 
// compiled .rlib in 'target' directory
//  ls target
pub mod english {
    // Sub-Modules Declared
    // english::us_english
    pub mod greetings {
        // Sub-Module Contents
        pub fn hello() -> String {
            "Hello.".to_string()
        }
    }

    // Export a Public Interface (Private is Default)
    pub mod farewells {
        pub fn goodbye() -> String {
            "Goodbye.".to_string()
        }
    }
}

// Module Declaration for Multiple Files
// Rust expects a 'german.rs' or
// 'german/mod.rs' file with Module contents
pub mod german;
