use clap::Args;
use crate::utils;

#[derive(Args)]
pub struct CheckArgs;

impl super::Command for CheckArgs {
    fn run(self, file_name: Option<&str>) {
        match utils::check_data(file_name) {
            Ok(path) => println!("Found project at {:?}", path),
            Err(e) => eprintln!("Could not check for project: {}", e),
        }
    }
}
