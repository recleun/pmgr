use crate::utils;
use clap::{error::ErrorKind, Args, CommandFactory};

use super::Cli;

#[derive(Args)]
pub struct CheckArgs;

impl super::Command for CheckArgs {
    fn run(self, file_name: &str) {
        match utils::check_data(file_name) {
            Ok(path) => println!("Found project at {:?}", path),
            Err(e) => {
                let _ = Cli::command()
                    .error(
                        ErrorKind::Io,
                        format!("Failed to check for a project: {}", e),
                    )
                    .print();
            }
        }
    }
}
