use clap::{error::ErrorKind, CommandFactory};

use super::ExpectWith;
use crate::{data::Project, Cli};

use std::{
    env,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

pub fn check_data(file_name: Option<&str>) -> Result<PathBuf, io::Error> {
    let file_name = file_name.unwrap_or(".pmgr");
    let current_dir = env::current_dir()?;

    if let Some(path) = check_data_with_path(&current_dir, file_name) {
        Ok(path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No project root found",
        ))
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

pub fn get_data(file_name: Option<&str>) -> Option<Project> {
    let file = check_data(file_name).and_then(File::open);

    match file {
        Ok(path) => {
            let data = serde_json::from_reader(path).expect_with("Failed to read project data");

            return Some(data);
        }
        Err(e) => {
            let _ = Cli::command()
                .error(
                    ErrorKind::Io,
                    format!(
                        "Failed to get project root: {}",
                        e
                    ),
                )
                .print();
            None
        }
    }
}

pub fn write_data(file_name: Option<&str>, data: &Project) {
    let path = check_data(file_name).expect_with("Failed to open the project file");

    fs::write(
        path,
        serde_json::to_string(data).expect("Failed to serialize project data"),
    )
    .expect("Failed to write project data");
}
