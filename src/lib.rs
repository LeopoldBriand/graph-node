#![deny(missing_docs)]
//! Graph-node is an another library to deal with directed and undirected graphs.
//! It's an attempt to write a simple and easy to read graph library
//! ## Installation

//! ## Usage

//! `Tree` and `Node` are designed to work with custom data type while providing a
//! common interface for tree construction and navigation.

//! 1. Implement one of the builder trait to your data structure

//! ```rs
//! struct TestModel {
//!     name: String,
//!     children: Vec<String>,
//!     parents: Vec<String>
//! }
//! impl DirectedGraphBuilder for TestModel {
//!     fn build_child_key(&self) -> Vec<String> {
//!         return self.children.clone();
//!     }
//!     fn build_node_key(&self) -> String {
//!         return self.name.clone();
//!     }
//!     fn build_parent_key(&self) -> Vec<String> {
//!         return self.parents.clone();
//!     }
//! }
//! ```


//! 2. Then you can build your graph

//! ```rs
//! let data: Vec<TestModel> = getData();
//! let graph: Graph<Directed, TestModel> = Graph::new(data);
//! ``` 


/// The `node` module contains the different types of nodes used by the different types of graphs. 
/// It is these nodes that will contain the specific data that will be returned by the different graph methods.
pub mod node;

/// The `edge` module contails Edge and Edges types used in different graphs.
pub mod edge;

/// The `builder` module contains the different features used to build the nodes from the data.
pub mod builders;

/// The `graph` module contain the undirected graph struct.
// pub mod undirected_graph;

/// The `directed_graph` module contain the directed graph struct.
//pub mod directed_graph;

/// main graph module
pub mod graph;

/// Phantom types for traits implementations
pub mod types;
