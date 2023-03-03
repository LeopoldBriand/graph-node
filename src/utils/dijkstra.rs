use crate::{graph::{Graph}, types::weighted_graph::Weighted, edge::Edges, builders::UndirectedGraphBuilder};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// ## Introduction
/// Dijkstra's algorithm is an path finding algorithm.
/// It return's the shortest path between two nodes in a weighted graph
/// 
/// Dijkstra struct is only used to call search function like Dijkstra::search(...)
/// ## Exemple
/// ### This graph will be used all along the exemple
/// ```mermaid
/// flowchart LR
///   Paris ---|1054| Berlin
///   Brest ---|591| Paris
///   Berlin ---|1502| Roma
///   Berne ---|572| Paris
///   Wien ---|840| Berne
///   Roma ---|924| Berne
///   Roma ---|1122| Wien
///   Bruxelles ---|897| Praha
///   Praha ---|333| Wien
///   Paris ---|312| Bruxelles
/// ```
/// 
/// ### Step by step code 
/// ```rust
/// // Imports 
/// use graph_node::builders::UndirectedGraphBuilder;
/// use graph_node::types::{Undirected, weighted_graph::Weighted};
/// use graph_node::node::Node;
/// use graph_node::graph::Graph;
/// use graph_node::utils::dijkstra::{Dijkstra, Path};
/// 
/// // First define the data structure
/// #[derive(Clone)]
/// struct UndirectedTestModel {
///     city_name: String,
///     connected_cities: Vec<(String, f64)>,
/// }
/// impl UndirectedTestModel {
///     pub fn new(city_name: String, connected_cities: Vec<(String, f64)>) -> UndirectedTestModel {
///         return UndirectedTestModel { city_name, connected_cities }
///     }
/// }
/// 
/// impl UndirectedGraphBuilder for UndirectedTestModel {
///     fn build_neighbour_keys(&self) -> Vec<String> {
///         return self.connected_cities.clone()
///             .into_iter()
///             .map(|city| city.0)
///             .collect();
///     }
///     fn build_node_key(&self) -> String {
///         return self.city_name.clone();
///     }
/// }
/// 
/// impl Weighted<Undirected, UndirectedTestModel> for Graph<Undirected, UndirectedTestModel> {
///     fn build_edge(&self, node: Node<Undirected, UndirectedTestModel>, other_node_key: String) -> (String, (String, f64)) {
///         match node.data.connected_cities
///         .into_iter()
///         .find(|city| city.0 == other_node_key) {
///             Some(connection) => (node.key,(other_node_key, connection.1)),
///             None => (node.key,(other_node_key, 0.0)),
///         }
///     }
/// }
/// 
/// // Then create graph
/// let mut data: Vec<UndirectedTestModel> = Vec::new();
/// data.push(UndirectedTestModel::new("Paris".to_string(), vec![("Berlin".to_string(), 1054.0), ("Bruxelles".to_string(), 312.0)]));
/// data.push(UndirectedTestModel::new("Berlin".to_string(), vec![("Roma".to_string(), 1502.0)]));
/// data.push(UndirectedTestModel::new("Brest".to_string(), vec![("Paris".to_string(), 591.0)]));
/// data.push(UndirectedTestModel::new("Berne".to_string(), vec![("Paris".to_string(), 572.0)]));
/// data.push(UndirectedTestModel::new("Wien".to_string(), vec![("Berne".to_string(), 840.0)]));
/// data.push(UndirectedTestModel::new("Roma".to_string(), vec![("Berne".to_string(), 924.0), ("Wien".to_string(), 1122.0)]));
/// data.push(UndirectedTestModel::new("Bruxelles".to_string(), vec![("Praha".to_string(), 897.0)]));
/// data.push(UndirectedTestModel::new("Praha".to_string(), vec![("Wien".to_string(), 333.0)]));
/// let mut graph = Graph::<Undirected, UndirectedTestModel>::new(data);
/// graph.build_edges();
///
/// // Now Dijkstra algorithm can be called
/// let path: Option<Path> = Dijkstra::search(graph, "Paris".to_owned(), "Praha".to_owned());
/// ```
/// This return a Option<[Path]> with the involved nodes and total path weight : `Some(Path { nodes: ["Paris", "Bruxelles", "Praha"], weight: 1209.0 })`
/// 
/// 
/// 
/// 

pub struct Dijkstra {
    paths:Vec<Path>
}

