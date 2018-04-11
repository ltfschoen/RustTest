extern crate lifetimes;

use lifetimes::{longest, longest_with_announcement};

fn main() {
    let string1 = String::from("abcd"); // `String` type
    {
        let string2 = "xyz"; // String Literal type

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);

        let result2 = longest_with_announcement(string1.as_str(), string2, "welcome aboard");
    }
}