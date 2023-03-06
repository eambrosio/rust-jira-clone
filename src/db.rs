use std::fs::{read_to_string, write};

use anyhow::{Ok, Result};

use crate::models::{DBState, Epic, Status, Story};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_state_string = read_to_string(self.file_path.to_owned())?;
        let db_state: DBState = serde_json::from_str(&db_state_string)?;

        Ok(db_state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        // serialize db_state to json and store it in self.file_path
        let db_as_string = serde_json::to_string(&db_state)?;
        write(self.file_path.to_owned(), db_as_string)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::fs::remove_file;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_owned(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/read_db_should_fail_with_invalid_json.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();

            let db = JSONFileDatabase {
                file_path: file_path.clone(),
            };

            let result = db.read_db();

            remove_file(file_path).unwrap();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/read_db_should_parse_json_file.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();

            let db = JSONFileDatabase {
                file_path: file_path.clone(),
            };

            let result = db.read_db();

            remove_file(file_path).unwrap();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let file_path = "./data/write_db_should_work.json".to_owned();

            let path = tmpfile.into_temp_path();
            path.persist(&file_path).unwrap();

            let db = JSONFileDatabase {
                file_path: file_path.clone(),
            };

            let story = Story {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
            };
            let epic = Epic {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            remove_file(file_path).unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}