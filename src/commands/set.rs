use clap::{error::ErrorKind, Args, CommandFactory, Parser, Subcommand};
use url::Url;

use crate::utils;

use super::Cli;

#[derive(Subcommand)]
pub enum Commands {
    Set(Set),
}

#[derive(Parser)]
pub struct Set {
    #[structopt(subcommand)]
    pub set_commands: SetCommands,
}

#[derive(Subcommand)]
pub enum SetCommands {
    /// Set the title of the project
    Title(SetTitleArgs),
    /// Set the description of the project
    Description(SetDescArgs),
    /// Set the repository link of the project
    Repo(SetRepoArgs),
}

#[derive(Args)]
pub struct SetTitleArgs {
    /// The new title for the project
    pub title: String,
}

#[derive(Args)]
pub struct SetDescArgs {
    /// The new description for the project
    pub desc: Vec<String>,
}

#[derive(Args)]
pub struct SetRepoArgs {
    /// The link of the project's repository
    pub repo: String,
}

impl super::Command for SetTitleArgs {
    fn run(self, file_name: &str) {
        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };

        data.information.title = Some(self.title);

        utils::write_data(file_name, &data);

        println!("New project title set successfully");
    }
}

impl super::Command for SetDescArgs {
    fn run(self, file_name: &str) {
        if self.desc.is_empty() {
            let _ = Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No description was specified",
                )
                .print();
            return;
        }

        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };

        data.information.description = Some(self.desc.join(" "));

        utils::write_data(file_name, &data);

        println!("New project description set successfully");
    }
}

impl super::Command for SetRepoArgs {
    fn run(self, file_name: &str) {
        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };

        if Url::parse(&self.repo).is_err() {
            let _ = Cli::command()
                .error(
                    ErrorKind::InvalidValue,
                    "An invalid URL was specified",
                )
                .print();
            return;
        }

        data.information.repo = Some(self.repo);

        utils::write_data(file_name, &data);

        println!("New project repository link set successfully");
    }
}
