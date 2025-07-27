use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");
    println!("Instructions: Enter 'rock', 'paper', 'scissors'. Type 'quit' to exit.");
    loop {
        println!("\n Make your choice:");
        let user_choice = get_user_choice();
        if user_choice == "quit" {
            println!("Thanks for playing!");
            break;
        }
        let computer_choice = get_computer_choice();
        println!("Computer chose: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GamerResult::Win => println!("You win!"),
            GamerResult::Lose => println!("You lose!"),
            GamerResult::Draw => println!("It's a draw!"),
        }
    }
}

fn get_user_choice() -> String {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim().to_lowercase();
    match choice.as_str() {
        "rock" | "paper" | "scissors" | "quit" => choice,
        _ => {
            println!("Invalid input. Please enter 'rock', 'paper', 'scissors', or 'quit'.");
            get_user_choice()
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let random_index = rand::thread_rng().gen_range(0..choices.len());
    choices[random_index].to_string()
}

enum GamerResult {
    Win,
    Lose,
    Draw,
}

fn determine_winner(user_choice: &str, computer_choice: &str) -> GamerResult {
    match (user_choice, computer_choice) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => GamerResult::Win,
        (a, b) if a == b => GamerResult::Draw,
        _ => GamerResult::Lose,
    }
}
