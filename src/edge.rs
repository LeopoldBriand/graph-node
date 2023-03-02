use std::collections::HashMap;

type Key = String; // This represent a node key
type Weight = f64; // This represent a link wheight
/// This represent all the links between nodes with their weight
pub type Edges = HashMap<Key, HashMap<Key, Weight>>;