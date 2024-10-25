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
    fn simple_create() {
        let file_name = ".simple-create.pmgr";
        common::clean(file_name);

        let mut project = Project::new();

        insert_groups!(project, "group1", "group2", "group3");

        commands::init::InitArgs.run(file_name);

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> [],
            "group3" -> [],
            // should not create duplicates
            "group3" -> [],
        );

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data, project);

        common::clean(file_name);
    }

    #[test]
    fn create_with_parent() {
        let file_name = ".create-with-parent.pmgr";
        common::clean(file_name);

        let mut project = Project::new();
        let group1 = Group::new("group1");
        let mut group2 = Group::new("group2");
        let group3 = Group::new("group3");
        let mut group4 = Group::new("group4");
        let group5 = Group::new("group5");

        push_groups!(group2, "group3", "group4");
        push_groups!(group4, "group5");

        insert_groups!(project, group1, group2, group3, group4, group5);

        commands::init::InitArgs.run(file_name);

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> ["group3", "group4"],
            "group4" -> ["group5"],
        );

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data, project);

        common::clean(file_name);
    }
}
