use clap::Parser;
use pmgr::commands::{Cli, Commands, Command};

#[cfg(debug_assertions)]
const FILE_NAME: &str = ".debug.pmgr";
#[cfg(not(debug_assertions))]
const FILE_NAME: &str = ".pmgr";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => args.run(FILE_NAME),
        Commands::Read(args) => args.run(FILE_NAME),
        Commands::List(args) => args.run(FILE_NAME),
        Commands::Info(args) => args.run(FILE_NAME),
        Commands::Init(args) => args.run(FILE_NAME),
        Commands::Set(args) => {
            match args.set_commands {
                pmgr::set::SetCommands::Title(args) => args.run(FILE_NAME),
                pmgr::set::SetCommands::Description(args) => args.run(FILE_NAME),
                pmgr::set::SetCommands::Repo(args) => args.run(FILE_NAME),
            }
        },
        Commands::ShellCompletions(args) => args.run(),
        Commands::Create(args) => args.run(FILE_NAME),
        Commands::Delete(args) => args.run(FILE_NAME),
        Commands::Watch(args) => args.run(FILE_NAME),
        Commands::Unwatch(args) => args.run(FILE_NAME),
        Commands::Add(args) => {
            match args.add_commands {
                pmgr::add::AddCommands::Note(args) => args.run(FILE_NAME),
                pmgr::add::AddCommands::Task(args) => args.run(FILE_NAME),
            }
        },
        Commands::Remove(args) => {
            match args.remove_commands {
                pmgr::remove::RemoveCommands::Note(args) => args.run(FILE_NAME),
                pmgr::remove::RemoveCommands::Task(args) => args.run(FILE_NAME),
            }
        },
        Commands::Task(args) => {
            match args.task_commands {
                pmgr::task::TaskCommands::Complete(args) => args.run(FILE_NAME),
                pmgr::task::TaskCommands::Undo(args) => args.run(FILE_NAME),
                pmgr::task::TaskCommands::Progress(args) => args.run(FILE_NAME),
            }
        },
    }
}
