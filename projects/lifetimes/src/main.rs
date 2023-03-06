extern crate lifetimes;

use lifetimes::{longest, longest_with_announcement};

fn main() {
    let _string1 = String::from("abcd"); // `String` type
    {
        let _string2 = "xyz"; // String Literal type

        let _result = longest(_string1.as_str(), _string2);
        println!("The longest string is {}", _result);

        let _result2 = longest_with_announcement(_string1.as_str(), _string2, "welcome aboard");
    }
}