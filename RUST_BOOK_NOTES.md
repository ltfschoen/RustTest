## DEPENDENCIES

* Import type into scope of program if not already in the **prelude**.

* External dependency `extern crate rand` is equivalent of calling `use rand`. Prefix with `rand::` to use rand library.

* Statement used to import I/O **crate** for interaction from terminal

* Statement used to import the `Ordering` **enum** and associated **variant** outcomes of comparison `Less`, `Greater`, `Equal`

* Statement to import Rng **trait** to the **scope** since it defines random number generator methods including `thread_rng` that is local to current **thread of execution** and seeded by the operating system. `thread_rng().gen_range(<exclusive>, <inclusive>)`

## VARIABLES AND CONSTANTS

* Create **mutable** variable (since variables are by **default immutable**) as intent to allow other parts of the code to change it and bind to a value that is a new instance of say a string type using an **associated function (aka static method)** of the String type from the standard library.

* **Constants** declared with `const` are always immutable, cannot be converted with `mut` (whereas variables declared with `let` can be), must be annotated with a type, declared with capitals, can only be set to a constant expression computed at compile time (not the result of a function call or a value computed at runtime), are valid during program execution in the scope they were declared, and are best stored in a single place for update and maintenance.

* Immutable variables provide **safety** and easy of **concurrency** since the Rust compiler guarantees its value will not change, which reduces state management and makes code easier to reason about.

* Trade-off between performance and code clarity. Using mutable variables may make code easier to write, but immutability may result in less bugs.

* **Shadow** previously declared mutable value of `guess` for reuse then perform transformations on its value redeclaring it as a a new immutable variable with the same name using `let` and with new value but even of a different type. Shadowing saves us coming up with new variable names (i.e. myvar instead of say myvar_str, myvar_num, etc). It is different to marking a variable as `mut`.
* Example: **Bind** `guess` to expression `guess.trim().parse()`. trim() removes whitespace and newline characters. parse() will parse a string into a number.

## TYPE ANNOTATION & INFERENCE

* Type annotation of variables with a colon `:` used when many types possible

* Rust infers as say being a string since it has **type inference**.

## TYPES

* Rust is **statically typed** so must know variable types at compile time

* Compiler may infer the type based on value used

* Multiple number types may have value between 1 and 100, where default is `i32` (32-bit number), and others are `u32` (unsigned 32-bit number for small positive numbers) and `i64` (64-bit number).

### DATA TYPES

#### SCALAR

* **Integer types**
    * Unsigned 32-bit int: `u8` (0 to 2^n-1, 0 to 255), `u16`, `u32`, `u64` (length 64-bit), `usize` (length `arch` depending on whether computer is 32-bit or 64-bit)
    * Signed 32-bit int: `i8` (-2^7 to 2^7-1, -128 to 127), `i16`, `i32`, `i64`, `isize` (used when indexing a collection)
        * Stored using Twos Complement representation

    * Note:
        * All number literals except byte literal allow:
            * Type suffix: i.e. `57u8` (57 of type u8)
            * Visual separator: i.e. 98_222 (i.e. 98222)
        * Integer types default to `i32` (fastest even on 64-bit systems)

* **Floating-point types**
    * Primitive types
        * `f32` is single precision float
        * `f64` is double precision float
    * Note:
        * `f64` is default, same speed as `f32` but more precision on new CPUs)
    * Example:
        ```rust
        let x: 2.0;       // f64 default
        let y: f32 = 3.0; // f32
        ```
* **Numeric operations** (see Appendix B of Rust Book)
    * `+`, `-`, `*`, `/`, `%` (remainder)
    * Example: `let remainder: 43 % 5;`

* **Boolean**
    * `bool` (`true` or `false`)
    * Example: `let f: bool = false;`

