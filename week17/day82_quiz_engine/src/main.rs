use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Question {
    question: String,
    answer: String,
}

fn main() {
    let file = File::open("questions.csv").expect("❌ Failed to open CSV file");
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut score = 0;
    let mut total = 0;
    println!("🧠 Welcome to the Rust Quiz!");
    println!("----------------------------\n");
    for result in reader.deserialize::<Question>() {
        let q: Question = result.expect("❌ Failed to parse row");
        total += 1;
        println!("❓ {}", q.question);
        print!("Your answer: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let user_answer = input.trim();
        if user_answer.eq_ignore_ascii_case(&q.answer) {
            println!("✅ Correct!\n");
            score += 1;
        } else {
            println!("❌ Incorrect! Correct answer: {}\n", q.answer);
        }
    }

    println!("📊 You scored {}/{}!", score, total);
}
