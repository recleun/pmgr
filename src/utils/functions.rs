use crate::data::Project;
use super::ExpectWith;

use std::{
    env, fs::File, io, path::{Path, PathBuf}
};

pub fn check_data(file_name: Option<&str>) -> Result<PathBuf, io::Error> {
    let file_name = file_name.unwrap_or(".pmgr");
    let current_dir = env::current_dir()?;

    if let Some(path) = check_data_with_path(&current_dir, file_name) {
        Ok(path)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Failed to find a project root in current directory or parents"))
    }
}

fn check_data_with_path(path: &Path, file_name: &str) -> Option<PathBuf> {
    let possible_path = path.join(file_name);
    if possible_path.exists() {
        return Some(possible_path);
    }

    path.parent()
        .and_then(|p| check_data_with_path(p, file_name))
}

pub fn get_data(file_name: Option<&str>) -> Project {
    let file = check_data(file_name)
        .and_then(File::open)
        .expect_with("Failed to open the project file");

    let data = serde_json::from_reader(file)
        .expect_with("Failed to read project data");

    data
}