* **Characters**
    * `char` is Unicode Scalar Value from U+0000 to U+D7FF and U+E000 to U+10FFFF (more than just ASCII)
    * Note:
        * `char` specified using **single quotes**, whereas strings specified with double quotes)
    * Example: `let z_letter = 'z';`

### COMPOUND

* Note:
    * Compound types group multiple values of other types into single type

* **Tuples**
    * Access individual values from a tuple:
        * Pattern matching to destructure a tuple value
        * Directly using a period `.` followed by index of value to access

    * Usage: Use for grouping values of a variety of types into single compound element type
    * Type annotations are optional
    * Example:
        ```rust
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let (x, y, z) = tup;      // destructuring
        let five_hundred = tup.0; // access first index in tuple
        println!("The value of y is: {}", y);
        ```

* **Arrays**
    * Usage:
        * Allocates data on the stack instead of the heap
        * Use when only need fixed number of elements
        * Rust panics (immediately exits program with error)
        if access index greater than array length
        * Vector collection type allows grow or shrink
    * Example:
        ```rust
        let months = ["Jan", "Feb", "Mar"];
        let feb = months[1];
        ```

    * **Array Slices**
        * Refer to part of an array by taking a slice of type `&[i32]`
        that stores a reference to the first element and a length.
        ```rust
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        ```

## COMPLEX

* **String Literal**
    * **Immutable** string value that is hard-coded
    * Pros:
        * Fast and efficient since immutable and contents known in executable
        at compile time
    * Cons:
        * Does not cater for string values that are not known at compile time

* **Strings**
    * `String` type may be used to take user input and store it on the **heap**
    * Supports **Mutable** (growable text) by allocating memory on the **heap**
    to hold the data that is unknown at compile time, so memory is requested from
    operating system at runtime (i.e. when we call `String::from`).
    **Rust uses a pattern of automatically returning memory to the operating
    system after the variable that owns it goes out of scope** by calling the
    `drop` function, similar to Resource Acquisition Is Initialization (RAII)
    in C++.
    * Pros:
        * Allows for using string values that are not known at compile time
        * Mutation of `String` type by appending a String Literal
            ```rust
            let mut s = String::from("hello");
            s.push_str(", world!");
            ```

    * **String Slices**
        * See book/second-edition/ch04-03-slices.html

        * String Literals are Slices
            * Slice pointing to specific point in binary where `&str` is immutable reference

            ```rust
            let s: &str = "Hello, world!";
            println!("{}", s);
            ```

