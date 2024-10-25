mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    use std::{fs, io};
    use pmgr::{
        commands, data::{Group, Project}, utils, Command
    };
    use common;

    #[test]
    fn simple_init() {
        let file_name = ".simple-init.pmgr";
        common::clean(file_name);

        commands::init::InitArgs.run(file_name);

        if !fs::metadata(file_name).is_ok() {
            panic!("Project was not initalized");
        }
        common::clean(file_name);
    }

    #[test]
    fn simple_check() {
        let file_name = ".simple-check.pmgr";
        common::clean(file_name);

        // data doesn't exist for this check
        match utils::check_data(file_name) {
            Ok(data) => panic!("Data shouldn't exist, exists at: {:?}", data),
            Err(_) => (),
        }

        commands::init::InitArgs.run(file_name);

        // data exists for this check
        match utils::check_data(file_name) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    panic!("{}", e);
                }
            }
        }
        common::clean(file_name);
    }

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

    #[test]
    fn simple_select() {
        let file_name = ".simple-select.pmgr";
        common::clean(file_name);

        commands::init::InitArgs.run(file_name);

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> ["group3", "group4"],
            "group4" -> ["group5"],
        );

        watch_groups!(file_name, true, "group4");

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups, vec!["group4", "group5"]);

        watch_groups!(file_name, true, "group1", "group2");

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups, vec!["group1", "group2", "group3", "group4", "group5"]);

        common::clean(file_name);
    }

    #[test]
    fn simple_deselect() {
        let file_name = ".simple-deselect.pmgr";
        common::clean(file_name);

        commands::init::InitArgs.run(file_name);

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> ["group3", "group4"],
            "group4" -> ["group5"],
        );

        watch_groups!(file_name, true, "group2");
        watch_groups!(file_name, false, "group5");

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups, vec!["group2", "group3", "group4"]);

        watch_groups!(file_name, false, "group2");

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups.len(), 0);

        common::clean(file_name);
    }
}
