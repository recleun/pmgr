use crate::data::{self, Group, TaskState};
use crate::{utils, Cli};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory, Subcommand, ValueEnum, Parser};

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

#[derive(Subcommand)]
pub enum Commands {
    Task(Task)
}

#[derive(Parser)]
pub struct Task {
    #[structopt(subcommand)]
    pub task_commands: TaskCommands,
}

#[derive(Subcommand)]
pub enum TaskCommands {
    /// Set task(s) as complete
    Complete(TaskCompleteArgs),
    /// Set task(s) as incomplete
    Undo(TaskUndoArgs),
    /// View the progress of a group or watched groups
    Progress(TaskProgressArgs)
}

#[derive(Args)]
pub struct TaskCompleteArgs {
    /// The group that you will set its notes as complete
    pub group_name: String,
    /// The ID(s) of the task(s) that you want to set as complete
    pub ids: Vec<usize>,
}

#[derive(Args)]
pub struct TaskUndoArgs {
    /// The group that you will set its notes as incomplete
    pub group_name: String,
    /// The ID(s) of the task(s) that you want to set as incomplete
    pub ids: Vec<usize>,
}

#[derive(Args)]
pub struct TaskProgressArgs {
    /// The group that you want to see the progress for
    pub group_name: Option<String>,
}

fn display_progress(group: Group) {
    let max_chars = 40;
    let mut used_chars = 0;

    let finished_tasks: Vec<data::Task> = group
        .tasks
        .clone()
        .into_iter()
        .filter(|t| t.state == TaskState::Complete)
        .collect();

    let unfinished_tasks: Vec<data::Task> = group
        .tasks
        .clone()
        .into_iter()
        .filter(|t| t.state == TaskState::Incomplete)
        .collect();

    let progress_percentage = finished_tasks.len() * 100 / group.tasks.len();

    let mut parsed_progress = String::new();

    while used_chars != max_chars {
        if used_chars * 100 / max_chars >= progress_percentage {
            parsed_progress.push(' ');
        } else {
            parsed_progress.push('=');
            if (used_chars + 1) * 100 / max_chars >= progress_percentage && used_chars + 1 != max_chars {
                parsed_progress.push('>');
            }
        }
        used_chars += 1;
    }

    println!("\n[{}]", group.name);
    println!("[{}] %{}\n", parsed_progress, progress_percentage);

    for task in unfinished_tasks {
        println!("  [ ] {}", task.task);
    }
    for task in finished_tasks {
        println!("  [x] {}", task.task);
    }
}

impl super::Command for TaskCompleteArgs {
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
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No task ID was specified",
                )
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

        for id in &self.ids {
            group.tasks[id - 1].state = TaskState::Complete;
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        let mut formatted_ids: String = self.ids.iter().map(|i| i.to_string() + ", ").collect();
        formatted_ids.truncate(formatted_ids.len() - 2);

        println!(
            "Successfully set following tasks for group `{}` as complete: {}",
            self.group_name, formatted_ids
        );
    }
}

impl super::Command for TaskUndoArgs {
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
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No task ID was specified",
                )
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

        for id in &self.ids {
            group.tasks[id - 1].state = TaskState::Incomplete;
        }

        data.groups.insert(self.group_name.clone(), group);
        utils::write_data(file_name, &data);

        let mut formatted_ids: String = self.ids.iter().map(|i| i.to_string() + ", ").collect();
        formatted_ids.truncate(formatted_ids.len() - 2);

        println!(
            "Successfully set following tasks for group `{}` as incomplete: {}",
            self.group_name, formatted_ids
        );
    }
}

impl super::Command for TaskProgressArgs {
    fn run(self, file_name: &str) {
        let Some(data) = utils::get_data(file_name) else {
            return;
        };
        
        if self.group_name.is_some() {
            let group_name = self.group_name.unwrap();
            let group = data.get_group(&group_name).clone();
            let groups = data.get_group_descendants(&group_name);
            display_progress(group);
            for g in &groups {
                display_progress(data.get_group(g));
            }
        } else {
            for g in &data.active_groups {
                display_progress(data.get_group(g));
            }
        }
    }
}
