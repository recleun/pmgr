use clap::{error::ErrorKind, Args, CommandFactory};

use crate::{utils, Cli};

#[derive(Args)]
pub struct UnwatchArgs {
    /// The name(s) of the group(s) that you want to unwatch
    pub group_names: Vec<String>,
    /// Use this flag to unwatch all watched group (ignotes GROUP_NAMES)
    #[arg(short, long)]
    pub all: bool,
}

impl super::Command for UnwatchArgs {
    fn run(self, file_name: &str) {
        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };

        let mut to_unwatch: Vec<String> = vec![];

        if self.all && !data.active_groups.is_empty() {
            to_unwatch = data.active_groups.clone();
        } else if self.all {
            let _ = Cli::command()
                .error(ErrorKind::Io, "No groups are being watched")
                .print();
            return;
        } else {
            if self.group_names.is_empty() {
                let _ = Cli::command()
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        "No groups specified to unwatch",
                    )
                    .print();
                return;
            }

            let mut unwatched_groups: Vec<&str> = vec![];
            let mut undefined_groups: Vec<&str> = vec![];
            for group in &self.group_names {
                if !data.groups.contains_key(group) {
                    undefined_groups.push(group);
                } else if !data.active_groups.contains(group) {
                    unwatched_groups.push(group);
                }
            }

            if !unwatched_groups.is_empty() && !undefined_groups.is_empty() {
                let _ = Cli::command()
                    .error(
                        ErrorKind::ValueValidation,
                        format!("Following groups are already not watched: {},\nFollowing groups are not created: {}", unwatched_groups.join(", "), undefined_groups.join(", ")),
                    )
                    .print();
                return;
            } else if !unwatched_groups.is_empty() {
                let _ = Cli::command()
                    .error(
                        ErrorKind::ValueValidation,
                        format!(
                            "Following groups are already not watched: {}",
                            unwatched_groups.join(", ")
                        ),
                    )
                    .print();
                return;
            } else if !undefined_groups.is_empty() {
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

            for group in &self.group_names {
                to_unwatch.push(group.to_string());
                to_unwatch.append(&mut data.get_group_descendants(group));
            }
        }

        let unwatched = to_unwatch.clone();

        while !to_unwatch.is_empty() {
            if data.active_groups.contains(&to_unwatch[0]) {
                let index = data
                    .active_groups
                    .iter()
                    .position(|g| g == to_unwatch[0].as_str())
                    .expect("Group specified to be unwatched was not found in watched groups");
                data.active_groups.remove(index);
            }
            to_unwatch.remove(0);
        }
        utils::write_data(file_name, &data);

        println!("Unwatched group(s) successfully: {}", unwatched.join(", "));
    }
}
