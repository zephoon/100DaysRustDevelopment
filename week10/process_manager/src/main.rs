use std::io::{self, Write};
use sysinfo::{Pid, Process, Signal, System};

fn main() {
    println!("🧠 Rust Process Manager");
    let mut sys = System::new_all();
    sys.refresh_all();

    loop {
        println!("\n1. List all processes");
        println!("2. Search process by name");
        println!("3. Kill process by PID");
        println!("4. Exit");

        let choice = prompt("Choose an option: ");
        match choice.as_str() {
            "1" => list_processes(&sys),
            "2" => {
                let name = prompt("Enter name to search: ");
                search_processes(&sys, &name);
            }
            "3" => {
                let pid_str = prompt("Enter PID to kill: ");
                if let Ok(pid) = pid_str.parse::<u32>() {
                    let pid = Pid::from_u32(pid);
                    kill_process(pid);
                } else {
                    println!("❌ Invalid PID.");
                }
            }
            "4" => {
                println!("👋 Exiting Process Manager.");
                break;
            }
            _ => println!("❌ Invalid choice."),
        }

        sys.refresh_all();
    }
}

fn list_processes(sys: &System) {
    println!("{:<8} {:<20} {:<10}", "PID", "Name", "CPU%");
    for proc in sys.processes().values() {
        println!(
            "{:<8} {:<20} {:>6.2}",
            proc.pid(),
            proc.name(),
            proc.cpu_usage()
        );
    }
}

fn search_processes(sys: &System, keyword: &str) {
    let keyword = keyword.to_lowercase();
    let found: Vec<_> = sys
        .processes()
        .values()
        .filter(|proc| proc.name().to_lowercase().contains(&keyword))
        .collect();
    if found.is_empty() {
        println!("❌ No processes found matching '{}'.", keyword);
    } else {
        println!("{:<8} {:<20} {:<10}", "PID", "Name", "CPU%");
        for p in found {
            println!("{:<8} {:<20} {:>6.2}", p.pid(), p.name(), p.cpu_usage());
        }
    }
}

fn kill_process(pid: Pid) {
    let mut sys = System::new_all();
    sys.refresh_processes();
    if let Some(process) = sys.process(pid) {
        if process.kill_with(Signal::Kill).is_some() {
            println!("✅ Killed process {} ({})", pid, process.name());
        } else {
            println!("❌ Failed to send kill signal.");
        }
    } else {
        println!("❌ Process with PID {} not found.", pid);
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
