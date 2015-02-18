// Import the Ordering Enum from the Rust Standard Library
// http://doc.rust-lang.org/std/cmp/enum.Ordering.html
// Note: Enums are a powerful tool for data representation and 'generics' may be used
// to make Enums generic across Types to become even more powerful.
// Ordering Enum form used for Comparisons
// enum Ordering {
//   Less,
//   Equal,
//   Greater,
// }
// :: Symbol indicates a Namespace
// 'std' Module contains Sub-Module 'cmp' containing Sub-Sub-Module 'Ordering'
// The Enum Variants (Equal, Less, Greater) have been imported instead of just the Ordering
// Namespace itself to achieve compactness and convenience. This avoids full scoping and 
// avoids long notation (i.e. use just Equal instead of Ordering::Equal), however name conflicts
// may arise (so it is best practice to import and apply the Namespace combined with a
// Variants together)
use std::cmp::Ordering;

// Configure the Ordering Enum
// 'cmp' is a Function that compares two given parameters and returns an Ordering
// where depending on the difference between these parameters, the returned Ordering 
// may be Ordering:Less, Ordering::Greater, or Ordering::Equal
// Variants of the Enum (Ordering) are Namespaced under the Enum itself.
fn cmp(a: i32, b: i32) -> Ordering {
  if a < b      { Ordering::Less }
  else if a > b { Ordering::Greater }
  else          { Ordering::Equal }
}

/*
  Rust is an Expression-Based Systems language.
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

    // Compound Data Structure Type Tuple is simple fixed size ordered list where &str is String Slice
    let tuple1: (i32, &str) = (1, "hello"); // Mutable. Type annotated. Without: let x = (1, "hello");
    let mut tuple2: (i32, i32) = (1, 2); // Type annotated. Without: let x = (1, "hello");
    let tuple3: (i32, i32) = (3, 4);

    // Note that for a Tuple with a Single value the trailing commas are mandatory
    let (x,): (i32,) = (1,);

    if tuple2 != tuple3 {
        tuple2 = tuple3; // Assign Tuple to another Mutable Tuple (with same Arity and Types)
        println!("tuple2 and tuple3 not equal so assigning");
    } else {
        println!("tuple2 and tuple3 do not have same Arity, Types, Order, and Value, not assigning");
    }

    /*
      Destructuring Let (access parameters of Tuple).
      Let statement performs Multiple Bindings assignments when LHS and RHS Pattern Match
    */
    let (t1, t2) = tuple2;

    println!("t1 and t2 is {}", t1 + t2); // Note: Expects t1 and t2 to be same Type

    // Tuples to return Multiple Values from a Function
    let (tnt1, tnt2) = next_two(5);
    println!("tnt1, tnt2 = {}, {}", tnt1, tnt2);

    try_structs();

    try_tuple_structs();

    try_newtype();

    try_enums();

    try_pattern_matching();
}

/*
  Rust Structs are of Record Type form similar to Tuples.
  Elements in Structs each have a Name (aka Field/Member) and a Type
*/

// Declare a Struct with a name (camel case). Values Immutable by Default
struct PointInSpace {
    x: i32,
    y: i32
}

fn try_structs() {
  // Create Instance (Mutable) of Struct using Let and set each Field (any order)
  let mut origin = PointInSpace { x: 0, y: 0 }; // origin: PointInSpace

  origin.x = 20;

  println!("Origin is at ({}, {})", origin.x, origin.y); // Dot Notation to access Fields
}

/*
  Rust Tuple Structs are a hybrid data type similar to Tuple and Struct.
  Elements in Tuple Structs DO NOT have a Name, they only have a Type.
  Structs are preferred over Tuple Structs since they have actual Names for Elements.
  Note: Tuple Structs must use round brackets and colon
*/

struct Color (
  i32,
  i32,
  i32
);

struct PointInWater (
  i32, 
  i32, 
  i32
);

fn try_tuple_structs() {
  let black = Color(0, 0, 0);
  let origin = PointInWater(0, 0, 0);
}

// Declare a Newtype.
struct Inches (i32);

// Newtypes are Tuple Structs with only One Element. Beneficial for creating 
// New Types similar to others (Cloning)
fn try_newtype() {
  let length = Inches(10);
  // Extract inner Integer Type by Desctructing the Let
  let Inches(integer_length) = length; // Assigns length to integer_length
  println!("Newtype length is {} inches", integer_length);
}

// Enums variants are a "Sum Type" that ties a set of alternatives to a specific name
fn try_enums() {
  // Define a Character to be either a Digit or Other. The alternative names may be 
  // used via their fully scoped names i.e. Character::Other
  // Sub-datastructure types allowable in an Enum include Structs, Tuple Structs
  // Enums do not have access to operators: ==, !=, *, +, <, >=

  enum Character {
    Digit(i32), // Character::Digit is a name tied to an i32
    Other, // Other is just a name (not tied to a specific Type)
  }

  // Assignment
  let ten  = Character::Digit(10);

  let x = 5;
  let y = 10;

  // // 'ordering' variable is of Ordering type (ordering: Ordering)
  // // so it contains one of the three values defined in the 'cmp' Function
  // // and we discover which one by using Conditional Operators to check
  // let ordering = cmp(x, y);

  // if ordering == Ordering::Less { println!("Less"); }
  // else if ordering == Ordering::Greater { println!("Greater"); } 
  // else if ordering == Ordering::Equal { println!("Equal"); }

  // Replace the above 'if/else' with 'match' Pattern Matching instead
  // with less noise and which supports Exhaustiveness Checking across all
  // possible Variants of the Ordering Enum
  match cmp(x, y) {
      Ordering::Less => println!("less"),
      Ordering::Greater => println!("greater"),
      Ordering::Equal => println!("equal"),
  }
}

// Pattern Matching is implemented by 'match' and allows elegantly deconstructing Enums
// (known as the Sum Type in Type Theory) whilst using the 'match' keyword 
// instead of avoiding 'if/else' (for multiple or a complex set of option cases)
fn try_pattern_matching() {
  let x = 5;

  // 'match' takes Expression and branches into Arms depending on its Value.
  // Evaluation is performed only on the matching Arm.
  match x {
    // Arms are of the form 'val => expression'
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    // Exhaustiveness Checking ensures all possible patterns of Values are covered 
    // with the '_' Catch-all- Arm so if any Expression used that were not included 
    // (not all bases covered) it prevents a compile error (as '_' will match)
    _ => println!("something else"),
  }
}

// Returned Tuple is a Single Value (containing Multiple Values)
fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }

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