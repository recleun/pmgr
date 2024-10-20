use crate::data::{Group, Task, TaskState};
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
    /// View progress of tasks
    Progress,
}

fn display_progress(group: Group) {
    // [########################################] %100
    // [##############################          ] %70
    let max_chars = 40;
    let mut used_chars = 0;

    let finished_tasks: Vec<Task> = group
        .tasks
        .clone()
        .into_iter()
        .filter(|t| t.state == TaskState::Complete)
        .collect();
    let progress_percentage = finished_tasks.len() * 100 / group.tasks.len();

    let mut parsed_progress = String::new();

    while used_chars != max_chars {
        if (used_chars * 100 / max_chars) >= progress_percentage {
            parsed_progress.push(' ');
        } else {
            parsed_progress.push('#');
        }
        used_chars += 1;
    }

    println!("\n[{}]", group.name);
    println!("[{}] %{}", parsed_progress, progress_percentage);
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
        } else if self.ids.len() == 0 && !matches!(self.subcommand, Subcommands::Progress) {
            let _ = Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No task ID was specified",
                )
                .print();
            return;
        }

        let mut group = data.get_group(&self.group_name).clone();

        if matches!(self.subcommand, Subcommands::Progress) {
            let groups = data.get_group_descendants(&self.group_name);
            display_progress(group);
            for g in &groups {
                display_progress(data.get_group(g));
            }
            return;
        }

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
                    group.tasks[id - 1].state = TaskState::Complete;
                }
            }
            Subcommands::Undo => {
                for id in &self.ids {
                    group.tasks[id - 1].state = TaskState::Incomplete;
                }
            }
            _ => (),
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        let mut formatted_ids: String = self.ids.iter().map(|i| i.to_string() + ", ").collect();
        formatted_ids.truncate(formatted_ids.len() - 2);

        match self.subcommand {
            Subcommands::Complete => {
                println!(
                    "Successfully set following tasks for group `{}` as complete: {}",
                    self.group_name, formatted_ids
                );
            }
            Subcommands::Undo => {
                println!(
                    "Successfully set following tasks for group `{}` as incomplete: {}",
                    self.group_name, formatted_ids
                );
            }
            _ => (),
        }
    }
}
