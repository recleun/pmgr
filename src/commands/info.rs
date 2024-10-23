use crate::utils;
use clap::Args;

#[derive(Args)]
pub struct InfoArgs;

impl super::Command for InfoArgs {
    fn run(self, file_name: &str) {
        let Some(data) = utils::get_data(file_name) else {
            return;
        };

        println!();

        let mut info_displayed = false;

        if let Some(title) = &data.information.title {
            println!("Project Title: {}", title);
            info_displayed = true;
        }
        if let Some(desc) = &data.information.description {
            println!("Description: {}", desc);
            info_displayed = true;
        }
        if let Some(repo) = &data.information.repo {
            println!("Project Title: {}", repo);
            info_displayed = true;
        }

        if !info_displayed {
            println!("No project information was set, use `pmgr set` command to add information");
        }
    }
}
