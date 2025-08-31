use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Write};

fn main() {
    println!("🔊 CLI Soundboard");

    // Map of key -> filename
    let sounds: HashMap<u8, &str> = [
        (1, "sounds/clap.mp3"),
        (2, "sounds/ding.mp3"),
        (3, "sounds/wow.mp3"),
    ]
    .iter()
    .cloned()
    .collect();
    for (key, name) in &sounds {
        println!("{}. {}", key, name);
    }
    print!("🎛️ Choose a sound to play (1-3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<u8>() {
        Ok(choice) if sounds.contains_key(&choice) => {
            let path = sounds.get(&choice).unwrap();

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let file = File::open(path).expect("❌ Failed to open audio file");
            let source = Decoder::new(BufReader::new(file)).expect("❌ Failed to decode audio");

            sink.append(source);
            sink.sleep_until_end();
        }
        _ => println!("❌ Invalid choice."),
    }
}
