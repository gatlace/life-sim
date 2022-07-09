use crate::{backend::database::insert_choice, inputs::*};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Error, Formatter},
    result::Result,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub id: Uuid,
    pub name: String,
    pub decisions: Vec<Decision>,
    pub min_age: i32,
    pub max_age: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,
    pub description: String,
    pub effects: Vec<Effect>,
}

impl Decision {
    pub fn new() -> Decision {
        let description = get_str("\nDecision description: ");
        let mut effects = Vec::with_capacity(get_int("Number of effects: ") as usize);

        for _ in 0..effects.capacity() {
            effects.push(Effect::new());
        }

        Decision {
            id: Uuid::new_v4(),
            description,
            effects,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Effect {
    pub id: Uuid,
    pub property: Property,
    pub value: String,
    pub duration: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Property {
    Money,
    Credit,
}

impl Property {
    pub fn from_str(s: &str) -> Result<Property, ()> {
        match s {
            "money" => Ok(Property::Money),
            "credit" => Ok(Property::Credit),
            _ => Err(()),
        }
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Property::Money => write!(f, "money"),
            Property::Credit => write!(f, "credit"),
        }
    }
}

impl Effect {
    pub fn new() -> Effect {
        Effect {
            id: Uuid::new_v4(),
            property: Property::from_str(get_str("\nEffect property: ").to_lowercase().trim())
                .unwrap(),
            value: get_str("Effect value: "),
            duration: get_int("Effect duration: "),
        }
    }
}

pub fn new() -> Choice {
    println!("\nNew Choice");
    let name = get_str("Choice name: ");
    let min_age = get_int("Minimum age: ");
    let max_age = get_int("Maximum age: ");
    let mut decisions = Vec::with_capacity(get_int("Number of decisions: ") as usize);
    for _ in 0..decisions.capacity() {
        decisions.push(Decision::new());
    }

    let choice = Choice {
        id: Uuid::new_v4(),
        name,
        decisions,
        min_age,
        max_age,
    };

    insert_choice(&choice);

    choice
}

impl Display for Choice {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "CHOICE:\n{}", self.name)?;
        for decision in &self.decisions {
            write!(f, "\n    Decision: {}", decision.description)?;
            for effect in &decision.effects {
                write!(f, "\n        Effect: {}", effect.property)?;
                write!(f, " - {}", effect.value)?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}
