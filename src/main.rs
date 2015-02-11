/// Note: The first line in Markdown is the 'summary' line
///
/// # Main
///
/// * Rust is an Expression-Based Systems language.
/// * Rust's document comments support Markdown notation for exporting to HTML with [rustdoc](http://doc.rust-lang.org/book/documentation.html) and parsed by the [hoedown]https://github.com/hoedown/hoedown) Library
/// * Generate documentation with rustdoc ./src/main.rs
/// * Run tests with rustdoc --test ./src/main.rs

/// ```
/// let a = 1000;
/// ```

#[doc = "
Directly apply Rust document comments using the doc attribute on items.
The Rust compiler otherwise converts triple slash-based document comments to doc
attributes on items implicitly.
"]

//Declare main function
fn main() {
    //! Note: This document comment uses ! to apply it to the comment parent rather than what follows.

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

    // If statement based on Branch concept
    if a == 5 {
      println!("x is five");
    } else {
      println!("x is not five");
    }

    let (x, y, z) = (1, 2, 3);  // assign immutable bindings x, y, z to 1, 2, 3

    let mut b: i32 = 10; // assign mutable binding 'b'
    b = 20; // re-assign to new Value

    /*
      Rust's 'If' is an Expression. It returns a value to initialise
      the 'Let' binding ("Declaration Statement") with. 'Let' is assigned an Expression.
      Assignment to an already bound variable is an Expression.
      Assignment Value in Rust is the special Unit Type () in Rust's Type System.
      In other languages 'If' is a Statements (does not return value).

      Rust's "Expression Statement" converts an Expression into a Statement.
      Semi-colons separate each Expression.
      Semi-colons convert Expression into Statement, throws Value away, returns Unit Type ()
      (causing Type Error when expect a different Type such as i32 to be valid)
    */
    let c: i32 = if b == 20 { 25 } else { 15 };  // Explicitely specify 'c' as of Integer Type
    println!("b is: {}", b); // Interpret interpolation with moustaches
    println!("c is: {}", c);

    incr(b, c); // Call Fn passing Args
    print_sum(b, c); // Call Fn passing Args
}

// All control paths must Return a Value
fn incr(m: i32, n: i32) -> i32 {
    if n < m { 
      return n; // Early Return
    } else {
      println!("incrementing n from: {}", n);
      return n + 10;
    }
}

/* 
  Rust forces Declare expected Type of incoming Fn Args. Type Inference in Fn body.
  Rust allows Declaration expected Type of outgoing Return Value after '->'
*/ 
fn print_sum(m: i32, n: i32) -> () { // Return the Unit Type () (aka nil)
    if m < n {
      println!("sum with m < n is: {}", m + n);
    } else {
      println!("sum with m >= n is: {}", m + n);
    }
}