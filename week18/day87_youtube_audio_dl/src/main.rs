use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("🎧 YouTube Audio Downloader");

    print!("🔗 Enter YouTube URL: ");
    io::stdout().flush().unwrap();
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();

    if url.is_empty() {
        println!("❌ No URL provided.");
        return;
    }
    println!("⬇️ Downloading audio...");

    let status = Command::new("yt-dlp")
        .args([
            "--extract-audio",
            "--audio-format",
            "mp3",
            "--output",
            "%(title)s.%(ext)s",
            url,
        ])
        .status();
    match status {
        Ok(s) if s.success() => println!("✅ Audio downloaded successfully!"),
        Ok(_) => println!("❌ yt-dlp returned an error."),
        Err(e) => println!("❌ Failed to run yt-dlp: {}", e),
    }
}
