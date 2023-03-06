use std::collections::HashMap;

pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub id: Option<u32>,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: None,
            name: name,
            description: description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

pub struct Story {
    pub id: Option<u32>,
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: None,
            name: name,
            description: description,
            status: Status::Open,
        }
    }
}

pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<String, Epic>,
    pub stories: HashMap<String, Story>,
}
