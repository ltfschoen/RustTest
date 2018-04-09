extern crate rand;

use std::io;
use std::process;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    // Getter
    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn comparison(&self, secret_number: &u32) -> i32 {
        match self.value.cmp(&(*secret_number as i32)) {
            Ordering::Less => {
                println!("Too small!");
                return -1;
            },
            Ordering::Greater => {
                println!("Too big!");
                return 1;
            },
            Ordering::Equal => {
                println!("You win!");
                return 0;
            }
        }
    }
}

fn sample(guess: &str) -> String {
    return guess.to_string();
}

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = sample(&guess);

        // Allow potentially negative numbers
        let guess: Guess = match guess.trim().parse::<i32>() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {:?}", guess);

        let result = guess.comparison(&secret_number);
        if result == 0 { process::exit(1); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let actual_guess: &str = "1";
        let expected_response: String = "1".to_string();
        assert_eq!(sample(&actual_guess), expected_response);
    }

    #[test]
    fn test_guess() {
        let actual_guess_number: i32 = 99;
        let actual_guess_instance: Guess = Guess::new(actual_guess_number);
        let expected_guess_instance_value: i32 = actual_guess_instance.value();
        assert_eq!(99, expected_guess_instance_value);
    }

    #[test]
    fn test_comparison() {
        let actual_guess_number: i32 = 50;
        let actual_guess_instance: Guess = Guess::new(actual_guess_number);
        let actual_secret_number: u32 = 50;
        let actual_comparison_result: i32 = actual_guess_instance.comparison(&actual_secret_number);
        let expected_comparison_result: i32 = 0;
        assert_eq!(actual_comparison_result, expected_comparison_result);
    }
}