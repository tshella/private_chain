pub fn validate_node(node_id: &str, allowed_nodes: &[String]) -> bool {
    allowed_nodes.contains(&node_id.to_string())
}
