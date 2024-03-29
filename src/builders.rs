/// This trait is needed to build `Node<Undirected, T>` struct and construct an undirected graph.
/// # Examples
/// ```
/// use graph_node::builders::UndirectedGraphBuilder;
/// 
/// struct TestModel {
///     name: String,
///     friends: Vec<String>,
/// }
/// impl UndirectedGraphBuilder for TestModel {
///     fn build_neighbour_keys(&self) -> Vec<String> {
///         self.friends.clone()
///     }
///     fn build_node_key(&self) -> String {
///         self.name.clone()
///     }
/// }
/// ```
pub trait UndirectedGraphBuilder {
    /// Should be implemented to return an array of the keys of nodes that are linked with the node.
    fn build_neighbour_keys(&self) -> Vec<String>;
    /// Should return the node key.
    fn build_node_key(&self) -> String;
}

/// This trait is needed to build `Node<Directed, T>` struct and construct a directed graph.
/// # Examples
/// ```
/// use graph_node::builders::DirectedGraphBuilder;
/// 
/// struct TestModel {
///     name: String,
///     children: Vec<String>,
///     parents: Vec<String>
/// }
/// impl DirectedGraphBuilder for TestModel {
///     fn build_child_key(&self) -> Vec<String> {
///         self.children.clone()
///     }
///     fn build_node_key(&self) -> String {
///         self.name.clone()
///     }
///     fn build_parent_key(&self) -> Vec<String> {
///         self.parents.clone()
///     }
/// }
/// ```
pub trait DirectedGraphBuilder {
    /// Should be implemented to return an array of the keys of nodes that are linked from the node.
    fn build_child_key(&self) -> Vec<String>;
    /// Should return the node key.
    fn build_node_key(&self) -> String;
    /// Should be implemented to return an array of the keys of nodes that are linked to the node.
    fn build_parent_key(&self) -> Vec<String>;
}