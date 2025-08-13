use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;

fn main() {
    println!("🧵 Multi-threaded Counter");

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let current = counter_clone.fetch_add(1, Ordering::SeqCst);
                println!("Thread {} incremented counter to {}", i, current + 1);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("🎯 Final count: {}", counter.load(Ordering::SeqCst));
}
