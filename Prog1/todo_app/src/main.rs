use clap::{Parser, Subcommand};
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "simple to do application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Complete { id: usize },
    Remove { id: usize },
}

#[derive(Debug)]
enum TodoError {
    FileError(String),
    ParseError(String),
    TaskNotFound(String),
    InvalidInput(String),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::FileError(msg) => write!(f, "File Error: {}", msg),
            TodoError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            TodoError::TaskNotFound(msg) => write!(f, "Task {} nt found", msg),
            TodoError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    task: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            task: Vec::new(),
            next_id: 1,
        }
    }

    fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(filename).exists() {
            let contents = fs::read_to_string(filename)?;
            let todo_list = serde_json::from_str(&contents)?;
            Ok(todo_list)
        } else {
            Ok(TodoList::new())
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn add_task(&mut self, description: String) -> Result<(), TodoError> {
        if description.trim().is_empty() {
            return Err(TodoError::InvalidInput(
                "Description cannot be empty".to_string(),
            ));
        }

        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.task.push(task);
        self.next_id += 1;
        Ok(())
    }

    fn complete_task(&mut self, id: usize) -> Result<(), TodoError> {
        match self.task.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.completed = true;
                Ok(())
            }
            None => Err(TodoError::TaskNotFound(id.to_string())),
        }
    }
}

fn run_command(cli: Cli) -> Result<(), TodoError> {
    let mut todo_list =
        TodoList::load_from_file("tasks.json").map_err(|e| TodoError::FileError(e.to_string()))?;

    match cli.command {
        Commands::Add { task } => {
            todo_list.add_task(task)?;
            println!("Task added succesfully");
        }
        Commands::Complete { id } => {
            todo_list.complete_task(id)?;
            println!("Task {} marked as complete", id);
        }
        _ => {}
    }

    todo_list
        .save_to_file("tasks.json")
        .map_err(|e| TodoError::FileError(e.to_string()))?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_command(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_add_task() {
        let mut todo_list = TodoList::new();
        let res = todo_list.add_task("Test task".to_string());
        assert!(res.is_ok());
        assert_eq!(todo_list.task.len(), 1);
        assert_eq!(todo_list.task[0].description, "Test task");
    }

    #[test]
    fn test_empty_task() {
        let mut todo_list = TodoList::new();
        let res = todo_list.add_task(" ".to_string());
        assert!(res.is_err());
    }
}
