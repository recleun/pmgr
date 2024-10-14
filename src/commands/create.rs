use crate::data::{Group, Project};
use crate::{utils, Cli};
use clap::error::ErrorKind;
use clap::{Args, CommandFactory};

#[derive(Args)]
pub struct CreateArgs {
    pub group_name: String,
    pub parent_group: Option<String>,
}

impl super::Command for CreateArgs {
    fn run(self, file_name: Option<&str>) {
        let mut data: Project = utils::get_data(file_name);

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
                        format!("Specified parent group `{}` was not found", self.group_name),
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
