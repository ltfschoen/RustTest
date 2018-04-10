/* Generic Function over some type `T` that has
 * one parameter `list` that is a slice of values
 * of type `T` and returns a value of same type `T`.
 * Usage: Call with a slice of `i32` or `char` values.
 */
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* Find largest item in a slice of i32 values
 */
fn largest_i32(list: &[i32]) -> i32 {
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
fn largest_char(list: &[char]) -> char {
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
    println!("The largest number is {}", result);

    // Repeat using the Generic Function
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

#[cfg(test)]
mod tests {
    use ::examples::function_definitions;

    #[test]
    fn test_largest_i32() {
        let actual_number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
        let expected_response: i32 = 100;
        assert_eq!(function_definitions::largest_i32(&actual_number_list), expected_response);
    }

    #[test]
    fn test_largest_char() {
        let actual_char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
        let expected_response: char = 'y';
        assert_eq!(function_definitions::largest_char(&actual_char_list), expected_response);
    }

    #[test]
    fn test_largest_using_i32_list_with_generic_function() {
        let actual_number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
        let expected_response: i32 = 100;
        assert_eq!(function_definitions::largest(&actual_number_list), expected_response);
    }

    #[test]
    fn test_largest_using_char_list_with_generic_function() {
        let actual_char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
        let expected_response: char = 'y';
        assert_eq!(function_definitions::largest(&actual_char_list), expected_response);
    }
}
