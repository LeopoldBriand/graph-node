use super::node::{DirectedNode};
use super::node::{DirectedGraphBuilder};

pub struct DirectedGraph<T> where T: DirectedGraphBuilder + Clone {
    pub nodes: Vec<DirectedNode<T>>,
    pub has_circular_ref: bool
}

impl<T: DirectedGraphBuilder + Clone> DirectedGraph<T> {
    pub fn new(data: Vec<T>) -> DirectedGraph<T> {
        let nodes: Vec<DirectedNode<T>> = Vec::new();
        let mut graph = DirectedGraph {nodes, has_circular_ref: false};
        graph.check_circular_ref();
        graph.build_nodes(data);
        DirectedGraph::build_relationship(& mut graph.nodes);
        return graph;
    }
    pub fn get_root_nodes(&self) -> Vec<DirectedNode<T>> {
        self.nodes
            .iter()
            .filter(|node| node.has_parents() == false)
            .cloned()
            .collect()
    }
    pub fn get_leaf_nodes(&self) -> Vec<DirectedNode<T>> {
        self.nodes
            .iter()
            .filter(|node| node.has_children() == false)
            .cloned()
            .collect()
    }
    pub fn get_circular_nodes(&self) -> Vec<DirectedNode<T>> {
        self.nodes
            .iter()
            .filter(|node| node.has_circular_ref == true)
            .cloned()
            .collect()
    }
    pub fn get_child_nodes(&self, current_node: DirectedNode<T>) -> Vec<DirectedNode<T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                return current_node.child_keys.contains(&node.key)
            })
            .cloned()
            .collect()
    }
    pub fn get_parent_nodes(&self, current_node: DirectedNode<T>) -> Vec<DirectedNode<T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                return current_node.parent_keys.contains(&node.key)
            })
            .cloned()
            .collect()
    }
    pub fn get_sibling_nodes(&self, current_node: DirectedNode<T>) -> Vec<DirectedNode<T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                return node.parent_keys.iter().any(|key| current_node.parent_keys.contains(key)) 
            })
            .cloned()
            .collect()
    }
    fn build_nodes(&mut self, data: Vec<T>) {
        let mut nodes: Vec<DirectedNode<T>> = Vec::new();
        for d in data  {
            let new_node = DirectedNode::new(d);
            match nodes.iter().find(|node| node.key == new_node.key) {
                Some(n) => eprintln!("Error: Duplicate node with key: {}, only the first one is added to the graph", n.key),
                None => nodes.push(new_node),
            } 
        }
        self.nodes = nodes;
    }
    fn build_relationship(nodes: &mut Vec<DirectedNode<T>>) {
        let cloned_nodes = nodes.clone();
        for node in nodes {
            let parents: Vec<DirectedNode<T>> = cloned_nodes
                .iter()
                .filter(|cn| cn.child_keys.contains(&(node.key.clone())))
                .cloned()
                .collect();
            let children: Vec<DirectedNode<T>> = cloned_nodes
                .iter()
                .filter(|cn| cn.parent_keys.contains(&(node.key.clone())))
                .cloned()
                .collect();
            for parent_node in parents {
                if !node.parent_keys.contains(&parent_node.key) { node.parent_keys.push(parent_node.key)}
            }
            for child_node in children {
                if !node.child_keys.contains(&child_node.key) { node.child_keys.push(child_node.key)}
            }
        }
    }
    fn check_circular_ref(&mut self) {
        let root_nodes = self.get_root_nodes();
        if root_nodes.len() == 0 && self.nodes.len() > 0 {
            eprintln!("Graph has no root nodes and could have circular reference but cannot determine where.");
            self.has_circular_ref = true;
        }
        self.recurse_check(root_nodes, Vec::new());
    }
    fn recurse_check(&mut self, nodes: Vec<DirectedNode<T>>, accumulator: Vec<String>) {
        for mut node in nodes {
            let mut acc = accumulator.clone();
            if acc.contains(&node.key) {
                node.has_circular_ref = true;
                self.has_circular_ref = true;
                return;
            }
            acc.push(node.clone().key);
            if node.has_children() {
                self.recurse_check(self.get_child_nodes(node), acc);
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
    children: Vec<String>,
    parents: Vec<String>
}
impl TestModel {
    pub fn new(name: String, children: Vec<String>, parents: Vec<String>) -> TestModel {
        return TestModel { name, children, parents }
    }
}

impl DirectedGraphBuilder for TestModel {
    fn build_child_key(&self) -> Vec<String> {
        return self.children.clone();
    }
    fn build_node_key(&self) -> String {
        return self.name.clone();
    }
    fn build_parent_key(&self) -> Vec<String> {
        return self.parents.clone();
    }
}
#[allow(dead_code)]
fn test_collection() -> Vec<TestModel> {
    let mut collection = Vec::new();
    collection.push(TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]));
    collection.push(TestModel::new("name2".to_string(), vec!["name3".to_string()], vec!["name1".to_string()]));
    collection.push(TestModel::new("name3".to_string(), vec!["name4".to_string()], vec!["name2".to_string(), "name1".to_string()]));
    collection.push(TestModel::new("name4".to_string(), vec![], vec!["name3".to_string()]));
    return collection;
}
#[allow(dead_code)]
fn test_collection_with_duplicated_key() -> Vec<TestModel> {
    let mut collection = Vec::new();
    collection.push(TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]));
    collection.push(TestModel::new("name1".to_string(), vec!["name3".to_string()], vec![]));
    return collection;
}

