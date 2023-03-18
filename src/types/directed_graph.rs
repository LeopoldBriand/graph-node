use crate::edge::Edges;
use crate::graph::Graph;
use crate::builders::DirectedGraphBuilder;
use crate::types::Directed;
use crate::node::Node;

impl<T: DirectedGraphBuilder + Clone> Graph<Directed, T> {
    /// Return a new [Graph] with directed attributes and nodes build on top of datas.
    /// It will automaticaly build nodes relationship and check for any circular references
    pub fn new(data: Vec<T>) -> Graph<Directed, T> {
        let nodes: Vec<Node<Directed, T>> = Vec::new();
        let edges: Edges = Edges::new();
        let mut graph = Graph {
            nodes, 
            edges,
            has_circular_ref: false,
            graph_type: std::marker::PhantomData::<Directed>,
        };
        graph.build_nodes(data);
        graph.build_relationship();
        if graph.get_root_nodes().is_empty() && !graph.nodes.is_empty() {
            eprintln!("Graph has no root nodes and could have circular reference but cannot determine where.");
            graph.has_circular_ref = true;
        } else {
            graph.check_circular_ref();
        }
        graph
    }
    /// Get all nodes that have no parents. 
    /// Warning: This return a copy of the nodes
    pub fn get_root_nodes(&self) -> Vec<&Node<Directed, T>> {
        self.nodes
            .iter()
            .filter(|node| !node.has_parents())
            .collect()
    }
    /// Get all nodes that have no children.
    /// Warning: This return a copy of the nodes
    pub fn get_leaf_nodes(&self) -> Vec<&Node<Directed, T>> {
        self.nodes
            .iter()
            .filter(|node| !node.has_children())
            .collect()
    }
    /// Get every nodes that match a common parent of a given node.
    /// Warning: This return a copy of the nodes
    pub fn get_sibling_nodes(&self, current_node: &Node<Directed, T>) -> Vec<&Node<Directed, T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                return node.get_parent_keys().iter().any(|key| current_node.get_parent_keys().contains(key)) 
            })
            .collect()
    }
    /// Get every nodes that are in a graph cycle.
    /// Warning: This return a copy of the nodes
    pub fn get_circular_nodes(&self) -> Vec<&Node<Directed, T>> {
        self.nodes
            .iter()
            .filter(|node| node.is_in_circular_ref)
            .collect()
    }
    /// Get every parents of a given node.
    /// Warning: This return a copy of the nodes
    pub fn get_parent_nodes(&self, current_node: &Node<Directed, T>) -> Vec<&Node<Directed, T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                current_node.get_parent_keys().contains(&node.key)
            })
            .collect()
    }
    /// Get every childs of a given node.
    /// Warning: This return a copy of the nodes
    pub fn get_child_nodes(&self, current_node: &Node<Directed, T>) -> Vec<&Node<Directed, T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                current_node.get_child_keys().contains(&node.key)
            })
            .collect()
    }
    fn build_nodes(&mut self, data: Vec<T>) {
        let mut nodes: Vec<Node<Directed, T>> = Vec::new();
        for d in data  {
            let new_node: Node<Directed, T> =  Node::<Directed, T>::new(d);
            match nodes.iter().find(|node| node.key == new_node.key) {
                Some(n) => eprintln!("Error: Duplicate node with key: {}, only the first one is added to the graph", n.key),
                None => nodes.push(new_node),
            } 
        }
        self.nodes = nodes;
    }
    fn build_relationship(&mut self) {
        let cloned_nodes = self.nodes.clone();
        for node in &mut self.nodes {
            let parents: Vec<&Node<Directed, T>> = cloned_nodes
                .iter()
                .filter(|cn| cn.get_child_keys().contains(&(node.key.clone())))
                .collect();
            let children: Vec<&Node<Directed, T>> = cloned_nodes
                .iter()
                .filter(|cn| cn.get_parent_keys().contains(&(node.key.clone())))
                .collect();
            for parent_node in parents {
                if !node.get_parent_keys().contains(&parent_node.key) { node.add_parent(parent_node.key.clone())}
            }
            for child_node in children {
                if !node.get_child_keys().contains(&child_node.key) { node.add_child(child_node.key.clone())}
            }
        }
    }
    fn check_circular_ref(&mut self) {
        let root_nodes = self.get_root_nodes();
        let root_keys: Vec<String> = root_nodes.clone().into_iter().map(|node| node.key.clone()).collect(); 
        self.recurse_check(root_keys, Vec::new());
    }
    fn recurse_check(&mut self, node_keys: Vec<String>, accumulator: Vec<String>) {
        for key in node_keys {
            let mut acc = accumulator.clone();
            if let Some(node) = self.get_node_by_key(key.clone()) {
                if acc.contains(&key) {
                    let mut new_node = node.clone();
                    new_node.is_in_circular_ref = true;
                    self.update_node_by_key(key, new_node);
                    self.has_circular_ref = true;
                    return;
                }
                acc.push(key.clone());
                if node.has_children() {
                    self.recurse_check(node.get_child_keys(), acc);
                }
            };
        }
    }
}

// ----------------------------------------------------------------
//                     Tests
// ----------------------------------------------------------------

#[derive(Clone)]
struct TestModel {
    name: String,
    children: Vec<String>,
    parents: Vec<String>
}
impl TestModel {
    pub fn new(name: String, children: Vec<String>, parents: Vec<String>) -> TestModel {
        TestModel { name, children, parents }
    }
}

