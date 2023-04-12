use std::thread;

pub fn play() {
    // closure with explicit type annotations
    let closure_1 = |x: u32| -> u32 {
        x + 1
    };
    // equivalent closures
    let closure_2 = |x: u32| -> u32 { x + 1 };
    // type annotations inferred from usage when called
    let closure_3 = |x| { x + 1 };
    closure_3(1);
    // equivalent closures without optional brackets or type annotations
    let closure_4 = |x| x + 1;
    closure_4(1);

    // equivalent function
    fn  function_1 (x: u32) -> u32 { x + 1 }

    // closure captures environment values by **borrowing immutably** for use in function body
    // e.g. define closure that captures an immutable reference to a vector since it only needs to print its value
    // so the `name` variable is accessible before and after the call to the closure
    let name = "name 1".to_string();
    // bind variable to closure definition
    let borrows_immutably = || println!("name: {:?}", name);
    println!("name is accessible: {:?}", name);
    borrows_immutably();
    println!("name accessible: {:?}", name);

    // closure captures environment values by **borrowing mutably** for use in function body
    let mut name = "name 1".to_string();
    // bind variable to closure definition
    let mut borrows_mutably = || { name = "name 2".to_string() };
    // **cannot print here** using an immutable borrow since no other borrows are allowed when there is a mutable borrow 
    borrows_mutably();
    // mutable borrow ends since closure isn't used again
    println!("name is accessible: {:?}", name);

    // closure captures environment values by **taking ownership** of them for use in function body
    // by using the `move` keyword
    let mut name = "name 1".to_string();
    // bind variable to closure definition
    let mut takes_ownership = move || {
        println!("name from closure: {:?}", name);
        name = "name 2".to_string();
    };
    takes_ownership();
    // **cannot print here** since the closure has taken ownership so no other borrows are allowed
    // println!("name is NOT accessible: {:?}", name);

    // closure captures environment values by **taking ownership** of them for use in function body
    // by using the `move` keyword (i.e. passing a closure as an argument to a new thread
    // to move the data ownership to it)
    let name = "name 1".to_string();
    let taking_ownership = move || println!("name from thread: {:?}", name);
    thread::spawn(taking_ownership)
        .join()
        .unwrap();

    // closures capture and handle values from the environment, and are affected by
    // which traits the closure implements.
    //
    // traits are how functions and structs can specify what kinds of closures they can use.
    //
    // these `Fn` traits may be specified as trait bounds, for example if `FnOnce` is used it
    // specifies that the closure must be able to be called at most once
    //
    // closures will automatically implement one, two, or all three of these `Fn` traits,
    // in an additive fashion, depending on how the closure’s body handles the values.
    //
    // `FnOnce` closures:
    //    - can be called only once.
    //    - All closures implement at least this trait (since all closures are callable)
    //    - A closure that moves captured values out of its body will **only** implement `FnOnce` and
    //      none of the other `Fn` traits.
    //    - note: to count how many times the function is called capture a mutable reference.
    //    - e.g. `unwrap_or_else`
    //
    // `FnMut` closures: 
    //    - can be called more than once
    //    - don’t move captured values out of their body,
    //    - might mutate the captured values.
    //    - e.g. `sort_by_key` from std library calls the closure multiple times
    //      once for each item in a slice for each struct instance that is being sorted by their keys
    //
    // `Fn` closures:
    //    - can be called more than once without mutating their environment
    //      (important in cases such as calling a closure multiple times concurrently)
    //    - don’t move captured values out of their body
    //    - don’t mutate captured values
    //    - closures that capture nothing from their environment
}
