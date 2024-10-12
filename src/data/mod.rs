use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub groups: Vec<Group>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    name: String,
    notes: Vec<Note>,
    tasks: Vec<Task>,
    groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: usize,
    note: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    task: String,
}

impl Project {
    pub fn new() -> Self {
        Project {
            groups: vec![],
        }
    }
}

impl Group {
    pub fn new(name: &str) -> Self {
        Group {
            name: name.to_string(),
            notes: vec![],
            tasks: vec![],
            groups: vec![],
        }
    }
}
