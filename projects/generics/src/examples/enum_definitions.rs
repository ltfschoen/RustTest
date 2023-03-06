use std::io;

pub fn run() {
    loop {
        println!("Input any value.");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input = user_input;

        // Allow potentially negative numbers
        let _result: () = match user_input.to_string().trim().parse::<i32>() {
            Ok(T) => { println!("Success: {:?}", T); T; },
            Err(E) => { println!("Error: {:?}", E); E; }
        };
    }
}
