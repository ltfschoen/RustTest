#![feature(box_syntax)]

// main.rs is Crate Root of a Binary Crate

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

// Import the Standard Input std::old_io module of the Module System that contains function stdin()
// where 'std' is Rust's Standard Library
// Importing the module with only one level of qualification is best practice (avoid prefixing with std::)
use std::old_io;

// Import the Random Number Generation Module
// http://doc.rust-lang.org/std/rand/fn.random.html
use std::rand;

use std::str;

// Import Reference-Counted Pointer Type (over immutable values)
use std::rc::Rc;

// Import Weak Pointers for Reference-Counting (the weak version of Rc<T>)
use std::rc::Weak;

// Import RefCell for Reference-Counting to provide Interior Mutability 
// (for Heap allocated T with many readers) by wrapping objects to be
// mutated to achieve mutability through a Shared Reference
use std::cell::RefCell;

// use std::clone::Clone;

// Import Pointers (for creation of Null)
use std::ptr;

// Import the num Module (dependency added to Cargo.toml)
extern crate num;
use num::bigint::{ToBigInt, RandBigInt};

// Import Infinite Iterator count()
use std::iter::count;

// Import Float that supports f32 and f64
use std::num::Float;

// Import Crate Module with shapes
use hello_world::shapes;
// use hello_world::shapes::HasArea;

// Compile and link to the 'hello_world' Crate so its Modules may be used in main.rs
extern crate hello_world;

// Import Crate Modules into current scope using shorter names
// Best Practice is to import the module rather than the function directly
// to avoid namespace conflict
use hello_world::german::{greetings, farewells}; // Shortcut syntax to import nultiple names from same Module

// Configure the Ordering Enum
// 'cmp' is a Function that compares two given parameters and returns an Ordering
// where depending on the difference between these parameters, the returned Ordering 
// may be Ordering:Less, Ordering::Greater, or Ordering::Equal
// Variants of the Enum (Ordering) are Namespaced under the Enum itself.
fn cmp(a: u64, b: u64) -> Ordering {
    if a < b      { Ordering::Less }
    else if a > b { Ordering::Greater }
    else          { Ordering::Equal }
}

