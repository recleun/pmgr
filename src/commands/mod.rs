use std::{fs, io::{self, Write}};
use clap::{self, Args, CommandFactory, Parser, Subcommand};
use clap_complete::aot::{generate, Generator, Shell};
pub mod read;
pub mod list;
pub mod check;
pub mod init;
pub mod set;
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
    /// Print the project's JSON data
    Read(read::ReadArgs),
    /// List a group's data or active groups data
    List(list::ListArgs),
    /// Checks for a project in current and parent directories
    Check(check::CheckArgs),
    /// Initialize a project in the current directory
    Init(init::InitArgs),
    /// Set project's information fields
    Set(set::Set),
    /// Create a group in the current project
    Create(create::CreateArgs),
    /// Delete a group in the current project
    Delete(delete::DeleteArgs),
    /// Watch a group or a list of groups
    Watch(watch::WatchArgs),
    /// Unwatch a group or a list of groups
    Unwatch(unwatch::UnwatchArgs),
    /// Add some data to a group
    Add(add::Add),
    /// Remove some data from a group
    Remove(remove::Remove),
    /// Do some commands to tasks
    Task(task::Task),
    /// Generate shell completions for pmgr
    ShellCompletions(ShellCompletionArgs),
}

pub trait Command {
    fn run(self, file_name: &str);
}

#[derive(Args)]
pub struct ShellCompletionArgs {
    shell: Shell,
    path: Option<String>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command, out: &mut dyn Write) {
    generate(gen, cmd, cmd.get_name().to_string(), out);
}

impl ShellCompletionArgs {
    pub fn run(self) {
        let mut cmd = Cli::command();

        println!("Generating shell completions for {}...", self.shell);

        if let Some(path) = self.path {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .expect("Failed to write completions to file");

            print_completions(self.shell, &mut cmd, &mut file);
        } else {
            print_completions(self.shell, &mut cmd, &mut io::stdout());
        }
    }
}
