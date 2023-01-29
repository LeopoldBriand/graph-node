use super::builders::{DirectedGraphBuilder, GraphBuilder};

/// The node struct for undirected graph implementation 
#[derive(Clone)]
pub struct GraphNode<T: Clone> {
    /// The `data` variable stores the data that were used to build the node.
    pub data: T,
    /// The key which identifies the node.
    pub key: String,
    /// The list of keys of the nodes that are linked with the node.
    pub neighbour_keys: Vec<String>,
    /// This variable is set to true if the node is inside a graph cycle.
    pub has_circular_ref: bool
}


impl<T> GraphNode<T> where T: GraphBuilder + Clone {
    /// Return a new node from data that implement DirectedNode trait
    pub fn new(data: T) -> GraphNode<T> {
        let key = data.build_node_key();
        let neighbour_keys = data.build_neighbour_keys();
        GraphNode { data, key, neighbour_keys, has_circular_ref: false}
    }
}

/// The node struct for directed graph implementation 
#[derive(Clone)]
pub struct DirectedNode<T: Clone> {
    /// The `data` variable stores the data that were used to build the node.
    pub data: T,
    /// The key which identifies the node.
    pub key: String,
    /// The list of keys of the nodes that are linked to the node.
    pub parent_keys: Vec<String>,
    /// The list of keys of the nodes that are linked from the node.
    pub child_keys: Vec<String>,
    /// This variable is set to true if the node is inside a graph cycle.
    pub has_circular_ref: bool
}

impl<T> DirectedNode<T> where T: DirectedGraphBuilder + Clone {
    /// Return a new node from data that implement DirectedNode trait
    pub fn new(data: T) -> DirectedNode<T> {
        let key = data.build_node_key();
        let parent_keys = data.build_parent_key();
        let child_keys = data.build_child_key();
        DirectedNode { data, key, parent_keys, child_keys, has_circular_ref: false}
    }
    /// Return a true if the node has one parent ore more
    pub fn has_parents(&self) -> bool {
        return self.parent_keys.len() > 0;
    }
    /// Return a true if the node has one child ore more
    pub fn has_children(&self) -> bool {
        return self.child_keys.len() > 0;
      }
}