// Struct
struct User {
    username: String,
    email: String,
    age: i32,
    eye_color: EyeColor,
    ip_address: IpAddr,
    voicemail_message: VoiceMail,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn death(&self) -> Option<&i32> {
        match self.age >= 18 {
            true => {
                println!("Yup, you're gonna die");
                Some(&self.age)
            },
            false => None
        }
    }
}

struct Color(i32, i32, i32);

// Tuple Struct
struct EyeColor(Color);

struct Ipv4Address(u8, u8, u8, u8);

struct Ipv6Address(String);

// Enum - Custom Data Type that embeds address data structs inside enum variants
enum IpAddr {
    V4(Ipv4Address),
    V6(Ipv6Address)
}

// Enum - Embedding struct types in its variants instead of creating separate structs
enum VoiceMail {
    Quit,                           // No associated data
    Version { x: i32, y: i32 },     // Anonymous struct embedded
    Message(String),                // Single string of data
    Color(Color)                    // Anonymous tuple struct embedded
}

// Define Method on Enum
impl VoiceMail {
    fn call(&self) -> String {
        println!("Hello");
        "Hello".to_string()
    }
}

fn main() {
    let red = EyeColor(Color(255, 0, 0));

    /* Parse hard-coded string so it's acceptable to be less robust and just `unwrap`.alloc_jemalloc
     * `parse` returns a `Result` value so compiler makes us handle the
     * possibility of an `Err` variant occurring
     */
    let _user2_ip_address = IpAddr::V6(Ipv6Address(String::from("192.168.0.1")))
                                .parse()
                                .unwrap();

    let _user2_voicemail_message = VoiceMail::Message(String::from("Hi"));

    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
        50
    );

    println!("User1's email is: {}", user1.email);

    user1.email = String::from("fixedemail@example.com");

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        age: 30,
        eye_color: red,
        ip_address: _user2_ip_address,
        voicemail_message: _user2_voicemail_message,
        ..user1
    };

    match user1.death() {
        Some(death) => println!("User1's death is at {:#?}", death),
        None    => println!("Users only find out when they die after 18."),
    }
}

fn build_user(email: String, username: String, age: i32) -> User {
    let black = EyeColor(Color(0, 0, 0));
    let home = IpAddr::V4(Ipv4Address(127, 0, 0, 1));
    let voicemail_message_default = VoiceMail::Message(String::from("Hello"));
    User {
        email: email,
        username: username,
        age: age,
        eye_color: black,
        ip_address: home,
        voicemail_message: voicemail_message_default,
        active: true,
        sign_in_count: 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_user_with_username() {
        let email = String::from("someone@example.com");
        let username = String::from("someusername123");
        let age = 50;
        let actual_user_instance: User = build_user(email, username, age);
        let expected_response: String = "someusername123".to_string();
        assert_eq!(actual_user_instance.username, expected_response);
    }

    #[test]
    fn test_build_user_with_voicemail_message() {
        let email = String::from("someone@example.com");
        let username = String::from("someusername123");
        let age = 50;
        let voicemail_message_default = VoiceMail::Message(String::from("Hello"));
        let mut actual_user_instance: User = build_user(email, username, age);
        actual_user_instance.voicemail_message = voicemail_message_default;
        let expected_response: String = "Hello".to_string();
        assert_eq!(actual_user_instance.voicemail_message.call(), expected_response);
    }

    #[test]
    fn test_user_death() {
        let email = String::from("someone@example.com");
        let username = String::from("someusername123");
        let age = 50;
        let mut actual_user_instance: User = build_user(email, username, age);
        let expected_response: Option<&i32> = Some(&50);
        assert_eq!(actual_user_instance.death(), expected_response);
    }
}