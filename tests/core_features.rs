use std::fs;

fn clean(file_name: &str) {
    let file = fs::metadata(file_name).is_ok();
    if file {
        if let Err(e) = fs::remove_file(file_name) {
            panic!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    use std::{fs, io};
    use pmgr::{
        commands, data::{Group, Project}, utils, Command
    };

    #[test]
    fn simple_init() {
        let file_name = ".simple-init.pmgr";
        clean(file_name);

        commands::init::InitArgs.run(file_name);

        if !fs::metadata(file_name).is_ok() {
            panic!("Project was not initalized");
        }
        clean(file_name);
    }

    #[test]
    fn simple_check() {
        let file_name = ".simple-check.pmgr";
        clean(file_name);

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
        clean(file_name);
    }

    #[test]
    fn simple_create() {
        let file_name = ".simple-create.pmgr";
        clean(file_name);

        let mut project = Project::new();
        project.groups.insert("group1".to_string(), Group::new("group1"));
        project.groups.insert("group2".to_string(), Group::new("group2"));
        project.groups.insert("group3".to_string(), Group::new("group3"));

        commands::init::InitArgs.run(file_name);

        commands::create::CreateArgs {
            group_name: "group1".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group2".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group3".to_string(),
            parent_group: None,
        }.run(file_name);
        // expect to not create a group (duplicate group)
        commands::create::CreateArgs {
            group_name: "group3".to_string(),
            parent_group: None,
        }.run(file_name);

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data, project);

        clean(file_name);
    }

    #[test]
    fn create_with_parent() {
        let file_name = ".create-with-parent.pmgr";
        clean(file_name);

        /*
         * simple structure used in test:
         *
         * -group1:
         *   -
         * -group2:
         *   -group3:
         *     -
         *   -group4:
         *     -group5:
         *       -
         */
        let mut project = Project::new();
        let group1 = Group::new("group1");
        let mut group2 = Group::new("group2");
        let group3 = Group::new("group3");
        let mut group4 = Group::new("group4");
        let group5 = Group::new("group5");

        group2.groups.push("group3".to_string());
        group2.groups.push("group4".to_string());
        group4.groups.push("group5".to_string());

        project.groups.insert(group1.name.to_string(), group1);
        project.groups.insert(group2.name.to_string(), group2);
        project.groups.insert(group3.name.to_string(), group3);
        project.groups.insert(group4.name.to_string(), group4);
        project.groups.insert(group5.name.to_string(), group5);

        commands::init::InitArgs.run(file_name);

        commands::create::CreateArgs {
            group_name: "group1".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group2".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group3".to_string(),
            parent_group: Some("group2".to_string()),
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group4".to_string(),
            parent_group: Some("group2".to_string()),
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group5".to_string(),
            parent_group: Some("group4".to_string()),
        }.run(file_name);
        // expect to not create a group (duplicate group)
        commands::create::CreateArgs {
            group_name: "group5".to_string(),
            parent_group: Some("group1".to_string()),
        }.run(file_name);

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data, project);

        clean(file_name);
    }

    #[test]
    fn simple_select() {
        let file_name = ".simple-select.pmgr";
        clean(file_name);

        commands::init::InitArgs.run(file_name);

        /*
         * simple structure used in test:
         *
         * -group1: (selected in stage 2)
         *   -
         * -group2: (selected in stage 2)
         *   -group3:
         *     -
         *   -group4: (selected in stage 1)
         *     -group5:
         *       -
         */
        commands::create::CreateArgs {
            group_name: "group1".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group2".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group3".to_string(),
            parent_group: Some("group2".to_string()),
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group4".to_string(),
            parent_group: Some("group2".to_string()),
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group5".to_string(),
            parent_group: Some("group4".to_string()),
        }.run(file_name);

        commands::watch::WatchArgs {
            group_names: vec!["group4".to_string()],
        }.run(file_name);

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups, vec!["group4", "group5"]);

        commands::watch::WatchArgs {
            group_names: vec!["group1".to_string()],
        }.run(file_name);
        commands::watch::WatchArgs {
            group_names: vec!["group2".to_string()],
        }.run(file_name);

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups, vec!["group1", "group2", "group3", "group4", "group5"]);

        clean(file_name);
    }

    #[test]
    fn simple_deselect() {
        let file_name = ".simple-deselect.pmgr";
        clean(file_name);

        commands::init::InitArgs.run(file_name);

        /*
         * simple structure used in test:
         *
         * -group1: (selected in stage 2)
         *   -
         * -group2: (selected in stage 2)
         *   -group3:
         *     -
         *   -group4: (selected in stage 1)
         *     -group5:
         *       -
         */
        commands::create::CreateArgs {
            group_name: "group1".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group2".to_string(),
            parent_group: None,
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group3".to_string(),
            parent_group: Some("group2".to_string()),
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group4".to_string(),
            parent_group: Some("group2".to_string()),
        }.run(file_name);
        commands::create::CreateArgs {
            group_name: "group5".to_string(),
            parent_group: Some("group4".to_string()),
        }.run(file_name);

        commands::watch::WatchArgs {
            group_names: vec!["group2".to_string()],
        }.run(file_name);

        commands::unwatch::UnwatchArgs {
            group_names: vec!["group5".to_string()],
        }.run(file_name);

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups, vec!["group2", "group3", "group4"]);

        commands::unwatch::UnwatchArgs {
            group_names: vec!["group2".to_string()],
        }.run(file_name);

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data.active_groups.len(), 0);

        clean(file_name);
    }
}
