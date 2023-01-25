pub trait Node<T> {}

pub trait GraphBuilder {
    fn build_neighbour_keys(&self) -> Vec<String>;
    fn build_node_key(&self) -> String;
}
pub trait DirectedGraphBuilder {
    fn build_child_key(&self) -> Vec<String>;
    fn build_node_key(&self) -> String;
    fn build_parent_key(&self) -> Vec<String>;
}

#[derive(Clone)]
pub struct GraphNode<T: Clone> {
    pub data: T,
    pub key: String,
    pub neighbour_keys: Vec<String>,
    pub has_circular_ref: bool
}

impl<T> Node<T> for GraphNode<T> where T: Clone {}

impl<T> GraphNode<T> where T: GraphBuilder + Clone {
    pub fn new(data: T) -> GraphNode<T> {
        let key = data.build_node_key();
        let neighbour_keys = data.build_neighbour_keys();
        GraphNode { data, key, neighbour_keys, has_circular_ref: false}
    }
}

#[derive(Clone)]
pub struct DirectedNode<T: Clone> {
    pub data: T,
    pub key: String,
    pub parent_keys: Vec<String>,
    pub child_keys: Vec<String>,
    pub has_circular_ref: bool
}

impl<T> Node<T> for DirectedNode<T> where T: Clone{}

impl<T> DirectedNode<T> where T: DirectedGraphBuilder + Clone {
    
    pub fn new(data: T) -> DirectedNode<T> {
        let key = data.build_node_key();
        let parent_keys = data.build_parent_key();
        let child_keys = data.build_child_key();
        DirectedNode { data, key, parent_keys, child_keys, has_circular_ref: false}
    }
    pub fn has_parents(&self) -> bool {
        return self.parent_keys.len() > 0;
    }
    pub fn has_children(&self) -> bool {
        return self.child_keys.len() > 0;
      }
}