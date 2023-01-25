use super::node::Node;
use super::node::NodeBuilder;

#[derive(Clone)]
pub struct Graph<T: NodeBuilder + Clone> {
    pub nodes: Vec<Node<T>>,
    pub has_circular_ref: bool
}

impl<T: NodeBuilder + Clone> Graph<T> {
    pub fn new(data: Vec<T>) -> Graph<T> {
        let nodes: Vec<Node<T>> = Vec::new();
        let mut graph = Graph {nodes, has_circular_ref: false};
        graph.check_circular_ref();
        graph.build_nodes(data);
        return graph;
    }
    pub fn get_root_nodes(&self) -> Vec<Node<T>> {
        self.nodes
            .iter()
            .filter(|node| node.has_parents() == false)
            .cloned()
            .collect()
    }
    pub fn get_leaf_nodes(&self) -> Vec<Node<T>> {
        self.nodes
            .iter()
            .filter(|node| node.has_children() == false)
            .cloned()
            .collect()
    }
    pub fn get_circular_nodes(&self) -> Vec<Node<T>> {
        self.nodes
            .iter()
            .filter(|node| node.has_circular_ref == true)
            .cloned()
            .collect()
    }
    pub fn get_child_nodes(&self, current_node: Node<T>) -> Vec<Node<T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                return current_node.child_keys.contains(&node.key)
            })
            .cloned()
            .collect()
    }
    pub fn get_parent_nodes(&self, current_node: Node<T>) -> Vec<Node<T>> {
        self.nodes
            .iter()
            .filter(|node| {
                if node.key == current_node.key {return false}
                return current_node.parent_keys.contains(&node.key)
            })
            .cloned()
            .collect()
    }
    pub fn get_sibling_nodes(&self, current_node: Node<T>) -> Vec<Node<T>> {
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
        let mut nodes: Vec<Node<T>> = Vec::new();
        for d in data  {
            let new_node = Node::new(d);
            match nodes.iter().find(|node| node.key == new_node.key) {
                Some(n) => eprintln!("Error: Duplicate node with key: {}, only the first one is added to the graph", n.key),
                None => nodes.push(new_node),
            } 
        }
        self.nodes = nodes;
    }
    fn check_circular_ref(&mut self) {
        let root_nodes = self.get_root_nodes();
        if root_nodes.len() == 0 && self.nodes.len() > 0 {
            eprintln!("Graph has no root nodes and could have circular reference but cannot determine where.");
            self.has_circular_ref = true;
        }
        self.recurse_check(root_nodes, Vec::new());
    }
    fn recurse_check(&mut self, nodes: Vec<Node<T>>, accumulator: Vec<String>) {
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