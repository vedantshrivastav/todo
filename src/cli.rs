use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "todo", about = "A simple todo manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add { title: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}
