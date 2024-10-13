use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub active_groups: Vec<String>,
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
            active_groups: vec![],
        }
    }

    pub fn get_group(&self, name: &str) -> Option<usize> {
        self.groups.iter().position(|group| group.name == name)
    }

    pub fn get_group_descendants(&self, group_name: &str) -> Vec<String> {
        let group = &self.groups[Self::get_group(self, group_name).expect("Group name specified was not found")];

        let mut children: Vec<String> = vec![];
        for child in &group.groups {
            let g = &self.groups[Self::get_group(self, child).expect("Group name specified was not found")];
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