/*
  Rust is an Expression-Based Systems language.
  Declare main function
*/
fn main() {

    loop {

        println!("Hello in German: {}", greetings::guten_tag() );

        // Standard Input
        println!("Enter a number...");

        // Note that the read_line() method may be called on result of stdin() and may return a line of input
        // (i.e. when using terminal, but perhaps not for a cron job)
        let input = old_io::stdin() // std::old_io::stdio::StdinReader
                      .read_line() // IoResult<String> is a Generic form to match any type of input
                      .ok() // Option<String> (assumes valid result, same as Match statement)
                      // String (without valid value the program terminates. handles error message explicitely)
                      .expect("Failed to read line"); 

        // parse() function takes a &str and converts it to say u32

        // The following two lines of code are equivalent
        // where Result returned by parse is converted into an Option by using the ok method as well
        // let input_num = input.parse::<u32>(); // input_num: Option<u32>
        // The parse() Function receives all input from Standard Input including line breaks (i.e. \n) such as "4\n"
        // so trim() method defined on &str to remove those aspects
        // i32 allows both positive and negative inputs
        let input_num: Result<i32, _> = input.trim().parse(); // input_num: Result<i32, <i32 as FromStr>::Err>

        // Unwrap input_num that has the type Option<u32> rather than u32 by using 'match'
        let unwrapped_num = match input_num {
            Ok(unwrapped_num) => unwrapped_num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        // Metaprogramming calling a macro and passing staticaly allocated string arg
        println!("Hello, {}", unwrapped_num);

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
        // String Slices have an associated lifetime.
        // stringLiteral is a binding reference to another string that is the Statically Allocated String Slice.
        // stringLiteral is a &'static str
        // String Slices are a pointer and a length (similar to Vector Slices) so they offer a 
        // view into an already-allocated string (stringLiteral or String)
        let mut string_literal: &str = "String Slice / String Literal"; // Should not be able to declare as 'let mut'

        // String Slices when used as function arguments are written without an explicit lifetime
        // and so their lifetime is inferred
        fn take_slice(slice: &str) {
            println!("Got: {}", slice);
        }

        take_slice(string_literal);

        // In-Memory Strings (formerly StrBuf) allocate memory (heap-allocated) and control their data. They cannot be converted into 
        // the equivalent of a &'static str (Static String) like a &str, as In-Memory Strings are not pre-configured 
        // to live for the entire lifetime of the compiled program like &str. They can infact be converted from String 
        // into a slice &'a str by using the as_slice() method (retaining its original String lifetime)
        // Strings are growable and UTF-8. Strings may be created from a String Slice with the to_string() method
        let mut string_in_memory: String = "In-Memory String".to_string();
        string_in_memory.push_str(" Modified");

        // Functions that pass a reference (i.e. prefixed &) are automatically coerced to a String Slice
        string_literal = try_strings(string_literal, string_in_memory, &string_literal);

        println!("stringLiteral trim() is {}", string_literal.trim() );

        println!("stringLiteral is {}", string_literal);

        try_arrays();

        try_vectors();

        try_random_numbers();

        let string_to_borrow = "borrow";
        let string_to_own = "own".to_string();

        try_strings_again(string_to_borrow, string_to_own);

        try_indexing_strings();

        try_pointers_reference_and_boxes();

        try_recursive_data_structures();

        try_reference_counted_boxes();

        try_lifetimes();

        try_patterns();

        try_method_syntax();

        try_builder_pattern();

        try_closures();

        try_iterators();

        try_generics();

        try_traits();

        try_traits_with_where_clause();

        try_static_dispatch();

        try_dynamic_dispatch();

        try_macros();

    }
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
    let a = Character::Digit(10);
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

fn try_strings(s_l: &str, s_m: String, s_m_ref: &str) -> &'static str {

    let mut _s_m: String = s_m; // Declare s_m as a mutable local variable _s_m

    let mut _s: String = s_l.to_string();
    _s.push_str(", is a growable string and is guaranteed to be UTF-8");
    println!("{}", _s);

    // s_m_ref is a reference to a String that has been automatically coerced into String Slice
    let mut _s_m_ref: String = s_m_ref.to_string();

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

    // Return a the Static String Slice below by explicitly defining the return type as &'static str (rather than just &str)
    // so that the function definition knows what parameter to associate the lifetime of the return value with when
    // there is a second reference parameter. This overcomes Lifetime Inference.
    // See http://users.rust-lang.org/t/signatures-of-functions-with-borrowed-return-type-value-lifetimes-and-mutability-of-string-literals/519
    return "Test String to override the non-mutable value of the String Slice / Literal\n\n\n";
}

// Strings (sequence of Unicode Scalar values encoded as stream of UTF-8 bytes)
// Strings are not null terminated and may contain null bytes
// Use Borrowing (&str) unless Ownership (String) required to avoid complex Lifetimes
fn try_strings_again(string_to_borrow: &str, string_to_own: String) {

    // Convert Stack-Allocated Array of Bytes into a &str (String Slice)
    let z: &[u8] = &[b'a', b'b'];
    let stack_str: &str = str::from_utf8(z).unwrap();
    println!("stack_str is: {}", stack_str.to_string());

    // Generic Function over different types
    fn get_string_length(param: &str) -> usize {
        param.len()
    }

    println!("string_to_borrow length is {}", get_string_length(string_to_borrow) );
    println!("string_to_own length is {}", get_string_length(&string_to_own) );
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

fn try_random_numbers() {
    // Modulo returns remainder of division. Use 100 it to limit random 
    // values to b/w 0 and 99 (+1 for b/w 1 and 100)
    // Explicit type hinting used so Rust knows range within which to generate
    let secret_number: u64 = (rand::random::<u64>() % 100) + 1; // i32 secret

    println!("Secret number is: {}", secret_number);

    let guess: u64 = (rand::random::<u64>() % 100) + 1;

    println!("Guess is: {}", guess);

    match cmp(guess, secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win");
            return; // Exit when win
        },
    }
}

fn try_indexing_strings() {

    // Access a character of a UTF-8 string by iterating, which is an O(n) operation
    // by using three levels of Unicode (and associated Encodings)
    // Do not use direct indexing as each character may be a variable no. of bytes

    let string_of_chars = "u͔n͈̰̎i̙̮͚̦c͚̉o̼̩̰͗d͔̆̓ͥé";

    // 1. Rust iterator to iterate over each Byte of the Underlying Data Type (Code Units for storage)
    //    Note: Individual Byte representation of each Code Point
    //    i.e. 117 248 155 ...
    for letter in string_of_chars.bytes() {
        println!("{}", letter);
    }

    // 2. Rust iterator to iterate over each Character (Code Points / Unicode Scalar Values)
    //    Note: Individual Code Points of each Grapheme are printed (some Combine Characters)
    //    i.e. u ͔ n ̎ ͈ ̰ i
    for letter in string_of_chars.chars() {
        println!("{}", letter);
    }

    // 3. Rust iterator to iterate over each Visible Character (Graphemes)
    //    Note: 'letter' has type &str 
    //    (single Grapheme may consist of multiple Code Points)
    //    i.e. u͔n͈̰̎i̙̮͚̦
    for letter in string_of_chars.graphemes(true) {
        println!("{}", letter);
    }

}


// Rust has many Types of Pointers
// Note:
// - Variable Bindings and Pointers are of different Types and cannot be added together
// - Pointers may allocate Heap memory addresses (not on the Stack)
// - Pointers used when unknown memory allocation requirements at compile time
//   for structures that may change in size. Instead deal with at runtime and
//   simply use pointer at compile time. 
//
fn try_pointers_reference_and_boxes() {

    // New Variable Binding gives a Name to this Value to be stored on the Stack
    // i.e. Value 5 stored at Memory Stack address 0xd3e030
    let var_binding1: i32 = 5; // Val 5 stored at 0x000001 (Memory Stack address)
    let var_binding2: i32  = 10; // Val 10 stored at 0x000002 (Memory Stack address)

    // 1. Rust's "Reference" Pointer Type (Borrow Ownership)
    //    - Immutable by default
    //    - Zero overhead (safety checks by compiler at compile time,
    //      which is Region Points aka Lifetimes Theory, where Named Value stored on
    //      the Stack is valid from where Declared to when it goes Out of Scope
    //    where LHS is a "reference" to the RHS
    //    - Stack allocation preferred over Heap allocation
    //    - Reference Pointers (Default Type) preferred for allocation
    // 2. Rust's "Box" Pointer Type (Box<T>) (Simple Heap Allocation)
    //    - std::rc::Rc
    //    - Heap allocated and automatically Deallocated when go 
    //      (from memory) Out of Scope
    //    - Boxes do not use Garbage Collection (GC) or Reference Counting (RC)
    //    - Boxes are an "Affine Type > Region Kind" (at Compile Time the Rust Compiler
    //      determines when the Box enters/leaves the Scope and inserts 
    //      appropriate calls automatically) 
    //      * Similar to C's malloc/free)
    //      * Rust allocates correct memory based on types
    //      * Rust automatically frees the memory at end of scope when not used anymore
    //      * Rust prevents any other writable pointers from aliasing to this
    //        Heap allocation, which prevents writing to invalid pointers
    //
    let mut var_reference_pointer1: &i32 = &var_binding1; // Val 0x000001 stored at 0x000003 (Memory Stack address)

    // Not permitted to assign to an immutable borrowed reference pointer
    // *var_reference_pointer1 = 20;

    // Print the Memory Stack address where var_reference_pointer1 is stored
    // using the pointer format string
    println!("{:p}", var_reference_pointer1); // i.e. 0x000003

    let mut var_reference_pointer1 = &30;

    if var_reference_pointer1 == &30 {
        let var_reference_pointer2 = &mut &var_reference_pointer1;
        let var_reference_pointer3 = &mut &var_reference_pointer1;
    }

    // Mutable borrowed reference pointers are not allowed to alias 
    // more than once until borrow ends
    let mut var_reference_pointer4 = 40;
    let alias_1 = &mut var_reference_pointer4;
    // let alias_2 = &mut var_reference_pointer4;

    println!("{:p}", var_reference_pointer1);

    // Dereference the Pointer using the * Dereferencing Operator to 
    // access the Value at the Memory Stack address that it points to
    println!("{}", *var_reference_pointer1 + var_binding2);

    // Also prints the Value of var_reference_pointer1 since
    // println! performs Automatic Dereferencing of it
    println!("{}", var_reference_pointer1 + var_binding2);

    // Note that the following code works without the reference operators
    // However, Mutable Borrows where add_one() requests a mutable reference
    // is not allowed (i.e. (p: &mut i32) )
    fn add_one(p: &i32) -> i32 {
        *p + 1
    }

    // Call to add_one causes p to Borrow Ownership (the value does not "move") of the value of the 
    // var_reference_pointer1 handle. add_one finishes Borrowing when it returns a value
    println!("{}", add_one(&var_reference_pointer1) );


    let var_box1 = Box::new(20);
    let var_rc1 = Rc::new(30);

    // * - Dereference the Pointer
    // & - takes reference to contents
    // Rust knows that var_box1 is being Borrowed by the
    // add_one() function and allows reading the value
    // (when using Boxes and Reference Pointers together)
    println!("{}", add_one(&*var_box1) );
    // Borrowing multiple times is allowable when not simultaneous
    println!("{}", add_one(&*var_box1) );
    println!("{}", add_one(&*var_rc1) );

}

fn try_recursive_data_structures() {

    // Cons List (Recursive Data Structure)

    #[derive(Debug)]
    enum List<T> {

        // Reference to List inside Cons enum variant is Box
        // as not know length of variant, so List must be Heap allocated
        Cons(T, Box<List<T>>),
        Nil,
    }

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list);
}

