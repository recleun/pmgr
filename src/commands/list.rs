use crate::{data::Group, utils, Cli};
use clap::{error::ErrorKind, Args, CommandFactory};

#[derive(Args)]
pub struct ListArgs {
    group_name: Option<String>,
}

impl super::Command for ListArgs {
    fn run(self, file_name: Option<&str>) {
        let Some(data) = utils::get_data(file_name) else {
            return;
        };

        let mut groups: Vec<Group> = vec![];

        if self.group_name.is_some() {
            let group_name = &self.group_name.unwrap().clone();

            if !data.groups.contains_key(group_name) {
                let _ = Cli::command()
                    .error(
                        ErrorKind::InvalidValue,
                        format!("No group with name `{}` found", group_name),
                    )
                    .print();
                return;
            }

            let group = data.get_group(group_name);
            groups.push(group.clone());
            if group.groups.len() > 0 {
                let descendants = data.get_group_descendants(group_name);
                for descendant in descendants {
                    groups.push(data.get_group(&descendant));
                }
            }
        } else {
            if data.active_groups.len() == 0 {
                let _ = Cli::command()
                    .error(ErrorKind::Io, "No groups are being watched")
                    .print();
                return;
            }
            for group in &data.active_groups {
                groups.push(data.get_group(group));
            }
        }

        for group in &groups {
            println!("\n[{}]\n", group.name);
            if group.notes.len() > 0 {
                println!("  Notes:");
                let mut note_count = 0;
                for note in &group.notes {
                    note_count += 1;
                    println!("    {} - {}", note_count, note.note);
                }
                println!("");
            }
            if group.tasks.len() > 0 {
                println!("  Tasks:");
                let mut task_count = 0;
                for task in &group.tasks {
                    task_count += 1;
                    println!("    {} - [ ] {}", task_count, task.task);
                }
                println!("");
            }
            if group.notes.len() == 0 && group.tasks.len() == 0 {
                println!("Group is empty...\n");
            }
        }
    }
}
