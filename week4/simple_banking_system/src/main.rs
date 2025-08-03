use std::io::{self, Write};

#[derive(Debug)]
struct Account {
    id: usize,
    name: String,
    balance: f64,
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n🏦 Banking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Exit");

        match input("Choose an option:").as_str() {
            "1" => {
                let name = input("Account holdername:");
                let amount = input("Initial deposit amount:")
                    .parse::<f64>()
                    .unwrap_or(0.0);
                accounts.push(Account {
                    id: next_id,
                    name,
                    balance: amount,
                });
                println!("Account created successfully with ID: {}", next_id);
                next_id += 1;
            }
            "2" => {
                let id = input("Account ID:").parse::<usize>().unwrap_or(0);
                match accounts.iter().find(|acc| acc.id == id) {
                    Some(acc) => {
                        println!(
                            "Account ID: {}, Name: {}, Balance: ${:.2}",
                            acc.id, acc.name, acc.balance
                        );
                    }
                    None => {
                        println!("Account not found.");
                    }
                }
            }
            "3" => {
                let id = input("Account ID: ").parse::<usize>().unwrap_or(0);
                let amt = input("Amount to deposit: ").parse::<f64>().unwrap_or(0.0);
                match accounts.iter_mut().find(|acc| acc.id == id) {
                    Some(acc) => {
                        acc.balance = amt;
                        println!(
                            "Account ID: {}, Name: {}, New Balance: ${:.2}",
                            acc.id, acc.name, acc.balance
                        );
                    }
                    None => {
                        println!("Account not found.");
                    }
                }
            }
            "4" => {
                let id = input("Account ID: ").parse::<usize>().unwrap_or(0);
                let amt = input("Amount to withdraw: ").parse::<f64>().unwrap_or(0.0);
                match accounts.iter_mut().find(|acc| acc.id == id) {
                    Some(acc) => {
                        if acc.balance >= amt {
                            acc.balance -= amt;
                            println!(
                                "Account ID: {}, Name: {}, New Balance: ${:.2}",
                                acc.id, acc.name, acc.balance
                            );
                        } else {
                            println!("❌ Insufficient funds.");
                        }
                    }
                    None => {
                        println!("Account not found.");
                    }
                }
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => {}
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
