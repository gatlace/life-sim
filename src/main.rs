mod backend;
mod inputs;
mod models;

use backend::Location;
use models::*;
use people::*;

fn main() {
    let choices = backend::load_choices(Location::Database);
    backend::reset_db();
    backend::save_choices(Location::Database, &choices);
    let mut person = Person::new("John".to_string(), "Doe".to_string());

    for choice in choices {
        for decision in choice.decisions {
            for effect in decision.effects {
                person.add_effect(&effect);
            }
        }
    }

    person.apply_effects();

    backend::save_people(Location::Database, Some(&[&person]));
}