// Reference-Counted Boxes (Rc<T> type)
// - std::rc::Rc;
// - Shared Owner of multiple Immutable Values
// - Owner remains allocated whilst at least one Pointer pointing to it
// - Destruction when last Owner is gone
// - Non-Sendable since it avoids the overhead of Automatic Reference Counting (ARC)
// - Thread-Local
// - Weak<T> Pointer (Non-Owning) to a Box achieved using 'downgrade' method
// - Rc<T> Pointer may be upgraded from a Weak<T> Pointer achieved. Returns 'None' if value already dropped

// Reference-Counting Data Structures

struct BigUniverse {
  one: i32,
  two: i32,
  name: String,
  age: u64,
  age_bigint: num::bigint::BigInt,
  // ...
  billion: i32,
}

struct Galaxy {
    name: String,
    // RefCell used to store the Galaxy's vector of Planets so we may
    // mutate it through a Shared Reference.
    // Weak<T> Reference Pointers are used, as they do not contribute to the total
    // Reference-Count and because doing so avoids memory leaks (where objects
    // remain allocated to memory) that may otherwise result
    // from just using Strong Rc<T> Reference Pointers between Galaxy and planets
    // (which may result in cycles between objects where two objects point at each other
    // and prevent Reference-Counts reaching zero because at least one of them must be mutable).
    // This problem arises because Rc<T> enforces memory safety by only giving Shared Reference
    // to the object it wraps, which do not allow direct mutation.
    // To overcome this, we have imported std::cell::RefCell for Reference-Counting to provide
    // Interior Mutability and the wrapping of objects to be mutated to achieve mutability through
    // a Shared Reference. RefCell enforces Rust borrowing rules at runtime.
    planets: RefCell<Vec<Weak<Planet>>>,
}

struct Planet {
    id: i32,
    owner: Rc<Galaxy>,
}

// http://doc.rust-lang.org/std/rc/index.html
fn try_reference_counted_boxes() {

    // Return a Pointer from a function to avoid copying a large data structure,
    // since by passing around a Box, we are only copying a Pointer rather than a billion
    // int's that comprise BigUniverse
    fn process_big_data_structure(my_universe1: Box<BigUniverse>) -> BigUniverse {
        *my_universe1
    }

    let my_universe1 = Box::new(BigUniverse {
        one: 1,
        two: 2,
        // http://doc.rust-lang.org/0.11.0/num/bigint/struct.BigInt.html
        // https://github.com/rust-lang/num/blob/master/src/bigint.rs
        name: "Vortex".to_string(),
        age: 13800000000,
        age_bigint: 13800000000.to_bigint().unwrap(), // returns 915098112 since i32
        billion: 1000000000,
    });

    // Use the previously included 'box_syntax' feature gate syntax (flexibility without losing performance)
    // The value returned from the process_big_data_structure() function is boxed.
    // No copying is performed, instead the try_reference_counted_boxes() function block allocates enough 
    // memory for the box, passes a pointer to that memory into the process_big_data_structure() function
    // as as my_universe1, and then process_big_data_structure() writes the value straight into the Box<T>.
    let generated_big_universe: Box<BigUniverse> = box process_big_data_structure(my_universe1);

    fn print_big_data_structure(pointer_param: &u64) {
        println!("My Big Universe is {} years old", *pointer_param)
    }

    print_big_data_structure(&generated_big_universe.age);

    println!("My Big Universe is {} years old", generated_big_universe.age_bigint);

    let my_galaxy1 = Galaxy { 
        name: "Milky Way".to_string(), // String::from_str("Milky Way")
        planets: RefCell::new(Vec::new()),
    };

    // Create a Reference-Counted Galaxy (Owner)
    // with Rc<T> so it may be pointed to by multiple Planets
    // (not using the default Box<T> as this type only caters for a single owner)
    let my_galaxy1_owner : Rc<Galaxy> = Rc::new(my_galaxy1);

    // Create Planets and Stars belonging to the Galaxy (Owner)
    // Increment the Reference-Count by cloning the Rc<T> object
    let earth = Planet { id: 1, owner: my_galaxy1_owner.clone() };
    let mars = Planet { id: 2, owner: my_galaxy1_owner.clone() };

    let earth_owner : Rc<Planet> = Rc::new(earth);
    let mars_owner : Rc<Planet> = Rc::new(mars);

    // Add different planets (i.e. Earth, Mars) to the Galaxy by
    // mutably borrowing from RefCell, since Galaxy's "planets"
    // property holds its planets
    // Clone method is used to create new references
    my_galaxy1_owner.planets.borrow_mut().push(earth_owner.clone().downgrade());
    my_galaxy1_owner.planets.borrow_mut().push(mars_owner.clone().downgrade());

    // Iterate over the planets in a galaxy and print to stdout
    // Since planet_opt is a Weak Reference (i.e. Weak<Planet>) it cannot 
    // guarantee its objects are still allocated, so we must turn them into 
    // Strong References by calling upgrade() on them, which in turn returns
    // Option, containing a reference to our object if it still exists
    for planet_opt in my_galaxy1_owner.planets.borrow().iter() {
        let planet = planet_opt.upgrade().unwrap();
        println!("Planet {} owned by {}", planet.id, planet.owner.name);
    }

    // Drops only the my_galaxy1_owner (owner) Reference-Count object (not the Galaxy it wraps)
    // Galaxy it wraps remains allocated whilst other Rc<T> objects still point to the Rc<T> wrapper
    // Note: Destructor of my_galaxy1_owner will ensure memory is cleaned up at the end of the
    // function, so explicit 'drop' is not required
    // drop(my_galaxy1_owner);

    println!("Earth {} owned by {}", earth_owner.id, earth_owner.owner.name);
    println!("Mars {} owned by {}", mars_owner.id, mars_owner.owner.name);

    // End of method causes destruction of: my_galaxy1_owner, earth, mars 
    // so there are no more Strong References (i.e. Rc<T>) to the planets
    // After these are destroyed then Planets gets destroyed, causing the Reference-Count
    // on Milky Way to become zero, so Milky Way gets destroyed
}

