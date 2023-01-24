use super::*;
#[derive(Clone)]
struct TestModel {
    name: String,
    children: Vec<String>,
    parents: Vec<String>
}
impl TestModel {
    pub fn new(name: String, children: Vec<String>, parents: Vec<String>) -> TestModel {
        return TestModel { name, children, parents }
    }
}

impl NodeBuilder for TestModel {
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
fn test_collection() -> Vec<TestModel> {
    let mut collection = Vec::new();
    collection.push(TestModel::new("name1".to_string(), vec!["name2".to_string(), "name3".to_string()], vec![]));
    collection.push(TestModel::new("name2".to_string(), vec!["name3".to_string()], vec!["name1".to_string()]));
    collection.push(TestModel::new("name3".to_string(), vec!["name4".to_string()], vec!["name2".to_string(), "name1".to_string()]));
    collection.push(TestModel::new("name4".to_string(), vec![], vec!["name3".to_string()]));
    return collection;
}
#[test]
fn basic_tree() {
    let tree: Tree<TestModel> = Tree::new(test_collection());
    assert_eq!(tree.nodes.len(), 4, "should have nodes");
    assert_eq!(tree.get_root_nodes().len(), 1, "should have root nodes");
    assert_eq!(tree.get_leaf_nodes().len(), 1, "should have leaf nodes");
    let root_node = tree.get_root_nodes()[0].clone();
    assert_eq!(tree.get_child_nodes(root_node).len(), 2, "root node should have children");
    let leaf_node = tree.get_leaf_nodes()[0].clone();
    assert_eq!(tree.get_parent_nodes(leaf_node).len(), 1, "leaf node should have parents");
    let node = tree.nodes[0].clone();
    assert_eq!(node.data.name, "name1", "data is accessible");
}