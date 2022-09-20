use log::info;

use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Mappings {}

#[derive(Debug, Deserialize)]
pub struct KeyMap {
    mappings: HashMap<String, HashMap<String, String>>,
}

impl KeyMap {
    pub fn new() -> KeyMap {
        let mut file = File::open("keymap_example.yaml").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");

        let key_map = serde_yaml::from_str::<KeyMap>(&contents).expect("Could not read keymap");

        info!("Read keymap");

        return key_map;
    }

    pub fn get_event(&self, aircraft: &String, key: &String) -> Option<&String> {
        self.mappings.get(aircraft).map(|a| a.get(key)).flatten()
    }
}
