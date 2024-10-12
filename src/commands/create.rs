use crate::utils;
use crate::data::{Project, Group};
use std::{fs, process};
use clap::Args;

#[derive(Args)]
pub struct CreateArgs {
    group_name: String,
}

impl super::Command for CreateArgs {
    fn run(self, file_name: Option<&str>) {
        if let Ok(path) = utils::check_data(file_name) {
            let mut data: Project = utils::get_data(file_name);
            data.groups.push(Group::new(&self.group_name));

            let data = serde_json::to_string(&data).expect("Failed to serialize project data");
            fs::write(path, data).expect("Failed to write project data");

            println!("Added group `{}` to project successfully", self.group_name);
        } else {
            eprintln!("Could not find a project in current directory or parents");
            process::exit(-1);
        }
    }
}