* **Structs**

    * Defining: `struct` used to define a grouping of **fields** (names and types of the pieces of data)

    * Instantiating: State the name of the defined Struct and provide Key/Value pairs for each field

    * Accessing Fields of a Struct Instance: Use dot notation
        * Note: We cannot mark only certain fields as mutable
        * Example:
            ```rust
            let mut user1 = User {
                email: String::from("a@a.com")
            };
            println!("User email is: {}: ", user1.email);
            ```

    * Modifying Fields of a "mutable" Struct Instance
        * Example 1:
            ```rust
            user1.email = String::from("b@b.com");
            ```

        * Example 2: Implicitly return new instance of Struct from Function body
        taking some function arguments and translating them into fields of the Struct
            ```rust
            fn build_user(email: String, username: String) -> User {
                User {
                    email: email,
                    username: username,
                    active: true,
                    sign_in_count: 1,
                }
            }
            ```

    * Remaining Fields Populated using `..` syntax:
        * Example:
            ```
            let user2 = User {
                email: String::from("another@example.com"),
                username: String::from("anotherusername567"),
                ..user1
            };
            ```

    * **Methods**
        * Differ from **Functions** since they are defined in the context
        of a Struct (or Enum or Trait object), and their first parameter
        is `self` to represent the Instance of the Struct that the Method
        is being called on.
        * Multiple `impl` blocks of the same Struct may be used to house the various
        Methods and Associated Functions
        * Pros:
            * Organises capabilities of a Struct instance type into a `impl` block

        * Example 1: Function
            ```rust
            #[derive(Debug)]
            struct Rectangle {
                width: u32,
                height: u32
            }

            fn main() {
                let rect1 = Rectangle { width: 30, height: 50 };

                println!("rect1 is {:?}", rect1);

                println!(
                    "The area of the rectangle is {} square pixels.",
                    area(&rect1)
                );
            }

            fn area(rectangle: &Rectangle) -> u32 {
                rectangle.width * rectangle.height
            }
            ```

        * Example 2: Method (refactored to use Method instead of Function)
            * If we are only Reading from the Struct data we use `&self`
            * If we want to Write to the Struct instance we use `&mut self` as first parameter
            ```rust
            #[derive(Debug)]
            struct Rectangle {
                width: u32,
                height: u32
            }

            impl Rectangle {
                fn area(&self) -> u32 {
                    self.width * self.height
                }
            }

            fn main() {
                let rect1 = Rectangle { width: 30, height: 50 };

                println!("rect1 is {:?}", rect1);

                println!(
                    "The area of the rectangle is {} square pixels.",
                    rect1.area()
                );
            }
            ```

    * **Associated Functions** (i.e. Static Functions of a Struct that do not require a Struct Instance)
        * Do not require `self` since do not require an instance of the Struct, so are not methods
        * Namespaced by the Struct so call with say `let sq = Rectangle::square(3);`

        * Examples 1: `String::from`

        * Example 2:
            * See `fn square` in projects/shapes/src/main.rs

    * **Tuple Structs**
        * Structs without labels. See book/second-edition/ch05-01-defining-structs.html#tuple-structs-without-named-fields-to-create-different-types

    * **Ownership of Struct Data**

        * Use the "owned" `String` type rather than the `&str` String Slice type
        if want instances of the Struct to own all of its data and for that data
        to be valid for as long as the entire Struct is valid.

        * **Store References in Structs using Lifetime Specifiers**
            * Structs may store "references" to data that is "owned" by something else
            through use of "lifetimes" that ensure the data "referenced" by the
            Struct is valid as long as the Struct is
                ```
                struct User {
                    username: &str,
                    email: &str,
                    ...
                ```

* **Enums and Pattern Matching**

    * Definition:
        * Allow define a type by enumerating its possible values
        * Allows encoding meaning along with data
        * `Option` enum expresses that a value can be something or nothing
        (since Rust does not have nulls), so the compiler
        can check that we have handled all cases we should be handling to prevent bugs.
        It is included in the Prelude automatically. The `<T>` syntax is a generic type parameter,
        and means the `Some` variant of the `Option`enum can hold one piece of data of any type.
            ```
            enum Option<T> {
                Some(T),
                None,
            }
            ```
            * In order to have a value that can possibly be null, you must explicitly opt in
             by making the type of that value `Option<T>`. Then, when you use that value,
             you are required to explicitly handle the case when the value is null.
             Whenever a value has a type that isn't an `Option<T>`, you can safely assume
             that the value isn't null
            * Note: Must convert `Option<T>` to a `T` before can perform `T` operations with it
        * Pattern Matching in `match` expression allows running different code
        for differernt enum values
        * `if let` for handling enums conveniently is syntactic sugar (less code than using `match`)
        * Enum values can only be one of the variants
        * Enums may contain anonymous structs as data
        * Enums may define embedded methods using `impl`
        * Enums may be used to create Custom Data Types and instances of them

    * See book/second-edition/ch06-01-defining-an-enum.html
    * See projects/users/src/main.rs
    * Reference on Options: https://medium.com/adventures-in-rust/deal-with-it-option-type-in-rust-4246e1dd9e47

    * **`match` Control Flow Operator**

        * Compare value against series of patterns and execute code based on match
        * Compiler checks all possible cases are handled since Rust matches are **exhaustive**

