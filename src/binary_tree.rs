#[derive(Clone)]
pub struct Node<'a> {
    pub text: &'a str,
    pub left_child: NodeChild<'a>,
    pub right_child: NodeChild<'a>,
}

type NodeChild<'a> = Option<Box<Node<'a>>>;

impl<'a> Node<'a> {
    pub fn new(value: &'a str, left_child: NodeChild<'a>, right_child: NodeChild<'a>) -> Self {
        Self {
            text: value,
            left_child,
            right_child,
        }
    }

    fn new_child(node: Self) -> NodeChild<'a> {
        Some(Box::new(node))
    }
}

fn print_tree_paths_rec(node: &NodeChild, path_text: String) {
    if let Some(node) = node {
        let path_text = format!("{}{path_text}", node.text);

        if node.left_child.is_none() && node.right_child.is_none() {
            println!("{path_text}");
            return;
        }

        print_tree_paths_rec(&node.left_child, path_text.clone());
        print_tree_paths_rec(&node.right_child, path_text);
    }
}

pub fn print_tree_paths(root: Node) {
    let root = &Node::new_child(root);
    print_tree_paths_rec(root, "".to_owned());
}

pub fn get_example_tree<'a>() -> Node<'a> {
    let left_branch = Node::new("google", None, None);

    let right_branch = Node::new("studia", None, None);
    let right_child = Node::new(".pw", Node::new_child(right_branch), None);
    let right_child = Node::new(".edu", None, Node::new_child(right_child));

    let root = Node::new(
        ".pl",
        Node::new_child(left_branch),
        Node::new_child(right_child),
    );
    let root = Node::new(".", Node::new_child(root), None);

    root
}
