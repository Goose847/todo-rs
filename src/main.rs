use std::error::Error;
use std::fmt;
use std::path::Path;

use clap::Parser;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json;

// Command Line Interface
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { text: String },
    Remove { id: usize },
    Done { id: usize },
    List,
}

// Error handling
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    NotFound(usize),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Json(e) => write!(f, "Json error: {}", e),
            AppError::NotFound(id) => write!(f, "{} not found", id),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Json(e)
    }
}

// Data models
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    text: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    list: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    pub fn add(&mut self, text: String) {
        self.list.push(Task {
            id: self.next_id,
            text,
            done: false,
        });
        self.next_id += 1;
    }

    pub fn mark_done(&mut self, id: usize) {
        let task: Option<&mut Task> = self.list.iter_mut().find(|task| task.id == id);
        task.unwrap().done = true;
    }

    pub fn remove_task(&mut self, id: usize) {
        if let Some(index) = self.list.iter().position(|task| task.id == id) {
            self.list.remove(index);
        }
    }

    pub fn display_tasks(&self) {
        for task in &self.list {
            let mut check = "[]";
            if task.done {
                check = "[x]";
            }
            println!("{0} - {1}: {2}", task.id, task.text, check);
        }
    }
}

// IO
fn load(path: &Path) -> Result<TaskList, Box<dyn Error>> {
    let json_string = std::fs::read_to_string(path).unwrap();

    if json_string.is_empty() {
        return TaskList;
    }
    let tasks = serde_json::from_str(&json_string)?;

    Ok(tasks)
}
fn save(path: &Path, tasks: &TaskList) -> Result<(), Box<dyn Error>> {
    let contents: String = serde_json::to_string(tasks).unwrap();
    std::fs::write(path, contents).unwrap();
    Ok(())
}

// Orchastration and Wiring
fn main() {
    run()
}

fn run() {
    let cli = Cli::parse();

    let default_file: &Path = Path::new("/Users/connorgoosen/.todo_rs.json");
    let mut list: TaskList = load(default_file).unwrap();

    match cli.command {
        Commands::Add { text } => list.add(text),
        Commands::Done { id } => list.mark_done(id),
        Commands::Remove { id } => list.remove_task(id),
        Commands::List => list.display_tasks(),
    };

    let _ = save(default_file, &list);
}
