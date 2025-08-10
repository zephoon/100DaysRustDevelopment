use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");
    println!("I am thinking of a number between 1 and 100. Can you guess it?");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("Congratulation! You guessed the number!");
                break;
            }
        };
    }
}
