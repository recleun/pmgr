mod common;

#[cfg(test)]
mod tests {
    use super::*;
    use common;
    use pmgr::{commands, Command};
    use std::fs;

    #[test]
    fn simple_init() {
        let file_name = ".simple-init.pmgr";
        common::clean(file_name);

        commands::init::InitArgs.run(file_name);

        if !fs::metadata(file_name).is_ok() {
            panic!("Project was not initalized");
        }
        common::clean(file_name);
    }
}
