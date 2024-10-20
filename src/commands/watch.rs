use clap::{error::ErrorKind, Args, CommandFactory};

use crate::{utils, Cli};

#[derive(Args)]
pub struct WatchArgs {
    /// The name(s) of the group(s) that you want to watch
    pub group_names: Vec<String>,
}

impl super::Command for WatchArgs {
    fn run(self, file_name: &str) {
        if self.group_names.len() == 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No groups specified to be watched",
                )
                .print();
            return;
        }

        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };
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
                    format!("Following groups are already watched: {},\nFollowing groups are not created: {}", already_active.join(", "), undefined_groups.join(", ")),
                )
                .print();
            return;
        } else if already_active.len() > 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "Following groups are already watched: {}",
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
            "Added group(s) to be watched successfully: {}",
            self.group_names.join(", ")
        );
    }
}
