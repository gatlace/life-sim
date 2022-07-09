use crate::{choices::*, people::*};
use postgres::{Client, NoTls};
use std::{fs::File, io::Read};
use uuid::Uuid;

// Initalization

const INITDB: &str = "./src/initdb.sql";

/// returns a postgres client
fn client() -> Client {
    Client::connect("host=localhost user=postgres", NoTls).expect("Failed to connect to database")
}

/// initializes the database
pub fn init() {
    let mut file = File::open(INITDB).expect("Failed to open initdb file");
    let mut contents = String::new();
    let mut client = client();

    file.read_to_string(&mut contents)
        .expect("Failed to read initdb file");

    if client
        .query(
            "select schema_name from information_schema.schemata where schema_name = 'life_sim'",
            &[],
        )
        .unwrap()
        .len()
        > 0
    {
        println!("Database already initialized");
        return;
    }

    client
        .batch_execute(&contents)
        .expect("Failed to initialize database");
    println!("Database initialized");
}

/// resets database
pub fn reset_db() {
    println!("Resetting database...");
    let mut client = client();
    client
        .execute("drop schema life_sim cascade", &[])
        .expect("Failed to drop schema");
    init();
}

// select queries

/// loads all decisions for a choice
fn get_decisions_from_choice_id(choice_id: Uuid) -> Vec<Decision> {
    let mut client = client();
    let query = "select * from life_sim.decisions where choice_id = $1";

    let rows = client
        .query(query, &[&choice_id])
        .expect("Failed to execute query");

    let decisions: Vec<Decision> = rows
        .iter()
        .map(|row| Decision {
            id: row.get(0),
            description: row.get(2),
            effects: get_effects_from_decision_id(row.get(0)),
        })
        .collect();

    decisions
}

/// loads all effects for a decision
fn get_effects_from_decision_id(decision_id: Uuid) -> Vec<Effect> {
    let mut client = client();
    let query = "select * from life_sim.effects where decision_id = $1";

    let rows = client
        .query(query, &[&decision_id])
        .expect("Failed to execute query");

    let effects: Vec<Effect> = rows
        .iter()
        .map(|row| Effect {
            id: row.get(0),
            property: match row.get(2) {
                "money" => Property::Money,
                "credit" => Property::Credit,
                _ => panic!("Unknown property"),
            },
            value: row.get(3),
            duration: row.get(4),
        })
        .collect();

    effects
}


pub fn load_choices_from_database() -> Vec<Choice> {
    let mut client = client();
    let query = "select * from life_sim.choices";

    let rows = client.query(query, &[]).expect("Failed to get choices");

    let choices: Vec<Choice> = rows
        .iter()
        .map(|row| Choice {
            id: row.get(0),
            name: row.get(1),
            min_age: row.get(2),
            max_age: row.get(3),
            decisions: get_decisions_from_choice_id(row.get(0)),
        })
        .collect();

    println!("Loaded {} choices from database", choices.len());
    choices
}

fn get_effects_from_person_id(id: Uuid) -> Vec<Effect> {
    let mut client = client();
    let query = "select * from life_sim.person_effect_links where person_id = $1";

    let rows = client
        .query(query, &[&id])
        .expect("Failed to execute query");

    let effects: Vec<Effect> = rows
        .iter()
        .map(|row| Effect {
            id: row.get(0),
            property: match row.get(2) {
                "money" => Property::Money,
                "credit" => Property::Credit,
                _ => panic!("Unknown property"),
            },
            value: row.get(3),
            duration: row.get(4),
        })
        .collect();

    effects
}

pub fn load_people_from_database() -> Vec<Person> {
    let mut client = client();
    let query = "select * from life_sim.people";
    let rows = client.query(query, &[]).expect("Failed to execute query");

    let people: Vec<Person> = rows
        .iter()
        .map(|row| Person {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            age: row.get(3),
            money: row.get(4),
            credit: row.get(5),
            effects: get_effects_from_person_id(row.get(0)),
        })
        .collect();

    println!("Loaded {} people from database", people.len());
    people
}
// insert queries

/// inserts a choice into the database
pub fn insert_choice(choice: &Choice) {
    let mut client = client();
    let query = "INSERT INTO life_sim.choices (id, name, min_age, max_age) VALUES ($1, $2, $3, $4) returning id";

    client
        .execute(
            query,
            &[&choice.id, &choice.name, &choice.min_age, &choice.max_age],
        )
        .expect("Failed to create choice");

    for decision in &choice.decisions {
        let query = "INSERT INTO life_sim.decisions (id, description, choice_id) VALUES ($1, $2, $3) returning id";
        client
            .execute(query, &[&decision.id, &decision.description, &choice.id])
            .expect("Failed to create decision");

        for effect in &decision.effects {
            let query = "INSERT INTO life_sim.effects (id, property, value, duration, decision_id) VALUES ($1, $2, $3, $4, $5)";
            client
                .query(
                    query,
                    &[
                        &effect.id,
                        &effect.property.to_string(),
                        &effect.value,
                        &effect.duration,
                        &decision.id,
                    ],
                )
                .expect("Failed to create effect");
        }
    }
    println!("Choice inserted, id: {}", choice.id);
}

/// inserts a person into the database
pub fn insert_person(person: &Person) {
    let mut client = client();
    let query = "INSERT INTO life_sim.people (id, first_name, last_name, age, money, credit) VALUES ($1, $2, $3, $4, $5, $6) returning id";

    client
        .execute(
            query,
            &[
                &person.id,
                &person.first_name,
                &person.last_name,
                &(person.age as i32),
                &(person.money as i32),
                &(person.credit as i32),
            ],
        )
        .expect("Failed to create person");

    for effect in person.effects.iter() {
        let query =
            "INSERT INTO life_sim.person_effect_links (id, person_id, effect_id, time_left) VALUES ($1, $2, $3, $4)";

        client
            .execute(
                query,
                &[
                    &(Uuid::new_v4()),
                    &person.id,
                    &effect.id,
                    &(effect.duration as i32),
                ],
            )
            .expect("Failed to create person effect link");
    }
    println!("Person inserted, id: {}", person.id);
}

// update queries

/// removes a choice from the database
pub fn remove_choice(choice_id: i32) {
    let mut client = client();
    let query = "DELETE FROM life_sim.choices WHERE id = $1";

    client
        .execute(query, &[&choice_id])
        .expect("Failed to delete choice");
    println!("Choice deleted, id: {}", choice_id);
}
