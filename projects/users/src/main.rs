// Struct
struct User {
    username: String,
    email: String,
    eye_color: EyeColor,
    sign_in_count: u64,
    active: bool,
}

// Tuple Struct
struct EyeColor(i32, i32, i32);

fn main() {
    let red = EyeColor(255, 0, 0);

    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    println!("User1's email is: {}", user1.email);

    user1.email = String::from("fixedemail@example.com");

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        eye_color: red,
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    let black = EyeColor(0, 0, 0);
    User {
        email: email,
        username: username,
        eye_color: black,
        active: true,
        sign_in_count: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_user_with_username() {
        let email = String::from("someone@example.com");
        let username = String::from("someusername123");
        let actual_user_instance: User = build_user(email, username);
        let expected_response: String = "someusername123".to_string();
        assert_eq!(actual_user_instance.username, expected_response);
    }
}