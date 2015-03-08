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

// Import the num Module (dependency added to Cargo.toml)
extern crate num;
use num::bigint::{ToBigInt, RandBigInt};

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
        println!("Enter your name...");

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
  age: num::bigint::BigInt,
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

struct Star {
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
        age: 13800000000.to_bigint().unwrap(),
        billion: 1000000000,
    });

    // Use the previously included 'box_syntax' feature gate syntax (flexibility without losing performance)
    // The value returned from the process_big_data_structure() function is boxed.
    // No copying is performed, instead the try_reference_counted_boxes() function block allocates enough 
    // memory for the box, passes a pointer to that memory into the process_big_data_structure() function
    // as as my_universe1, and then process_big_data_structure() writes the value straight into the Box<T>.
    let generated_big_universe: Box<BigUniverse> = box process_big_data_structure(my_universe1);

    println!("My Big Universe is {} years old", generated_big_universe.age);

    // Create a Reference-Counted Galaxy (Owner)
    let my_galaxy1 : Rc<Galaxy> = Rc::new(
        Galaxy { 
            name: "Milky Way".to_string(), // String::from_str("Milky Way")
            planets: RefCell::new(Vec::new())
        }
    );

    // Create Planets and Stars belonging to the Galaxy (Owner)
    // Increment the Reference-Count by cloning the Rc<T> object
    let earth = Planet { id: 1, owner: my_galaxy1.clone() };
    let mars = Planet { id: 2, owner: my_galaxy1.clone() };

    // Add different planets (i.e. Earth, Mars) to the Galaxy by
    // mutably borrowing from RefCell, since Galaxy's "planets"
    // property holds its planets
    // my_galaxy1.planets.borrow_mut().push(earth.clone().downgrade());
    // my_galaxy1.planets.borrow_mut().push(mars.clone().downgrade());

    // Iterate over the planets in a galaxy and print to stdout
    // Since planet_opt is a Weak Reference (i.e. Weak<Planet>) it cannot 
    // guarantee its objects are still allocated, so we must turn them into 
    // Strong References by calling upgrade() on them, which in turn returns
    // Option, containing a reference to our object if it still exists
    for planet_opt in my_galaxy1.planets.borrow().iter() {
        let planet = planet_opt.upgrade().unwrap();
        println!("Planet {} owned by {}", planet.id, planet.owner.name);
    }

    // Drops only the my_galaxy1 (owner) Reference-Count object (not the Galaxy it wraps)
    // Galaxy it wraps remains allocated whilst other Rc<T> objects still point to the Rc<T> wrapper
    drop(my_galaxy1);

    println!("Earth {} owned by {}", earth.id, earth.owner.name);
    println!("Mars {} owned by {}", mars.id, mars.owner.name);

    // End of method causes destruction of: my_galaxy1, earth, mars 
    // so there are no more Strong References (i.e. Rc<T>) to the planets
    // After these are destroyed then Planets gets destroyed, causing the Reference-Count
    // on Milky Way to become zero, so Milky Way gets destroyed
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