use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    groups: Vec<Group>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    notes: Vec<Note>,
    tasks: Vec<Task>,
    groups: Vec<Group>,
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
