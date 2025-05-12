use std::net::SocketAddr;
use crate::config::NodeConfig;
use crate::access_control::validate_node;

pub struct Node {
    pub id: String,
    pub address: SocketAddr,
    pub is_authorized: bool,
}

impl Node {
    pub fn from_config(path: &str) -> Self {
        let config = NodeConfig::load(path);
        let is_authorized = validate_node(&config.node_id, &config.allowed_nodes);
        Node {
            id: config.node_id,
            address: config.bind_address.parse().expect("Invalid bind address"),
            is_authorized,
        }
    }

    pub fn start(&self) {
        println!(
            "🚀 Node '{}' starting at {} | Authorized: {}",
            self.id, self.address, self.is_authorized
        );
    }
}
