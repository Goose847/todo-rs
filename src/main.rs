use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use clap::Parser;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json;

const default_file: &str = "/Users/connor/.todo_rs.json";

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
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let tasks = serde_json::from_reader(reader)?;

    Ok(tasks)
}
fn save(path: &Path, tasks: &TaskList) -> Result<(), Box<dyn Error>> {
    let contents: String = serde_json::to_string(tasks).unwrap();
    fs::write(path, contents).unwrap();
    Ok(())
}

// Orchastration and Wiring
fn main() {
    run()
}

fn run() {
    let _cli = Cli::parse();
}
