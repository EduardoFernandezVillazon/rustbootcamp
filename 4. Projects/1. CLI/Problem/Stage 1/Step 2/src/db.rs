use anyhow::Result;
use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

use crate::models::{DBState, Epic, Story, Status};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState, anyhow::Error> {
        let database_content = fs::read_to_string(&self.file_path)?;
        let database_state = serde_json::from_str(&database_content).with_context(|| format!("Could not parse JSON from database located in {}", self.file_path));
        println!("{:?}", database_state);
        match database_state{
            Ok(db_state) => Ok(db_state),
            Err(e) => Err(e)
        }

    }

    fn write_db(&self, db_state: &DBState) -> Result<(), anyhow::Error> {
        let database_state_string = serde_json::to_string(&db_state).with_context(|| format!("Database state {:?} was not correctly serialized to JSON string", &db_state));
        match database_state_string{
            Ok(database_state_string) => {
                fs::write(&self.file_path, database_state_string).with_context(|| format!("Could not write JSON to database located in{}", self.file_path))?;
                Ok(())
            },
            Err(e) => Err(e)
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}