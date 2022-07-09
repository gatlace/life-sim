use crate::choices::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub money: i32,
    pub credit: i32,
    pub effects: Vec<Effect>,
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Person {
        Person {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            age: 0,
            money: 0,
            credit: 0,
            effects: Vec::new(),
        }
    }

    pub fn add_effect(&mut self, effect: &Effect) {
        self.effects.push(effect.clone());
    }

    pub fn apply_effects(&mut self) {
        for effect in self.effects.iter_mut() {
            match effect.property {
                Property::Money => self.money += effect.value.parse::<i32>().unwrap(),
                Property::Credit => self.credit += effect.value.parse::<i32>().unwrap(),
            }

            effect.duration -= 1;
            println!("affected {} {} by {}", self.first_name, effect.property, effect.value);
        }
    } 
}
