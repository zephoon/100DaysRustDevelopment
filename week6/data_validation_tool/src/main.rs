use regex::Regex;
use std::io::{self, Write};

fn main() {
    println!("✅ Data Validation Tool");
    loop {
        println!("\nChoose what to validate:");
        println!("1. Email");
        println!("2. Phone Number");
        println!("3. Password Strength");
        println!("4. Exit");

        match input("Your Choice:").as_str() {
            "1" => {
                let email = input("Enter email:");
                if is_valid_email(&email) {
                    println!("✅ Valid email.");
                } else {
                    println!("❌ Invalid email.");
                }
            }
            "2" => {
                let phone = input("Enter phone number (e.g., +1234567890): ");
                if is_valid_phone(&phone) {
                    println!("✅ Valid phone number.");
                } else {
                    println!("❌ Invalid phone number.");
                }
            }
            "3" => {
                let pwd = input("Enter password: ");
                if is_strong_password(&pwd) {
                    println!("✅ Strong password.");
                } else {
                    println!("❌ Weak password (must be 8+ chars, contain upper, lower, digit, and symbol).");
                }
            }
            "4" => {
                println!("Existing.");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[\w\.\-]+@[\w\.-]+\.\w+$").unwrap();
    re.is_match(email)
}

fn is_valid_phone(phone: &str) -> bool {
    let re = Regex::new(r"^\+?[0-9]{10,15}$").unwrap();
    re.is_match(phone)
}

fn is_strong_password(pwd: &str) -> bool {
    let re = Regex::new(r"[\w\.\-]+@[\w\.-]+\.\w+$").unwrap();
    re.is_match(pwd)
}
