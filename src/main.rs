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

// Newtypes are Tuple Structs with only One Element
fn try_newtype() {
  let length = Inches(10);
  // Extract inner Integer Type by Desctructing the Let
  let Inches(integer_length) = length; // Assigns length
  println!("Newtype length is {} inches", integer_length);
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