// Compile and link to the 'languages' Crate so its Modules may be used in main.rs
extern crate languages;

// Import Crate Modules into current scope using shorter names
// Best Practice is to import the module rather than the function directly
// to avoid namespace conflict
use languages::german::{greetings, farewells}; // Shortcut syntax to import nultiple names from same Module
use languages::english::{greetings as gdays, farewells as cyas}; // Shortcut syntax to import nultiple names from same Module

fn main() {
    println!("Hello in German: {}", greetings::guten_tag() );
    println!("Goodbye in German: {}", farewells::auf_wiedersehen() );
    println!("Hello in English: {}", gdays::hello() );
    println!("Goodbye in English: {}", cyas::goodbye() );
}
