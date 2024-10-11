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
    use std::{fs, io};
    use pmgr::{
        commands,
        utils,
        Command
    };

    #[test]
    fn simple_init() {
        let file_name = ".simple-init.pmgr";
        clean(file_name);

        if fs::metadata(file_name).is_ok() {
            panic!("A project already exists, unable to init");
        }

        commands::init::InitArgs.run(Some(file_name));

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
        match utils::check_data(Some(file_name)) {
            Ok(data) => panic!("Data shouldn't exist, exists at: {:?}", data),
            Err(_) => (),
        }

        commands::init::InitArgs.run(Some(file_name));

        // data exists for this check
        match utils::check_data(Some(file_name)) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    panic!("{}", e);
                }
            }
        }
        clean(file_name);
    }
}
