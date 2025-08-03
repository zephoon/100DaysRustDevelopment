use std::io::{self, Write};

#[derive(Debug)]
struct Contact {
    id: usize,
    name: String,
    email: String,
    phone: String,
}

fn main() {
    println!("Hello, world!");
    let mut contacts: Vec<Contact> = Vec::new();
    let mut next_id = 1;
    loop {
        println!("\n📇 Contact Manager:");
        println!("1. Add Contact");
        println!("2. View Contacts");
        println!("3. Search Contact");
        println!("4. Delete Contact");
        println!("5. Exit");
        let choice = input("Enter your choice:");
        match choice.trim() {
            "1" => {
                let name = input("Name:");
                let phone = input("Phone:");
                let email = input("Email:");
                contacts.push(Contact {
                    id: next_id,
                    name,
                    email,
                    phone,
                });
                println!("Contact added with ID {}", next_id);
                next_id += 1;
            }
            "2" => {
                if contacts.is_empty() {
                    println!("No contacts.");
                } else {
                    for c in &contacts {
                        println!(
                            "ID: {}, Name: {}, Email: {}, Phone: {}",
                            c.id, c.name, c.email, c.phone
                        );
                    }
                }
            }
            "3" => {
                let query = input("Search by name or email:");
                let results: Vec<&Contact> = contacts
                    .iter()
                    .filter(|c| c.name.contains(&query) || c.email.contains(&query))
                    .collect();
                if results.is_empty() {
                    println!("No match found.");
                } else {
                    for c in &contacts {
                        println!(
                            "ID: {}, Name: {}, Email: {}, Phone: {}",
                            c.id, c.name, c.email, c.phone
                        );
                    }
                }
            }
            "4" => {
                let id = input("Enter contact ID to delete:")
                    .parse::<usize>()
                    .unwrap_or(0);
                let len_before = contacts.len();
                contacts.retain(|c| c.id != id);
                if contacts.len() < len_before {
                    println!("Contact deleted.");
                } else {
                    println!("ID not found.");
                }
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option."),
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
