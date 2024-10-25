mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use common;
    use pmgr::{
        commands,
        data::{Group, Project},
        utils, Command,
    };

    #[test]
    fn simple_delete() {
        let file_name = ".simple-delete.pmgr";
        common::clean(file_name);

        commands::init::InitArgs.run(file_name);

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> ["group3", "group4"],
            "group4" -> ["group5"],
        );

        delete_groups!(file_name, "group4");

        create_groups_local!(
            project,
            group1 -> [],
            group2 -> ["group3"],
            group3 -> [],
        );

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.groups, project.groups);

        project.groups.remove("group3");
        project.groups.get_mut("group2").unwrap().groups = vec![];

        delete_groups!(file_name, "group3");

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.groups, project.groups);

        common::clean(file_name);
    }
}
