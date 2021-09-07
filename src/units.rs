use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fmt, fs};

#[derive(Debug)]
pub struct Units {
    pub all_units: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Building {
    pub unit_type: String,
    pub unit_name: String,
    pub cost: i32,
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.unit_name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub buildings: Vec<Building>,
    pub units: Vec<Unit>,
    pub defenses: Vec<Unit>,
}

impl FromWorld for Units {
    fn from_world(_world: &mut World) -> Self {
        Units {
            all_units: load_units().unwrap(),
        }
    }
}

fn load_units() -> Result<Data> {
    let file = fs::read_to_string("assets/json/units.json").expect("Oops, something went wrong.");
    let v: Data = serde_json::from_str(&file)?;

    println!("{:?}", v.buildings[0]);
    Ok(v)
}
