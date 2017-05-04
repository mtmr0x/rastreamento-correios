#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter};
use rustc_serialize::json::{Json, ToJson};

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    person_name: String,
}

impl ToJson for Person {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("person_name".to_string(), self.person_name.to_json());
        Json::Object(map)
    }
}

fn main() {
    let mut server = Nickel::new();
    server.get("/", middleware!("Hello Fucking World"));

    server.get("/:name", middleware! {|req|
        let name = req.param("name").unwrap();

        let person = Person {
            person_name: name.to_string(),
        };
        person.to_json()
    });

    server.listen("127.0.0.1:6767");
}