* **Collections**

    * Definition
        * Contain multiple values
        * Collections point to data stored in the **heap** (differing from Arrays and Tuples)
            * Data does not need to be known at compile time and may grow/shrink during runtime
        * Collections each have different capabilities and **costs**
        * See Collections in the documentation for more collection types

    * **Vectors**
        * Definition: Allows storing multiple number of values of the **same type**
        in single data structure and **stores them next to each other in memory**.

        If we know the exhaustive set of types we want at runtime to store as a list of items
        then we can store those **different types** by defining an **enum type** and
        then storing elements of different types within it, and then store the enum in the
        vector.

        If we **don't know the different types** prior to runtime then we'd use a **Trait Object**.
            * Benefit: Using an Enum means we can be explicit about what types are allowed in the
            vector, and avoids errors since when operations performed on elements of the vector
            when using the Enum plus a `match` expression then Rust will ensure at compile time all
            cases are handled correctly. Rust will know what types will be in
            the vector at compile time and how much memory it needs on the heap to store each
            element
        * Memory:
            * When a vector goes out of scope the vector and its elements are freed
        * Usage: List of items like lines in a text file, or prices of items in shopping cart
        * Examples:
            * Creating Vectors
                * Empty with Type Annotation
                    * Create **empty** Vector (using a **type annotation** since no values inserted yet)
                    to hold elements of type `i32` type, where `Vec<T>` type from standard library
                    holds **any** type.
                        ```
                        let v: Vec<i32> = Vec::new();
                        ```
                * Pre-Populated with Inferred Type
                    * Create a Vector with **initial values** using **vec!** macro,
                    where Rust infers the type as being `Vec<i32>`:
                        ```
                        let v = vec![1, 2, 3];
                        ```

                    * Using an Enum to store different types in a vector:
                        ```
                        enum SpreadsheetCell {
                            Int(i32),
                            Float(f64),
                            Text(String),
                        }

                        let row = vec![
                            SpreadsheetCell::Int(3),
                            SpreadsheetCell::Text(String::from("blue")),
                            SpreadsheetCell::Float(10.12),
                        ];
                        ```

            * Updating Elements of a Vector
                ```
                {
                    let mut v = Vec::new();
                    v.push(5);
                }
                ```

            * Reading Elements of a Vector
                * Option 1: Indexing Syntax
                    * Usage: Accessing out of bounds causes
                    behaviour where program to `panic!` and crash program
                    ```
                    let v = vec![1, 2, 3, 4, 5];
                    let does_not_exist = &v[100];
                    let does_not_exist = v.get(100);
                    ```

                * Option 2: `get` Method that returns `Option<&T>`
                    * Usage: Accessing out of bounds returns `None` without
                    panicking. Then use logic to handle either `Some(&element)`
                    or `None` with feedback to CLI/UI so more user friendly
                    ```
                    let v = vec![1, 2, 3, 4, 5];
                    let *third: Option<&i32> = v.get(100);

                    match *third {
                        Some(x) => { println!("Reachable element {}", x); () },
                        None => { println!("Unreachable element"); }
                    }
                    ```

            * Modifying Elements of a Vector
                * Note we **cannot** add an element to a Vector whilst holding a
                reference to an element as shown below.
                    ```rust
                    let first = &v[0];

                    v.push(6);
                    ```

                * We can iterate over mutable references to each element in a mutable vector
                    ```rust
                    let mut v = vec![100, 32, 57];
                    for i in &mut v {
                        *i += 50;
                    }
                    ```
                    * Note: To change the value that the mutable reference refers to,
                    we have to use the dereference operator (`*`) to get to the value
                    in `i` before we can use the `+=` operator .

            * Iterating over Elements in a Vector
                ```rust
                let v = vec![100, 32, 57];

                // Get immutable references to each element in vector of i32 values
                for i in &v {
                    println!("{}", i);
                }
                ```


    * **String**

    * **Hash Map**
        * Definition: Allows association of a value with a key

