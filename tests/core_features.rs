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
            group_names: vec![$group.to_string()],
            all: false,
        }.run($file_name);)*
    };
}

/// Simply checks whether or not the project is even initalized.
macro_rules! generate_ending_check1 {
    ($file_name: ident, $_: expr) => {
        if !fs::metadata($file_name).is_ok() {
            panic!("Project was not initalized");
        }
    };
}

/// This checks the current saved data to that of "project" (the second passed item).
macro_rules! generate_ending_check2 {
    ($file_name: ident, $project: expr) => {{
        let Some(data) = utils::get_data($file_name) else {
            panic!("Failed to get data");
        };
        assert_eq!(data, $project);
    }};
}

/// This just checks if the file does _not_ exist, and if not, checks if the error
/// is [`NotFound`](std::io::ErrorKind::NotFound).
macro_rules! generate_ending_check3 {
    ($file_name: ident, $_: expr) => {{
        if let Err(e) = utils::check_data($file_name) {
            if e.kind() != io::ErrorKind::NotFound {
                panic!("{}", e);
            }
        }
    }};
}

/// Generates ending checks for watch/unwatch commands.
///
/// ```rs
/// generate_watch_ending_check!(file_name, {
///     [<groups-to-edit>] -> <watch> => [ <final-groups> ]
/// })
/// ```
///
/// `<groups-to-edit>` are the groups that you want to either watch or unwatch, and
/// `<watch>` is a boolean indicating whether to watch or not. The final part is
/// optional, when present, it'll check the current values of watched tables and make
/// sure it matches `<final-groups>` (or panics if not).
macro_rules! generate_watch_ending_check {
    (
        $file_name: ident,
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

/// Internal macro for `generate_test`.
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

/// The actual macro to generate tests. The simplest form is:
///
/// ```rs
/// generate_test!(".test.pmgr" => {}, generate_ending_check1)
/// ```
///
/// Which simply creates a new pmgr project and checks whether or not it exists.
///
/// The macro has 2 match arms, the first 2 accepted values are the same in both, and
/// they are:
/// *  file_name - the file name that the test will be applied on. Should be different
///    for every test to ensure they don't overlap!
/// * extra_block - an optional block of code which is ran before
///   [`Init`](pmgr::commands::init::InitArgs) command is ran.
///
/// After those 2, the 3rd value varies, in match arm #1, it's a simple `ident`, which
/// must be a macro that accepts an `ident` and an `expr`. That macro is called at the
/// very end of the test, right before the final [`clean`].
///
/// For match arm #2, the syntax goes like:
/// ```rs
/// (file_name, project) => { ... }
/// ```
///
/// Where `file_name` and `project` are an `ident` and an `expr` respectfully. What's
/// in the body (`{ ... }`) is completely up to you which gives you complete freedom
/// of what to do inside it.
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
    fn create_with_parent() {
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
                generate_watch_ending_check!(
                    file_name,
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
                generate_watch_ending_check!(
                    file_name,
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
