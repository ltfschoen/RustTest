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

    let n: i32 = 4;
    let mut o: i32 = 0;

    o = try_loops(n);

    println!("o is {}", o);

    // String Slices (aka String Literals) of Type &str are Statically Allocated (i.e. saved inside compiled program 
    // and existing for entire run duration). The have fixed size and cannot be mutated (i.e. let mut). 
    // stringLiteral is a binding reference to another string that is the Statically Allocated String Slice.
    let mut stringLiteral: &str = "String Slice / String Literal"; // Should not be able to declare as 'let mut'

    // In-Memory Strings (formerly StrBuf) allocate memorary and control their data. They cannot be converted into 
    // the equivalent of a &'static str (Static String) like a &str, as In-Memory Strings are not pre-configured 
    // to live for the entire lifetime of the compiled program like &str. They can infact be converted from String 
    // into a slice &'a str by using the as_slice() method (retaining its original String lifetime)
    let mut stringInMemory: String = "In-Memory String".to_string();

    stringLiteral = try_strings(stringLiteral, stringInMemory);

    println!("stringLiteral is {}", stringLiteral);

    try_arrays();

    try_vectors();
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
    Missing, // Missing is just a name (not tied to a specific Type)
  }

  // Assignment
  let a  = Character::Digit(10);
  let b = Character::Missing;

  // Retrieve values contained in Enums, Error Handling where function output does not 
  // match the expected Type
  match a {
    Character::Digit(n) => print!("\na is {} ", n),
    Character::Missing => print!("\na is missing"),
  }

  match b {
    Character::Digit(n) => print!("\nb is {}", n),
    Character::Missing => print!("\nb is missing"),
  }

  let x = 5;
  let y = 10;

  // // 'ordering' variable is of Ordering type (ordering: Ordering)
  // // so it contains one of the three values defined in the 'cmp' Function
  // // and we discover which one by using Conditional Operators to check
  // let ordering = cmp(x, y);

  // if ordering == Ordering::Less { println!("Less"); }
  // else if ordering == Ordering::Greater { println!("Greater"); } 
  // else if ordering == Ordering::Equal { println!("Equal"); }

  // Replace the above 'if/else' with 'match' Pattern Matching expression instead
  // with less noise and which supports Exhaustiveness Checking across all
  // possible Variants of the Ordering Enum
  println!("{}", match cmp(x, y) {
    Ordering::Less => "\nless",
    Ordering::Greater => "\ngreater",
    Ordering::Equal => "\nequal",
  });

  // Equivalent to the above
  match cmp(x, y) {
    Ordering::Less => println!("\nless"),
    Ordering::Greater => println!("\ngreater"),
    Ordering::Equal => println!("\nequal"),
  };
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

fn try_loops(n: i32) -> (i32) {
  let mut o = 0;
  // For Loop with variable m over Iterator Expression 0..n (with start and end positions)
  // that produces a series of elements where each element is a loop itertion fetched from 
  // the Iterator and bound to the variable m during each cycle through the loop body.
  // The Iterator is upper bound exclusive and so prints 0 through to n-1

  for m in 0..n {
    if m % 2 == 0 { continue; } // Approval condition only processes even iterations.
    println!("for loop with: {}", m);
    o += m;
  }

  let mut done: bool = false;

  println!("{}", o);

  while !done {
    o += o - 2;
    println!("while loop with: {}", o);
    if o % 3 == 0 { done = true; }
  }

  // Infinite Loop intentionally
  loop { // Use this instead of while true {}
    o -= 1;
    if o % 7 == 0 { break; } // End iteration early
  }

  return o;
}

// Strings are a re-sizable Data Structure. 
// String is a sequence of Unicode scalar values validly encoded as a stream of UTF-8 bytes.
// Strings may contain null bytes (as they are not null-terminated)

fn try_strings(s_l: &str, s_m: String) -> &str {

  let mut _s_m: String = s_m; // Declare s_m as a mutable local variable _s_m

  let mut _s: String = s_l.to_string();
  _s.push_str(", is a growable string and is guaranteed to be UTF-8");
  println!("{}", _s);

  // Function to Convert Type &str into Type String (cheap solution that allocates it to memory)
  fn convert_to_string_in_memory_taking_string_in_memory(memory_string: String) {
    let memory_string_slice: &str = memory_string.as_slice();
    println!("Converted Type &str (String Slice / Literal) into Type String (In-Memory String): {}", memory_string_slice);
  }

  // Coerce Type &str into Type String
  convert_to_string_in_memory_taking_string_in_memory(_s_m);

  // Function to Convert Type String into Type &str
  fn convert_to_string_literal_taking_string_slice(slice: &str) {
    println!("Converted Type String (In-Memory String) into Type &str (String Slice / Literal): {}", slice);
  }

  // Coerce Type String into Type &str by prefixing with an & symbol
  convert_to_string_literal_taking_string_slice(&_s);

  // The following attempt to perform the same conversion with the parameter passed into the function gives error:
  //   - error: use of moved value: `s_m`
  //   - note: `s_m` moved here because it has type `collections::string::String`, which is moved by default
  //   - use `ref` to override

  // let mut _s_m2: String = s_m.to_string(); // Declare s_m as a mutable local variable _s_m

  // convert_to_string_literal_taking_string_slice(&_s_m2);

  return "Test String to override the value of the String Slice / Literal that we are supposedly not meant to be able to mutate";
}

// Arrays (sequence of elements of same "List" Type and of fixed length)
// Arrays are immutable by default
// Arrays "List" Type has T notation [T; N]
fn try_arrays() {
  let arr_imm = [1, 2, 3, 4]; 

  println!("arr_imm element 2 is: {}", arr_imm[2]);

  // Declare and initialise each element with value of 0
  let mut arr_mut = [0; 3];

  // Slices of Type &[T] are created from an existing variable
  let arr_mut_slice = &arr_mut[1..3]; // Slice with only elements at index 1 and 2

  println!("arr_mut has {} elements", arr_mut.len());
  println!("arr_mut_slice has {} elements", arr_mut_slice.len());

  for e in arr_mut.iter() {
    println!("arr_mut element e: {}", e);
  }

}

// Vectors are Arrays of "dynamic" length implemented as standard library Type Vec<T>
// Vectors allocate their data on the heap 
// Vectors are to Slices what String is to &str
// Vectors are created using the vec! macro
fn try_vectors() {
  let mut v_mut = vec![1, 2, 3];

  let v_mut_length_before = v_mut.len();

  v_mut.push(4);

  println!("v_mut length change from {} to {}", v_mut_length_before, v_mut.len() );

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