## TYPE CONVERSION

* Convert String Literal to `String`
    * Use the `from` function
    * Example
        ```rust
        let mut s = String::from("hello");
        ```

* Convert `String` into array of bytes
    ```rust
    let bytes = s.as_bytes();
    ```

## FUNCTIONS

* Functions may be defined before or after where they are called

* Example
    ```rust
    fn main() {
        another_function(5, 6);
    }

    fn another_function(x: i32, y: i32) {
        println!("Another function with parameters x and y is: {}, {}", x, y);
    }
    ```

### FUNCTION AUTOMATIC REFERENCING AND DEREFERENCING

* Rust uses **automatic referencing and dereferencing** when calling a method.
Rust automatically adds in `&`, `&mut`, or `*` so the `object` matches the method signature.
Since methods have a name and a receiver of type `self`, when a call to a method is made and
these are given, Rust determines if the method is reading `&self`, mutating `&mut self` or
consuming `self`. Rust makes "borrowing" implicity for method receivers.

* Example: Both lines are equivalent
    ```rust
    p1.distance(&p2);
    (&p1).distance(&p2);
    ```

### FUNCTION PARAMETERS

* **Parameter type annotations** must be declared in function signature

* Note that `_` is catch all of all values (say for function arguments ).
    * See book/second-edition/ch06-02-match.html#the-_-placeholder

* Call associated function with say: `std::io::stdin()` to return instance of terminal standard input `std::io::Stdin`. Call `readline` method on standard input handle to obtain user input passing a single mutable reference argument to change the string content with the user input (allow safe and easy access to data without having to copy it to memory multiple times).

### FUNCTION BODY

* **Statements - instructions performing action without returning a value** (including function definitions)
    * Example: `let y = 6;`

* **Expressions** - evaluate to returning a resulting value
    * **Do not include an ending semi-colon to an expression otherwise
    it becomes a statement that will no longer return a value**, i.e.
        ```rust
        let x = 5;

        // y is bound to value evaluated and returned from block
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {}", y);
        ```
    * Examples:
        * Calling a function or a macro is an expression
        * Blocks `{}` used to create expression


### FUNCTION RETURN VALUES

* Type of return value declared after an arrow `->`
* **Functions implicitly return the last expression in block of function body**
* Early return from function using `return` keyword and a value
* Do not name return values
* Example:
    ```rust
    fn increment(x: i32) -> i32 {
        x + 1
    }

    fn main() {
        let x = increment(5);
    }
    ```

## ERROR HANDLING

* `read_line` returns an instance of `io::Result` submodule **enum** type (with fixed set of enum variant values: `Ok`, `Err`) that provides an `expect` method. Either write error handling to suppress errors using a `match` expression or crash it by using `expect`.

* `continue` means skip to next iteration of the loop

* Compare two values using `cmp` returns `Ordering` **enum** **variant**, then for a `match` expression choose a matching **arm** **pattern** to run and to decide the next action based on the variant returned.

## CONTROL FLOW

* **if expression** with `if`, `else if`, and `else` with arms with blocks
* **condition must be boolean**
* `if` is an expression so may be used on right side of a `let` statement to bind the variable to the outcome of the `if` expression
* Example:
    ```rust
    let condition = true;

    // where the type returned by each arm is different then an error occurs
    // since variables must have a single type at compile time
    let number = if condition {
        5
    } else {
        6
    };
    ```

## LOOPS

* `break` to stop executing the loop
* Example:
    ```rust
    loop {
        break;
    }
    ```

## CONDITIONAL LOOPS

* `while` loop to loop over elements of a collection

    * Cons: Slow since compiler adds runtime code to perform
    conditional check on every element on every loop iteration
    * Example:
        ```rust
        let ages = [30, 40, 50, 60];
        let mut index = 0;

        while index < 5 {
            println!("current age value is: {}", ages[index]);
            index = index + 1;
        }
        ```

