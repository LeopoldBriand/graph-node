use std::collections::HashMap;
use crate::types::{Undirected, Directed};
use crate::builders::{DirectedGraphBuilder, UndirectedGraphBuilder};

/// The direction of a link between to nodes
#[derive(Clone)]
#[derive(PartialEq)]
enum LinksDirection {
    /// Undirected Nodes
    Both,
    /// From parent to child
    From,
    /// From child to parent
    To,
}

/// The node struct for undirected graph implementation 
#[derive(Clone)]
pub struct Node<NodeType, T: Clone> {
    /// The `data` variable stores the data that were used to build the node.
    pub data: T,
    /// The key which identifies the node.
    pub key: String,
    links: HashMap<String, LinksDirection>,
    /// This variable is set to true if the node is inside a graph cycle.
    pub is_in_circular_ref: bool,
    node_type: std::marker::PhantomData<NodeType>,
}

impl<T> Node<Undirected, T> where T: UndirectedGraphBuilder + Clone {
    /// The node for undirected graph implementation 
    pub fn new(data: T) -> Node<Undirected, T> {
        let key = data.build_node_key();
        let links: HashMap<String, LinksDirection> = data
            .build_neighbour_keys()
            .into_iter()
            .map(|el| (el, LinksDirection::Both))
            .collect();
        Node { 
            data, 
            key, 
            links, 
            is_in_circular_ref: false, 
            node_type: std::marker::PhantomData::<Undirected>,
        }
    }
    
    /// The list of keys of the nodes that are linked with the node.
    pub fn get_neighbour_keys(&self) -> Vec<String> {
        self.links.clone().into_keys().collect()
    }
}

impl<T> Node<Directed, T> where T: DirectedGraphBuilder + Clone {
    /// The node for directed graph implementation 
    pub fn new(data: T) -> Node<Directed, T> {
        let key = data.build_node_key();
        let parents_keys: HashMap<String, LinksDirection> = data
            .build_parent_key()
            .into_iter()
            .map(|el| (el, LinksDirection::From))
            .collect();
        let child_keys: HashMap<String, LinksDirection> = data
            .build_child_key()
            .into_iter()
            .map(|el| (el, LinksDirection::To))
            .collect();
        let links = parents_keys.into_iter().chain(child_keys).collect();
        Node { 
            data, 
            key, 
            links, 
            is_in_circular_ref: false, 
            node_type: std::marker::PhantomData::<Directed>,
        }
    }
    /// The list of keys of the nodes that are linked to the node.
    pub fn get_parent_keys(&self) -> Vec<String> {
        let filtered_hash: HashMap<String, LinksDirection> = self.links.clone()
            .into_iter()
            .filter(|(_key, value)| { value == &LinksDirection::From })
            .collect();
        filtered_hash.into_keys().collect()
    }
    /// The list of keys of the nodes that are linked from the node.
    pub fn get_child_keys(&self) -> Vec<String> {
        let filtered_hash: HashMap<String, LinksDirection> = self.links.clone()
            .into_iter()
            .filter(|(_key, value)| { value == &LinksDirection::To })
            .collect();
        filtered_hash.into_keys().collect()
    }
    /// Add parent with th node key
    pub fn add_parent(&mut self, key: String) {
        self.links.insert(key, LinksDirection::From);
    }
    /// Add child with the node key
    pub fn add_child(&mut self, key: String) {
        self.links.insert(key, LinksDirection::To);
    }
    /// Return a true if the node has one parent ore more
    pub fn has_parents(&self) -> bool {
        !self.get_parent_keys().is_empty()
    }
    /// Return a true if the node has one child ore more
    pub fn has_children(&self) -> bool {
        !self.get_child_keys().is_empty()
    }
}