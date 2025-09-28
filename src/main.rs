use crate::{
    data_types::units::MetricValue,
    logic::{Haj, HajFren, ROOT},
};
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
    /*
        let mut anotherHaj: Haj<TestPerson> = Haj::load_from("shorkattak".to_owned());
        anotherHaj.write_self();
    */

    let hf: HajFren<TestPerson> = HajFren::null();
    let hf_now: HajFren<TestPerson> = HajFren::null_now();
    // Convert the Point to a JSON string.
    let mut haj: Haj<TestPerson> = Haj::new();

    haj.instert_record(hf_now);
    haj.instert_record(hf);

    haj.write_self();

    let somelen = MetricValue::new(
        300.0,
        data_types::units::MetricPrefix::centi,
        data_types::units::MetricUnit::Meter,
    );

    println!("{}", somelen.to_string());

    // Prints serialized = {"x":1,"y":2}
}
