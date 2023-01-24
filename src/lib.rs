pub trait NodeBuilder {
    fn build_child_key(&self) -> Vec<String>;
    fn build_node_key(&self) -> String;
    fn build_parent_key(&self) -> Vec<String>;
}

#[derive(Clone)]
pub struct Node<T: Clone> {
    data: T,
    key: String,
    parent_keys: Vec<String>,
    child_keys: Vec<String>,
    has_circular_ref: bool
}

impl<T: NodeBuilder + Clone> Node<T> {
    pub fn new(data: T) -> Node<T> {
        let key = data.build_node_key();
        let parent_keys = data.build_parent_key();
        let child_keys = data.build_child_key();
        Node { data, key, parent_keys, child_keys, has_circular_ref: false}
    }
    pub fn has_parents(&self) -> bool {
        return self.parent_keys.len() > 0;
    }
    pub fn has_children(&self) -> bool {
        return self.child_keys.len() > 0;
      }
}

#[derive(Clone)]
pub struct Tree<T: NodeBuilder + Clone> {
    nodes: Vec<Node<T>>,
    has_circular_ref: bool
}

impl<T: NodeBuilder + Clone> Tree<T> {
    pub fn new(data: Vec<T>) -> Tree<T> {
        let nodes: Vec<Node<T>> = Vec::new();
        let mut tree = Tree {nodes, has_circular_ref: false};
        tree.check_circular_ref();
        tree.build_nodes(data);
        return tree;
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
            nodes.push(Node::new(d));
        }
        self.nodes = nodes;
    }
    fn check_circular_ref(&mut self) {
        let root_nodes = self.get_root_nodes();
        if root_nodes.len() == 0 && self.nodes.len() > 0 {
            eprintln!("Tree has no root nodes and could have circular reference but cannot determine where.");
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

#[cfg(test)]
mod test;