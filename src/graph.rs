use crate::{node::Node, edge::Edges};

/// Graph structure
pub struct Graph<GraphType, T> where T: Clone {
    /// List of the nodes of the graph.
    pub nodes: Vec<Node<GraphType, T>>,
    /// List of the links between the nodes.
    pub edges: Edges,
    /// Is set to true when a graph has a circular reference or has no root nodes.
    pub has_circular_ref: bool,
    pub(crate) graph_type: std::marker::PhantomData<GraphType>,
}

impl<GraphType: Clone, T: Clone> Graph<GraphType, T> {
    /// Add a new node in the graph
    pub fn add_node(&mut self, node: Node<GraphType, T> ) {
        self.nodes.push(node);
    }
    /// Get node by key
    pub fn get_node_by_key(&self, key: String) -> Option<&Node<GraphType, T>> {
        match self.nodes.iter().position(|node| node.key == key) {
            Some(index) => Some(&self.nodes[index]),
            None => None
        }
    }
    /// Update a node with his key
    pub fn update_node_by_key(&mut self, key: String, new_node: Node<GraphType, T> ) {
        if let Some(index) = self.nodes.iter().position(|node| node.key == key) {
            self.nodes[index] = new_node;
        }
    }
    /// Delete a node from the graph found by his key
    pub fn delete_node_by_key(&mut self, key: String) {
        if let Some(index) = self.nodes.iter().position(|node| node.key == key) {
            self.nodes.swap_remove(index);
        }
    }
}