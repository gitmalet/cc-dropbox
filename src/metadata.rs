use std::error::Error;
use chrono::{Local, DateTime};
use serde;

pub struct DTWrapper;

impl DTWrapper {
    fn dt_to_string(date: DateTime<Local>) -> String {
        return date.to_rfc3339();
    }

    fn to_datetime(str: String) -> DateTime<Local> {
        let dt = match str.parse::<DateTime<Local>>() {
            Ok(o) => return o,
            Err(e) => panic!("Dateconversion failed: {}", e.description()),
        };

        return Local::now();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    //File specific
    pub size: String,
    pub bytes: usize,
    pub rev: String,
    pub revision: usize,
    pub client_mtime: String,
    pub modified: String,
    pub mime_type: String,
    //Common
    pub icon: String,
    pub thumb_exists: bool,
    pub is_dir: bool,
    pub root: String,
    //pub name: String,
    pub path: String,
}

impl MetaData {
    pub fn get_client_mtime(&self) -> DateTime<Local> {
        DTWrapper::to_datetime(self.client_mtime.clone())
    }

    pub fn get_modified(&self) -> DateTime<Local> {
        DTWrapper::to_datetime(self.modified.clone())
    }
}
