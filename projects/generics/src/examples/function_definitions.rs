/* Generic Function over some type `T` that has
 * one parameter `list` that is a slice of values
 * of type `T` and returns a value of same type `T`.
 * Usage: Call with a slice of `i32` or `char` values.
 */
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        /* Overcome error `binary operation `>` cannot be applied to type `T`
         * an implementation of `std::cmp::PartialOrd` might be missing for `T`.
         * cannot move out of type `[T]`, a non-copy slice, cannot move out of borrowed content
         * 
         * We are trying to compare two values of type `T` using `>` operator
         * that is defined as default method of standard library trait
         * `std::cmp::PartialOrd`, so we must specify `PartialOrd` (already in the
         * scope from the Prelude) in the 
         * Trait Bounds for `T` so that the `largest` function will work on slices 
         * of any type `T` that we compare.
         * With `largest` function Generic now, we also need to allow for `list` 
         * parameter to have types that do not implement `Copy` Trait, which would
         * prevent us from assigning them to the `largest` variable to store them on the stack
         * since they are not of a known size.
         * Solve by only allowing this code to be called with types that implement the `Copy` Trait
         * by adding `Copy` to the Trait Bounds of `T` (i.e. `i32`, `char`).
         * Note: To remove restriction so the `largest` function allows types other than those that
         * implement the `Copy` Trait, we can specify `T` has Trait Bound `Clone` instead of `Copy`
         * so we clone each value in the slice when we want the `largest` function to have "ownership"
         * and this would potentially make more heap allocations where types that own heap data are 
         * used like `String` (which may be slow for large amounts of data).
         */
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* Find largest item in a slice of i32 values
 */
pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* Find largest item in slice of char values
 */
pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');

    // Repeat using the Generic Function
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {} (using Generics and Trait Bounds)", result);

    // Repeat using the Generic Function
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {} (using Generics and Trait Bounds)", result);
}