impl DirectedGraphBuilder for TestModel {
    fn build_child_key(&self) -> Vec<String> {
        self.children.clone()
    }
    fn build_node_key(&self) -> String {
        self.name.clone()
    }
    fn build_parent_key(&self) -> Vec<String> {
        self.parents.clone()
    }
}
#[allow(dead_code)]
fn test_collection() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]),
        TestModel::new("name2".to_string(), vec!["name3".to_string()], vec!["name1".to_string()]),
        TestModel::new("name3".to_string(), vec!["name4".to_string()], vec!["name2".to_string(), "name1".to_string()]),
        TestModel::new("name4".to_string(), vec![], vec!["name3".to_string()]),
    ]
}
#[allow(dead_code)]
fn test_collection_with_duplicated_key() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]),
        TestModel::new("name1".to_string(), vec!["name3".to_string()], vec![]),
    ]
}

#[allow(dead_code)]
fn test_collection_without_parent_key() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]),
        TestModel::new("name2".to_string(), vec!["name3".to_string()], vec![]),
        TestModel::new("name3".to_string(), vec!["name4".to_string()], vec![]),
        TestModel::new("name4".to_string(), vec![], vec![]),
    ]
}

#[allow(dead_code)]
fn test_collection_without_child_key() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec![], vec![]),
        TestModel::new("name2".to_string(), vec![], vec!["name1".to_string()]),
        TestModel::new("name3".to_string(), vec![], vec!["name2".to_string(), "name1".to_string()]),
        TestModel::new("name4".to_string(), vec![], vec!["name3".to_string()]),
    ]
}

#[allow(dead_code)]
fn test_collection_with_circular_references_without_root_nodes() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]),
        TestModel::new("name2".to_string(), vec!["name3".to_string()], vec![]),
        TestModel::new("name3".to_string(), vec!["name4".to_string()], vec![]),
        TestModel::new("name4".to_string(), vec!["name1".to_string()], vec![]),
    ]
}
#[allow(dead_code)]
fn test_collection_with_circular_references_with_root_nodes() -> Vec<TestModel> {
    vec![
        TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]),
        TestModel::new("name2".to_string(), vec!["name3".to_string()], vec![]),
        TestModel::new("name3".to_string(), vec!["name4".to_string()], vec![]),
        TestModel::new("name4".to_string(), vec!["name2".to_string()], vec![]),
    ]
}

#[test]
fn basic_graph() {
    let data = test_collection();
    let graph = Graph::<Directed, TestModel>::new(data);
    assert_eq!(graph.nodes.len(), 4, "should have nodes");
    assert_eq!(graph.get_root_nodes().len(), 1, "should have root nodes");
    assert_eq!(graph.get_leaf_nodes().len(), 1, "should have leaf nodes");
    let root_node = graph.get_root_nodes()[0].clone();
    assert_eq!(graph.get_child_nodes(&root_node).len(), 2, "root node should have children");
    let leaf_node = graph.get_leaf_nodes()[0].clone();
    assert_eq!(graph.get_parent_nodes(&leaf_node).len(), 1, "leaf node should have parents");
    let node = graph.nodes[0].clone();
    assert_eq!(node.data.name, "name1", "data is accessible");
}

#[test]
fn duplicated_nodes() {
    let data = test_collection_with_duplicated_key();
    let graph = Graph::<Directed, TestModel>::new(data);
    assert_eq!(graph.nodes.len(), 1, "should have only one nodes");
}

#[test]
fn without_parents() {
    let data = test_collection_without_parent_key();
    let graph = Graph::<Directed, TestModel>::new(data);
    let data2 = test_collection();
    let graph2 = Graph::<Directed, TestModel>::new(data2);
    assert_eq!(graph.nodes.len(), graph2.nodes.len(), "should have same number of nodes");
    assert_eq!(graph.get_leaf_nodes().len(), graph2.get_leaf_nodes().len(), "should have same number of leaf nodes");
    assert_eq!(graph.get_root_nodes().len(), graph2.get_root_nodes().len(), "should have same number of root nodes");
}

#[test]
fn without_children() {
    let data = test_collection_without_child_key();
    let graph = Graph::<Directed, TestModel>::new(data);
    let data2 = test_collection();
    let graph2 = Graph::<Directed, TestModel>::new(data2);
    assert_eq!(graph.nodes.len(), graph2.nodes.len(), "should have same number of nodes");
    assert_eq!(graph.get_leaf_nodes().len(), graph2.get_leaf_nodes().len(), "should have same number of leaf nodes");
    assert_eq!(graph.get_root_nodes().len(), graph2.get_root_nodes().len(), "should have same number of root nodes");
}

#[test]
fn circular_references() {
    let data_without_root_nodes = test_collection_with_circular_references_without_root_nodes();
    let data_with_root_nodes = test_collection_with_circular_references_with_root_nodes();
    let graph_without_root_nodes = Graph::<Directed, TestModel>::new(data_without_root_nodes);
    let graph_with_root_nodes = Graph::<Directed, TestModel>::new(data_with_root_nodes);
    assert_eq!(graph_without_root_nodes.nodes.len(), 4, "graph_without_root_nodes should have nodes");
    assert_eq!(graph_with_root_nodes.nodes.len(), 4, "graph_with_root_nodes should have nodes");
    assert_eq!(graph_without_root_nodes.has_circular_ref, true,"should have circular refs without root nodes");
    assert_eq!(graph_without_root_nodes.get_circular_nodes().len(), 0,"should not have circular nodes without root nodes");
    assert_eq!(graph_with_root_nodes.has_circular_ref, true,"should have circular refs with root nodes");
    assert_eq!(graph_with_root_nodes.get_circular_nodes().len(), 2,"should have circular nodes with root nodes");
    
}