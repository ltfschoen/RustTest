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
