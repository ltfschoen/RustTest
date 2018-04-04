## DEPENDENCIES

    * Import type into scope of program if not
    already in the prelude.

    * External dependency `extern crate rand` is equivalent of
    calling `use rand`. Prefix with `rand::` to use rand library.

    * Statement used to import terminal I/O crate

    * Statement used to import the Ordering enum and associated
    variant outcomes of comparison Less, Greater, Equal

    * Statement to import Rng trait to the scope since it defines
    random number generator methods including thread_rng
    that is local to current thread of execution and seeded by the
    operating system. gen_range(exclusive, inclusive)

## VARIABLES

    * Create mutable variable and bind to value
    to a new instance of string type using an
    associated function (aka static method) of
    the String type from the standard library.

    * Shadow previous value of `guess` with new value.
    Bind `guess` to expression `guess.trim().parse()`.
    trim() removes whitespace and newline characters.
    parse() will parse a string into a number.
    Use the colon `:` to annotate the variables type

## TYPE INFERENCE

    * Rust infers as being a string since it has type
    inference.

## FUNCTIONS & ERROR HANDLING

    * Call associated function on with: std::io::stdin()
    to return instance of terminal standard input
    std::io::Stdin. Call readline method on standard
    input handle to obtain user input passing a single
    mutable reference argument to change
    the string content with the user input
    (allow safe and easy access to data without having
    to copy it to memory multiple times).
    read_line returns instance of io::Result submodule
    enum type (with fixed set of enum variant values: Ok, Err)
    that provides an expect method. Either write error
    handling to suppress errors using a match expression
    or crash it by using `expect`.

    * Note that `_` is catch all of all values

    * `continue` means skip to next iteration of the loop

    * Compare two values using cmp and returns variant of
    Ordering enum, then for a match expression choose an
    matching arm pattern to run and decide next
    action based on the variant returned.

## TYPES

    * Multiple number types may have value between 1 and 100,
    where default is i32 (32-bit number), and others are
    u32 (unsigned 32-bit number for small positive numbers)
    and i64 (64-bit number).