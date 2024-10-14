use clap::{error::ErrorKind, Args, CommandFactory};

use crate::{data::Project, utils, Cli};

#[derive(Args)]
pub struct SelectArgs {
    pub group_names: Vec<String>,
}

impl super::Command for SelectArgs {
    fn run(self, file_name: Option<&str>) {
        if self.group_names.len() == 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No groups specified to be selected",
                )
                .print();
            return;
        }

        let mut data: Project = utils::get_data(file_name);
        let mut already_active: Vec<&str> = vec![];
        let mut undefined_groups: Vec<&str> = vec![];
        for group in &self.group_names {
            if !data.groups.contains_key(group) {
                undefined_groups.push(group);
            } else if data.active_groups.contains(group) {
                already_active.push(group);
            }
        }

        if already_active.len() > 0 && undefined_groups.len() > 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!("Following groups are already active: {},\nFollowing groups are not created: {}", already_active.join(", "), undefined_groups.join(", ")),
                )
                .print();
            return;
        } else if already_active.len() > 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "Following groups are already active: {}",
                        already_active.join(", ")
                    ),
                )
                .print();
            return;
        } else if undefined_groups.len() > 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "Following groups are not created: {}",
                        undefined_groups.join(", ")
                    ),
                )
                .print();
            return;
        }

        let mut to_select: Vec<String> = vec![];

        for group in &self.group_names {
            to_select.push(group.to_string());
            to_select.append(&mut data.get_group_descendants(group));
        }

        data.active_groups.append(&mut to_select);
        data.active_groups.sort();
        data.active_groups.dedup();
        utils::write_data(file_name, &data);

        println!(
            "Selected group(s) successfully: {}",
            self.group_names.join(", ")
        );
    }
}
