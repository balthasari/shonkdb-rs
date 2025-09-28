use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, to_string, to_string_pretty, to_writer, to_writer_pretty};
use std::fmt::format;
use std::fs::OpenOptions;
use std::{iter::zip, path::Path, sync::mpsc::Iter, vec};
use uuid::{Uuid, serde::urn::serialize};

/*
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
*/
pub static ROOT: &str = "./hajhom/";

#[derive(Serialize, Deserialize)]
pub struct Haj<T>
where
    T: Default,
{
    path: String, //eigentlich of type Path
    records: Vec<HajFren<T>>,
}

impl<T> Haj<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    pub fn new() -> Self {
        Self {
            path: "shorkattak".to_owned(),
            records: Vec::new(),
        }
    }

    pub fn by_uuid(&self, uuid: Uuid) -> Option<&HajFren<T>> {
        self.records.iter().find(|rec| rec.uuid == uuid)
    }
    pub fn by_uuid_mut(&mut self, uuid: Uuid) -> Option<&mut HajFren<T>> {
        self.records.iter_mut().find(|rec| rec.uuid == uuid)
    }

    pub fn remove_by_uuid(&mut self, uuid: Uuid) -> Option<HajFren<T>> {
        if let Some(index) = self.records.iter().position(|rec| rec.uuid == uuid) {
            Some(self.records.remove(index))
        } else {
            None
        }
    }

    pub fn data(&self) -> impl Iterator {
        self.records.iter().map(|rec| &rec.data)
    }
    pub fn data_mut(&mut self) -> impl Iterator {
        self.records.iter_mut().map(|rec| &mut rec.data)
    }

    pub fn by_tag(&self, tag: &Tag, complement: bool) -> Vec<&HajFren<T>> {
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

    pub fn insert_data(&mut self, data: T) {
        let fren = HajFren::new_with_data(data);

        self.records.push(fren);
    }

    pub fn instert_data_tags(&mut self, data: T, tags: Vec<Tag>) {
        let fren = HajFren::new_with_data_tags(data, tags);

        self.records.push(fren);
    }

    pub fn instert_record(&mut self, hajfren: HajFren<T>) {
        self.records.push(hajfren);
    }

    pub fn write_self(&mut self) {
        let file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(&self.path)
            .unwrap();

        to_writer_pretty(file, self).unwrap();
    }

    pub fn load_from(path: String) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)
            .unwrap();

        from_reader(file).unwrap()
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

    pub fn null_now() -> Self {
        HajFren {
            uuid: Uuid::new_v4(),
            dt_created: Utc::now(),
            dt_changed: Utc::now(),
            files: Option::None,
            data: T::default(),
            tags: Vec::default(),
        }
    }

    pub fn new_with_data(data: T) -> Self {
        HajFren {
            uuid: Uuid::new_v4(),
            dt_created: Utc::now(),
            dt_changed: Utc::now(),
            files: Option::None,
            data: data,
            tags: Vec::default(),
        }
    }

    pub fn new_with_data_tags(data: T, tags: Vec<Tag>) -> Self {
        HajFren {
            uuid: Uuid::new_v4(),
            dt_created: Utc::now(),
            dt_changed: Utc::now(),
            files: Option::None,
            data: data,
            tags: tags,
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
