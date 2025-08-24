use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

const STORE_FILE: &str = "kv_store.json";

#[derive(Serialize, Deserialize)]
struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn new() -> Self {
        if let Ok(file) = File::open(STORE_FILE) {
            let reader = BufReader::new(file);
            if let Ok(data) = serde_json::from_reader(reader) {
                return Store { data };
            }
        }
        Store {
            data: HashMap::new(),
        }
    }
    fn save(&self) {
        if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(STORE_FILE) {
            serde_json::to_writer_pretty(&mut file, self).unwrap();
        }
    }
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        self.save();
    }
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save();
    }
    fn list(&self) {
        println!("Stored Keys: ");
        for key in self.data.keys() {
            println!("- {}", key);
        }
    }
}

fn main() {
    println!("🗃️ Key-Value Store (set/get/delete/list/exit)");
    let mut store = Store::new();

    loop {
        let cmd = prompt("\n> ");
        let parts: Vec<&str> = cmd.trim().splitn(3, ' ').collect();
        match parts.get(0).map(|s| s.to_lowercase()) {
            Some(op) if parts.len() == 3 && op == "set" => {
                store.set(parts[1].to_string(), parts[2].to_string());
                println!("✅ Set key '{}' to '{}'", parts[1], parts[2]);
            }
            Some(op) if parts.len() == 2 && op == "get" => match store.get(parts[1]) {
                Some(value) => println!("🔑 {}: {}", parts[1], value),
                None => println!("❌ Key '{}' not found", parts[1]),
            },
            Some(op) if parts.len() == 2 && op == "delete" => {
                store.delete(parts[1]);
                println!("✅ Deleted key '{}'", parts[1]);
            }
            Some(op) if op == "list" => store.list(),
            Some(op) if op == "exit" => {
                println!("👋 Exiting...");
                break;
            }
            _ => println!("❌ Invalid command. Use: set/get/delete/list/exit"),
        }
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
