use crate::{fg_color, utils, Cli};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap::builder::styling;

#[derive(Args)]
pub struct RemoveArgs {
    pub group_name: String,
    pub ids: Vec<usize>,
}

#[derive(Subcommand)]
pub enum Commands {
    Remove(Remove),
}

#[derive(Parser)]
pub struct Remove {
    #[structopt(subcommand)]
    pub remove_commands: RemoveCommands,
}

#[derive(Subcommand)]
pub enum RemoveCommands {
    /// Remove note(s) from a group
    Note(RemoveNoteArgs),
    /// Remove task(s) from a group
    Task(RemoveTaskArgs),
}

#[derive(Args)]
pub struct RemoveNoteArgs {
    /// The group that you will remove a note from
    pub group_name: String,
    /// The ID(s) of the note(s) you want to remove
    pub ids: Vec<usize>,
}

#[derive(Args)]
pub struct RemoveTaskArgs {
    /// The group that you will remove a task from
    pub group_name: String,
    /// The ID(s) of the task(s) you want to remove
    pub ids: Vec<usize>,
}

impl super::Command for RemoveNoteArgs {
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
        } else if self.ids.is_empty() {
            let _ = Cli::command()
                .error(ErrorKind::MissingRequiredArgument, "No data ID specified")
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();

        let mut invalid_ids: Vec<String> = vec![];
        let mut remove_count = 0;

        for id in &self.ids {
            if group.notes.len() < *id {
                invalid_ids.push(id.to_string());
            }
        }

        if !invalid_ids.is_empty() {
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

        for id in &self.ids {
            group.notes.remove(id - 1 - remove_count);
            remove_count += 1;
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        let mut formatted_ids: String = self.ids.iter().map(|i| i.to_string() + ", ").collect();
        formatted_ids.truncate(formatted_ids.len() - 2);

        println!(
            "Removed note(s) from group `{}` {}: {}",
            fg_color!(self.group_name, Yellow),
            fg_color!("successfully", Green),
            formatted_ids
        );
    }
}

impl super::Command for RemoveTaskArgs {
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
        } else if self.ids.is_empty() {
            let _ = Cli::command()
                .error(ErrorKind::MissingRequiredArgument, "No data ID specified")
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();

        let mut invalid_ids: Vec<String> = vec![];
        let mut remove_count = 0;

        for id in &self.ids {
            if group.tasks.len() < *id {
                invalid_ids.push(id.to_string());
            }
        }

        if !invalid_ids.is_empty() {
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

        for id in &self.ids {
            group.tasks.remove(id - 1 - remove_count);
            remove_count += 1;
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        let mut formatted_ids: String = self.ids.iter().map(|i| i.to_string() + ", ").collect();
        formatted_ids.truncate(formatted_ids.len() - 2);

        println!(
            "Removed task(s) from group `{}` {}: {}",
            fg_color!(self.group_name, Yellow),
            fg_color!("successfully", Green),
            formatted_ids
        );
    }
}
