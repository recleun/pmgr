use clap::{error::ErrorKind, Args, CommandFactory};

use crate::{utils, Cli};

#[derive(Args)]
pub struct DeselectArgs {
    pub group_names: Vec<String>,
}

impl super::Command for DeselectArgs {
    fn run(self, file_name: Option<&str>) {
        if self.group_names.len() == 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No groups specified to be deselected",
                )
                .print();
            return;
        }

        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };
        let mut unselected_groups: Vec<&str> = vec![];
        let mut undefined_groups: Vec<&str> = vec![];
        for group in &self.group_names {
            if !data.groups.contains_key(group) {
                undefined_groups.push(group);
            } else if !data.active_groups.contains(group) {
                unselected_groups.push(group);
            }
        }

        if unselected_groups.len() > 0 && undefined_groups.len() > 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!("Following groups are already not active: {},\nFollowing groups are not created: {}", unselected_groups.join(", "), undefined_groups.join(", ")),
                )
                .print();
            return;
        } else if unselected_groups.len() > 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "Following groups are already not active: {}",
                        unselected_groups.join(", ")
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

        let mut to_deselect: Vec<String> = vec![];

        for group in &self.group_names {
            to_deselect.push(group.to_string());
            to_deselect.append(&mut data.get_group_descendants(group));
        }

        while to_deselect.len() > 0 {
            println!("to_deselect[0]: {}", to_deselect[0]);
            if data.active_groups.contains(&to_deselect[0]) {
                let index = data
                    .active_groups
                    .iter()
                    .position(|g| g == to_deselect[0].as_str())
                    .expect("Group specified to deselect was not found in active groups");
                data.active_groups.remove(index);
            }
            to_deselect.remove(0);
        }
        utils::write_data(file_name, &data);

        println!(
            "Deselected group(s) successfully: {}",
            self.group_names.join(", ")
        );
    }
}
