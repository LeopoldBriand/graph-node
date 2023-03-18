use crate::edge::Edges;
use crate::graph::Graph;
use crate::builders::UndirectedGraphBuilder;
use crate::types::Undirected;
use crate::node::Node;

impl<T: UndirectedGraphBuilder + Clone> Graph<Undirected, T> {
    /// Return a new [Graph] with undirected attributes and nodes build on top of datas.
    /// It will automaticaly build nodes relationship and check for any circular references
    pub fn new(data: Vec<T>) -> Graph<Undirected, T> {
        let nodes: Vec<Node<Undirected, T>> = Vec::new();
        let edges: Edges = Edges::new();
        let mut graph = Graph {
            nodes,
            edges,
            has_circular_ref: false,
            graph_type: std::marker::PhantomData::<Undirected>,
        };
        graph.build_nodes(data);
        graph.check_circular_ref();
        graph
    }
    /// Get every nodes that are in a graph cycle.
    /// Warning: This return a copy of the nodes
    pub fn get_circular_nodes(&self) -> Vec<Node<Undirected, T>> {
        self.nodes
            .iter()
            .filter(|node| node.is_in_circular_ref)
            .cloned()
            .collect()
    }
    /// Get every nodes linked with a given node.
    /// Warning: This return a copy of the nodes
    pub fn get_neighbour_nodes(&self, current_node: &Node<Undirected, T>) -> Vec<&Node<Undirected, T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                node.get_neighbour_keys().contains(&node.key)
            })
            .collect()
    }
    fn build_nodes(&mut self, data: Vec<T>) {
        let mut nodes: Vec<Node<Undirected, T>> = Vec::new();
        for d in data  {
            let new_node: Node<Undirected, T> = Node::<Undirected, T>::new(d);
            match nodes.iter().find(|node| node.key == new_node.key) {
                Some(n) => eprintln!("Error: Duplicate node with key: {}, only the first one is added to the graph", n.key),
                None => nodes.push(new_node),
            } 
        }
        self.nodes = nodes;
    }
    fn check_circular_ref(&mut self) {
        if self.nodes.is_empty() {
            eprintln!("Graph has no nodes.");
        } else {
            let node = self.nodes[0].clone();
            let visited_nodes_keys: Vec<String> = vec![node.key.clone()];
            self.recurse_check(visited_nodes_keys, node.get_neighbour_keys());
        }
    }
    fn recurse_check(&mut self, node_keys: Vec<String>, accumulator: Vec<String>) {
        for key in node_keys.clone() {
            let mut acc = accumulator.clone();
            if accumulator.contains(&key) {
                //node.is_in_circular_ref = true; // Maybe set all nodes in accumulator to true ?
                self.has_circular_ref = true;
                return;
            }
            acc.push(key.clone());
            if let Some(node) = self.get_node_by_key(key) {
                let neighbours_to_visit: Vec<String> = node
                .get_neighbour_keys()
                .into_iter()
                .filter(|neighbour_key| {
                    !node_keys.iter().any(|node_key| node_key == neighbour_key)
                })
                .collect();
                if !neighbours_to_visit.is_empty() {
                    self.recurse_check( acc, neighbours_to_visit.clone());
                }
            }
        
        }
    }

}

// ----------------------------------------------------------------
//                     Tests
// ----------------------------------------------------------------

#[derive(Clone)]
struct TestModel {
    name: String,
    friends: Vec<String>
}
impl TestModel {
    pub fn new(name: String, friends: Vec<String>) -> TestModel {
        TestModel { name, friends }
    }
}

impl UndirectedGraphBuilder for TestModel {
    fn build_neighbour_keys(&self) -> Vec<String> {
        self.friends.clone()
    }
    fn build_node_key(&self) -> String {
        self.name.clone()
    }
}
#[allow(dead_code)]
fn test_collection() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()]),
        TestModel::new("name2".to_string(), vec!["name1".to_string(), "name4".to_string()]),
        TestModel::new("name3".to_string(), vec!["name1".to_string()]),
        TestModel::new("name4".to_string(), vec!["name2".to_string()]),
    ]
}
#[allow(dead_code)]
fn test_collection_with_duplicated_key() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()]),
        TestModel::new("name1".to_string(), vec!["name3".to_string()]),
    ]
}
#[test]
fn basic_graph() {
    let data = test_collection();
    let graph = Graph::<Undirected, TestModel>::new(data);
    assert_eq!(graph.nodes.len(), 4, "should have nodes");
    let node = graph.nodes[0].clone();
    assert_eq!(node.data.name, "name1", "data is accessible");
}

#[test]
fn duplicated_nodes() {
    let data = test_collection_with_duplicated_key();
    let graph = Graph::<Undirected, TestModel>::new(data);
    assert_eq!(graph.nodes.len(), 1, "should have only one nodes");
}