use serde_derive::*;
use serde_json;
use std::fs;

use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub ignore_dirs: Vec<String>,
    #[serde(default)]
    pub ignore_paths: Vec<String>,
}

impl Config {
    pub fn new(path: &PathBuf) -> Self {
        let path = path.join(".hammersyncconfig");
        match fs::read_to_string(&path) {
            Ok(d) => serde_json::from_str(&d).unwrap(),
            Err(_) => {
                let c = Config {
                    ignore_dirs: vec![],
                    ignore_paths: vec![],
                };
                c
            }
        }
    }
}
