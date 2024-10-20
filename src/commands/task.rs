use crate::data::TaskState;
use crate::{utils, Cli};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, ValueEnum};

#[derive(Args)]
pub struct TaskArgs {
    pub subcommand: Subcommands,
    pub group_name: String,
    pub ids: Vec<usize>,
}

#[derive(ValueEnum, Clone)]
pub enum Subcommands {
    /// Set task(s) as complete
    Complete,
    /// Set task(s) as incomplete
    Undo,
}

impl super::Command for TaskArgs {
    fn run(self, file_name: &str) {
        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };

        if !data.groups.contains_key(&self.group_name) {
            let _ = Cli::command()
                .error(
                    ErrorKind::InvalidValue,
                    format!("Specified group `{}` does not exist", self.group_name),
                )
                .print();
            return;
        } else if self.ids.len() == 0 {
            let _ = Cli::command()
                .error(ErrorKind::MissingRequiredArgument, "No task ID was specified")
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();

        let mut invalid_ids: Vec<String> = vec![];

        for id in &self.ids {
            if group.tasks.len() < *id {
                invalid_ids.push(id.to_string());
            }
        }

        if invalid_ids.len() > 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::InvalidValue,
                    format!(
                        "Some given IDs are out of range: {}",
                        invalid_ids.join(", ")
                    ),
                )
                .print();
            return;
        }

        match self.subcommand {
            Subcommands::Complete => {
                for id in &self.ids {
                    group.tasks[id-1].state = TaskState::Complete;
                }
            },
            Subcommands::Undo => {
                for id in &self.ids {
                    group.tasks[id-1].state = TaskState::Incomplete;
                }
            },
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        let mut formatted_ids: String = self.ids.iter().map(|i| i.to_string() + ", ").collect();
        formatted_ids.truncate(formatted_ids.len() - 2);

        match self.subcommand {
            Subcommands::Complete => {
                println!("Successfully set following tasks for group `{}` as complete: {}", self.group_name, formatted_ids);
            }
            Subcommands::Undo => {
                println!("Successfully set following tasks for group `{}` as incomplete: {}", self.group_name, formatted_ids);
            }
        }
    }
}