impl Dijkstra {
    /// Take a weighted graph and nodes keys in parameters and return list of nodes
    pub fn search<GraphType, T: Clone>(g: Graph<GraphType, T>, origin_key: String, dest_key: String) -> Option<Path>
    where Graph<GraphType, T>: Weighted<GraphType, T> {
        let mut instance = Dijkstra {paths: vec![Path{nodes: vec![origin_key.clone()], weight: 0.0}]};
        return instance.dijkstra_recursive_search(origin_key, g.edges, instance.paths[0].clone(), dest_key);
    }
    // Create recursive function
    fn dijkstra_recursive_search(&mut self, src_key: String, edges: Edges, path: Path, dest_key: String) -> Option<Path> {
        // For each edge of source node create a path
        for edge in &edges[&src_key] {
            if edge.0 == &dest_key { // Shortest path found
                return Some(path.new_path_with(edge.0, *edge.1));
            } else if !path.nodes.contains(&edge.0) { // Node never been visited
                self.paths.push(path.new_path_with(edge.0, *edge.1));
            }
        }
        // Continue searching with smalest path and remove it from stack
        self.paths.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());
        match self.paths.pop() {
            Some(path) => {
                self.dijkstra_recursive_search(path.nodes.last().unwrap().clone(), edges, path,  dest_key)
            },
            None => { // No more path to search on
                return None;
            }
        }
    }
}


/// Struct describing the smalest path returned by dijkstra
#[derive(Clone)]
#[derive(Debug)]
pub struct Path {
    nodes: Vec<String>,
    weight: f64,
}
impl Path {
    fn new_path_with(&self, key: &String, weight: f64) -> Path {
        let mut nodes = self.nodes.clone();
        nodes.push(key.clone());
        Path {nodes, weight: self.weight + weight}
    }
}


// ----------------------------------------------------------------
//                     Tests
// ----------------------------------------------------------------

#[derive(Clone)]
struct UndirectedTestModel {
    city_name: String,
    connected_cities: Vec<(String, f64)>,
}
impl UndirectedTestModel {
    pub fn new(city_name: String, connected_cities: Vec<(String, f64)>) -> UndirectedTestModel {
        return UndirectedTestModel { city_name, connected_cities }
    }
}

impl UndirectedGraphBuilder for UndirectedTestModel {
    fn build_neighbour_keys(&self) -> Vec<String> {
        return self.connected_cities.clone()
            .into_iter()
            .map(|city| city.0)
            .collect();
    }
    fn build_node_key(&self) -> String {
        return self.city_name.clone();
    }
}
#[allow(dead_code)]
fn undirected_test_collection() -> Vec<UndirectedTestModel> {
    let mut collection = Vec::new();
    collection.push(UndirectedTestModel::new("Paris".to_string(), vec![("Berlin".to_string(), 1054.0), ("Brest".to_string(), 591.0), ("Berne".to_string(), 572.0), ("Bruxelles".to_string(), 312.0)]));
    collection.push(UndirectedTestModel::new("Berlin".to_string(), vec![("Paris".to_string(), 1054.0), ("Roma".to_string(), 1502.0)]));
    collection.push(UndirectedTestModel::new("Brest".to_string(), vec![("Paris".to_string(), 591.0)]));
    collection.push(UndirectedTestModel::new("Roma".to_string(), vec![("Berlin".to_string(), 1502.0), ("Berne".to_string(), 924.0), ("Wien".to_string(), 1122.0)]));
    collection.push(UndirectedTestModel::new("Berne".to_string(), vec![("Paris".to_string(), 572.0), ("Wien".to_string(), 840.0), ("Roma".to_string(), 924.0)]));
    collection.push(UndirectedTestModel::new("Wien".to_string(), vec![("Berne".to_string(), 840.0), ("Praha".to_string(), 333.0), ("Roma".to_string(), 1122.0)]));
    collection.push(UndirectedTestModel::new("Bruxelles".to_string(), vec![("Praha".to_string(), 897.0), ("Paris".to_string(), 312.0)]));
    collection.push(UndirectedTestModel::new("Praha".to_string(), vec![("Bruxelles".to_string(), 897.0), ("Wien".to_string(), 333.0)]));
    return collection;
}

#[test]
fn complete_undirected_graph() {
    use crate::types::Undirected;
    use crate::node::Node;
    #[cfg(test)]
    impl Weighted<Undirected, UndirectedTestModel> for Graph<Undirected, UndirectedTestModel> {
        fn build_edge(&self, node: Node<Undirected, UndirectedTestModel>, other_node_key: String) -> (String, (String, f64)) {
            match node.data.connected_cities
            .into_iter()
            .find(|city| city.0 == other_node_key) {
                Some(connection) => (node.key,(other_node_key, connection.1)),
                None => (node.key,(other_node_key, 0.0)),
            }
        }
    }
    let data = undirected_test_collection();
    let mut graph = Graph::<Undirected, UndirectedTestModel>::new(data);
    graph.build_edges();
    assert_eq!(graph.nodes.len(), 8, "should have nodes");
    assert_eq!(graph.edges["Paris"].len(), 4, "first node should have 4 connection");
    let node = graph.nodes[0].clone();
    assert_eq!(node.data.city_name, "Paris", "data is accessible");
    let path: Path = Dijkstra::search(graph, "Paris".to_owned(), "Praha".to_owned()).unwrap();
    assert_eq!(path.nodes[0], "Paris", "checking first node of the path");
    assert_eq!(path.nodes[1], "Bruxelles", "checking second node of the path");
    assert_eq!(path.nodes[2], "Praha", "checking last node of the path");
    assert_eq!(path.weight, 1209.0, "checking last node of the path");
    
}