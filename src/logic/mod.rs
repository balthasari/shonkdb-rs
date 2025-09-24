use std::fs::File;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize)]
struct Haj {}

#[derive(Serialize, Deserialize)]
struct Shonk {
    tables: Vec<String>,
    used_tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize)]
struct Record {
    uuid: Uuid,
    dt_created: DateTime<Utc>,
    files: Option<Vec<HajFile>>,
}

//TO BE VERBESSERT
#[derive(Serialize, Deserialize)]
struct HajFile {
    path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
struct Tag {
    name: String,
}

pub struct ShonkAccess {
    dbroot: Shonk,
}

impl ShonkAccess {
    fn new(path: String) -> Self {
        ShonkAccess { dbroot: Shonk {} }
    }
}
