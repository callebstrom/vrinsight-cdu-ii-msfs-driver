use log::info;

use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EventWithValue {
    pub event: String,
    pub value: u32
}

#[derive(Debug, Deserialize)]
pub struct VarWithValue {
    pub var: String,
    pub value: u32
}

#[derive(Debug, Deserialize)]
pub struct Delay {
    pub delay: u64
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Action {
    EventWithValue(EventWithValue),
    EventWithoutValue(String),
    VarWithValue(VarWithValue),
    Delay(Delay)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ActionOrActionSequence {
    Single(Action),
    Sequence(Vec<Action>),
}

#[derive(Debug, Deserialize)]
pub struct KeyMap {
    port: String,
    mappings: HashMap<String, HashMap<String, ActionOrActionSequence>>,
}

impl KeyMap {
    pub fn new() -> KeyMap {
        let mut file = File::open("keymap_example.yaml").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");

        serde_yaml::from_str::<KeyMap>(&contents).expect("Could not read keymap")
    }

    pub fn get_event(&self, aircraft: &String, key: &String) -> Option<&ActionOrActionSequence> {
        info!("Read keymap: {:#?}", aircraft);
        self.mappings.get(aircraft).map(|a| a.get(key)).flatten()
    }

    pub fn get_port(&self) -> &String {
        return &self.port;
    }
}