* `for` loop (**preferred** for looping over array)

    * Pros: Increase safety of code and eliminates change of bugs
    since avoids accessing out of bounds index or accessing insufficient indexes
    * `rev` method reverses the range
    * Example:
        ```rust
        let ages = [30, 40, 50, 60];

        for element in ages.iter() {
            println!("current age value is: {}", element);
        }

        for number in (1..4).rev() {
            println!("{}!", number);
        }
        ```

## OWNERSHIP

* **Ownership**

    * About
        * Rust memory is managed through an **ownership** system with a set of rules the compiler checks at compile time
    * Pros:
        * Allows Rust to make memory safety guarantees without need for garbage collector
        * No run time costs incurred for any ownership features
        * Managing **heap** data
            * Tracks what parts of code are using certain data on the **heap**
            * Minimises duplicate data on the **heap**
            * Cleaning up unused data on the **heap** to avoid running out of space

    * **Rules** (of Ownership)
        * Each value in Rust has a variable that is called its **owner**
        * Only one **owner** may exist at a time
        * Value is dropped when **owner** goes out of scope

    * **Stack**
        * Available at run time
        * LIFO - Pushing data onto the stack and popping off the stack
        * Fast access to data from top (not need search where to insert/remove data from)
        * Faster processing since data is all close together
        * Usage: With a **Known fixed size** for all data on the stack
        * Example 1
            * Calling a function, values passed to the function, **pointers** to data on the **heap**, and local variables of a function, are pushed onto the **stack**
            * End of a function causes values to be popped off the **stack**
        * Data Types stored on the stack including:
            * Integers, Floating-point numbers, Boolean, Chars, Tuples, and Arrays
        * Example 2
            * Since its dealing with integers that are simple values with a **Known fixed size** it binds value 5 to x, makes copy of value in x and binds it to y, and then pushes the two 5 values onto **stack** entirely so copies of actual values are quick (since not storing any values on the **heap** an no pointers necesary)
                ```rust
                let x = 5;
                let y = x;
                ```
        * Special annotation called `Copy` Trait may be placed on simple scalar types (i.e. integer, boolean, char, tuples with elements containing simple types) (if the type has not already implemented the `Drop` Trait) that are stored on the **stack** so an older variable is still usable after assignment. See Appendix C of Rust Second Edition

    * **Heap**
        * Usage: With a **Unknown size** data or size that may change at compile time
        * Less organised
        * **Allocating on the heap** (aka allocating) is where amount of space is requested to store data in empty spot on operating system that is marked in use and we are returned a **pointer** (address of the location)
        * **Pointer** is a **known fixed size** that we can store on the **stack** but when we want to retrieve the actual data we must follow the pointer
        * Slower access to data since must follow the **pointer** to get to data and different parts of data in different places
        * Data Types stored on the heap include:
            * Strings

    * **Move** (aka "shallow copy" + invalidation)

        * Example 1
            * Since its dealing with a `String` type it stores the variables group of data on
            the **stack** that is made up of a pointer (to the memory on the **heap** holding the string value with each char at different index), length (in bytes that `String` value is using), and capacity (total memory in bytes the `String` received from the operating system) (see diagram in book/second-edition/ch04-01-what-is-ownership.html).
            * Assigning `s1` to `s2` causes `String` data on the **stack** to be copied (including the **pointer** to memory on the **heap**), but does not copy the data on the **heap**.
            * **Double Free** error is a **memory safe bug** that could occur when Rust automatically calls the `drop` function to clean up **heap** memory for a variable when it goes out of scope (but where two variables on the **heap** have **pointer** to the same location, so they both try to free the same memory, which may lead to memory corruption and security vulnerabilities), but Rust ensures **memory safety** but **invalidating** the first variable `s1` so it does not try to free any memory when it goes out of scope. So trying to use and print `s1` in the example below will not work. Note: This is called a **"move"**, which is similar to a **"shallow copy"** but where the first variable `s1`'s pointer, length, and capacity are copied
            without the actual data, and in addition `s1` is **invalidated** (i.e. `s1` was **moved** to `s2`).
                ```rust
                let s1 = String::from("hello");
                let s2 = s1;
                println!("{}, world!", s1);
                ```
            * Benefits:
                * Avoids being expensive in terms of runtime performance
                * Rust never automatically creates a **"deep"** copy of data so automatic copying is inexpensive in terms of runtime performance

    * **Copy** (aka "deep copy")

        * `clone` method deeply copies both the **stack** and **heap** data of the `String`
        so it may be expensive

        * Example:
            ```rust
            let s1 = String::from("hello");
            let s2 = s1.clone();
            println!("s1 = {}, s2 = {}", s1, s2);
            ```

    * **Variable Scope**
        * Variables are valid from the point of declaration to the end of current **scope**

    * **Functions, Ownership Transfer, and Scope**

        * Example:
            ```rust
            fn main() {
                let s = String::from("hello");  // s comes into scope.
                takes_ownership(s);             // s's value moves into the function and is
                                                // so Ownership is transferred and s is
                                                // not a Copy so it is no longer valid here
                                                // and Rust's static check
                                                // would throw compile time error
                                                // if we used s after the call
            } // Here, s goes out of scope. s's value was moved, so nothing special happens.
            ```

    * **Return Values, Ownership Transfer, Scope, References, Mutable References, Dangling References**

        * Example 1: Passing **ownership** to every function and then have it return ownership each time is tedious
            ```rust
            fn main() {
                let s1 = get_string();              // function "moves" ownership in its return
                                                    // value of a string being assigned to s1

                let s2 = String::from("hello");     // s2 comes into scope

                let s3 = takes_and_gives_back(s2);  // s2 is "moved" into function,
                                                    // which also moves its return value into s3
            } // Here, s3 goes out of scope and is "dropped" since it is a variable with a string
              // so its data is on the heap (which is cleaned up by drop when it goes out of scope
              // unless the data has been "moved" to be owned by another variable.
              // s2 goes out of scope but was "moved", so nothing happens
              // s1 goes out of scope and is "dropped".
            ```

        * Example 2: Allow a function to use a value but without having to transfer **ownership**
        so we can still use the `String` afterward

            * Use **Borrowing** by passing a **Reference** with `&` to an object as a parameter to functions
            (creates a reference that uses a pointer to refer to a value without taking ownership of it),
            instead of passing the ownership of the value. Since it does not own the value, the value
            it points to will not be dropped when the reference goes out of scope.
            Reference: second-edition/ch04-02-references-and-borrowing.html
            * Note: **Dereferencing** operator is `*`

                ```rust
                fn main() {
                    let s1 = String::from("hello");

                    // Pass &s1 into the function
                    let len = calculate_length(&s1);

                    // Still able to use s1 here since did not pass ownership
                    println!("The length of '{}' is {}.", s1, len);
                }

                // &String s is a pointer reference to String s1
                fn calculate_length(s: &String) -> usize {
                    s.len()
                } // Here, s goes out of scope but since
                  // it does not have ownership of what
                  // it refers to, nothing happens.
                ```

            * **References** are **immutable** so we cannot modify it in the function that was called
            unless we create a **Mutable Reference** with `&mut` and receive it with `&mut` too.

                ```rust
                fn main() {
                    let mut s = String::from("hello");

                    // Create a Mutable Reference
                    change(&mut s);
                }

                // Accept a Mutable Reference
                fn change(some_string: &mut String) {
                    some_string.push_str(", world");
                }
                ```

                * **No Simultaneous Mutable References** to a single value are allowed at a time
                is a Rust restriction to prevent **data races** at compile time (where a
                data race is similar to a race condition where behaviours occur including multiple
                pointers accessing same data at same time with at least one pointer being used
                to write to the data, and with no mechanism to synchronise access to the data)

                * **Multiple Mutable References** are allowed as long as they are not simultaneous
                (i.e. not in the same block)
                    ```rust
                    let mut s = String::from("hello");

                    {
                        let r1 = &mut s;
                    } // r1 goes out of scope here, so we can make a new reference with no problems.

                    let r2 = &mut s;
                    ```

