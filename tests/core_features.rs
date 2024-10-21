use std::fs;

/// Creates a list of groups for a test
/// ```rs
/// create_groups(
///     file_name,
///     // group1 will be created
///     "group1" -> []
///     // group2 will be created with group3 and group4 as subgroups
///     "group2" -> ["group3", "group4"]
///     // group4 will be created with group5 as a subgroup
///     "group4" -> ["group5"]
/// );
/// ```
macro_rules! create_groups {
    (
        $file_name:ident,
        $($group:literal -> [$($subgroup:literal$(,)?)*],)*
    ) => {
        $(
            commands::create::CreateArgs {
                group_name: $group.to_string(),
                parent_group: None,
            }.run($file_name);
            $(
                commands::create::CreateArgs {
                    group_name: $subgroup.to_string(),
                    parent_group: Some($group.to_string()),
                }.run($file_name);
            )*
        )*
    };
}

/// Inserts a list of groups into a project
/// ```rs
/// // Insert group objects
/// insert_groups!(project, group1, group2, group3);
///
/// // Or create them on demand
/// insert_groups!(project, "group1", "group2", "group3");
/// ```
macro_rules! insert_groups {
    (
        $project:ident, $($group:ident$(,)?)*
    ) => {
        $($project.groups.insert($group.name.to_string(), $group);)*
    };
    (
        $project:ident, $($group:literal$(,)?)*
    ) => {
        $($project.groups.insert($group.to_string(), Group::new($group));)*
    };
}

/// Push a list of groups into a group
/// ```rs
/// push_groups!(group, "subgroup1", "subgroup2");
/// ```
macro_rules! push_groups {
    (
        $group:ident, $($subgroup:literal$(,)?)*
    ) =>{
        $($group.groups.push($subgroup.to_string());)*
    };
}

/// Set watch status for a list of groups
/// ```rs
/// // To watch:
/// watch_groups!(file_name, true, "group1", "group2");
/// // To unwatch:
/// watch_groups!(file_name, false, "group1", "group2");
/// ```
macro_rules! watch_groups {
    (
        $file_name:ident, true, $($group:literal$(,)?)*
    ) =>{
        $(commands::watch::WatchArgs {
            group_names: vec![$group.to_string()]
        }.run($file_name);)*
    };
    (
        $file_name:ident, false, $($group:literal$(,)?)*
    ) =>{
        $(commands::unwatch::UnwatchArgs {
            group_names: vec![$group.to_string()]
        }.run($file_name);)*
    };
}

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

        clean(file_name);
    }
}
