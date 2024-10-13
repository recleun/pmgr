use crate::utils;
use crate::data::{Project, Group};
use clap::Args;

#[derive(Args)]
pub struct CreateArgs {
    pub group_name: String,
    pub parent_group: Option<String>,
}

impl super::Command for CreateArgs {
    fn run(self, file_name: Option<&str>) {
        let mut data: Project = utils::get_data(file_name);

        if let Some(_) = data.get_group(self.group_name.as_str()) {
            eprintln!("A group with the name `{}` already exists", self.group_name);
            return;
        }

        if let Some(parent_name) = self.parent_group {
            match data.get_group(&parent_name) {
                Some(parent) => data.groups[parent].groups.push(self.group_name.clone()),
                None => {
                    eprintln!("Specified parent group was not found");
                    return;
                }
            };
        }
        data.groups.push(Group::new(&self.group_name));
        utils::write_data(file_name, &data);

        println!("Added group `{}` to project successfully", self.group_name);
    }
}
