use crate::data::Group;
use crate::{utils, Cli};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory};

#[derive(Args)]
pub struct CreateArgs {
    /// The name of the group that you want to create
    pub group_name: String,
    /// The name of the parent group (if there is one)
    pub parent_group: Option<String>,
}

impl super::Command for CreateArgs {
    fn run(self, file_name: &str) {
        let Some(mut data) = utils::get_data(file_name) else {
            return;
        };

        if data.groups.contains_key(&self.group_name) {
            let _ = Cli::command()
                .error(
                    ErrorKind::InvalidValue,
                    format!("A group with the name `{}` already exists", self.group_name),
                )
                .print();
            return;
        }

        if let Some(parent_name) = self.parent_group {
            if data.groups.contains_key(&parent_name) {
                let mut parent = data.get_group(&parent_name);
                parent.groups.push(self.group_name.to_string());
                data.groups.insert(parent_name, parent);
            } else {
                let _ = Cli::command()
                    .error(
                        ErrorKind::InvalidValue,
                        format!("Specified parent group `{}` was not found", parent_name),
                    )
                    .print();
                return;
            }
        }
        data.groups
            .insert(self.group_name.to_string(), Group::new(&self.group_name));
        utils::write_data(file_name, &data);

        println!("Added group `{}` to project successfully", self.group_name);
    }
}
