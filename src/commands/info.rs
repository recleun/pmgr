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

        if let Some(title) = &data.information.title {
            println!("Project Title: {}", title);
        }
        if let Some(desc) = &data.information.description {
            println!("Description: {}", desc);
        }
        if let Some(repo) = &data.information.repo {
            println!("Project Title: {}", repo);
        }

        if data.information.title.is_none()
            && data.information.description.is_none()
            && data.information.repo.is_none()
        {
            println!("No project information was set, use `pmgr set` command to add information");
        }
    }
}
