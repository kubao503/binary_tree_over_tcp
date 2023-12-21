use super::*;

fn get_simple_tree() -> Node {
    let mut tree_creator = TreeCreator::new(1);
    let node_data = NodeData(-1, -1, String::from("root"));
    tree_creator.add_node(node_data);
    tree_creator.get_tree()
}

fn get_complex_tree_creator() -> TreeCreator {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data);

    let node_data = NodeData(0, -1, String::from("root"));
    tree_creator.add_node(node_data);

    tree_creator
}

fn get_isolated_tree_creator() -> TreeCreator {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data);

    let node_data = NodeData(-1, -1, String::from("root"));
    tree_creator.add_node(node_data);

    tree_creator
}

fn get_unfinished_tree_creator() -> TreeCreator {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data);

    tree_creator
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
    let tree_creator = get_complex_tree_creator();
    assert_eq!(tree_creator.node_referenced, [true, false]);

    let mut root = tree_creator.get_tree();
    assert_eq!(root.left().text, "left child");
}

#[test]
#[should_panic(expected = "Not all non-root nodes have parent")]
fn test_tree_conectivity() {
    let tree_creator = get_isolated_tree_creator();
    assert_eq!(tree_creator.node_referenced, [false, false]);
    tree_creator.get_tree();
}

#[test]
#[should_panic(expected = "Not all nodes are present")]
fn test_unfinished_tree() {
    let tree_creator = get_unfinished_tree_creator();
    tree_creator.get_tree();
}

#[test]
#[should_panic(expected = "Two references to the same node")]
fn test_two_references_to_the_same_node() {
    let mut tree_creator = get_complex_tree_creator();

    let node_data = NodeData(0, 1, String::from("super root"));
    tree_creator.add_node(node_data);
}
