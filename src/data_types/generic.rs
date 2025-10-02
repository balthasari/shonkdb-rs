use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Default)]
pub struct Person {
    name: PersonName,
    pronouns: String,
    bio: String,
    email: EmailAddress,
    birth_date: Option<DateTime<Utc>>,
}
#[derive(Deserialize, Serialize)]
enum PersonName {
    full_name {
        first_name: String,
        secondary_names: Option<Vec<String>>,
        last_name: String,
    },
    pseudonym(String),
}
impl Default for PersonName {
    fn default() -> Self {
        PersonName::pseudonym("Shonklover".to_owned())
    }
}
#[derive(Deserialize, Serialize, Default)]
struct Link {
    descriptor: String,
    link: String,
    https: bool,
}
#[derive(Deserialize, Serialize, Default)]
struct EmailAddress {
    loacl: String,
    domain: String,
}
#[derive(Deserialize, Serialize, Default)]
struct ImageMeta {
    creation: DateTime<Utc>,
    artist: Person,
    subtitle: String,
    description: String,
}