* **Borrowing** (with Reference as Function Parameters)

    * Definition: Calling a function with **references** as function parameters

* **Dangline References**

    * Definition: Dangling Pointer that references a memory location that was freed after being given
    to somewhere else but where the pointer to that memory is still retained

    * Rust compiler guarantees that **references** will never be **dangling references** since
    given a reference to some data the compiler ensures the data does not go out of scope before
    the reference to the data goes out of scope.

        * Example 1:
            ```rust
            fn main() {
                let reference_to_nothing = dangle();
            }

            fn dangle() -> &String {           // dangle returns a reference to a String
                let s = String::from("hello"); // s is a new String

                &s                             // Try to return a reference to the String, s
                                               // but the reference is pointing to an invalid `String`
            } // Here, s goes out of scope, and is "dropped". Its memory goes away. Danger!
            ```

        * Solution: Return `String` directly so **ownership** is **moved** out and nothing is "dropped"

            ```rust
            fn no_dangle() -> String {
                let s = String::from("hello");

                s
            }
            ```

* **Slices**

    * Definition: Data Type that does not have **ownership** and allows **referencing** a
    sequence of elements in a collection (rather than the whole collection).
    Reference: book/second-edition/ch04-03-slices.html

    * See example implementation in ./projects/find_word/src/main.rs

