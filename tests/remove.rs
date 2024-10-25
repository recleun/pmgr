mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use common;
    use pmgr::{
        commands,
        data::{Group, Note, Project, Task, TaskState},
        utils, Command,
    };

    #[test]
    fn simple_remove() {
        let file_name = ".simple-add.pmgr";
        common::clean(file_name);
        commands::init::InitArgs.run(file_name);

        create_groups_local!(
            project,
            group1 -> [],
            group2 -> ["group3"],
            group3 -> [],
        );

        add_tasks_local!(
            project,
            "group1",
            TaskState::Incomplete -> "task1",
            TaskState::Incomplete -> "task3",
            TaskState::Incomplete -> "task5",
        );

        add_notes_local!(project, "group3", "note1", "note3", "note5",);

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> ["group3"],
            "group3" -> [],
        );

        add_tasks!(file_name, "group1", "task1", "task2", "task3", "task4", "task5",);

        add_notes!(file_name, "group3", "note1", "note2", "note3", "note4", "note5",);

        remove_tasks!(file_name, "group1", 2, 4);
        remove_notes!(file_name, "group3", 2, 4);

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };

        assert_eq!(data.groups, project.groups);
    }
}
