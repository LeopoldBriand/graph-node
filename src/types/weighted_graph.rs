use std::collections::HashMap;
use crate::builders::{DirectedGraphBuilder,UndirectedGraphBuilder};
use crate::graph::Graph;
use crate::node::Node;
use crate::types::{Undirected, Directed};

/// This trait implement weighted graphs.
pub trait Weighted<GraphType, T> where T: Clone{
    /// This function is the default implementation for weighted graph.
    /// It should return (first_node, (second_node_key, wheight_of_link)) with keys as String and wheight as f64
    fn build_edge(&self, node: Node<GraphType, T>, other_node_key: String) -> (String, (String, f64)) {
        (node.key, (other_node_key, 1.0)) 
    }
}

/// Implementation of Weighted Undirected Graph
impl<T: UndirectedGraphBuilder + Clone> Graph<Undirected, T> {
    /// Used to build edges of Weigthed Graph
    pub fn build_edges(&mut self) where Self : Weighted<Undirected, T> {
        for node in &self.nodes {
            for child_key in node.get_neighbour_keys() {
                let result = self.build_edge(node.clone(), child_key);
                match self.edges.get_mut(&result.0) {
                    Some(edge) => {
                        edge.insert(result.1.0, result.1.1);
                    },
                    None => {
                        let mut edge = HashMap::new();
                        edge.insert(result.1.0, result.1.1);
                        self.edges.insert(result.0, edge);
                    }
                }
            }
        }
    }

    /// Get the wheight of an edge
    pub fn get_edge_weight(&self, from_node_key: String, to_node_key: String) -> Option<f64> where Self: Weighted<Undirected, T> {
        match self.edges.get(&from_node_key) {
            Some(edge) => {
                edge.get(&to_node_key).copied()
            }
            None => None
        }
    }
}

/// Implementation of Weighted Undirected Graph
impl<T: DirectedGraphBuilder + Clone> Graph<Directed, T> {
    /// Used to build edges of Weigthed Graph
    pub fn build_edges(&mut self) where Self: Weighted<Directed, T> {
        for node in &self.nodes {
            for child_key in node.get_child_keys() {
                let result = self.build_edge(node.clone(), child_key);
                match self.edges.get_mut(&result.0) {
                    Some(edge) => {
                        edge.insert(result.1.0, result.1.1);
                    },
                    None => {
                        let mut edge = HashMap::new();
                        edge.insert(result.1.0, result.1.1);
                        self.edges.insert(result.0, edge);
                    }
                }
            }
        }
    }
    /// Get the wheight of an edge
    pub fn get_edge_weight(&self, from_node_key: String, to_node_key: String) -> Option<f64> where Self: Weighted<Directed, T> {
        match self.edges.get(&from_node_key) {
            Some(edge) => {
                edge.get(&to_node_key).copied()
            }
            None => None
        }
    }
}

// ----------------------------------------------------------------
//                     Tests
// ----------------------------------------------------------------

#[derive(Clone)]
struct UndirectedTestModel {
    city_name: String,
    connected_cities: Vec<(String, f64)>,
}
impl UndirectedTestModel {
    pub fn new(city_name: String, connected_cities: Vec<(String, f64)>) -> UndirectedTestModel {
        UndirectedTestModel { city_name, connected_cities }
    }
}

impl UndirectedGraphBuilder for UndirectedTestModel {
    fn build_neighbour_keys(&self) -> Vec<String> {
        self.connected_cities.clone()
            .into_iter()
            .map(|city| city.0)
            .collect()
    }
    fn build_node_key(&self) -> String {
        self.city_name.clone()
    }
}

#[allow(dead_code)]
fn undirected_test_collection() -> Vec<UndirectedTestModel> {
    vec![
        UndirectedTestModel::new("Paris".to_string(), vec![("Berlin".to_string(),1054.3), ("Brest".to_string(),591.2)]),
        UndirectedTestModel::new("Berlin".to_string(), vec![("Paris".to_string(), 1054.3), ("Roma".to_string(),1502.1)]),
        UndirectedTestModel::new("Brest".to_string(), vec![("Paris".to_string(), 591.2)]),
        UndirectedTestModel::new("Roma".to_string(), vec![("Berlin".to_string(), 1502.1)]),
    ]
}

#[test]
fn basic_undirected_graph() {
    #[cfg(test)]
    impl Weighted<Undirected, UndirectedTestModel> for Graph<Undirected, UndirectedTestModel> {
        fn build_edge(&self, node: Node<Undirected, UndirectedTestModel>, other_node_key: String) -> (String, (String, f64)) {
            match node.data.connected_cities
            .into_iter()
            .find(|city| city.0 == other_node_key) {
                Some(connection) => (node.key,(other_node_key, connection.1)),
                None => (node.key,(other_node_key, 0.0)),
            }
        }
    }
    let data = undirected_test_collection();
    let mut graph = Graph::<Undirected, UndirectedTestModel>::new(data);
    graph.build_edges();
    assert_eq!(graph.nodes.len(), 4, "should have nodes");
    assert_eq!(graph.edges["Paris"].len(), 2, "first node should have 2 connection");
    let node = graph.nodes[0].clone();
    assert_eq!(node.data.city_name, "Paris", "data is accessible");

}