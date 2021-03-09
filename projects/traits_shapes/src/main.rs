// Import Crate Module with shapes
use traits_shapes::shapes;
// use traits_shapes::shapes::HasArea;

// Compile and link to the 'hello_world' Crate so its Modules may be used in main.rs
extern crate traits_shapes;

// Traits are defined similar to the 'impl' keyword that is used to call a function
// with method syntax (implementing a 'struct')
// 'trait' blocks are similar to the 'impl' block (however only a Type Signature is
// defined Without a Body) and 'impl Trait for Item' is used rather than 'impl Item'
// Rule: Traits must be used in any scope where the Trait method is used.
// Rule: Trait or the type the 'impl' of the trait is being written for must be inside the crate
fn main() {
    let my_circle = shapes::Circle {
      x: 0.0f64,
      y: 0.0f64,
      radius: 1.0f64,
  };

  let my_square = shapes::Square {
      x: 0.0f64,
      y: 0.0f64,
      side: 1.0f64,
  };

  // print_area is now Generic and ensures only correct Types are passed in
  shapes::print_area(my_circle); // outputs 3.141593
  shapes::print_area(my_square); // outputs 1

  shapes::print_area(5); // outputs 5
}
