use super::*;

fn get_simple_tree() -> Node {
    let mut tree_creator = TreeCreator::new(1);
    let node_data = NodeData(-1, -1, String::from("root"));
    tree_creator.add_node(node_data);
    tree_creator.get_tree()
}

fn get_complex_tree() -> Node {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data);

    let node_data = NodeData(0, -1, String::from("root"));
    tree_creator.add_node(node_data);

    tree_creator.get_tree()
}

#[test]
fn test_tree_creator() {
    let root = get_simple_tree();
    assert_eq!(root.text, "root");
}

#[test]
#[should_panic(expected = "Child is None")]
fn test_tree_creator_left_child() {
    let mut root = get_simple_tree();
    root.left();
}

#[test]
#[should_panic(expected = "Child is None")]
fn test_tree_creator_right_child() {
    let mut root = get_simple_tree();
    root.right();
}

#[test]
fn test_complex_tree() {
    let mut root = get_complex_tree();
    assert_eq!(root.left().text, "left child");
}

#[test]
#[should_panic(expected = "Not all non-root nodes have parent")]
fn test_tree_conectivity() {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data);

    let node_data = NodeData(-1, -1, String::from("root"));
    tree_creator.add_node(node_data);

    assert_eq!(tree_creator.has_parent, [false, false]);
    tree_creator.get_tree();
}
