use crate::logic::{Haj, HajFren};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, to_string, to_string_pretty, to_writer, to_writer_pretty};
pub mod data_types;
pub mod logic;

#[derive(Default, Deserialize, Serialize)]
struct TestPerson {
    name: String,
    age: usize,
}

fn main() {
    let hf: HajFren<TestPerson> = HajFren::null();

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string_pretty(&hf).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
}