## MODULES

* Modules are **namespaces** containing definitios of functions or types
* Module visibility may be **public** or **private**
    * `private` by default: functions, types, constants, and modules
* Rust module system allows splitting code into chunks for code reuse
* Extract functions, structs, and enums into different modules
* Declare module with `mod` followed by block of code or in another file
* Use `use` keyword to import Modules or Enums into another scope.
Bring all names into scope with a Glob Operator like `use TrafficLight::*;`,
but may cause name conflicts.
    * Example:
        ```
        pub mod a {
            pub mod series {
                pub mod of {
                    pub fn nested_modules() {}
                }
            }
        }

        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }

        use TrafficLight::{Red, Yellow};
        // use TrafficLight::*;

        use a::series:of

        fn main() {
            of::nested_modules();

            let red = Red;
            let yellow = Yellow;
            let green = Green;
            // let green = TrafficLight::Green;
        }
        ```
*  Example Tests of a lib.rs file:
    * We are in the `communicator` library
    * Paths are relative to the current module `tests` inside `mod tests`
    * With `use` the paths are relative to the crate root by default.
    * Add to `tests` module the make the `client` module in scope by going up
    one module in the module hierarchy in order to call `client::connect()`,
    which is good way to start from root when deep in the module hierarchy.
    `super::` functionality changes the path given to `use` so it is relative
    to the parent module instead of the root module.
    * Example: lib.rs
        ```
        // Communicator Library

        pub mod client;

        pub mod network;

        #[cfg(test)]
        mod tests {
            //
            use communicator;

            #[test]
            fn test_client_connect() {
                // Option 1:
                super::client::connect();
                // Option 2:
                communicator::client::connect();
                // Option 3:
                ::client::connect();
        ```
* Rust looks in main.rs or lib.rs by default, which is where we tell Rust
where to find other files
* If using an external crate within a submodule of a project,
the `extern crate ___` should go in the root module (so in src/main.rs or src/lib.rs).
The submodules then refer to items from external crates as if the items are
top-level modules.


* **Privacy Rules**
    * Public - accessed through any parent module
    * Private - accessed only by immediate parent + any child modules

* Example
    * See projects/communicator

## COMMENTS

* Comments `//`