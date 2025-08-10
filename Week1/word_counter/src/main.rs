use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <file_path>");
        return;
    };
    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", e);
        return;
    }
    let word_count = count_words(&contents);
    println!("Word count: {}", word_count);
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
