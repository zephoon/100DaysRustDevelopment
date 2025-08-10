use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

fn main() {
    println!("🗂️ Config File Parser (Key=Value Format)");
    let file_path = prompt("Enter path to config file (e.g. config.txt):");
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let config = parse_config(&content);
            println!("Parsed Config:");
            for (key, value) in config {
                println!("{} = {}", key, value);
            }
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

fn parse_config(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("#") {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    map
}

/// CLI prompt
fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
