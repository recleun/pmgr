use clap::Args;
use crate::utils::get_data;

#[derive(Args)]
pub struct ReadArgs;

impl super::Command for ReadArgs {
    fn run(self, file_name: &str) {
        println!("{:?}", get_data(file_name));
    }
}
