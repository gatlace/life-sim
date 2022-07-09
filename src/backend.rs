pub mod database;
pub mod json;

use crate::models::*;

pub enum Location {
    Database,
    Json,
}

pub fn load_choices(location: Location) -> Vec<choices::Choice> {
    match location {
        Location::Database => database::load_choices_from_database(),
        Location::Json => json::load_choices_from_json(),
    }
}

pub fn load_people(location: Location) -> Vec<people::Person> {
    match location {
        Location::Database => database::load_people_from_database(),
        Location::Json => json::load_people_from_json(),
    }
}

pub fn save_choices(location: Location, choices: &Vec<choices::Choice>) {
    match location {
        Location::Database => {
            for choice in choices {
                database::insert_choice(choice);
            }
        }
        Location::Json => json::save_choices_to_json(),
    }
}

pub fn save_people(location: Location, people: Option<&[&people::Person]>) {
    match location {
        Location::Database => {
            for person in people.unwrap() {
                database::insert_person(&person);
            }
        }
        Location::Json => json::save_people_to_json(),
    }
}

pub fn reset_db() {
    database::reset_db();
}

pub fn init_db() {
    database::init();
}
