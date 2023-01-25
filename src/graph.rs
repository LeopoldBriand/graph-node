use super::node::{GraphNode};
use super::node::{GraphBuilder};

#[derive(Clone)]
pub struct Graph<T> where T: GraphBuilder + Clone {
    pub nodes: Vec<GraphNode<T>>,
    pub has_circular_ref: bool
}

impl<T: GraphBuilder + Clone> Graph<T> {
    pub fn new(data: Vec<T>) -> Graph<T> {
        let nodes: Vec<GraphNode<T>> = Vec::new();
        let mut graph = Graph {nodes, has_circular_ref: false};
        graph.check_circular_ref();
        graph.build_nodes(data);
        return graph;
    }
    pub fn get_circular_nodes(&self) -> Vec<GraphNode<T>> {
        self.nodes
            .iter()
            .filter(|node| node.has_circular_ref == true)
            .cloned()
            .collect()
    }
    pub fn get_neighbour_nodes(&self, current_node: GraphNode<T>) -> Vec<GraphNode<T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                return node.neighbour_keys.contains(&node.key); 
            })
            .cloned()
            .collect()
    }
    fn build_nodes(&mut self, data: Vec<T>) {
        let mut nodes: Vec<GraphNode<T>> = Vec::new();
        for d in data  {
            let new_node = GraphNode::new(d);
            match nodes.iter().find(|node| node.key == new_node.key) {
                Some(n) => eprintln!("Error: Duplicate node with key: {}, only the first one is added to the graph", n.key),
                None => nodes.push(new_node),
            } 
        }
        self.nodes = nodes;
    }
    fn check_circular_ref(&mut self) {
        if self.nodes.len() == 0 {
            eprintln!("Graph has no nodes.");
        } else {
            let node = self.nodes[0].clone();
            let visited_nodes_keys: Vec<String> = vec![node.key.clone()];
            self.recurse_check(visited_nodes_keys, self.get_neighbour_nodes(node));
        }
    }
    fn recurse_check(&mut self, visited_nodes_keys: Vec<String>, nodes_to_visit: Vec<GraphNode<T>>) {
        for mut node in nodes_to_visit.clone() {
            let mut acc = visited_nodes_keys.clone();
            if visited_nodes_keys.contains(&node.key) {
                node.has_circular_ref = true; // Maybe set all nodes in accumulator to true ?
                self.has_circular_ref = true;
                return;
            }
            acc.push(node.key.clone());
            let neighbours_to_visit: Vec<GraphNode<T>> = self
                .get_neighbour_nodes(node)
                .iter()
                .filter(|neighbour| {
                    match nodes_to_visit.iter().find(|node_to_visit| node_to_visit.key == neighbour.key) {
                        Some(_) => false,
                        None => true
                    }
                })
                .cloned()
                .collect();
            if neighbours_to_visit.len() > 0 {
                self.recurse_check( acc, neighbours_to_visit);
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
        return TestModel { name, friends }
    }
}

impl GraphBuilder for TestModel {
    fn build_neighbour_keys(&self) -> Vec<String> {
        return self.friends.clone();
    }
    fn build_node_key(&self) -> String {
        return self.name.clone();
    }
}
#[allow(dead_code)]
fn test_collection() -> Vec<TestModel> {
    let mut collection = Vec::new();
    collection.push(TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()]));
    collection.push(TestModel::new("name2".to_string(), vec!["name1".to_string(), "name4".to_string()]));
    collection.push(TestModel::new("name3".to_string(), vec!["name1".to_string()]));
    collection.push(TestModel::new("name4".to_string(), vec!["name2".to_string()]));
    return collection;
}
#[allow(dead_code)]
fn test_collection_with_duplicated_key() -> Vec<TestModel> {
    let mut collection = Vec::new();
    collection.push(TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()]));
    collection.push(TestModel::new("name1".to_string(), vec!["name3".to_string()]));
    return collection;
}
#[test]
fn basic_graph() {
    let data = test_collection();
    let graph: Graph<TestModel> = Graph::new(data);
    assert_eq!(graph.nodes.len(), 4, "should have nodes");
    let node = graph.nodes[0].clone();
    assert_eq!(node.data.name, "name1", "data is accessible");
}

#[test]
fn duplicated_nodes() {
    let data = test_collection_with_duplicated_key();
    let graph: Graph<TestModel> = Graph::new(data);
    assert_eq!(graph.nodes.len(), 1, "should have only one nodes");
}