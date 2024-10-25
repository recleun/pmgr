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
#[macro_export]
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

#[macro_export]
macro_rules! create_groups_local {
    (
        $project:ident,
        $($group:ident -> [$($subgroup:literal$(,)?)*],)*
    ) => {
        #[allow(unused_mut)]
        let mut $project = Project::new();
        $(
            let mut $group = Group::new(stringify!($group));
            $group.groups = vec![$($subgroup.to_string(),)*];
        )*
        insert_groups!($project, $($group,)*);
    };
}

#[macro_export]
macro_rules! delete_groups {
    (
        $file_name:ident, $($group:literal$(,)?)*
    ) => {
        $(println!("group name: {}", $group);)*
        commands::delete::DeleteArgs {
            group_names: vec![$($group.to_string(),)*]
        }.run($file_name);
    };
}

#[macro_export]
macro_rules! add_tasks {
    (
        $file_name:ident,
        $group:literal,
        $($task:literal$(,)?)*
    ) => {
        $(
            commands::add::AddTaskArgs {
                group_name: $group.to_string(),
                text: vec![$task.to_string()],
            }.run($file_name);
        )*
    };
}

#[macro_export]
macro_rules! add_tasks_local {
    (
        $project:ident,
        $group:literal,
        $(TaskState::$state:ident -> $task:literal$(,)?)*
    ) => {
        $(
            $project.groups.get_mut($group).unwrap().tasks.push(Task {
                task: $task.to_string(),
                state: TaskState::$state,
            });
        )*
    };
}
#[macro_export]
macro_rules! add_notes {
    (
        $file_name:ident,
        $group:literal,
        $($note:literal$(,)?)*
    ) => {
        $(
            commands::add::AddNoteArgs {
                group_name: $group.to_string(),
                text: vec![$note.to_string()],
            }.run($file_name);
        )*
    };
}

#[macro_export]
macro_rules! add_notes_local {
    (
        $project:ident,
        $group:literal,
        $($note:literal$(,)?)*
    ) => {
        $(
            $project.groups.get_mut($group).unwrap().notes.push(Note {
                note: $note.to_string(),
            });
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
#[macro_export]
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
#[macro_export]
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
#[macro_export]
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
            group_names: vec![$group.to_string()],
            all: false,
        }.run($file_name);)*
    };
}

/// Ensures that the specified file_name does not exist, and if it does, gets removed.
pub fn clean(file_name: &str) {
    let file = fs::metadata(file_name).is_ok();
    if file {
        if let Err(e) = fs::remove_file(file_name) {
            panic!("{}", e);
        }
    }
}
