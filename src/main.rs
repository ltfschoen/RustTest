/*
  Declare main function
*/
fn main() {

    // Metaprogramming calling a macro and passing staticaly allocated string arg
    println!("Hello, world!");

    /* 
      Variable binding expression. 
      <let expression> is Full Pattern for Pattern Matching.
      Rust's Type Inference automatically determines an expression's Type.
      Alternatively be explicit and supply an expression's Type after a colon.
      Or simply annotate the Type in a comment to understand the Type Rust infers.
      Rust Bindings are immutable by Default. 
      Mutable Bindings require 'mut' prefix for Type Safety.
    */
    let a: i32 = 5; // assign immutable binding 'a' with Type i32 and Value 5
    let (x, y, z) = (1, 2, 3);  // assign immutable bindings x, y, z to 1, 2, 3

    let mut b: i32 = 10; // assign mutable binding 'b'
    b = 20; // re-assign to new Value

    println!("The value of b is: {}", b); // Interpret interpolation with moustaches
}