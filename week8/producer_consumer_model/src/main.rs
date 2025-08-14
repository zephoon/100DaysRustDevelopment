use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🔄 Producer-Consumer Demo");
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            println!("📤 Producing task {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
        println!("✅ Producer finished.");
    });
    let consumer = thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            println!("📥 Consuming task {}", task);
            thread::sleep(Duration::from_millis(500)); // simulate
        }
    });
    producer.join().unwrap();
    thread::sleep(Duration::from_secs(2));
    println!("👋 All tasks processed.");
}
