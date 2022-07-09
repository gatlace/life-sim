use crate::{
    backend::database::*,
    models::{choices::*, people::*},
};

use std::{
    fs::File,
    io::{Read, Write},
};

pub fn save_choices_to_json() {
    let choices = load_choices_from_database();
    let mut file = File::create("choices.json").expect("Failed to create file");
    let json = serde_json::to_string_pretty(&choices).expect("Failed to serialize json");

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Saved choices to json");
}
pub fn load_choices_from_json() -> Vec<Choice> {
    let mut file = File::open("choices.json").expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    println!("Choices loaded from file");

    let choices: Vec<Choice> =
        serde_json::from_str(&contents).expect("Failed to deserialize from json");

    println!("Loaded {} choices from json", choices.len());
    choices
}

pub fn save_people_to_json() {
    let people = load_people_from_database();
    let mut file = File::create("people.json").expect("Failed to create file");
    let json = serde_json::to_string_pretty(&people).expect("Failed to serialize json");

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Saved people to json");
}

pub fn load_people_from_json() -> Vec<Person> {
    let mut file = File::open("people.json").expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    println!("People loaded from file");

    let people: Vec<Person> =
        serde_json::from_str(&contents).expect("Failed to deserialize from json");

    println!("Loaded {} people from json", people.len());
    
    people
}
