use std::fs;

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

macro_rules! generate_ending_check1 {
    ($file_name: ident, $_: expr) => {
        if !fs::metadata($file_name).is_ok() {
            panic!("Project was not initalized");
        }
    };
}
macro_rules! generate_ending_check2 {
    ($file_name: ident, $project: expr) => {{
        let Some(data) = utils::get_data($file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data, $project);
    }};
}
macro_rules! generate_ending_check3 {
    ($file_name: ident, $_: expr) => {{
        if let Err(e) = utils::check_data($file_name) {
            if e.kind() != io::ErrorKind::NotFound {
                panic!("{}", e);
            }
        }
    }};
}
macro_rules! generate_ending_check_with_watch {
    (
        $file_name: ident,
        $_: expr,
        {
           $( [ $($name: literal),* $(,)? ] -> $value: literal $(=> [ $($match: literal),* $(,)?] )? ),* $(,)?
        }
    ) => {{
        $(
            if $value {
                watch_groups!($file_name, true $(, $name)*);
            } else  {
                watch_groups!($file_name, false $(, $name)*);
            }
            $(
                let Some(data) = utils::get_data($file_name) else {
                    panic!("Failed to get data");
                };
                let matches: Vec<&str> = vec![$($match,)*];
                assert_eq!(data.active_groups, matches);
            )?
        )*
    }};
}

macro_rules! __generate_test {
    (
        $file_name_variable: ident = $file_name: literal,
        $project: ident
        $(, $extra_block: block)?
        $(, $generate_extra_block_after: ident)?
        {
            $( $group: ident -> [
                $( $subgroup: ident$(,)?)*
            ], )*
        }
        $(, $ending_check: block)?
    ) => {
        let $file_name_variable = $file_name;
        clean($file_name_variable);

        // just here
        #[allow(unused_mut, unused_variables)]
        let mut $project = Project::new();
        $( $project.groups.insert(stringify!($group).to_string(), Group::new(stringify!($group))); )*
        $($extra_block)?

        commands::init::InitArgs.run($file_name_variable);

        $( $generate_extra_block_after!($file_name_variable); )?

        $(
            commands::create::CreateArgs {
                group_name: stringify!($group).to_string(),
                parent_group: None,
            }.run($file_name_variable);
            $(
                commands::create::CreateArgs {
                    group_name: stringify!($subgroup).to_string(),
                    parent_group: Some(stringify!($group).to_string()),
                }.run($file_name_variable);
            )*
        )*

        $( $ending_check )?
        clean($file_name_variable);
    };
}

macro_rules! generate_test {
    (
        $file_name: literal => {
            $( $group: ident -> [
                $( $subgroup: ident$(,)?)*
            ], )*
        }
        $(, $extra_block: block )?
        $(, $generate_ending_check: ident )?
    ) => {
        __generate_test!(
            file_name_variable = $file_name,
            project
            $(, $extra_block )?
            {
                $( $group -> [$( $subgroup, )*], )*
            },
            { $( $generate_ending_check!(file_name_variable, project) )? }
        )
    };
    (
        $file_name: literal => {
            $( $group: ident -> [
                $( $subgroup: ident$(,)?)*
            ], )*
        }
        $(, $extra_block: block )?
        $( ($file_name_variable: ident, $project: ident) => $body: block )?
    ) => {
        __generate_test!(
            file_name_variable = $file_name,
            project
            $(, $extra_block )?
            {
                $( $group -> [$( $subgroup, )*], )*
            },
            {
                $(
                    // to avoid annoying warnings.
                    #[allow(unused_variables)]
                    let $file_name_variable = file_name_variable;
                    #[allow(unused_variables)]
                    let $project = project;
                    $body;
                )?
            }
        )
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
    use pmgr::{
        commands,
        data::{Group, Project},
        utils, Command,
    };
    use std::{fs, io};

    #[test]
    fn simple_init() {
        generate_test!(".simple-init.pmgr" => {}, generate_ending_check1);
    }

    #[test]
    fn simple_check() {
        let file_name = ".simple-check.pmgr";
        generate_test!(
            ".simple-check.pmgr" => {},
            {
                if let Ok(data) = utils::check_data(file_name) {
                    panic!("Data shouldn't exist, exists at: {:?}", data)
                }
            },
            generate_ending_check3
        );
    }

    #[test]
    fn simple_create() {
        generate_test!(
            ".simple-create.pmgr" => {
                group1 -> [],
                group2 -> [],
                group3 -> [],
                group3 -> [],
            },
            generate_ending_check2
        );
    }

    #[test]
    fn create_with_parent_2() {
        generate_test!(
            ".create-with-parent.pmgr" => {
                group1 -> [],
                group2 -> [],
                group3 -> [],
                group3 -> [],
            },
            generate_ending_check2
        );
    }

    #[test]
    fn simple_select() {
        generate_test!(
            ".simple-select.pmgr" => {
                group1 -> [],
                group2 -> [group3, group4],
                group4 -> [group5],
            }
            (file_name, project) => {
                generate_ending_check_with_watch!(
                    file_name,
                    project,
                    {
                        ["group4"] -> true => ["group4", "group5"],
                        ["group1", "group2"] -> true => ["group1", "group2", "group3", "group4", "group5"],
                    }
                )
            }
        );
    }

    #[test]
    fn simple_deselect() {
        generate_test!(
            ".simple-deselect.pmgr" => {
                group1 -> [],
                group2 -> [group3, group4],
                group4 -> [group5],
            }
            (file_name, project) => {
                generate_ending_check_with_watch!(
                    file_name,
                    project,
                    {
                        ["group2"] -> true,
                        ["group5"] -> false => ["group2", "group3", "group4"],
                        ["group2"] -> false => [],
                    }
                )
            }
        );
    }
}
