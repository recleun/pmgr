use clap::Args;

use crate::{data::Project, utils};

#[derive(Args)]
pub struct DeselectArgs {
    pub group_names: Vec<String>,
}

impl super::Command for DeselectArgs {
    fn run(self, file_name: Option<&str>) {
        if self.group_names.len() == 0 {
            eprintln!("No groups specified to be deselected");
            return;
        }

        let mut data: Project = utils::get_data(file_name);
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
            eprintln!(
                "No changes happened, following groups are not active: ({}), following groups are not created: ({})",
                unselected_groups.join(", "), undefined_groups.join(", "));
            return;
        } else if unselected_groups.len() > 0 {
            eprintln!("No changes happened, following groups are not active: {}", unselected_groups.join(", "));
            return;
        } else if undefined_groups.len() > 0 {
            eprintln!("No changes happened, following groups are not created: {}", undefined_groups.join(", "));
            return;
        }

        let mut to_deselect: Vec<String> = vec![];

        for group in &self.group_names {
            to_deselect.push(group.to_string());
            to_deselect.append(&mut data.get_group_descendants(group));
        } 

        while to_deselect.len() > 0 {
            let index = data.active_groups.iter().position(|g| g == to_deselect[0].as_str())
                .expect("Group specified to deselect was not found in active groups");
            to_deselect.remove(0);
            data.active_groups.remove(index);
        }
        data.active_groups.sort();
        data.active_groups.dedup();
        utils::write_data(file_name, &data);

        println!("Deselected group(s) successfully: {}", self.group_names.join(", "));
    }
}
