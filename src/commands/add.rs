use crate::data::{Note, Task};
use crate::{utils, Cli};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, ValueEnum};

#[derive(Args)]
pub struct AddArgs {
    pub data_type: Data,
    pub group_name: String,
    pub text: Vec<String>,
}

#[derive(ValueEnum, Clone)]
pub enum Data {
    /// A note that contains text
    Note,
    /// A task with Done/Not Done states
    Task,
}

impl super::Command for AddArgs {
    fn run(self, file_name: Option<&str>) {
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
        } else if self.text.len() == 0 {
            let _ = Cli::command()
                .error(ErrorKind::MissingRequiredArgument, "No text was specified")
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();

        let mut was_note = true;
        match self.data_type {
            Data::Note => {
                group.notes.push(Note::new(&self.text.join(" ")));
            },
            Data::Task => {
                group.tasks.push(Task::new(&self.text.join(" ")));
                was_note = false;
            },
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        if was_note {
            println!("Added note to group `{}` successfully", self.group_name);
        } else {
            println!("Added task to group `{}` successfully", self.group_name);
        }
    }
}
