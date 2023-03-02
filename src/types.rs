/// Directed phantom type
#[derive(Clone)]
pub struct Directed;
/// Undirected phantom type
#[derive(Clone)]
pub struct Undirected;

/// Directed graph implementation
pub mod directed_graph;

/// Weighted graph implementation
pub mod weighted_graph;

/// Undirected graph implementation
pub mod undirected_graph;