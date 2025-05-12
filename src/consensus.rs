// Placeholder for a Proof of Authority system
// For now, only allow whitelisted nodes to write (you can extend this later)

pub fn is_authorized_node(node_id: &str) -> bool {
    // For demo: only allow node_id "node1" to write
    node_id == "node1"
}
