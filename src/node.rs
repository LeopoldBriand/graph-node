pub trait NodeBuilder {
    fn build_child_key(&self) -> Vec<String>;
    fn build_node_key(&self) -> String;
    fn build_parent_key(&self) -> Vec<String>;
}

#[derive(Clone)]
pub struct Node<T: Clone> {
    pub data: T,
    pub key: String,
    pub parent_keys: Vec<String>,
    pub child_keys: Vec<String>,
    pub has_circular_ref: bool
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