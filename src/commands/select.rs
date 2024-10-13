use clap::Args;

use crate::{data::Project, utils};

#[derive(Args)]
pub struct SelectArgs {
    pub group_names: Vec<String>,
}

impl super::Command for SelectArgs {
    fn run(self, file_name: Option<&str>) {
        if self.group_names.len() == 0 {
            eprintln!("No groups specified to be selected");
            return;
        }

        let mut data: Project = utils::get_data(file_name);
        let mut already_active: Vec<&str> = vec![];
        let mut undefined_groups: Vec<&str> = vec![];
        for group in &self.group_names {
            if data.groups.iter().find(|g| g.name == *group).is_none() {
                undefined_groups.push(group);
            } else if data.active_groups.contains(group) {
                already_active.push(group);
            }
        }

        if already_active.len() > 0 && undefined_groups.len() > 0 {
            eprintln!(
                "No changes happened, following groups are already active: ({}), following groups are not created: ({})",
                already_active.join(", "), undefined_groups.join(", "));
            return;
        } else if already_active.len() > 0 {
            eprintln!("No changes happened, following groups are already active: {}", already_active.join(", "));
            return;
        } else if undefined_groups.len() > 0 {
            eprintln!("No changes happened, following groups are not created: {}", undefined_groups.join(", "));
            return;
        }

        let mut to_select: Vec<String> = vec![];

        for group in &self.group_names {
            to_select.push(group.to_string());
            to_select.append(&mut data.get_group_descendants(group));
        }

        data.active_groups.append(&mut to_select);
        utils::write_data(file_name, &data);

        println!("Selected group(s) successfully: {}", self.group_names.join(", "));
    }
}
