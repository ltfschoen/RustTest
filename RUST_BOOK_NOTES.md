# Rust Notes

# Table of Contents
    * [Dependencies](#chapter-85d895)
    * [Variables & Constants](#chapter-09462f)
    * [Types](#chapter-479b03)
        * [Type Annotations & Inference](#chapter-c8acba)
        * [Data Types](#chapter-396c9e)
        * [Type Conversion](#chapter-829696)
    * [Functions](#chapter-6b339a)
        * [Function Automatic Referencing & Dereferencing](#chapter-b5ca92)
        * [Function Parameters](#chapter-646f48)
        * [Function Body](#chapter-a87d4f)
        * [Function Return Values](#chapter-a87d4f)
    * [Control Flow](#chapter-5db484)
    * [Loops](#chapter-e22d77)
    * [Conditional Loops](#chapter-cc79c9)
    * [Ownership](#chapter-9ccf22)
    * [Modules](#chapter-b944a7)
    * [Errors](#chapter-e6d169)
    * [Generic Types, Traits, Lifetimes](#chapter-576c5a)
    * [Testing](#chapter-26543f)
    * [Threads](#chapter-98e538)
    * [Channels](#chapter-a9f98d)
    * [Comments](#chapter-688de0)

Note: Generate a new chapter with `openssl rand -hex 3`

## Dependencies <a id="chapter-85d895"></a>

### Dependencies

* Import type into scope of program if not already in the **prelude**.

* External dependency `extern crate rand` is equivalent of calling `use rand`. Prefix with `rand::` to use rand library.

* Statement used to import I/O **crate** for interaction from terminal.

* Statement used to import the `Ordering` **enum** and associated **variant** outcomes of comparison `Less`, `Greater`, `Equal`.

* Statement to import `Rng` **trait** to the **scope** since it defines random number generator methods including `thread_rng` that is local to current **thread of execution** and seeded by the operating system. `thread_rng().gen_range(<exclusive>, <inclusive>)`

Examples: [guessing_game](./projects/guessing_game/src/main.rs).

## Variables & Constants <a id="chapter-09462f"></a>

* Create **mutable** variable (since variables are by **default immutable**) as intent to allow other parts of the code to change it and bind to a value that is a new instance of say a string type using an **associated function (aka static method)** of the String type from the standard library.

* **Constants** declared with `const` are always immutable, cannot be converted with `mut` (whereas variables declared with `let` can be), must be annotated with a type, declared with capitals, can only be set to a constant expression computed at compile time (not the result of a function call or a value computed at runtime), are valid during program execution in the scope they were declared, and are best stored in a single place for update and maintenance.

* Immutable variables provide **safety** and easy of **concurrency** since the Rust compiler guarantees its value will not change, which reduces state management and makes code easier to reason about.

* Trade-off between performance and code clarity. Using mutable variables may make code easier to write, but immutability may result in less bugs.

* **Shadow** previously declared mutable value of `guess` for reuse then perform transformations on its value redeclaring it as a new immutable variable with the same name using `let` and with new value but even of a different type. Shadowing saves us coming up with new variable names (i.e. myvar instead of say myvar_str, myvar_num, etc). It is different to marking a variable as `mut`.
* Example: **Bind** `guess` to expression `guess.trim().parse()`. trim() removes whitespace and newline characters. parse() will parse a string into a number. Examples: [guessing_game](./projects/guessing_game/src/main.rs).

## Types <a id="chapter-479b03"></a>

* Rust is **statically typed** so must know variable types at compile time.

* Compiler may infer the type based on value used.

* Multiple number types may have value between 1 and 100, where default is `i32` (32-bit number), and others are `u32` (unsigned 32-bit number for small positive numbers) and `i64` (64-bit number).

### Type Annotations & Inference <a id="chapter-09462f"></a>

* Type annotation of variables with a colon `:` used when many types possible.

* Rust infers as say being a string since it has **type inference**.

### Data Types <a id="chapter-396c9e"></a>

#### Scalar

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
        let x = 2.0;      // f64 default
        let y: f32 = 3.0; // f32
        ```
* **Numeric operations** (see Appendix B of Rust Book)
    * `+`, `-`, `*`, `/`, `%` (remainder)
    * Example: `let remainder = 43 % 5;`

* **Boolean**
    * `bool` (`true` or `false`)
    * Example: `let f: bool = false;`

* **Characters**
    * `char` is Unicode Scalar Value from U+0000 to U+D7FF and U+E000 to U+10FFFF (more than just ASCII)
    * Note:
        * `char` specified using **single quotes**, whereas strings specified with double quotes)
    * Example: `let z_letter = 'z';`

#### Compound

* Note:
    * Compound types groups multiple values of other types into single type

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
        * Allocates data on the **stack** instead of the **heap**
        * Use when only need **fixed number of elements**
        * Rust panics (immediately exits program with error)
        if access index greater than array length
        * **Vector** collection type allows dynamic number of elements
        (grow or shrink)
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

#### Complex

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

        * Example 2: Implicitly return new instance of Struct from Function body by
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
            ```rust
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

        * Example 1: **Function**
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

        * Example 2: **Method** (refactored to use Method instead of Function)
            * If we are only **reading** from the Struct data we use `&self`
            * If we want to **write** to the Struct instance we use `&mut self` as first parameter
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

    * **Associated Functions** (i.e. **Static Functions** of a Struct that do not require a Struct Instance)
        * Do not require `self` since do not require an **instance** of the Struct, so are not methods
        * Namespaced by the Struct so call with say `let sq = Rectangle::square(3);`

        * Examples 1: `String::from`

        * Example 2:
            * See `fn square` in [shapes](./projects/shapes/src/main.rs).

    * **Tuple Structs**
        * Structs without labels. See https://doc.rust-lang.org/1.30.0/book/second-edition/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types

    * **Ownership of Struct Data**
        * Use the **"owned" `String` type** rather than the **`&str` String Slice type**
        if want instances of the Struct to own all of its data and for that data
        to be valid for as long as the entire Struct is valid.

        * **Store References in Structs using Lifetime Specifiers**
            * Structs may store "references" to data that is "owned" by something else
            through use of "lifetimes" that ensure the data "referenced" by the
            Struct is valid as long as the Struct is:
                ```rust
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
            ```rust
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
        for different enum values
        * `if let` for handling enums conveniently is syntactic sugar (less code than using `match`)
        * Enum values can only be one of the variants
        * Enums may contain anonymous structs as data
        * Enums may define embedded methods using `impl`
        * Enums may be used to create Custom Data Types and instances of them

    * See book/second-edition/ch06-01-defining-an-enum.html
    * See [users](./projects/users/src/main.rs)
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
        then storing elements of different types within it, and then **store the enum in the vector**.

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
                        ```rust
                        let v: Vec<i32> = Vec::new();
                        ```
                * Pre-Populated with Inferred Type
                    * Create a Vector with **initial values** using **vec!** macro,
                    where Rust infers the type as being `Vec<i32>`:
                        ```rust
                        let v = vec![1, 2, 3];
                        ```

                    * Using an **Enum to store different types in a vector**:
                        ```rust
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

            * Reading Elements of a Vector
                * Option 1: Indexing Syntax
                    * Usage: Accessing out of bounds causes
                    program to `panic!` and crash
                    ```rust
                    let v = vec![1, 2, 3, 4, 5];
                    let does_not_exist = &v[100];
                    let does_not_exist = v.get(100);
                    ```

                * Option 2: `get` Method that returns `Option<&T>`
                    * Usage: Accessing out of bounds returns `None` without
                    panicking. Then use logic to handle either `Some(&element)`
                    or `None` with feedback to CLI/UI so more user friendly
                    ```rust
                    let v = vec![1, 2, 3, 4, 5];
                    let *third: Option<&i32> = v.get(100);

                    match *third {
                        Some(x) => { println!("Reachable element {}", x); () },
                        None => { println!("Unreachable element"); }
                    }
                    ```

            * Updating Elements of a Vector
                ```rust
                {
                    let mut v = Vec::new();
                    v.push(5);
                }
                ```

            * Modifying Elements of a Vector
                * Note we **cannot** add an element to a Vector whilst already holding a
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

    * **String Literal**
        * **Immutable** string value that is hard-coded
        * Pros:
            * Fast and efficient since immutable and contents known in executable
            at compile time
        * Cons:
            * Does not cater for string values that are not known at compile time

    * **Strings**
        * `String` type ("owned") is UTF-8 encoded and may be used to take user input and store it on the **heap**
            * Supports **Mutable** (growable text) by allocating memory on the **heap**
            to hold the data that is unknown at compile time, so memory is requested from
            operating system at runtime (i.e. when we call `String::from`).
            * **Rust uses a pattern of automatically returning memory to the operating system after the variable that owns it goes out of scope** by calling the
            `drop` function, similar to Resource Acquisition Is Initialization (RAII)
            in C++.
            * Pros:
                * Allows for using string values that are not known at compile time

            * Examples
                * Create New empty String
                    ```rust
                    let mut s = String::new();
                    ```

                * Access Index of Characters in a String
                    * Note: Rust strings don't support indexing, so we can't do
                    `let s1 = String::from("hello"); s1[0];`

                    * See for details of how to use here: https://github.com/rust-lang/book/blob/master/2018-edition/src/ch08-02-strings.md#indexing-into-strings

                * Mutation of `String` type by appending a String Literal
                    ```rust
                    let s = "initial content".to_string();
                    ```
                * Mutation of `String` type by appending a String Slice (equivalent to above).
                Note: `push_str` takes a String Slice since don't want "ownership" of the parameter.

                    ```rust
                    let mut s = String::from("hello");
                    s.push_str(", world!");

                    let mut s1 = String::from("foo");
                    let s2 = "bar";
                    s1.push_str(&s2);
                    println!("s2 is {}", s2);
                    ```

                * Mutation of `String` type by appending a single character
                    ```rust
                    let mut s = String::from("lo");
                    s.push('l');
                    ```

                * Concatenation of a `&str` to a `String` type (but it takes "ownership" **disadvantage**)
                    * Note: We can't add two `String` types together.
                    * Note: In the example `&s2` is `&String` type and is "coerced" into `&str` type
                    and when we use `+` (`add`) then Rust uses "deref coercion" to turn `&s2` into `&s2[..]`
                    and does not take "ownership" of parameter then `s2` remains a valid `String` after the
                    operation. `s1` is moved into the `+` (`add`) call and no longer valid after it since the 
                    call takes "ownership" of `self` (see signature of `+` (`add` in standard library)), but
                    ultimately the statement returns "ownership" of the result
                    ```rust
                    let s1 = String::from("Hello, ");
                    let s2 = String::from("world!");
                    let s3 = s1 + &s2; // Note that s1 has been "moved" here and can no longer be used
                    ```

                * **Concatenation** of complicated `String` combinations **without it taking "ownership"** of params so we don't lose access, by using the `format!` macro (it returns a `String` with contents but does not output to the screen like `println!`)
                    ```rust
                    let s1 = String::from("tic");
                    let s2 = String::from("tac");
                    let s3 = String::from("toe");

                    let s = format!("{}-{}-{}", s1, s2, s3);
                    ```

                * Iterate over Strings to Access Elements
                    * Allows performing operations on individual Unicode scalar values using either the
                    `chars()` or `bytes()` method
                    ```rust
                    for c in "नमस्ते".chars() {
                        println!("{}", c);
                    }
                    ```

                * Strings are UTF-8 encoded so we can include any properly encoded data on them 
                    ```rust
                    let hello = String::from("Olá");
                    ```

        * **String Slices**
            * See book/second-edition/ch04-03-slices.html

            * String Literals are Slices
                * Slice pointing to specific point in binary where `&str` ("borrowed" variant)
                is immutable reference

                ```rust
                let s: &str = "Hello, world!";
                println!("{}", s);
                ```

            * Specific indexing with a range to obtain the String Slice of particular bytes
                ```rust
                let hello = "Здравствуйте";
                let s = &hello[0..4];
                // `s` will be a `&str` containing first 4 bytes of the string, where each char
                // is 2 bytes, so `s` will be `Зд`
                ```

    * **Hash Map**
        * Definition: Allows association that maps key with a value with type `HashMap<K, V>`
        using a **hashing function** that determines how Keys and Values are stored in memory
        on the **heap**.
        HashMaps are **Homogeneous** so all Keys must be the same type and all Values must be the same type. 

        * Create HashMap
            ```rust
            use std::collections::HashMap;
            let mut scores = HashMap::new();
            ```
        
        * Add elements to HashMap (here keys are `String` type and values are `i32`)
            ```rust
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 50);
            ```

        * Access elements in HashMap
            ```rust
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 50);

            let team_name = String::from("Blue");
            // `get` returns Option<&V>, so the result will be `Some(&10)` or 
            // if no value for the key it returns `None`, which we have to handle
            let score = scores.get(&team_name);
            ```

        * Iterate over Keys and Values
            ```rust
            for (key, value) in &scores {
                println!("{}: {}", key, value);
            }
            ```
        
        * Overwrite values
            ```rust
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Blue"), 25);
            println!("{:?}", scores); // {"Blue": 25}
            ```

        * Only insert value if no value exists for a key
            * `Entry` takes key you want to check as parameter.
            * Returned value of the entry function is an enum called 
             `Entry` that represents a value that might or might not exist.
            * `or_insert` method on `Entry` is defined to return a mutable 
            reference to the value for the corresponding Entry key if that key exists, 
            and if not, inserts the parameter as the new value for this key and 
            returns a mutable reference to the new value
            ```rust
            scores.insert(String::from("Blue"), 10);
            scores.entry(String::from("Yellow")).or_insert(50);
            scores.entry(String::from("Blue")).or_insert(50); // no change since value exists
            println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}
            ```
        
        * Update Existing Value of a Key depending on the Old Value
            ```rust
            use std::collections::HashMap;
            let text = "hello world wonderful world";
            // HashMap with Keys: words, Value: count times seen a word
            let mut map = HashMap::new();
            for word in text.split_whitespace() {
                // count times each word appears in text.
                // add value of 0 if first time seen a word.
                // `or_insert` returns mutable reference to value for the Key `&mut V`
                // and stores it in `count` variable, but must first dereference `count` using 
                // an `*` before assigning to that value.
                // the mutable reference goes out of scope at end of the `for` loop
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }
            println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}.
            ```

        * Create HashMap from multiple Vectors of data using Tuples
            * Given team names and initial scores in two separate Vectors.
            Use `zip` method to Create a Vector of Tuples where we pair each team name with a score.
            Lastly use the `collect` method to convert the Vector of Tuples into a HashMap.
            Rust can infer the types that the HashMap contains based on the types of data in the Vectors. 

            ```rust
            use std::collections::HashMap;

            let teams  = vec![String::from("Blue"), String::from("Yellow")];
            let initial_scores = vec![10, 50];

            let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
            ```

        * Creation of HashMaps and Ownership

            ```rust
            use std::collections::HashMap;

            let field_name = String::from("Favorite color");
            let field_value = String::from("Blue");

            let mut map = HashMap::new();
            // "owned" values like String type are "moved" onto HashMap so later they are "invalid"
            // so instead we need to pass "references" to it.
            // Whereas other types like i32 (of Copy trait) are only "copied" onto the HashMap and don't
            // become "invalid" afterward
            map.insert(&field_name, &field_value);
            // field_name and field_value are invalid at this point, try using them and
            // see what compiler error you get!
            println!("fields are: {} {}", field_name, field_value);
            ```

        * Hashing Functions
            * Default **hasher** (hashing algorithm) that implements the 
            `BuildHasher` trait may be slow since it uses 
            cryptographically secure **hashing function** for better security

## Type Conversion <a id="chapter-829696"></a>

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

* Convert `String` into a string
    ```rust
    let str = "hello".to_string();
    ```

## Functions <a id="chapter-6b339a"></a>

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

### Function Automatic Referencing and Dereferencing <a id="chapter-b5ca92"></a>

* Rust uses **automatic referencing and dereferencing** when calling a method.
Rust automatically adds in `&`, `&mut`, or `*` so the `object` matches the method signature.
Since methods have a name and a receiver of type `self`, when a call to a method is made and
these are given, Rust determines if the method is reading `&self`, mutating `&mut self` or
consuming `self`. Rust makes "borrowing" implicit for method receivers.

* Example: Both lines are equivalent
    ```rust
    p1.distance(&p2);
    (&p1).distance(&p2);
    ```

### Function Parameters <a id="chapter-646f48"></a>

* **Parameter type annotations** must be declared in function signature

* Note that `_` is catch all of all values (say for function arguments).
    * See book/second-edition/ch06-02-match.html#the-_-placeholder

* Call associated function with say: `std::io::stdin()` to return instance of terminal standard input `std::io::Stdin`. Call `readline` method on standard input handle to obtain user input passing a single mutable reference argument to change the string content with the user input (allow safe and easy access to data without having to copy it to memory multiple times).

### Function Body <a id="chapter-a87d4f"></a>

* **Statements - instructions performing action without returning a value** (including function definitions)
    * Example: `let y = 6;`

* **Expressions** - evaluate to return a resulting value
    * Do not include an **ending semi-colon** to an expression otherwise
    it becomes a statement that will no longer return a value, i.e.
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
        * Blocks `{}` used to create **expression**


### Function Return Values <a id="chapter-a87d4f"></a>

* Type of return value declared after an arrow `->`
* **Functions implicitly returns the last expression in block of function body**
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

## Control Flow <a id="chapter-5db484"></a>

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

## Loops <a id="chapter-e22d77"></a>

* `break` to stop executing the loop
* Example:
    ```rust
    loop {
        break;
    }
    ```

## Conditional Loops <a id="chapter-cc79c9"></a>

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

## Ownership <a id="chapter-9ccf22"></a>

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
            * Since its dealing with integers that are simple values with a **Known fixed size** it binds value 5 to x, makes copy of value in x and binds it to y, and then pushes the two 5 values onto **stack** entirely so copies of actual values are quick (since not storing any values on the **heap** and no pointers necesary)
                ```rust
                let x = 5;
                let y = x;
                ```
        * Special annotation called `Copy` Trait may be placed on simple scalar types (i.e. integer, boolean, char, tuples with elements containing simple types) (if the type has not already implemented the `Drop` Trait) that are stored on the **stack** so an older variable is still usable after assignment. See Appendix C of Rust Second Edition

    * **Heap**
        * Usage: With **Unknown size** data or size that may change at compile time
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
            * **Double Free** error is a **memory safe bug** that could occur when Rust automatically calls the `drop` function to clean up **heap** memory for a variable when it goes out of scope (but where two variables on the **heap** have **pointer** to the same location, so they both try to free the same memory, which may lead to memory corruption and security vulnerabilities), but Rust ensures **memory safety** by **invalidating** the first variable `s1` so it does not try to free any memory when it goes out of scope. So trying to use and print `s1` in the example below will not work. Note: This is called a **"move"**, which is similar to a **"shallow copy"** but where the first variable `s1`'s pointer, length, and capacity are copied without the actual data, and in addition `s1` is **invalidated** (i.e. `s1` was **moved** to `s2`).
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

            * Use **Borrowing** by passing a **Reference** (`&`) to an object as a parameter to functions (**reference uses a pointer to refer to a value without taking ownership of it**). Since it does not own the value, the value it points to will not be dropped when the reference goes out of scope.
            Reference: second-edition/ch04-02-references-and-borrowing.html
            * Note: **Dereferencing** operator is `*`

                ```rust
                fn main() {
                    let s1 = String::from("hello");

                    // Pass "reference" (&s1 pointer to the value) into the function
                    let len = calculate_length(&s1);

                    // Still able to use s1 here since did not pass ownership, so the value
                    // the reference points to is not dropped with it goes out of scope 
                    println!("The length of '{}' is {}.", s1, len);
                }

                // &String s is a pointer reference to String s1
                fn calculate_length(s: &String) -> usize {
                    s.len()
                } // Here, s goes out of scope but since
                  // it does not have ownership of what
                  // it refers to it isn't dropped.
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

                * **No Simultaneous Mutable References** to a single value are allowed at a time. This
                is a Rust restriction to prevent **data races** at compile time (where a
                data race is similar to a race condition where behaviour occurs such that multiple
                pointers access the same data at the same time with at least one pointer being used
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

* **Borrowing** (passing References as Function Parameters)

    * Definition: Calling a function with **references** as function parameters

* **Dangling References**

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

    * Examples: [find_word](./projects/find_word/src/main.rs).

## Modules <a id="chapter-b944a7"></a>

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

        ```rust
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
* Example Tests of a lib.rs file:
    * We are in the `communicator` library
    * Paths are relative to the current module `tests` inside `mod tests`
    * With `use` the paths are relative to the crate root by default.
    * Add to `tests` module to make the `client` module in scope by going up
    one module in the module hierarchy in order to call `client::connect()`,
    which is a good way to start from the root when deep in the module hierarchy.
    `super::` functionality changes the path given to `use` so it is relative
    to the parent module instead of the root module.
    * Example: lib.rs
        ```rust
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

* Examples: [communicator](./projects/communicator/src/main.rs).

## Errors <a id="chapter-e6d169"></a>

* Error Handling
    * `read_line` returns an instance of `io::Result` submodule **enum** type (with fixed set of enum variant values: `Ok`, `Err`) that provides an `expect` method. Either write error handling to suppress errors using a `match` expression or crash it by using `expect`.
    * `continue` means skip to next iteration of the loop
    * Compare two values using `cmp` returns `Ordering` **enum** **variant**, then for a `match` expression choose a matching **arm** **pattern** to run and to decide the next action based on the variant returned.

* Rust categories of errors: 
    * **Recoverable**
        * Report problem to user and retry operation (i.e. file not found)
        * Type: Returning `Result<T, E>` 
            * `T` generic type parameter means value returned in Success case within `Ok` variant
            * `E` generic type parameter means type of error returned in Failure case with `Err` variant
        * Check if calling a function returns a `Result` by looking at the 
        Standard Library API Documentation or asking the Compiler.
        * We don't have to specify `Result::` before `Ok` or `Err` variants since it's imported
        in the Prelude
        * The Type of value that File::open returns inside `Err` variant is `io::Error`, 
        which is a struct provided by standard library that has method `kind` we can call
        to get an `io::ErrorKind` value. `io::ErrorKind` is an enum provided by the 
        standard library that has variants representing the different kinds of errors that 
        might result from an io operation.
        * `ref` is so error is only "referenced" and not "moved" into guard conditional.
        `ref` is used to match a value and give us a "reference" to it, whereas
        `&` matches a reference and gives us its value   
        * Example:
            * Open File
                ```rust
                use std::fs::File;
                use std::io::ErrorKind;

                fn main() {
                    let f = File::open("hello.txt");

                    let f = match f {
                        Ok(file) => file,
                        // Match guard
                        Err(ref error) if error.kind() == ErrorKind::NotFound => {
                            match File::create("hello.txt") {
                                Ok(fc) => fc,
                                Err(e) => {
                                    panic!(
                                        "Tried to create file but there was a problem: {:?}",
                                        e
                                    )
                                },
                            }
                        },
                        Err(error) => {
                            panic!(
                                "There was a problem opening the file: {:?}",
                                error
                            )
                        },
                    };
                }
                ```
            * Find out the Type of value returned by just trialing a type 
            as a Type Annotation and the Compiler will complain and tell us what it actually 
            returned `let f: u32 = File::open("hello.txt");`. We find that it returns 
            enum type `Result<T, E>` with generic parameter `T` for type of success
            value, `std::fs::File` (file handle), and `E` as error value of `std::io::Error`
            (error info).
            If `File::open` succeeds, the value in the variable `f` will be instance 
            of `Ok` that contains a file handle. If it fails, the value in `f` will
            be an instance of `Err` that contains more information about the error
                ```
                  4 |     let f: u32 = File::open("hello.txt");
                    |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
                    `std::result::Result`
                    |
                    = note: expected type `u32`
                                found type `std::result::Result<std::fs::File, std::io::Error>`
                ```

        * Helper Method Shortcuts of the `Result<T, E>` type for panic on error (instead of `match`, which is verbose)

            * `unwrap` (similar to `match`)
                * If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro

                * Example: Panic when no file exists:
                    ```rust
                    use std::fs::File;

                    fn main() {
                        let f = File::open("hello.txt").unwrap();
                    }
                    ```

            * `expect` (similar to `unwrap`)
                * Allows us to choose the Error message 

                * Example:
                    ```rust
                    use std::fs::File;

                    fn main() {
                        let f = File::open("hello.txt").expect("Failed to open hello.txt");
                    }
                    ```

    * **Unrecoverable** (and backtracing)
        * Due to a bug such as accessing out of bounds in array (which may expose 
        security vulnerabilities such as "buffer overread" from attackers)
        * Type: Call `panic!` macro that stops execution
            * Causes program unwinding up the stack and clean up data/memory used
            by each function encountered.
            Use the backtrace of functions the `panic!` call came from to find 
            part of code causing the problem, by running with the debug 
            environment variable enabled `RUST_BACKTRACE=1` (debug symbols are
            also enabled when running `cargo build` or just `cargo run`)
            * Immediately abort without cleaning up, so memory used by program
            must be cleaned up by the operating system
            * Read backtrace by starting from the top until you see files
            you wrote, which is the spot where the problem originated. 
            The lines above the lines mentioning your files are code that your code
            called; the lines below are code that called your code
        * Use `panic!` when code could enter a rare unexpected state, and the code 
        relies on not being in that bad state when it occurs, and should alert the
        developer so they can fix it during development. Also use it when calling 
        external code out of your control that returns invalid state that we cannot fix.
            * Example Use Cases:    
                * Verification finds values invalid that it must operate on, this is done for safety reasons since operations on invalid data may expose code to vulnerabilities (i.e. accessing wrong memory location that does not belong to current data structure)
                * Inputs to a function not meeting certain requirements is a violation of a contract (should be explained in API docs for the function) and always a caller-side bug but don't want calling code to have to handle it. It's the calling code programmers responsibility to fix the code
        * Do Not use `panic!` when bad state reached that is expected to happen, so
        instead return a `Result` and propagate the bad state upward to calling code that will decide how to handle it. 
            * Example Use Cases:
                * Parser given bad data
                * HTTP request returning state indicating rate limit hit
        * Switching to `panic!` in production release makes the binary as small
        as possible by adding to Cargo.toml:
            ```
            [profile.release]
            panic = 'abort'
            ```
            * Reference: https://doc.rust-lang.org/book/second-edition/ch09-01-unrecoverable-errors-with-panic.html
        * Use Panic in program:
            ```rust
            panic!("crash and burn");
            ```

* **Propagating Errors**

    * When a function calls something that fails, then instead of handling the error in that function, we can return the error to the calling code for a decision on the appropriate action where there is more info/logic to handle the error

        * Example:
            ```rust
            use std::fs::File;

            fn read_username_from_file() -> Result<String, io::Error> {
                let f = File::open("hello.txt");

                let mut f = match f {
                    Ok(file) => file,
                    Err(e) => return Err(e),
                };

                let mut s = String::new();

                match f.read_to_string(&mut s) {
                    Ok(_) => Ok(s),
                    Err(e) => Err(e),
                }
            }
            ```

* **Shortcut for Propagating Errors**

    * Use the `?` question mark operator

    * If the value of the `Result` is an `Ok`, the value inside `Ok` will get returned from this expression and the program will continue. If the value is an `Err`, the value inside `Err` will be returned from the whole function as if we had used the return keyword so the error value gets **propagated** to the calling code

    * `?` can only be used in functions that have a return type of `Result`

        * Example 1:
            ```rust
            use std::io;
            use std::io::Read;
            use std::fs::File;

            fn read_username_from_file() -> Result<String, io::Error> {
                // `?` returns value inside an `Ok` to variable `f` if success,
                // or if error occurs `?` will return early out of whole function
                // and give `Err` value to "calling" code
                let mut f = File::open("hello.txt")?;
                let mut s = String::new();
                // return errors to the calling code using `?`.
                // `?` placed after a `Result` value
                f.read_to_string(&mut s)?;
                Ok(s)
            }
            ```

        * Example 2: Chaining Method Calls

            ```rust
            use std::io;
            use std::io::Read;
            use std::fs::File;

            fn read_username_from_file() -> Result<String, io::Error> {
                let mut s = String::new();

                File::open("hello.txt")?.read_to_string(&mut s)?;

                Ok(s)
            }
            ```

* **Create Custom Type for Validation**

    * Rust Type System may be used to ensure valid input value, and guide
    the user toward a valid input with different behaviour
        * Example:
            * Given the scenario of expecting a positive number input from a user, then instead of using `u32` (only positive allowed), we would use `i32` instead and add a validation check to see that the number is in range

        * Examples: [guessing_game](./projects/guessing_game/src/main.rs).

## Generic Types, Traits, Lifetimes <a id="chapter-576c5a"></a>

* **Generics** 

    * Definition: 
        * Define custom Types, Functions, and Methods
        * Use to write functions that take parameters of
        a Generic Type (i.e. `Vec<T>`) instead of a concrete type (i.e. `i32`)
        * In the same way that an abstracted function body can operate on an abstract list instead of specific values to prevent code duplication, generics allow code to operate on abstract types

        * **Generic Data Types**
            * Use generics to create definitions for items like function signatures, structs, enums, and methods, and then use them with different concrete data types.

        * **Generics and their Performance Impact**
            * Rust uses **Monomorphization** (turning generic code into 
            specific code, so it restores the duplication for runtime) 
            of code that is using generics at compile time so the speed is no slower using Generic Types
            than with Concrete Types

    * Examples:
        `Option<T>`, `Vec<T>`, `HashMap<K, V>`, `Result<T, E>`

    * Setup Steps
        * **Generic Functions**
            * Extract a function to reduce code duplication
            * Create a Generic Function (based on two functions that only differ by the Type of parameters they accept, and their names)
                * Add Generics in the Function Signature instead of specifying the Data Types of the Parameters and Return Value to provide flexibility, extra functionality to callers, and preventing code duplication.

                * Examples: [generics - function_definition](./projects/generics/src/examples/function_definitions.rs).

                * Note: If get error `binary operation > cannot be applied to type T` and that an
                implementation of `std::cmp::PartialOrd` Trait might be missing for `T` then it 
                means we can only use types whose values can be **Ordered**, so to fix this it would be necessary to to enable comparisons using the `std::cmp::PartialOrd` Trait by implementing it on the type `T` to specify that the generic type `T` has a particular Trait (see Derivable Traits and Trait Bounds sections)

        * Generic Structs
            * Multiple Generic Type Parameters to allow multiple different types as parameters

            * Examples: [generics - function_definition](./projects/generics/src/examples/struct_definitions.rs).

        * Generic Enums
            * `Option<T>` Enum from Standard Library with Generic Type
                ```rust
                enum Option<T> {
                    Some(T),
                    None,
                }
                ```
            
            * Enum with Multiple Generic Types
                ```rust
                enum Result<T, E> {
                    Ok(T),
                    Err(E),
                }
                ```

                * Usage: Whereever we have an operation that may succeed and return value of type `T` or fail with error type `E`

* **Traits**

    * Definition: 
        * Define **Generic Behaviour that may be Shared between different Types**, where the 
        **Type's Behaviour** comprises the Methods we may call on that Type
        * Traits are similar to **"Interfaces"** in other languages
        * **Trait Definitions** group the Type's Method Signatures together to define a set of the Type's Behaviours 
        * Traits inform the Rust compiler about a specific Type's Functionality to: 
            * Share the Type's Functionality with other Types
        * **Trait Bounds** constrain Generic Types to ensure their type is limited to
        types that implement particular Trait or Behaviour by using Traits with Generic Type Parameters.
            * Allow **multiple Trait Bounds** on a Generic Type using the `+` syntax (i.e. use Display formatting as well as the `summarize` method on the type `T` by specifying `T: Summary + Display` to say that `T` may be any type that implements `Summary` and `Display`)
            * Alternately use the `where` clause after the Function Signature instead of using `+` to specify Trait Bounds so the Function Signature remains easy to read when multiple Trait Bounds are specified
            * **Trait Bounds to Conditionally Implement Methods**
                * Implement methods conditionally only for types that implement the traits specified in the `impl` Function Signature, by using a Trait Bound in the `impl` block using Generic Type Parameters.
            * Examples: [traits - lib.rs](./projects/traits/src/lib.rs) implemented in [traits - main.rs](./projects/traits/src/main.rs).

        * **Blanket Implementations**
            * Implementations of a Trait on any type that satisfies the Trait Bounds. See https://github.com/rust-lang/book/blob/master/2018-edition/src/ch10-02-traits.md

        * **Default Implementations**
            * Default behaviour for the implementations of the Trait's Method Signatures. When we implement the Trait for a particular Type we can use the Default or Override the Trait Method's Default Behaviour. Use with code below along with empty block for the implementation.

                * **WITHOUT Default** Implementation
                    ```rust
                    pub trait Summary {
                        // Signature
                        fn summarize(&self) -> String;
                    }

                    impl Summary for Tweet {
                        // Type-specific (i.e. `Tweet` only) implementation of the `Summary` Trait's
                        // `summarize` method
                        fn summarize(&self) -> String {
                            format!("{}: {}", self.username, self.content)
                        }
                    }
                    ```

                * **WITH Default** Implementation
                    ```rust
                    pub trait Summary {
                        // Default implementation of `Summary` Trait's `summarize` method, may be used with different Types other than `Tweet`.
                        fn summarize(&self) -> String {
                            String::from("(Read more...)")
                        }
                    }

                    impl Summary for Tweet {}
                    ```

    * Other Definitions:
        * **Coherence / Orphan Rule**
            * Where the Parent Type is not present there is a Restriction to ensure your code is not broken by another person's code. It's a rule that prevents two crates implementing the same Trait for the same Type where Rust wouldn't know which implementation to use. See https://github.com/rust-lang/book/blob/master/2018-edition/src/ch10-02-traits.md

    * Setup Steps
        * Trait combined with a Generic Type to constrain it to only types with a specific behaviour
        * When declaring a Trait with `trait`, instead of providing an implementation block after the Method Signature, we just use a semicolon, like an **"Interface"**. Each Type implementing this Trait must provide its own custom behavior for the Method Body. The compiler will enforce that any Type that has the Trait will have the exact Method Signature we defined.
        A Trait may have multiple Methods Signatures in its body: the method signatures are listed one per line and each line ends in a semicolon.
        * Use the Trait in other projects, assuming crate called `aggregator` with 
        `use aggregator::Summary;`

    * Examples: [traits - main.rs](./projects/traits/src/lib.rs)


* **Lifetimes**

    * Definition: 
        * All "references" in Rust have a **"lifetime"**, which is the scope where the "reference" is valid
        * Generic that allows us to inform the Compiler info about how "references" are related to each other, which allows us to "borrow" values in situations and still have Compiler check that "references" are still valid
        * Lifetimes ensure that "references" are valid as long as we need them to be, instead of ensuring that a type has the behavior we want
        * When the **lifetime** of a "reference" could be related in different ways then we must **annotate** the lifetimes using **Rust Generic Lifetime Parameters** (to ensure actual "references" will be valid at runtime). (i.e. given a variable `let a;` we could annotate that with `'a` annotation to show its lifetime) in a block. https://github.com/rust-lang/book/blob/master/2018-edition/src/ch10-03-lifetime-syntax.md
        * **"lifetime"**'s prevent **Dangling "references"** (program referencing data it did not intend to) 
        * **"lifetime"** of a variable "lives longer" if the scope of the variable is longer
        * Rust uses a **"borrow checker"** to determine if code is valid and a variable is not trying to "reference" memory used by another variable that has already been deallocated after going out of scope. At compile time Rust compares the size of different lifetimes to check if say lifetime `'a` refers to memory with lifetime of `'b` and rejects the program if `'b` lifetime is shorter (data of reference does not live as long as the reference)
        * **"borrow checker"** compares scopes to determine that all borrows are valid
        * **Lifetime Annotations** must be specified for functions or structs that use "references", since every "reference" has a "lifetime". 
        * **Lifetime** syntax is about connecting the lifetimes of various parameters' **input lifetimes** and return values of a functions **output lifetimes**. Once they're connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety
        * **Lifetime Parameters** describe the relationship of the lifetimes of multiple references to each other but do not actually affect the lifetimes at all. If a "lifetime" is used in isolation it doesn't have much meaning, as it's used for comparison between the lifetimes of different variables.
            * Examples:
                ```rust
                &i32        // a reference (without a "lifetime" parameter)
                &'a i32     // a reference (with an explicit "lifetime" parameter 'a)
                &'a mut i32 // a mutable reference (with an explicit "lifetime" parameter 'a)
                ```

    * **Lifetime Annotations in Function Signatures**
        * Express a constraint in the Function Signature that all "references" in the parameters passed into the function and the return value must have the same "lifetime"
            ```rust
            fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
                if x.len() > y.len() {
                    x
                } else {
                    y
                }
            }
            ```

    * **Lifetime Annotations in Struct Definitions**
        * Prevents Struct instances living longer than the "reference" it holds in its properties
            * Example:
                * Reference: https://github.com/rust-lang/book/blob/master/2018-edition/src/ch10-03-lifetime-syntax.md#lifetime-annotations-in-struct-definitions

                ```rust
                struct ImportantExcerpt<'a> {
                    part: &'a str,
                }

                fn main() {
                    let novel = String::from("Call me Ishmael. Some years ago...");
                    let first_sentence = novel.split('.')
                        .next()
                        .expect("Could not find a '.'");
                    // Create instance of `ImportantExcerpt` Struct that holds a
                    // "reference" to `first_sentence` variable String that is "owned"
                    // by `novel` variable.
                    let i = ImportantExcerpt { part: first_sentence };
                }
                ```
        * Note: More information about [`next()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)
    * **Lifetime Elision Rules**
        * Specific cases that the Rust Compiler considers, and if your code fits one of the cases then you do not need to write "lifetime" annotations explicitly, since "inference" may be in-built.
        Due to these rules we often do not have to use lifetime annotations in method signatures often.
        * Reference: https://github.com/rust-lang/book/blob/master/2018-edition/src/ch10-03-lifetime-syntax.md#lifetime-elision

    * **Lifetime Annotations in Method Definitions**
        * Ref: https://github.com/rust-lang/book/blob/master/2018-edition/src/ch10-03-lifetime-syntax.md#lifetime-annotations-in-method-definitions
    
    * **Static Lifetime**
        * `'static` denotes the entire "lifetime" of the program, which all String Literals have.
        * Warning: Do not use as a fallback. Fix lifetime problems and only make available as long as necessary
            * Example:
                ```rust
                let s: &'static str = "I have a static lifetime.";
                ```

    * **Generic Type Parameters, Trait Bounds, and Lifetimes Combined**

## Testing <a id="chapter-26543f"></a>

* Help `cargo test -- --help`

* Run tests in sequence rather than parallel, or control number of threads with `cargo test -- --test-threads=1`

* Show the `println!`'s when running tests with `cargo test -- --nocapture` to see printed values for passing tests as well as it disables the output capture behavior

* Run only `ignored` tests with `cargo test -- --ignored` (i.e. where using `#[ignore]` in test file)

* When comparing non-primitive types with `assert_eq!` and `assert_ne!` such as **Structs and Enums** then we must implement `PartialEq`, and `Debug` to print values when an assertion fails. We do this by adding `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition since they are Derivable Traits.

* **Custom Failure Messages** provide more meaning to failed tests to help us debug what happened instead of what we expected to happen.
    * Example:
        ```rust
        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(
                result.contains("Carol"),
                "Greeting did not contain name, value was `{}`", result
            );
        }
        ```
    
* Error Handling
    * Use `should_panic` if want the test to pass if code inside the function panics. Ensure to accompany it with an `expected` parameter so the test harness makes sure the failure message contains the provided text.

    * Example
        ```rust
        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 {
                    panic!("Guess value must be greater than or equal to 1, got {}.",
                        value);
                } else if value > 100 {
                    panic!("Guess value must be less than or equal to 100, got {}.",
                        value);
                }

                Guess { value }
            }
            ...
        }

        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn test_guess_greater_than_100() {
            Guess::new(200);
        }
        ```

    * Examples: [guessing_game](./projects/guessing_game/src/main.rs)

## Threads <a id="chapter-98e538"></a>

* Order of Threads Running
    * Concurrency provides no guarantee of order in which multiple threads will run and may pose risks such as:
        * Race Conditions (threads accessing data in inconsistent order)
        * Deadlocks (two threads waiting for each other to finish using
        data they are both using)
        * Bugs (that may be hard to reproduce)

* Runtime
    * Rust needs a "smaller runtime"
        * Pros
            * Maintains performance by calls to C language
            * Smaller binary
            * Crates may be used that implement M:N threading (M green threads per N OS threads, with larger binary overhead but with more control over which and when threads run and lowers cost of context switching)
        * Cons
            * Less features
            * No built-in "green threading" M:N model since requires "larger language runtime" to manage threads
    * Rust Standard Library provides 1:1 (one OS per one language thread) threading implementation (i.e. where language calls the OS's APIs to create threads) with a thread-related API, since "runtime support" is a priority for Rust

* Sleep & Thread Order & Thread Dependence
    * `thread::sleep` forces a thread to stop execution for a short duration, allowing a different thread to run. The order that threads get run is not guaranteed as it depends on how the OS schedules threads. If the main thread shuts down then child threads spawned from it also stop.

* Join `join` Handles Wait for All Threads to Finish
    * Fix to guarantee spawned thread runs and is completely run by assigning `thread::spawn` return value `JoinHandle` (an "owned" value) to a variable that we can call `join` method on and it will **wait** for its thread to finish before the main thread exits.
    * Calling `join` on the handle will **block** (prevents a thread from performing work or exiting) the thread currently running until the thread represented by the handle terminates (i.e. if child thread is spawned from the main thread that is currently running and `join` is then called on the child thread then the main thread that is currently running will be **blocked** until the child thread terminates, so the main thread will only re-commence work after the child thread has finished its work, instead of both the main thread and the child thread interleaving the work they are doing in parallel at the same time)

* `move` Closures with Threads
    * `move` before the parameter list when creating a new thread to:
        * Share data from one thread to be used in another thread
        * Move data so the new thread takes "ownership" of values by passing a Closure

* Capturing Values in Closures
    * **WITHOUT** Arguments Passed to Closure of New Thread - when not using any data from main thread in code of child spawned thread
    * **WITH** Arguments Passed to Closure of New Thread - captures data from main thread into the code of the child spawn thread 
        * Example: Create Vector in Main Thread and used in Child Spawned Thread
            * Rust Closure **by default infers and tries to "borrow" `v` since it only needs a "reference" to it** to use and capture `v` in its environment that `thread::spawn` runs so we may access `v` in the new thread 
            * Rust Closure must use `move` to override Rust's default of borrowing and instead force the closure to take "ownership" of values it uses (i.e. `v`) rather than allowing Rust to infer it should "borrow" the values (guaranteeing to Rust that the main thread will not use `v` anymore without violating ownership rules). This would then prevent the main thread from being able to `drop` `v` from the main thread afterward.
            * Rust does not know how long the child spawned thread will run and whether the "reference" to `v` will always be valid, or ever be valid in the case where the main thead that spawned the child thread drops `v` before the child thread is even spawned at all (i.e. if we were to run `drop(v);` on the main thread after spawning the child spawned thread that used and captured `v`)
            ```rust
            let v = vec![1, 2, 3];

            let handle = thread::spawn(move || {
                println!("Here's a vector: {:?}", v);
            });

            handle.join().unwrap();
            ```
           
        * See projects/sharding/src/example/threads.rs

### Channels <a id="chapter-a9f98d"></a>

* Definition: Channels are for message passing between threads

* **Message Passing**
    * Safe **message-sending concurrency** using the **Channel** where threads/actors communicate by sending each other messages
    containing data
    * Shares memory by communicating **DO NOT communicate by sharing memory**

    * **Channels** 
        * Definitions
            * `mpsc` stands for **Multiple producer, single consumer**. 
            Rust’s standard library implements channels with multiple sending ends 
            that produce values but only one receiving end that consumes the values
        * **Transmitter**
            * Call methods on transmitter
            * Sending thread to generate values and send down channel
            * Send method `send` takes value to send and returns `Result<T, E>` type.
            If receiving end already dropped then the send operation will return an error.
            * Example:
                ```rust
                use std::sync::mpsc;
                let val = String::from("hi");
                tx.send(val).unwrap();
                ```

        * **Receiver**
            * Checks receiving end for arriving messages
            * Receiver thread receives values and print them
            * Receiver methods:
                * `recv`
                    * Block the Main Thread of execution and waits until value sent down channel
                    * After detecting value from sender it returns `Result<T, E>` type
                    * When sender or receiver end closes the channel it returns an error
                    indicating no more values coming
                * `try_recv`
                    * Non-Blocking and returns `Result<T, E>` type immediately with
                    `Ok` value holding a message if one available and `Err` value if 
                    there aren't any messages.
                    * Example Usage:
                        * Loop that calls `try_recv` often, handles messages if available
                        or do other work before checking for message again 
                * Iterator instead of explicitly using `recv`
                    ```rust
                    let (sender, receiver) = mpsc::channel();
                    ...
                    // Waiting to receive messages from child spawned thread
                    for received in receiver {
                        println!("Received {}", received);
                    }
                    ```
                    
        * **Closed**
            * When either Transmitter or Receiver have dropped

        * **Ownership**
            * After sending a value across a channel to another thread it is error prone
            to continue using it in the original channel since other thread could modify 
            or drop the value before we try to use it in the original channel
                * Reference: https://github.com/rust-lang/book/blob/master/2018-edition/src/ch16-02-message-passing.md

        * **Multiple Senders**
            * Example
                ```rust
                let (sender, receiver) = mpsc::channel();
                let sender1 = mpsc::Sender::clone(&sender);
                let sender2 = mpsc::Sender::clone(&sender);
                ```

* See projects/sharding/src/example/threads.rs

## Comments <a id="chapter-688de0"></a>

* Reference: https://doc.rust-lang.org/reference/comments.html
* Module comments `!//`
* Comments `//`
* `#` is part of the Docsets