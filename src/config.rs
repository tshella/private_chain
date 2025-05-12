use serde::{Deserialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct NodeConfig {
    pub node_id: String,
    pub bind_address: String,
    pub allowed_nodes: Vec<String>,
}

impl NodeConfig {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse config file")
    }
}
