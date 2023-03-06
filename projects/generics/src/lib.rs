// Public API declaration with contents extracted into separate file
pub mod examples;

#[cfg(test)]
mod tests {
    use super::examples::{
        function_definitions
    };

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

    // #[test]
    // fn test_largest_using_i32_list_with_generic_function() {
    //     let actual_number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    //     let expected_response: i32 = 100;
    //     assert_eq!(function_definitions::largest(&actual_number_list), expected_response);
    // }

    // #[test]
    // fn test_largest_using_char_list_with_generic_function() {
    //     let actual_char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    //     let expected_response: char = 'y';
    //     assert_eq!(function_definitions::largest(&actual_char_list), expected_response);
    // }
}
