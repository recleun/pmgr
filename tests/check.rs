mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use common;
    use pmgr::{commands, utils, Command};
    use std::io;

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
}
