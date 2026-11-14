mod cli;
use clap::Parser;
use cli::{Cli, Command}; // needed for Cli::parse()
use serde::{Deserialize, Serialize};
mod load_task;
use load_task::load_tasks;
use load_task::save_tasks;
use std::fs;
use std::io::ErrorKind;

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    status: Status,
}

const FILE: &str = "tasks.json";
//functions
fn main() {
    let mut tasks = match load_tasks(FILE) {
        Ok(t) => t,
        Err(e) => {
            println!("Error loading tasks: {}", e);
            return;
        }
    };
    let cli = Cli::parse();

    match cli.command {
        Command::Add { title } => {
            let id = add_task(&mut tasks, title);
            println!("Added task {}", id);
        }
        Command::List => list_tasks(&tasks),
        Command::Done { id } => {
            if let Err(e) = complete_task(&mut tasks, id) {
                println!("Error: {}", e);
            }
        }
        Command::Delete { id } => {
            if let Err(e) = delete_task(&mut tasks, id) {
                println!("Error: {}", e);
            }
        }
    }
    if let Err(e) = save_tasks(FILE, &tasks) {
        println!("Error saving tasks: {}", e);
    }
}

fn add_task(tasks: &mut Vec<Task>, title: String) -> u32 {
    // compute the id
    let mut new_task_id: u32;
    if tasks.len() == 0 {
        new_task_id = 1;
    } else {
        new_task_id = tasks.iter().map(|t| t.id).max().unwrap() + 1;
    }
    // Build the task
    let task = Task {
        id: new_task_id,
        title: String::from(title),
        status: Status::Pending,
    };
    tasks.push(task);
    return new_task_id;
}

fn complete_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.status = Status::Done;
            Ok(())
        }
        None => Err(format!("Task {} not found", id)),
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: u32) -> Result<Task, String> {
    match tasks.iter().position(|t| t.id == id) {
        Some(index) => Ok(tasks.remove(index)),
        None => Err(format!("Task {} not found", id)),
    }
}
fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("{}. {} [{:?}]", task.id, task.title, task.status);
    }
}
