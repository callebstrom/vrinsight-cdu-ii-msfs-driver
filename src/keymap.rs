use log::info;

use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Event {
    WithValue { event: String, value: u32 },
    WithoutValue(String),
}

#[derive(Debug, Deserialize)]
pub struct KeyMap {
    port: String,
    mappings: HashMap<String, HashMap<String, Event>>,
}

impl KeyMap {
    pub fn new() -> KeyMap {
        let mut file = File::open("keymap_example.yaml").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");

        let key_map = serde_yaml::from_str::<KeyMap>(&contents).expect("Could not read keymap");

        return key_map;
    }

    pub fn get_event(&self, aircraft: &String, key: &String) -> Option<&Event> {
        info!("Read keymap: {:#?}", aircraft);
        self.mappings.get(aircraft).map(|a| a.get(key)).flatten()
    }

    pub fn get_port(&self) -> &String {
        return &self.port;
    }
}
