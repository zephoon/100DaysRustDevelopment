use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    loop {
        println!("\n To-Do List Menu:");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = get_input("Enter your choice:");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("Tasks saved. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to input");
    input
}
fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description:");
    let id = tasks.len() + 1; // Simple ID generation
    let task = Task {
        id,
        description: description.trim().to_string(),
        completed: false,
    };
    tasks.push(task);
    println!("Task added successfully.");
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for task in tasks {
            println!(
                "{} - {}: {}",
                task.id,
                if task.completed { " " } else { "X" },
                task.description
            );
        }
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = File::create("tasks.json").expect("Failed to create tasks.json");
    file.write_all(json.as_bytes())
        .expect("Failed to write tasks.json");
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to mark as completed:");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task {} marked as completed.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid task ID.")
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to delete:");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(index) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(index);
            println!("Task {} deleted successfully.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid task ID.");
    }
}
