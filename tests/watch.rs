mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use common;
    use pmgr::{commands, utils, Command};

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
        assert_eq!(
            data.active_groups,
            vec!["group1", "group2", "group3", "group4", "group5"]
        );

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
