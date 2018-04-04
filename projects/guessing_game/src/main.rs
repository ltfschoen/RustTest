extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn sample(guess: &str) -> String {
    return guess.to_string();
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = sample(&guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sample() {
        let actual_guess: (String) = "1".to_string();
        let expected_response: String = "1".to_string();
        assert_eq!(sample(&actual_guess), expected_response);
    }
}