use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
// use std::path::Path;

fn main() {
    println!("🗜️ File Compression Tool");
    println!("1. Compress a file");
    println!("2. Decompress a file");
    let choice = input("Enter your choice: ");
    match choice.trim() {
        "1" => {
            let src = input("Source file path:");
            let dest = input("Output .gz file path:");
            if compress_file(&src, &dest).is_ok() {
                println!("✅ File compressed to '{}'", dest);
            } else {
                println!("❌ Compression failed.");
            }
        }
        "2" => {
            let src = input("Source .gz file path:");
            let dest = input("Output decompressed file path:");
            if decompress_file(&src, &dest).is_ok() {
                println!("✅ File decompressed to '{}'", dest);
            } else {
                println!("❌ Decompressed failed.");
            }
        }
        _ => println!("Invalid choice."),
    }
}

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input = File::open(input_path)?;
    let reader = BufReader::new(input);
    let output = File::create(output_path)?;
    let writer = BufWriter::new(output);
    let mut encoder = GzEncoder::new(writer, Compression::default());
    let mut buffer = Vec::new();
    BufReader::new(reader).read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    encoder.finish()?;
    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input = File::open(input_path)?;
    let mut decoder = GzDecoder::new(BufReader::new(input));
    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    writer.write_all(&buffer)?;
    Ok(())
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}