#[allow(dead_code)]
fn test_collection_without_parent_key() -> Vec<TestModel> {
    let mut collection = Vec::new();
    collection.push(TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]));
    collection.push(TestModel::new("name2".to_string(), vec!["name3".to_string()], vec![]));
    collection.push(TestModel::new("name3".to_string(), vec!["name4".to_string()], vec![]));
    collection.push(TestModel::new("name4".to_string(), vec![], vec![]));
    return collection;
}

#[allow(dead_code)]
fn test_collection_without_child_key() -> Vec<TestModel> {
    let mut collection = Vec::new();
    collection.push(TestModel::new("name1".to_string(), vec![], vec![]));
    collection.push(TestModel::new("name2".to_string(), vec![], vec!["name1".to_string()]));
    collection.push(TestModel::new("name3".to_string(), vec![], vec!["name2".to_string(), "name1".to_string()]));
    collection.push(TestModel::new("name4".to_string(), vec![], vec!["name3".to_string()]));
    return collection;
}

#[test]
fn basic_graph() {
    let data = test_collection();
    let graph: DirectedGraph<TestModel> = DirectedGraph::new(data);
    assert_eq!(graph.nodes.len(), 4, "should have nodes");
    assert_eq!(graph.get_root_nodes().len(), 1, "should have root nodes");
    assert_eq!(graph.get_leaf_nodes().len(), 1, "should have leaf nodes");
    let root_node = graph.get_root_nodes()[0].clone();
    assert_eq!(graph.get_child_nodes(root_node).len(), 2, "root node should have children");
    let leaf_node = graph.get_leaf_nodes()[0].clone();
    assert_eq!(graph.get_parent_nodes(leaf_node).len(), 1, "leaf node should have parents");
    let node = graph.nodes[0].clone();
    assert_eq!(node.data.name, "name1", "data is accessible");
}

#[test]
fn duplicated_nodes() {
    let data = test_collection_with_duplicated_key();
    let graph: DirectedGraph<TestModel> = DirectedGraph::new(data);
    assert_eq!(graph.nodes.len(), 1, "should have only one nodes");
}

#[test]
fn without_parents() {
    let data = test_collection_without_parent_key();
    let graph: DirectedGraph<TestModel> = DirectedGraph::new(data);
    let data2 = test_collection();
    let graph2: DirectedGraph<TestModel> = DirectedGraph::new(data2);
    assert_eq!(graph.nodes.len(), graph2.nodes.len(), "should have same number of nodes");
    assert_eq!(graph.get_leaf_nodes().len(), graph2.get_leaf_nodes().len(), "should have same number of leaf nodes");
    assert_eq!(graph.get_root_nodes().len(), graph2.get_root_nodes().len(), "should have same number of root nodes");
}

#[test]
fn without_children() {
    let data = test_collection_without_child_key();
    let graph: DirectedGraph<TestModel> = DirectedGraph::new(data);
    let data2 = test_collection();
    let graph2: DirectedGraph<TestModel> = DirectedGraph::new(data2);
    assert_eq!(graph.nodes.len(), graph2.nodes.len(), "should have same number of nodes");
    assert_eq!(graph.get_leaf_nodes().len(), graph2.get_leaf_nodes().len(), "should have same number of leaf nodes");
    assert_eq!(graph.get_root_nodes().len(), graph2.get_root_nodes().len(), "should have same number of root nodes");
}