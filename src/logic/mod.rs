use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, to_string, to_string_pretty, to_writer, to_writer_pretty};
use std::{any::Any, iter::zip, path::Path, sync::mpsc::Iter, vec};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Shonk {
    tables: Vec<String>, //path to table
}
impl Shonk {
    fn new() -> Self {
        Self { tables: Vec::new() }
    }
    fn joined<'a, T: Default, Q: Default>(haj_t: &Haj<T>, haj_q: &Haj<Q>) -> impl Iterator {
        zip(haj_t.data(), haj_q.data())
    }
    fn joined_mut<'a, T: Default, Q: Default>(
        haj_t: &mut Haj<T>,
        haj_q: &mut Haj<Q>,
    ) -> impl Iterator {
        zip(haj_t.data_mut(), haj_q.data_mut())
    }
}

const ROOT: &str = "hajhom/";
const DATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/input.txt");

#[derive(Serialize, Deserialize)]
pub struct Haj<T: Default> {
    path: String, //eigentlich of type Path
    records: Vec<HajFren<T>>,
}

impl<T: Default> Haj<T> {
    fn new() -> Self {
        Self {
            path: "shorkattak".to_owned(),
            records: Vec::new(),
        }
    }

    fn by_uuid(&self, uuid: Uuid) -> Option<&HajFren<T>> {
        self.records.iter().find(|rec| rec.uuid == uuid)
    }
    fn by_uuid_mut(&mut self, uuid: Uuid) -> Option<&mut HajFren<T>> {
        self.records.iter_mut().find(|rec| rec.uuid == uuid)
    }

    fn remove_by_uuid(&mut self, uuid: Uuid) -> Option<HajFren<T>> {
        if let Some(index) = self.records.iter().position(|rec| rec.uuid == uuid) {
            Some(self.records.remove(index))
        } else {
            None
        }
    }

    fn data(&self) -> impl Iterator {
        self.records.iter().map(|rec| &rec.data)
    }
    fn data_mut(&mut self) -> impl Iterator {
        self.records.iter_mut().map(|rec| &mut rec.data)
    }

    fn by_tag(&self, tag: &Tag, complement: bool) -> Vec<&HajFren<T>> {
        if !complement {
            self.records
                .iter()
                .filter(|rec| rec.contains_tag(tag))
                .collect()
        } else {
            self.records
                .iter()
                .filter(|rec| rec.contains_tag(tag))
                .collect()
        }
    }

    fn write_self(&mut self) {
        //Write self to file at ROOT + path
    }

    fn load_from(path: String) -> Self {
        Self::new()
        //Actually: Return new Self-instance, deserialized from file at ROOT + path
    }
}

#[derive(Serialize, Deserialize)]
pub struct HajFren<T: Default> {
    uuid: Uuid,
    dt_created: DateTime<Utc>,
    dt_changed: DateTime<Utc>,
    files: Option<Vec<HajFile>>,
    data: T,
    tags: Vec<Tag>,
}
impl<T: Default> HajFren<T> {
    pub fn null() -> Self {
        HajFren {
            uuid: Uuid::nil(),
            dt_created: DateTime::default(),
            dt_changed: DateTime::default(),
            files: Option::None,
            data: T::default(),
            tags: Vec::default(),
        }
    }

    // DANGEROUS REF SHIT HERE
    fn contains_tag(&self, tag: &Tag) -> bool {
        match self.tags.iter().find(|t| t == &tag) {
            Some(_) => true,
            None => false,
        }
    }

    fn contains_all_tags(&self, tags: Vec<&Tag>) {}
}
//TO BE VERBESSERT
#[derive(Serialize, Deserialize)]
struct HajFile {
    path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Tag {
    name: String,
}
