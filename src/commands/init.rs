use std::{fs, io};
use clap::Args;
use crate::utils;
use crate::utils::ExpectWith;

#[derive(Args)]
pub struct InitArgs;

impl super::Command for InitArgs {
    fn run(self, file_name: Option<&str>) {
        let file_name = file_name.unwrap_or(".pmgr");
        let result = utils::check_data(Some(file_name));
        if let Ok(path) = result {
            return eprintln!("Found already existing project at: {:?}", path);
        }

        match result.unwrap_err() {
            e if e.kind() == io::ErrorKind::NotFound => {
                fs::write(file_name, "{}").expect_with("Failed to initialize project");
                println!("Project intialized successfully");
            }
            e => eprintln!("Couldn't initialize project: {}", e),
        }
    }
}
