use crate::data;
use crate::fg_color;
use crate::utils;
use crate::utils::ExpectWith;
use clap::builder::styling;
use clap::{error::ErrorKind, Args, CommandFactory};
use std::{fs, io};

use super::Cli;

#[derive(Args)]
pub struct InitArgs;

impl super::Command for InitArgs {
    fn run(self, file_name: &str) {
        let result = utils::check_data(file_name);
        if let Ok(path) = result {
            let _ = Cli::command()
                .error(
                    ErrorKind::Io,
                    format!("Found already existing project at: {:?}", path),
                )
                .print();
            return;
        }

        match result.unwrap_err() {
            e if e.kind() == io::ErrorKind::NotFound => {
                let project = serde_json::to_string(&data::Project::new())
                    .expect("Failed to serialize project data");
                fs::write(file_name, project).expect_with("Failed to initialize project");
                println!("Project intialized {}", fg_color!("successfully", Green));
            }
            e => {
                let _ = Cli::command()
                    .error(ErrorKind::Io, format!("Failed to initalize project: {}", e))
                    .print();
            }
        }
    }
}
