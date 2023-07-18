use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: Option<String>,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person {
            name: Some(name),
            age
        }
    }
}

fn main() {
    let jeremy = Person::new("jeremy".to_string(), 34);

    let as_json = serde_json::to_string_pretty(&jeremy).unwrap();

    println!("{as_json}");

    let json_string = r#"{ "age": 34 }"#;

    let from_json: Person = serde_json::from_str(&json_string).unwrap();

    dbg!(from_json);

}
