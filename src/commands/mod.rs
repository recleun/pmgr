use clap::{Parser, Subcommand};
pub mod read;
pub mod check;
pub mod init;
pub mod create;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the project's data
    Read(read::ReadArgs),
    /// Checks for a project in current and parent directories
    Check(check::CheckArgs),
    /// Initialize a project in the current directory
    Init(init::InitArgs),
    /// Create a group in the current project
    Create(create::CreateArgs),
}

pub trait Command {
    fn run(self, file_name: Option<&str>);
}
