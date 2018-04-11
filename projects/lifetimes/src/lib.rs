/* Given two strings, determine the longest.
 * - Parameters are String Slice "references" so the function 
 *   does not take "ownership" of the parameters
 * - Return type contains a "borrowed" value of either `x` or `y`
 *   that needs a Generic Lifetime Parameter associated so 
 *   the Borrow Checker knows the "lifetimes" of `x` and `y`.
 * - Lifetime Annotations are used in the Function Signature as Constraints 
 *   for Rust to enforce that for lifetime `'a` the function takes two parameters
 *   that are both string slices that live at least as long as 
 *   lifetime `'a`, as will the string slice returned from the function. 
 */
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
