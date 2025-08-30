use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file-or-directory-to-watch>", args[0]);
        return;
    }
    let path = Path::new(&args[1]);
    println!("👁️ Watching for changes in: {}", path.to_string_lossy());
    let (tx, rx) = channel();
    let config = Config::default()
        .with_compare_contents(true)
        .with_poll_interval(Duration::from_secs(1));
    let mut watcher: RecommendedWatcher = Watcher::new(tx, config).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(e) => {
                    if let EventKind::Modify(_) = e.kind {
                        println!("📝 File modified: {:?}", e.paths);
                    } else {
                        println!("📦 Event: {:?}", e);
                    }
                }
                Err(e) => eprintln!("❌ Watch error: {:?}", e),
            },
            Err(_) => break,
        }
    }
}