// Ownership (of Resources such as Memory) in Rust involves
// Owning Handle (Pointers) that are received for interaction when memory allocation is requested
// Rust performs the deallocation of handles when they go out of scope
// Memory Leakage Concept (function allocates bytes of memory each time it is called 
// but does not deallocate it each time, so eventually we run out of memory)
// Memory that is allocated should also be deallocated only once (the counts must match)
// Refer to Borrowing
//
// Lifetimes Concept of Rust's Ownership System prevents Dangling Pointers (i.e. where a reference
// to an acquired handle that points to a Resource is lent to a function that deallocates such that
// after returning from the function when the Resource is used an error occurs)
fn try_lifetimes() {

    // Static Lifetimes have a lifetime of the entire program
    // String Literals have the Type &'static str as the reference is always alive in data segment of final binary
    let my_static_string: &'static str = "Hello, world.";

    // Global that adds i32 to data segment of binary with my_static_integer referencing it
    static MY_GLOBAL: i32 = 20;
    let my_static_integer: &'static i32 = &MY_GLOBAL;

    println!("My Static Integer is: {}", my_static_integer);

    // Struct with a Lifetime declared as 'a to ensure any reference to MyBody cannot
    // outlive the reference to an i32 it contains.
    // Named Lifetimes are a way of giving the Scope of a bindings a name for understanding lifetimes
    struct MyBody<'a> {
        with_lifespan: &'a i32,
    }

     // same as `let _my_lifespan = 5; let my_lifespan = &_my_lifespan;`
    let my_lifespan = &100;
    let my_body = MyBody { with_lifespan: my_lifespan };

    println!("My Lifespan is: {}", my_body.with_lifespan);

    // Lifetime Elision (i.e. the option of omitting lifetime annotations) feature of Rust.
    // 'a is called a Lifetime (below the lifetime annotation is included, not elided) where the letter 'a' 
    // can be provided with a more descriptive name instead if desired
    // add_ten Function declares that it has one Lifetime 'a (multiple would be expressed as <'a, 'b>)
    // add_ten takes a mutable reference to an i32 with a lifetime of 'a
    // Input Lifetime is: (num: &'a mut i32)
    // Output Lifetime is: -> &'a i32
    fn add_ten<'a>(num: &'a mut i32) -> &'a i32 {
        *num += 10;
        num // Control path must return a value as an Output Lifetime is explicitly declared
    }

}

