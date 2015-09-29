use chrono::{Local, DateTime};
//use serde;

//#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetaData {
    //File specific
    pub size: String,
    pub bytes: usize,
    pub rev: String,
    pub client_mod_time: DateTime<Local>,
    pub last_mod: DateTime<Local>,
    //Common
    pub icon_name: String,
    pub thumbnail: bool,
    pub name: String,
    pub path: String,
}

//#[derive(Debug, Serialize, Deserialize)]
pub struct FolderMetaData {
    //Common
    pub icon_name: String,
    pub thumbnail: bool,
    pub name: String,
    pub path: String,
}
