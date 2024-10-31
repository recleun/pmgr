use crate::{data, fg_color};
use crate::{utils, Cli};
use clap::builder::styling;
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Add(Add),
}

#[derive(Parser)]
pub struct Add {
    #[structopt(subcommand)]
    pub add_commands: AddCommands,
}

#[derive(Subcommand)]
pub enum AddCommands {
    /// Add a note to a group
    Note(AddNoteArgs),
    /// Add a task to a group
    Task(AddTaskArgs),
}

#[derive(Args)]
pub struct AddNoteArgs {
    /// The group that you will add a note to
    pub group_name: String,
    /// The text that will show in the note
    pub text: Vec<String>,
}

#[derive(Args)]
pub struct AddTaskArgs {
    /// The group that you will add a task to
    pub group_name: String,
    /// The text that will show in the task
    pub text: Vec<String>,
}

impl super::Command for AddNoteArgs {
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
        } else if self.text.is_empty() {
            let _ = Cli::command()
                .error(ErrorKind::MissingRequiredArgument, "No text was specified")
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();
        group.notes.push(data::Note::new(&self.text.join(" ")));

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        println!("Added note to group `{}` {}", self.group_name, fg_color!("successfully", Green));
    }
}

impl super::Command for AddTaskArgs {
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
        } else if self.text.is_empty() {
            let _ = Cli::command()
                .error(ErrorKind::MissingRequiredArgument, "No text was specified")
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();
        group.tasks.push(data::Task::new(&self.text.join(" ")));

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        println!(
            "Added task to group `{}` {}",
            self.group_name,
            fg_color!("successfully", Green)
        );
    }
}