// Patterns
// - Let Bindings
// - Match Statements
fn try_patterns() {

    // Match Literals
    let my_input = 1;

    match my_input {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // WORKING - Match Multiple Patterns
    match my_input {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // WORKING - Match Range of Values
    match my_input {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }

    // WORKING - Match Range of Values (Bound to a Name)
    match my_input {
        // Parsed as (1 ... 5)
        e @ 1 ... 5 => println!("range element detected: {}", e),
        _ => println!("anything"),
    }

    // WORKING - Match Multiple Values (Bound to a Name)
    match my_input {
        // Note that | symbol takes two entirely independent patterns
        f @ 1000 | f @ 2000 => println!("century detected: {}", f),
        _ => println!("anything"),
    }

    // Matching on an Enum
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let my_input_for_matching = OptionalInt::Value(5);

    // Match Guards (with If)
    match my_input_for_matching {
        OptionalInt::Value(i) if i <= 5 => println!("Int < 5 detected"),
        // Ignore the Value and type in the Variant by using (..)
        OptionalInt::Value(..) => println!("Invalid Int detected"),
        OptionalInt::Missing => println!("Nothing detected"),
    }

    // Matching on a Pointer
    let my_input_pointer_for_matching: &i32 = &5;

    // Matching on a Mutable Pointer
    let mut my_mutable_input_pointer_for_matching: &i32 = &10;

    // Match and retrieve a Reference with 'ref'
    match my_input_pointer_for_matching {
        // ref Keyword creates a Reference for use in the pattern
        ref get_reference => println!("Reference detected to: {}", get_reference),
    }

    // Match and retrieve a Mutable Reference with 'ref'
    match my_mutable_input_pointer_for_matching {
        ref mut get_mut_reference => println!("Mutable Reference detected to: {}", get_mut_reference),
    }

    match my_input_pointer_for_matching {
        // &val destructures the value of my_input_pointer_for_matching
        &val => println!("Value detected: {}", val),
    }

    // Destructure a Struct inside of a Pattern

    // Create another Instance (Mutable) of Struct
    let mut another_origin = PointInSpace { x: 0, y: 0 }; // another_origin: PointInSpace

    another_origin.x = 50;

    match another_origin {
        // Use .. when we do not care about certain values
        PointInSpace { x: x, .. } => println!("Another Origin x value is at: {}", x),
    }

    // Match against a Slice or an Array
    let my_vector = vec!["match_this", "1"];

    match &my_vector[..] {
        ["match_this", second] => println!("The second element is {}", second),
        _ => {},
    }

}

fn try_method_syntax() {
    // Method Call Syntax via the 'impl' keyword to sequentially call multiple functions on input data
    // to allow the following syntax: x.foo().bar().baz();
    // Traditionally allowable syntax is: baz(bar(foo(x)));

    // Struct represents a Circle
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        // Methods take a special first parameter variants, where the
        // first parameter represents the x in x.foo(). It depends on what x is, either:
        // - self (where x is just a Value on the Stack)
        // - &self (where x is a Reference)
        // - &mut self (where x is a Mutable Reference)
        fn area(&self) -> f64 {
            // Import PI
            std::f64::consts::PI * (self.radius * self.radius)
        }

        // Return Type is Circle to grow a new circle with an area 100 times bigger
        fn grow(&self) -> Circle {
            Circle { x: self.x, y: self.y, radius: (self.radius * 10.0) }
        }
    }

    let my_circle = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("Circle area is: {}", my_circle.area());

    // Method Chaining (i.e. my_circle.grow().area() ) is achieved by
    // returning self from the grow() method
    let my_big_circle = my_circle.grow().area();
    println!("Big Circle area is: {}", my_big_circle);

    // Struct represents a Rectangle
    struct Rectangle {
        x: f64,
        y: f64,
    }

    impl Rectangle {
        // Static Methods used to build a new Rectangle
        fn new(x: f64, y: f64) -> Rectangle {
            Rectangle {
                x: x,
                y: y,
            }
        }

        fn area(&self) -> f64 {
            self.x * self.y
        }
    }

    // Static Method called using the Struct::method() syntax, whereas Normal Methods
    // are called with the ref.method() syntax
    let my_rectangle = Rectangle::new(3.0, 5.0);
    println!("Rectangle area is: {}", my_rectangle.area());

}

// Rust uses the Builder Pattern (as Method Overloading, Named Arguments, 
// and Variable Arguments are not supported)
// The Builder Pattern allows users to create objects by only allow the setting of 
// properties that concern them
// Rust's Type System is used to enforce concerns by using methods on
// TriangleBuilder to contrain making Triangles in custom ways
fn try_builder_pattern() {

    struct Triangle {
        base: f64,
        height: f64,
    }

    // Define Area method on Triangle
    impl Triangle {
        fn area(&self) -> f64 {
            0.5 * (self.base * self.height)
        }
    }

    struct TriangleBuilder {
        base: f64,
        height: f64,
    }

    // Define Builder methods on the TriangleBuilder struct
    impl TriangleBuilder {
        fn new() -> TriangleBuilder {
            TriangleBuilder { base: 100.0, height: 50.0, }
        }

        fn base(&mut self, base: f64) -> &mut TriangleBuilder {
            self.base = base;
            self
        }

        fn height(&mut self, height: f64) -> &mut TriangleBuilder {
            self.height = height;
            self
        }

        // TriangleBuilder::finalize()
        // Define Finalize method to create the final Triangle from the Builder
        fn finalize(&self) -> Triangle {
            Triangle { base: self.base, height: self.height }
        }
    }

    let my_triangle = TriangleBuilder::new()
                .base(300.0)
                .height(200.0)
                .finalize();

    println!("Triangle area: {}", my_triangle.area());

}

// Rust's Anonymous Functions are called Closures (particularly useful when 
// combined with functions that take closures as arguments)
fn try_closures() {

    // Closure created using |...| { ... } syntax
    // Closures "close over their environments"
    // Closures infer their argument and return types so declaration is not required
    // which differs from Named Functions that default to returning Unit (()).
    // 'addition' binding is created so that the Closure may be reused
    let addition = |x: i32, y: i32| { x + y };


    // Anonymous Closure syntax || means this is an Anonymous Closure that takes no arguments
    // Anonymous Closure has access to variables in the scope where it is defined
    // Anonymous Closures Borrow any variables that they use
    let printer = || { println!("Sum of {} plus {} is: {:?}", 6, 7, addition(6, 7)); };

    printer(); // prints "addition is: 5"


    // Moving Closures using the move || x * x keyword (differs from Ordinary Closures).
    // Moving Closures are most useful with Rust's Concurrency features (see Threads)
    // Moving Closures always takes Ownership of all variables that it uses, whereas
    // Ordinary Closures just create a Reference into the enclosing Stack Frame.


    // Closures as Arguments accepted by another Function drives efficiency with all information 
    // available at compile-time.
    // 'twice' takes two arguments (dim and f) and returns an i32, where dim is of type i32, whilst
    // f is a Function that takes an i32 argument and returns an i32 as 
    // indicated in the type parameter F.
    // F now represents any function that takes an i32 and returns an i32.
    fn twice<F: Fn(i32) -> i32>(dim: i32, f: F) -> i32 {
        // Call the Closure f, passing the dim argument each time
        f(dim) + f(dim)
    }

    // Closure that takes an integer and returns its square
    let square = |dim: i32| { dim * dim };

    // Named Function (Alternative)
    // http://users.rust-lang.org/t/precedence-of-variable-bound-closures-over-named-functions-during-naming-conflicts/580/1
    fn square(dim: i32) -> i32 { dim * dim };

    // Call the 'twice' Function passing it two variable bindings that include 
    // an integer (of type i32) and the 'square' Closure (of type Function)
    println!("Double the input is: {}", twice(5, square) ); // evaluates to (5 * 5) + (5 * 5) == 50

    // Define Inline
    println!("Triple the input is: {}", twice(5, |dim: i32| { dim * dim * dim }) );

    // Function that accepts two Closures.
    // F and G are: Type Parameters.
    // f and g both have the same Type Signature: Fn(i32) -> i32.
    // Two different Type Parameters F and G are used since the Unique Type of each Closure parameter 
    // prevents them being represented by the same Type Parameter as the other Closure.
    // Each Closure has its own Unique Type since the behaviour of a Closure is part of its Type, such that:
    // - Different Closures with Different Signatures have Different Types
    // - Different Closures with the Same Signature have Different Types
    fn compose<F, G>(x: i32, f: F, g: G) -> i32
        // 'where' clause allows Type Parameters to be described in a flexible manner
        where F: Fn(i32) -> i32, 
              G: Fn(i32) -> i32 {
        g(f(x))
    }

    println!("Composition of two Closure inputs is: {}", compose(5,
                                                        |n: i32| { n + 42 },
                                                        |n: i32| { n * 2 })
    ); // evaluates to 94

}

// Iterators return a sequence after next() method is called repeatedly
// - List of Iterators available in Rust http://doc.rust-lang.org/std/iter/
// - Iterators - gives a sequence of values.
// - Iterators - provide safe, efficient way to manipulate different lists
// - Iterator Adapters - operate on an Iterator, producing New Iterator with different output sequence
// - Consumers - operator on an Iterator, producing final set of values
fn try_iterators() {

    // Raw Null Pointer
    let my_null: *const i32 = ptr::null();

    // Mutable binding to 'range' for use as an Iterator
    let mut range = 0..2;

    // Loop (loop / match / break construct) with inner Match
    loop {
        // Match used on result of range.next() giving reference to next value of Iterator
        match range.next() {
            // next() returns Option<i32>, which is either Some(i32) when a value exists
            // or None when we run out of values
            Some(iteration) => {
                println!("{}", iteration); // outputs 0, 1
            },
            // Break out of the loop
            None => { break }
        }
    }

    // Iterate over contents of a Vector
    ////////////////////////////////////

    // WRONG
    // - Iterates through indexes then indexing the Vector
    // - Less Efficient due to extra bounds checking through using indexing (i.e. nums[i])
    let nums = vec![1, 2, 3];

    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    // RIGHT
    // - Iterates over Vectors directly
    // - Iterates through entire Vector
    // - Yields a Reference (&) to each element of Vector with Iterator. 
    // - No bounds checking required to be safe
    // - More Efficient
    // - Directly expresses what we mean
    let nums = vec![1, 2, 3];

    // num is of Type &i32 (a Reference to an i32) as we explicitly requested a Reference with &
    // so we are just Borrowing a Reference to the Data (just Pass-By-Reference without a Move)
    // rather than dealing with the Data itself, being its Owner, andhaving to make a Copy of it.
    // println! handles Dereferencing automatically so prefixing with * is optional
    for num in &nums {
        // println!("{}", num); OK too
        println!("{}", *num);
    }

    // Consumers
    ////////////
    // - Consumers operate on an Iterator returning value/values
    // - collect() on Iterator takes value/values returning 
    //   a Collection of results

    // collect() - is a Consumer
    ////////////

    // ::<> syntax is for Type Hints (we want Vector of Integers)
    // The Consumer (i.e. collect::) uses the Range Iterator to
    // generate a sequence
    let one_to_three = (1..3).collect::<Vec<i32>>();
    // outputs [1, 2]
    println!("one_to_three from: {:?}", one_to_three);

    // ::<_> syntax is for Partial Type (Placeholder) Hints
    // (for collecting into Vec<T> an inferring the type T)
    let one_to_three_using_inferred = (1..3).collect::<Vec<_>>();
    // outputs [1, 2]
    println!("one_to_three_using_inferred from: {:?}", one_to_three_using_inferred);

    // find() - is a Consumer
    /////////

    // find() takes a Closure and uses Reference to each element of 
    // an Iterator. Closure returns 'true' if match found else 'false'.
    // find() returns an Option since it may not find a matching element.
    let greater_than_ninety_five = (0..100)
                                .find(|x| *x > 95);

    match greater_than_ninety_five {
        Some(_) => println!("Detected numbers over 95"),
        None => println!("No numbers over 95 detected"),
    }

    // fold() - is a Consumer (use when given List and want Single result)
    /////////

    // http://doc.rust-lang.org/book/iterators.html

    // fold() takes the form:
    // fold(base, |accumulator, element| ...)
    //
    // fold() takes two arguments:
    // - 1st arg  - base - 
    // - 2nd arg (Closure taking two arguments)
    //            - accumulator - 
    //            - element - 

    // 1st Iteration -  Base is Accumulator's Value. 
    //                  Result stored in Accumulator on next Iteration
    // 2nd+ Iteration - Result of 1st Iteration is Accumulator's Value
    //
    // Note: Closure called each Iteration with result stored in Accumulator
    // on next Iteration
    //
    // Note: Iterators are 'lazy' as they do not need to generate all values upfront
    // since the Range simply creates a Value that Represents the sequence (instead
    // of actually generating a range of numbers i.e. (1..4) )
    let sum = (1..4)
            .fold(
                0,               // Base
                |sum, x| sum + x // Closure
            );
    println!("fold() Consumer returns single result given a list: {}", sum);

    // iter() - is a basic Iterator (similar to the Range Iterator)
    /////////
    // iter() may convert a Vector into an Iterator returning each element
    let nums = [1, 2];

    for num in nums.iter() {
        println!("iter() Iterator prints numbers in turn: {}", num);
    }

    // count() - is an Infinite Iterator
    // std::iter::count
    //////////

    // count() counts up from the value of the 1st argument until the maximum 
    // number represented by i32, adding the value of the 2nd argument each time
    // to the previous total count

    // Creates an Iterator that maps each element to an iterator, and 
    // yields the elements of the produced iterators
    let smalls = [2u, 3];
    let bigs = [0u, 1, 0, 1, 2, 3];
    let mut my_iterator = smalls.iter().flat_map(|&small| count(0u, 1).take(small));
    // Check that 'my_iterator' has the same elements as 'bigs'
    let mut i = 0;
    for iteration in my_iterator {
        // assert_eq!(iteration, bigs[i]);
        println!("assert_eq! with iteration: {} and bigs[i]: {}, is: {:?}", iteration, bigs[i], assert_eq!(iteration, bigs[i]) );
        i += 1;
    }

    // Iterator Adapters
    ////////////////////
    //
    // Iterator Adapters - take an Iterator and modify it to produce a New Iterator
    // Iterator Adapters are Lazy and do nothing unless Consumed.

    // map() - is an Iterator Adapter
    // map() is called on another Iterator (i.e. a Range Iterator) and produces a
    // New Iterator where the Closure given as an argument to map() is
    // called on each Element Reference. Since Iterator Adapters are Lazy the
    // Closure below would never execute. Using println!("{}", x) instead of x + 1 will
    // not print any numbers. Instead use a 'for'
    (1..100).map(|x| x + 1);

    // 'for' is used to execute a Closure on an Iterator for its Side-Effects
    // take(n) - returns an Iterator over next 'n' elements of the given Original Iterator
    //           but has No Side-Effect on the given Original Iterator
    //           (i.e. use it to constrain to only 5 iterations)

    // using the Infinite Iterator imported from std::iter::count
    for i in std::iter::count(2, 7).take(5) {
        println!("{}", i);
    }

    // filter() - is an Iterator Adaptor
    // filter() - takes a Closure as an argument that returns 'true' or 'false'. The New
    // Iterator filter() below produces only elements that the Closure returns 'true' for

    // (prints all even numbers b/w 1 and 100)
    // filter() does Not Consume elements it Iterates over, it is passed a reference to
    // each element through the filter() predicate (using &x patter to extract the
    // integer value itself)
    for i in (1..30).filter(|&x| x % 7 == 0) {
        println!("Modulus of 7 between 1 and 30 is: {}", i);
    }

    // Iterator Chaining
    (1..1000)                       // Iterator (Range)
        .filter(|&x| x % 2 == 0)    // Iterator Adapter (adapts the Iterator)
        .filter(|&x| x % 3 == 0)    // As above
        .take(5)
        .collect::<Vec<i32>>();     // Consume (the result)


}

// Generics are called Parametric Polymorphism in Type Theory.
// Generics are Types or Functions having Multiple (Poly) Forms (Morphs)
// over a given Parameter (Parametric).
// Generics empower developers to configure a Function or
// Data Type to work for Multiple Types of Arguments (i.e. both i32 and f64).
fn try_generics() {

    // Generic Form of OptionalInt declaration in Rust
    // where <T> means it is a Generic Data Type.
    // Within the 'enum' we substitute the a given type for T
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // Option<T> with additional Type Annotations.
    // The Variable Binding uses a Single Generic Definition for Multiple uses across Types.
    // Left-Side of Binding - Option<i32> in Type Declaration, T is value i32.
    // Right-Side of Binding - make Some(T) where T is the value 5 (which is i32)
    // Both sides must match to prevent a mismatched types error
    let my_var_binding_with_generics1: Option<i32> = Some(5);
    let my_var_binding_with_generics2: Option<f64> = Some(5.0f64);

    // Generic Form of Rust's built-in Result<T, E> type is generic over 
    // Multiple Types T and E, and is used to return the result of a computation or error.
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // Success returns f64. Failure returns String.
    let my_var_binding_with_generics3: Result<f64, String> = Ok(2.3f64);
    let my_var_binding_with_generics4: Result<f64, String> = Err("Error occured.".to_string());

    // Function that uses Rust's built-in Result<T, E> type
    fn inverse_64(input_param: f64) -> Result<f64, String> {
        // Check that input parameter is non-zero to prevent taking its inverse
        if input_param == 0.0f64 { return Err("input_param cannot be zero!".to_string()); }

        // With result wrapped in an Ok, we must perform the Match before using the result
        Ok(1.0f64 / input_param)
    }

    // Problem with the inverse64 Function is that it only works for 64bit Floating Point values
    // Without leveraging a Generic Function, we would have to write an additional 'inverse_32' Function
    // fn inverse_32(input_param: f32) -> Result<f32, String> {
    //     if input_param == 0.0f32 { return Err("input_param cannot be zero!".to_string()); }

    //     Ok(1.0f32 / input_param)
    // }

    // Exhaustive Match's using the 'inverse' Function
    let my_var_binding = inverse_64(25.0f64);

    // Match enforces that we handle the Err case
    match my_var_binding {
        Ok(my_var_binding) => println!("Inverse of 25 is {}", my_var_binding),
        Err(msg) => println!("Error: {}", msg),
    }

    // Generic Function combined with Traits is required to Combine the 
    // 'inverse_64' and 'inverse_32' Functions.
    // Traits must also be used since T is too generic and can be any type, and so 
    // may not implement ==.
    // Traits must be used to add a Trait Constraint to the Generic T to ensure that it
    // implements ==, since Traits define Function Type Signatures we can be sure any Type that 
    // implements HasEquals will have ==.
    // Trait Bounds may be used, specifically Float, and replacing the generic 0.0 and 1.0 with
    // methods from Float Trait. Both f32 and f64 implement Float (std::num::Float)
    // Trait Bounds use Monomorphization (mono: one, morph: form) and so
    // they are Statically Dispatched
    fn inverse_64_and_32_generic<T: Float>(input_param: T) -> Result<T, String> {
        if input_param == Float::zero() { return Err("input_param cannot be zero!".to_string()); }

        let one: T = Float::one();
        Ok(one / input_param)
    }

    println!("Inverse of {} is {:?}", 2.0f32, inverse_64_and_32_generic(2.0f32));
    println!("Inverse of {} is {:?}", 2.0f64, inverse_64_and_32_generic(2.0f64));

    println!("Inverse of {} is {:?}", 0.0f32, inverse_64_and_32_generic(0.0f32));
    println!("Inverse of {} is {:?}", 0.0f64, inverse_64_and_32_generic(0.0f64));

}

// Traits are defined similar to the 'impl' keyword that is used to call a function
// with method syntax (implementing a 'struct')
// 'trait' blocks are similar to the 'impl' block (however only a Type Signature is
// defined Without a Body) and 'impl Trait for Item' is used rather than 'impl Item'
// Rule: Traits must be used in any scope where the Trait method is used.
// Rule: Trait or the type the 'impl' of the trait is being written for must be inside the crate
fn try_traits() {

    // uses code in /src/shapes/shapes.rs

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

// Traits have a 'where' clause that overcomes complex amount of Trait Bounds
// (which appear between the Function on the far left and its Parameter List on the far right)
fn try_traits_with_where_clause() {
    use std::fmt::Debug;

    // Traits (WITHOUT 'where' clause)
    fn my_trait_without_where_clause<T: Clone, K: Clone + Debug>(x: T, y: K) {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }

    // Traits (WITH 'where' clause)
    fn my_trait_with_where_clause<T, K>(x: T, y: K) 
        where T: Clone, 
              K: Clone + Debug {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }

    my_trait_without_where_clause("Hello", "world");
    my_trait_with_where_clause("Hello", "workd");

    // Traits WITH 'where' clauses are more powerful than simpler syntax since
    // they allow Trait Bounds where the left-hand side is an arbitrary type (i.e. i32)
    // rather than just a plain type parameter like T
    trait ConvertTo<Output> {
        fn convert(&self) -> Output;
    }

    impl ConvertTo<i64> for i32 {
        fn convert(&self) -> i64 { *self as i64 }
    }

    // Called with T == i32
    fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
        x.convert()
    }

    // Called with T == i64
    fn inverse<T>() -> T
        // this is using ConvertTo as if it were "ConvertFrom<i32>"
        where i32: ConvertTo<T> {
        1i32.convert()
    }

    println!("inverse() is: {:?}", inverse() );

}

// Dispatch Dfn - mechanism to determine version to be run for Polymorphic code 
// Dispatch Forms -
// - Static Dispatch (preferred in Rust)
// - Dynamic Dispatch (use with Trait Objects)
fn try_static_dispatch() {

    // Trait and associated Implementation required
    // Purpose is to return a String from a given input
    trait TraitForDispatch {
        fn method(&self) -> String;
    }

    // Implement Trait called TraitForDispatch for u8
    impl TraitForDispatch for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    // Implement Trait called TraitForDispatch for String
    impl TraitForDispatch for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // Static Dispatch with Trait Bounds used with the Trait called TraitForDispatch.
    //
    // Note: Monomorphization performed by Rust whereby it generates a special function
    // (i.e. http://doc.rust-lang.org/book/static-and-dynamic-dispatch.html)
    // version of processTraitForDispatch method for each of u8 and String
    // and replaces the call sites with calls to the respective special version function.
    //
    // Benefits & Trade-Offs: 
    // - Static Dispatch allows function calls to be inlined since callee 
    //   known at compile time, which is good for optimization
    // - Fast and Flexible
    // - Code Bloat due to copies of the same function being generated by Rust and
    //   existing in the binary for each Type
    // - #[inline] and #[inline(always)] must be used carefully
    // - Efficiency higher for Static Dispatch Wrapper Function that does Dynamic Dispatch
    fn process_trait_for_dispatch<T: TraitForDispatch>(input_param: T) {
        input_param.method();
    }

    let input_u8 = 5u8;
    let input_str = "Hello".to_string();

    process_trait_for_dispatch(input_u8);
    process_trait_for_dispatch(input_str);

    // Retrieve Trait Object's Value out of a pointer to T (using Casts or Coercions)
    // where T is type (i.e. u8) that implements the Trait called TraitForDispatch
    // Coercions and Casts are identical and also work for Pointers (i.e. 
    // &mut T, &mut TraitForDispatch, Box<T>, and Box<TraitForDispatch>).
    // Coercion and Cast operations are effectively "Type Erasure" since they are 
    // erasing the compiler's knowledge about a specific type of pointer
    // let ref_to_trait: &T = ...;

    // Approach #1 (Casts) - 'as' keyword for casting
    // let cast = ref_to_trait as &TraitForDispatch;

    // Approach #2 (Coercion) - using a '&T' in a place that has a known type of '&TraitForDispatch' will implicitly coerce:
    // let coerce: &TraitForDispatch = ref_to_trait;

    // fn also_coerce(_unused: &TraitForDispatch) {}
    // also_coerce(ref_to_trait);

}

// Dynamic Dispatch used with Trait Objects (i.e. &Foo or Box<Foo>).
// - Trait Objects are values storing a value of any Type that Implements given Trait,
// - Trait Object's Types may only be known at run time.
// - Trait Object's methods are called by a record of function pointers (compiler's responsibility)
// - Implementing a Function (Copy generated) taking a Trait Object is specific to 
//   Types that implement the Trait for less Code Bloat but at cost of flower virtual
//   function calls and inhibiting inlining opportunities and optimizations.

fn try_dynamic_dispatch() {

    // Runtime Representation
    // - Trait Objects have a Runtime Representation of Trait Objects
    //   that is defined in std::raw Module as Structs with layouts
    //   similar to complicated built-in Types
    //
    // Runtime Representation Example:
    // - Trait Objects contain:
    //   - "data" Pointer - points to data (of unknown type T) stored 
    //     by Trait Object
    //   - "vtable" Pointer - points to Virtual Method Table of 
    //     associated Implementation of the Trait Object for T
    //     "vtable" is Struct of Function Pointers pointing to
    //     machine code for each Implementation Method to allow
    //     retrieval of correct pointers from "vtable" and subsequent
    //     dynamic calls with it.
    //   - "destructor" fields in each "vtable" points to Function that
    //     cleans up any resources (i.e. memory allocation, internal type) 
    //     of the "vtable" type for owning Trait Objects like Box<Foo>
    //     for when they go out of scope
    //   - "size" field stores the size of the erased type
    //   - "align" field stores the alignment requirements (not currently used)
    //
    // pub struct TraitObject {
    //     pub data: *mut (),
    //     pub vtable: *mut (),
    // }
    //
    // Note: For details see http://doc.rust-lang.org/book/static-and-dynamic-dispatch.html
    //
    // Benefits:
    //   - Pointers used for storing values when Types may be arbitrarily large
    //     and allows dependent crates to implement the Trait Object
    //   - Pointers with values behind them means only the size of the pointer
    //     is relevant, but the size of the value is not relevant to the Trait 
    //     Object being used
    //   - "Fat Pointer" implies a Trait Object is always a Pointer
    //   - Pointers overcome not knowing the size of the value at
    //     compile-time, which is important for passing as Function 
    //     argument, storing and manipulating it on the Stack and
    //     allocating and deallocating memory on the Heap for it.
}

// Summary of Rust Tools and Semantic Structure for Abstraction and Reuse, and the role of Macros
// - Functions have a Type Signature
// - Type Parameters have Trait Bounds
// - Overloaded Functions must belong to a Trait
// - Powerful Compile-Time correctness checking
// - Macro invocations allow Syntactic Level abstractions (expanded syntactic forms) that
//   occur early in compile-time before static checking and may therefore capture
//   patterns of code reuse that Rust's core abstractions cannot, which overcomes any 
//   inflexibility as it allows ease of expressing a visually identified
//   pattern in repeated code without reliance solely on Generic Functions or Traits
// - Macros offer truly concise and well-abstracted code
// - Macros should be designed to be well-behaved without having to understand their implementation
// - Macros should be designed to explicitly inform developers of compile-time errors since 
//   associated errors are difficult to interpret otherwise as they describe problems in expanded code
//   rather than source-level form (used by developers)

fn try_macros() {

    // vec! Macro (initialises a Vector with any number of elements)
    // Macro Name written informally as vec!
    // let x: Vec<u32> = vec![1, 2, 3];

    // Imagined in Syntactic Shorthand of vec! Macro
    let x: Vec<u32> = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        println!("temp_vec before is: {:?}", temp_vec);
        temp_vec
    };

    println!("x before is: {:?}", x);

    // Imagined implementation of Syntactic Shorthand of vec! Macro

    // Macro Name is "vec" using invocation syntax to distinguish a Macro from an ordinary function
    macro_rules! vec {
        // Macro Rules defined (Pattern Matching cases similar to 'match' expression)
        // however the matching occurs on Rust syntax trees compile-time.
        // Format: 
        // (  <matcher>:<identifier> ) => { <expanded_syntax_block_w_multiple_statements>
            // Special Matcher Syntax - given below (i.e. $x:expr) matches any Rust expression binding that syntax tree 
            // to Metavariable $x
            // Metavariable - $x 
            // (each Metavariable must be under at least as many $(...)* as it was matched against)
            // Identifier (Fragment Specifier) - "expr"
            // RegEx Wrapper for Macro Expansion - $( ).* matches "zero or more" expressions separated by commas
            // representing one "Layer" of repetitions to walk through in "lockstep" for all 
            // metavariables (i.e. $x) it contains.
            // $( )+ performs "one or more" match
        // Combine the Two Pairs of braces (cleaner code).
        // Macro's may be declared to achieve Macro Expansion for different Context 
        // (i.e. Shorthand for Data Type valid for say either Expression or Pattern)
        ( $( $x:expr ),* ) => {{
            let mut temp_vec = Vec::new();

            // Expansion (spliced syntax captured by the matcher)
            // - Each matched expression $x produces single 'push' statement
            //   in Macro Expansion
            // - Repetition in Macro Expansion proceeds in "lockstep" with
            //   repetition in the matcher
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }}; // semi-colon is optional
    }

    macro_rules! vec2 {
        // Special Matcher Syntax takes Rust Tokens that must match exactly
        (x => $e:expr) => (println!("mode X: {}", $e));
        (y => $e:expr) => (println!("mode Y: {}", $e));
    }

    vec2!(y => 3); // outputs "mode Y: 3"

    // Macro example with duplication of variables from outer
    // Repetition levels (with matcher syntax)
    macro_rules! o_O {
        (
            $(
                $x:expr; [ $( $y:expr ),* ]
            );*
        ) => {
            &[ $($( $x + $y ),*),* ]
        }
    }

    let a: &[i32]
        = o_O!( 10; [1, 2, 3];
                20; [4, 5, 6]);

    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    // Macro Hygiene
    // - Metavariable $x is parsed as Single Expression Node
    //   maintaining its position in syntax Tree even after substitution
    // - Rust avoids errors due to text substitution resulting in removal of brackets
    macro_rules! five_times {
        ($x:expr) => (5 * $x);
    }

    assert_eq!(25, five_times!(2 + 3));

    // Macro Systems Variable Capture (i.e. injection and expansion)
    // - Rust uses its Hygienic Macro System where:
    //   - Each Macro Expansion occurs in distinct Syntax Context
    //   - Each Variable is tagged with the Syntax Context where introduced
    // - Rust avoids errors due to variable shadowing
    // - Rust ensures that the 'state' variable in 'let state' does not conflict
    //   with the 'let state' defined within the Macro statement
    // - Variable bindings names outside the Macro must be passed into the
    //   Macro invocation to tag it with the correct Syntax Context
    macro_rules! log {
        ($msg:expr) => {{
            let state: i32 = 1; //get_log_state();
            if state > 0 {
                println!("log({}): {}", state, $msg);
            }
        }};
    }

    let state: &str = "reticulating splines";
    log!(state);

    // Macro (Basic 'let' binding)
    macro_rules! get_age {
        ($age:ident) => (let $age = 33);
    }

    get_age!(age);
    println!("{}", age);

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