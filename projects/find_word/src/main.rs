
/* Immutable "borrow" occurs when String Slice value passed to `first_word` function.
 * `first_word` function also compatible with String Literal Slices.
 * Not allowed to take mutable reference to truncate a immutable with `my_string.clear()`.
 */
fn main() {
    let my_string = String::from("hello world");

    println!("Given string {}: ", my_string);

    let _word = first_word(&my_string);

    let _my_string_literal = "hello world";

    let _word = first_word(&_my_string_literal[..]);
}

/* Purpose: Return "String Slice" value that is tied to the underlying data.
 * Parameter "borrowed" is the "reference" since we do not want ownership
 * and we use `&str` instead of `&String` so we may pass a String Slice directly
 * or a "reference" to a String.
 * Convert `String` into array of bytes. Iterate over `String` element by element
 * using `iter` that returns each element in a collection, and wrap `iter` with
 * `enumerate` to return each element in a tuple instead.
 * (i.e. (<index>, <reference_to_element>), which is (i, &item), where &item is
 * a reference to a single byte in the tuple).
 * Check if value is a space using Byte Literal Syntax.
 * Return "String Slice" using start of string (start index) and index of the space (end index).
 * If cannot find a space in the string, it means the whole string is one word
 * so the entire string should be returned with an expression.
 */
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let actual_first_word: String = String::from("hello world");
        // let expected_response: String = "hello".to_string();
        let expected_response: &str = "hello";
        assert_eq!(first_word(&actual_first_word), expected_response);
    }
}
