use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const FILE: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple command-line todo app in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as done
    Done {
        index: usize,
    },
}

fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE).exists() {
        return vec![];
    }

    let mut file = fs::File::open(FILE).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE)
        .expect("Failed to open file for writing");
    file.write_all(json.as_bytes()).expect("Failed to write tasks");
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            tasks.push(Task { description, done: false });
            save_tasks(&tasks);
            println!("Task added.");
        }
        Commands::List => {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{} {} {}", i + 1, status, task.description);
            }
        }
        Commands::Done { index } => {
            if index == 0 || index > tasks.len() {
                println!("Invalid task index.");
            } else {
                tasks[index - 1].done = true;
                save_tasks(&tasks);
                println!("Task marked as done.");
            }
        }
    }
}
