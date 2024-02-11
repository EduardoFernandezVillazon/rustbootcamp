use std::collections::HashMap;
use core::fmt::Debug;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]// TODO: derive the appropriate traits
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]// TODO: derive the appropriate traits
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![]
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]// TODO: derive the appropriate traits
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>
}

impl Debug for DBState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBState {{ last_item_id: {}, epics: {:?}, stories: {:?} }}", self.last_item_id, self.epics, self.stories)
    }
}