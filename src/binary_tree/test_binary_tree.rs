use super::*;

#[test]
fn test_node_new() {
    let node = Node::new(String::from("hello"));
    assert_eq!(node.text, "hello");
    assert!(node.left_child.is_none());
    assert!(node.right_child.is_none());
}
