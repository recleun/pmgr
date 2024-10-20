use clap::{Parser, Subcommand};
pub mod read;
pub mod list;
pub mod check;
pub mod init;
pub mod create;
pub mod delete;
pub mod watch;
pub mod unwatch;
pub mod add;
pub mod remove;
pub mod task;

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
    /// List a group's data or active groups data
    List(list::ListArgs),
    /// Checks for a project in current and parent directories
    Check(check::CheckArgs),
    /// Initialize a project in the current directory
    Init(init::InitArgs),
    /// Create a group in the current project
    Create(create::CreateArgs),
    /// Delete a group in the current project
    Delete(delete::DeleteArgs),
    /// Watch a group or a list of groups
    Watch(watch::WatchArgs),
    /// Unwatch a group or a list of groups
    Unwatch(unwatch::UnwatchArgs),
    /// Add some data to a group
    Add(add::AddArgs),
    /// Remove some data from a group
    Remove(remove::RemoveArgs),
    /// Do some commands to tasks
    Task(task::TaskArgs),
}

pub trait Command {
    fn run(self, file_name: &str);
}
