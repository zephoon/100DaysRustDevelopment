use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> std::io::Result<()> {
    println!("📤 TCP Client Connecting to 127.0.0.1:7878...");
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let reader = BufReader::new(stream.try_clone()?);
    thread::spawn(move || {
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("Server: {}", msg),
                Err(e) => {
                    println!("❌ Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line?;
        if msg == "exit" {
            println!("👋 Exiting TCP Client.");
            break;
        }
        stream.write_all(msg.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}
