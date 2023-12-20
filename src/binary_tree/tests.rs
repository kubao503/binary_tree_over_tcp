use super::*;

fn get_tree() -> Node {
    let mut tree_creator = TreeCreator::new();
    let node_data = NodeData(-1, -1, String::from("root"));
    tree_creator.add_node(node_data);
    tree_creator.get_tree()
}

#[test]
fn test_tree_creator() {
    let root = get_tree();
    assert_eq!(root.text, "root");
}

#[test]
#[should_panic(expected = "Child is None")]
fn test_tree_creator_left_child() {
    let mut root = get_tree();
    root.left();
}

#[test]
#[should_panic(expected = "Child is None")]
fn test_tree_creator_right_child() {
    let mut root = get_tree();
    root.right();
}
