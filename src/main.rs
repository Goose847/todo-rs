use clap::Parser;
use clap::Subcommand;


// Command Line Interface
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {text: String},
    Remove {id: usize},
    Done {id: usize},
    List,
}


// Data models
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    text: String,
    done: bool,
}

struct TaskList(Vec<Task>);

// Orchastration and Wiring
fn main() {
    run() 
}

fn run() {
    let _cli = Cli::parse();
}
