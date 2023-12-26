use super::*;

fn get_simple_tree() -> Node {
    let mut tree_creator = TreeCreator::new(1);
    let node_data = NodeData(-1, -1, String::from("root"));
    tree_creator.add_node(node_data).unwrap();
    tree_creator.get_tree().unwrap()
}

fn get_complex_tree_creator() -> TreeCreator {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data).unwrap();

    let node_data = NodeData(0, -1, String::from("root"));
    tree_creator.add_node(node_data).unwrap();

    tree_creator
}

fn get_isolated_tree_creator() -> TreeCreator {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data).unwrap();

    let node_data = NodeData(-1, -1, String::from("root"));
    tree_creator.add_node(node_data).unwrap();

    tree_creator
}

fn get_unfinished_tree_creator() -> TreeCreator {
    let mut tree_creator = TreeCreator::new(2);

    let node_data = NodeData(-1, -1, String::from("left child"));
    tree_creator.add_node(node_data).unwrap();

    tree_creator
}

#[test]
#[should_panic(expected = "Cannot instantiate zero-sized tree")]
fn test_empty_tree_creator() {
    let _ = TreeCreator::new(0);
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

    let mut root = tree_creator.get_tree().unwrap();
    assert_eq!(root.unwrap_left_child().text, "left child");
}

#[test]
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
    assert_eq!(
        tree_creator.get_tree(),
        Err(TreeCreatorError::ChildNodeWithoutParent(0))
    )
}

#[test]
fn test_two_references_to_the_same_node() {
    let mut tree_creator = get_complex_tree_creator();

    let node_data = NodeData(0, 1, String::from("super root"));
    assert!(tree_creator.add_node(node_data) == Err(TreeCreatorError::MultipleNodeReferences(0)));
}

#[test]
fn test_invalid_node_index() {
    let mut tree_creator = get_complex_tree_creator();

    let node_data = NodeData(1, 9, String::from("super root"));
    assert!(tree_creator.add_node(node_data) == Err(TreeCreatorError::InvalidNodeIndex(9)));
}

#[test]
fn test_self_referencing_node() {
    let mut tree_creator = get_unfinished_tree_creator();

    let node_data = NodeData(0, 1, String::from("root"));
    assert_eq!(
        tree_creator.add_node(node_data),
        Err(TreeCreatorError::InvalidNodeIndex(1))
    );
}

#[test]
fn test_tree_not_complete() {
    let tree_creator = get_unfinished_tree_creator();
    assert_eq!(
        tree_creator.get_tree(),
        Err(TreeCreatorError::NotComplete {
            expected: 2,
            actual: 1,
        })
    );
}
