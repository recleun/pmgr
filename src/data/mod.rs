use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub active_groups: Vec<String>,
    pub groups: HashMap<String, Group>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Group {
    pub name: String,
    pub notes: Vec<Note>,
    pub tasks: Vec<Task>,
    pub groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Note {
    pub id: usize,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Task {
    pub id: usize,
    pub task: String,
}

impl Project {
    pub fn new() -> Self {
        Project {
            groups: HashMap::new(),
            active_groups: vec![],
        }
    }

    pub fn get_group(&self, name: &str) -> Option<&Group> {
        self.groups.get(name)
    }

    pub fn get_group_descendants(&self, group_name: &str) -> Vec<String> {
        let group = self.get_group(group_name).expect("Group name specified was not found");

        let mut children: Vec<String> = vec![];
        for child in &group.groups {
            let g = self.get_group(child).expect("Group name specified was not found");
            children.push(child.to_string());
            children.append(&mut Self::get_group_descendants(self, &g.name));
        }

        children
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
