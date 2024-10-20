use crate::{utils, Cli};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, ValueEnum};

#[derive(Args)]
pub struct RemoveArgs {
    pub data_type: Data,
    pub group_name: String,
    pub ids: Vec<usize>,
}

#[derive(ValueEnum, Clone)]
pub enum Data {
    /// A note that contains text
    Note,
    /// A task with Done/Not Done states
    Task,
}

impl super::Command for RemoveArgs {
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
                .error(ErrorKind::MissingRequiredArgument, "No data ID specified")
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();

        match self.data_type {
            Data::Note => {
                let mut invalid_ids: Vec<String> = vec![];
                let mut remove_count = 0;

                for id in &self.ids {
                    if group.notes.len() < *id {
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

                for id in &self.ids {
                    group.notes.remove(id - 1 - remove_count);
                    remove_count += 1;
                }
            }
            Data::Task => {
                let mut invalid_ids: Vec<String> = vec![];
                let mut remove_count = 0;

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

                for id in &self.ids {
                    group.tasks.remove(id - 1 - remove_count);
                    remove_count += 1;
                }
            }
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        let mut formatted_ids: String = self.ids.iter().map(|i| i.to_string() + ", ").collect();
        formatted_ids.truncate(formatted_ids.len() - 2);

        match self.data_type {
            Data::Note => println!(
                "Removed note(s) from group `{}` successfully: {}",
                self.group_name, formatted_ids
            ),
            Data::Task => println!(
                "Removed task(s) from group `{}` successfully: {}",
                self.group_name, formatted_ids
            ),
        }
    }
}
