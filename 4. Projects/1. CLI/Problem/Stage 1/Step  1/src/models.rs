pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        status = Open;
        stories = Vec::new();
        Epic {name, description, status, stories}
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        status = Open;
        Story {name, description, status}
    }
}

#[derive(Debug, PartialEq)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    pub last_item_id: u32,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>

}