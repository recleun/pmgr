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
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Task {
    pub task: String,
}

impl Project {
    pub fn new() -> Self {
        Project {
            groups: HashMap::new(),
            active_groups: vec![],
        }
    }

    pub fn get_group(&self, name: &str) -> Group {
        self.groups.get(name)
            .expect("Specified group was not found")
            .clone()
    }

    pub fn get_group_descendants(&self, group_name: &str) -> Vec<String> {
        let group = self.get_group(group_name);

        let mut children: Vec<String> = vec![];
        for child in &group.groups {
            let g = self.get_group(child);
            children.push(child.to_string());
            children.append(&mut self.get_group_descendants(&g.name));
        }

        children
    }

    pub fn clean(&mut self) {
        let groups = self.groups.clone();

        for (k, _) in &groups {
            let g = self.get_group(k);
            for child in &g.groups {
                if !self.groups.contains_key(child) {
                    let index = g.groups.iter().position(|i| i == child).unwrap();
                    let mut g = g.clone();
                    g.groups.remove(index);
                    println!("removing: {}", child);
                    self.groups.insert(k.to_string(), g);
                }
            }
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

impl Note {
    pub fn new(note: &str) -> Self {
        Note {
            note: note.to_string(),
        }
    }
}

impl Task {
    pub fn new(task: &str) -> Self {
        Task {
            task: task.to_string(),
        }
    }
}
