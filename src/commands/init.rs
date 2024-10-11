use crate::utils;
use crate::utils::ExpectWith;
use crate::data;
use std::{fs, io};
use clap::Args;

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
                let project = serde_json::to_string(&data::Project::new()).expect("Failed to serialize project data");
                fs::write(file_name, project).expect_with("Failed to initialize project");
                println!("Project intialized successfully");
            }
            e => eprintln!("Couldn't initialize project: {}", e),
        }
    }
}
