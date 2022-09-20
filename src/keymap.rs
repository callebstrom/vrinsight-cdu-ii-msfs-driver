use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Mappings {}

#[derive(Debug, Deserialize)]
pub struct KeyMap {
    mappings: HashMap<String, String>,
}

impl KeyMap {
    pub fn new() -> KeyMap {
        let mut file = File::open("keymap_example.yaml").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");

        serde_yaml::from_str::<KeyMap>(&contents).expect("Could not read keymap")
    }

    pub fn get_event(&self, key: &String) -> String {
        self.mappings
            .get(key)
            .unwrap_or(&"".to_string())
            .to_string()
    }
}
