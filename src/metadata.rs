use chrono::{Local, DateTime};
use serde;

#[derive(Debug, Eq, Derialize, Deserialize)]
pub struct FileMetaData {
    //File specific
    size: String,
    bytes: usize,
    rev: String,
    client_mod_time: DateTime<Local>,
    last_mod: DateTime<Local>,
    //Common
    icon_name: String,
    thumbnail: bool,
    name: String,
    path: String,
}

#[derive(Debug, Eq, Derialize, Deserialize)]
pub struct FolderMetaData {
    //Common
    icon_name: String,
    thumbnail: bool,
    name: String,
    path: String,
}
