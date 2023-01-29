# graph-node

An another library to deal with directed and undirected graphs.

!! This is still under development and lots of features are missing.

## Installation

TODO

## Usage

`Tree` and `Node` are designed to work with custom data type while providing a
common interface for tree construction and navigation.

1. Implement one of the builder trait to your data structure

```rs
struct TestModel {
    name: String,
    children: Vec<String>,
    parents: Vec<String>
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
```


2. Then you can build your graph

```rs
let data: Vec<TestModel> = getData();
let graph: DirectedGraph<TestModel> = DirectedGraph::new(data);
```

## Graphs

There are 2 types of graph in the library:
 - Directed graphs (DirectedGraph)
 - Undirected graphs (Graph)

DirectedGraph can be built only on data that implement DirectedGraphBuilder trait and Clone trait.
Graph can be built only on data that implement GraphBuilder trait and Clone trait.