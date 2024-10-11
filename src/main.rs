use clap::Parser;
use pmgr::commands::{Cli, Commands, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => args.run(None),
        Commands::Read(args) => args.run(None),
        Commands::Init(args) => args.run(None),
    }
}
