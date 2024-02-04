use super::*;

struct TestLogger<'a>(&'a mut Vec<String>);

impl<'a> Logger for TestLogger<'a> {
    fn print(&mut self, string: &str) {
        self.0.push(string.to_owned());
    }
}

fn get_tree() -> Node {
    let mut root = Node::new("<root>".to_owned());
    root.left_child = Node::new_child("<left>".to_owned());
    root.right_child = Node::new_child("<right>".to_owned());

    root
}

#[test]
fn test_node_new() {
    let node = Node::new(String::from("hello"));
    assert_eq!(node.text, "hello");
    assert!(node.left_child.is_none());
    assert!(node.right_child.is_none());
}

#[test]
fn test_print_tree_paths() {
    let root = get_tree();
    let mut results = Vec::new();
    root.print_tree_paths(&mut TestLogger(&mut results));

    assert_eq!(results.len(), 2);
    assert_eq!(results[0], "<left><root>");
    assert_eq!(results[1], "<right><root>");
}
