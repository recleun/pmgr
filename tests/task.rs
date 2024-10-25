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
    fn task_complete() {
        let file_name = ".task-complete.pmgr";
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
            "group3",
            TaskState::Complete -> "task1",
            TaskState::Incomplete -> "task2",
            TaskState::Complete -> "task3",
            TaskState::Incomplete -> "task4",
            TaskState::Complete -> "task5",
        );

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> ["group3"],
            "group3" -> [],
        );

        add_tasks!(file_name, "group3", "task1", "task2", "task3", "task4", "task5",);

        complete_tasks!(
            file_name,
            "group3",
            1 -> TaskState::Complete,
            3 -> TaskState::Complete,
            5 -> TaskState::Complete,
        );

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };

        assert_eq!(data.groups, project.groups);

        common::clean(file_name);
    }

    #[test]
    fn task_undo() {
        let file_name = ".task-undo.pmgr";
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
            "group3",
            TaskState::Incomplete -> "task1",
            TaskState::Incomplete -> "task2",
            TaskState::Incomplete -> "task3",
            TaskState::Incomplete -> "task4",
            TaskState::Incomplete -> "task5",
        );

        create_groups!(
            file_name,
            "group1" -> [],
            "group2" -> ["group3"],
            "group3" -> [],
        );

        add_tasks!(file_name, "group3", "task1", "task2", "task3", "task4", "task5",);

        complete_tasks!(
            file_name,
            "group3",
            1 -> TaskState::Complete,
            3 -> TaskState::Complete,
            5 -> TaskState::Complete,
        );

        undo_tasks!(
            file_name,
            "group3",
            1 -> TaskState::Incomplete,
            3 -> TaskState::Incomplete,
            5 -> TaskState::Incomplete,
        );

        let Some(data) = utils::get_data(file_name) else {
            panic!("Failed to get data");
        };

        assert_eq!(data.groups, project.groups);

        common::clean(file_name);
    }
}
