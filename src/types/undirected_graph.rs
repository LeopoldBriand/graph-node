use crate::graph::Graph;
use crate::builders::UndirectedGraphBuilder;
use crate::types::Undirected;
use crate::node::Node;

impl<T: UndirectedGraphBuilder + Clone> Graph<Undirected, T> {
    /// Return a new [Graph] with undirected attributes and nodes build on top of datas.
    /// It will automaticaly build nodes relationship and check for any circular references
    pub fn new(data: Vec<T>) -> Graph<Undirected, T> {
        let nodes: Vec<Node<Undirected, T>> = Vec::new();
        let mut graph = Graph {
            nodes,
            has_circular_ref: false,
            graph_type: std::marker::PhantomData::<Undirected>,
        };
        graph.build_nodes(data);
        graph.check_circular_ref();
        return graph;
    }
    /// Get every nodes that are in a graph cycle.
    /// Warning: This return a copy of the nodes
    pub fn get_circular_nodes(&self) -> Vec<Node<Undirected, T>> {
        self.nodes
            .iter()
            .filter(|node| node.is_in_circular_ref == true)
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
                return node.get_neighbour_keys().contains(&node.key); 
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
        if self.nodes.len() == 0 {
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
            match self.get_node_by_key(key) {
                Some(node) => {
                    let neighbours_to_visit: Vec<String> = node
                    .get_neighbour_keys()
                    .into_iter()
                    .filter(|neighbour_key| {
                        match node_keys.iter().find(|&node_key| node_key == neighbour_key) {
                            Some(_) => false,
                            None => true
                        }
                    })
                    .collect();
                    if neighbours_to_visit.len() > 0 {
                        self.recurse_check( acc, neighbours_to_visit.clone());
                    }
                },
                None => {

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
        return TestModel { name, friends }
    }
}

impl UndirectedGraphBuilder for TestModel {
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