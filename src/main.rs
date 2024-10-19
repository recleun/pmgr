use clap::Parser;
use pmgr::commands::{Cli, Commands, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => args.run(None),
        Commands::Read(args) => args.run(None),
        Commands::Init(args) => args.run(None),
        Commands::Create(args) => args.run(None),
        Commands::Watch(args) => args.run(None),
        Commands::Unwatch(args) => args.run(None),
        Commands::Add(args) => args.run(None),
        Commands::Remove(args) => args.run(None),
    }
}
