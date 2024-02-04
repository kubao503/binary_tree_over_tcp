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
    assert!(root.left_child.is_none());
    assert!(root.right_child.is_none());
}

#[test]
fn test_complex_tree() {
    let tree_creator = get_complex_tree_creator();
    assert_eq!(
        tree_creator
            .nodes
            .iter()
            .map(|x| x.is_some())
            .collect::<Vec<bool>>(),
        [false, true]
    );

    let mut root = tree_creator.get_tree();
    assert_eq!(root.unwrap_left_child().text, "left child");
}

#[test]
#[should_panic(expected = "Not all non-root nodes have parent")]
fn test_tree_conectivity() {
    let tree_creator = get_isolated_tree_creator();
    assert_eq!(
        tree_creator
            .nodes
            .iter()
            .map(|x| x.is_some())
            .collect::<Vec<bool>>(),
        [true, true]
    );
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
