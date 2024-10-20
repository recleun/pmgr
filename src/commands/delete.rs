use clap::{error::ErrorKind, Args, CommandFactory};

use crate::{utils, Cli};

#[derive(Args)]
pub struct DeleteArgs {
    pub group_names: Vec<String>,
}

impl super::Command for DeleteArgs {
    fn run(self, file_name: &str) {
        if self.group_names.len() == 0 {
            let _ = Cli::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "No groups specified to delete",
                )
                .print();
            return;
        }

        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };
        let mut undefined_groups: Vec<&str> = vec![];
        for group in &self.group_names {
            if !data.groups.contains_key(group) {
                undefined_groups.push(group);
            }
        }

        if undefined_groups.len() > 0 {
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

        let mut to_delete: Vec<String> = vec![];

        for group in &self.group_names {
            to_delete.push(group.to_string());
            to_delete.append(&mut data.get_group_descendants(group));
        }

        while to_delete.len() > 0 {
            if data.active_groups.contains(&to_delete[0]) {
                let index = data
                    .active_groups
                    .iter()
                    .position(|g| g == to_delete[0].as_str())
                    .expect("Group specified to be unwatched was not found in watched groups");
                data.active_groups.remove(index);
            }
            data.groups.remove(&to_delete[0]);
            data.clean();
            to_delete.remove(0);
        }

        utils::write_data(file_name, &data);

        println!(
            "Deleted group(s) successfully: {}",
            self.group_names.join(", ")
        );
    }
}
