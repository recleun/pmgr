use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub groups: Vec<Group>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Group {
    pub name: String,
    pub notes: Vec<Note>,
    pub tasks: Vec<Task>,
    pub groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Note {
    pub id: usize,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Task {
    pub id: usize,
    pub task: String,
}

impl Project {
    pub fn new() -> Self {
        Project {
            groups: vec![],
        }
    }

    pub fn get_group(&self, name: &str) -> Option<usize> {
        self.groups.iter().position(|group| group.name == name)
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
