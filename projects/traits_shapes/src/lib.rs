// lib.rs is a Library Crate

///////////
//  
// ROOT MODULE AND SUB-MODULES
//
///////////

pub mod shapes {

  use std::f64::consts;

  // Traits implemented on Structs
  ////////////////////////////////

  // Traits are defined with just the Method Signature
  // Traits are then Implemented for a given Struct
  pub trait HasArea {
      fn area(&self) -> f64;
  }

  pub struct Circle {
      // Field sides must be public for access from main.rs
      pub x: f64,
      pub y: f64,
      pub radius: f64,
  }

  // Trait block that implements HasArea for Circle type
  impl HasArea for Circle {
      fn area(&self) -> f64 {
          consts::PI * (self.radius * self.radius)
      }
  }

  pub struct Square {
      // Field sides must be public for access from main.rs
      pub x: f64,
      pub y: f64,
      pub side: f64,
  }

  // Trait block that implements HasArea for Square type
  impl HasArea for Square {
      fn area(&self) -> f64 {
          self.side * self.side
      }
  }

  // Traits implemented on i32 (different types supported)
  // (considered poor style when used on primitives)
  ////////////////////////////////

  // Trait block that implements HasArea for i32 type
  // Note: Trait implementation on primitives is considered poor style
  impl HasArea for i32 {
      fn area(&self) -> f64 {
          *self as f64
      }
  }

  // Trait Constraint added to Generic T to ensure T implements the 'area' method,
  // otherwise T may be any type and so may not.
  // <T: HasArea> means any type that implements the HasArea trait.
  // Traits define Function Type Signatures, so any type that implements HasArea must have 
  // a .area() method.
  pub fn print_area<T: HasArea>(shape: T) {
      println!("This shape has an area of {}", shape.area());
  }
}
