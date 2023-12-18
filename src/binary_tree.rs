#[derive(Clone)]
pub struct Node<'a> {
    pub text: &'a str,
    pub left_child: Option<Box<Node<'a>>>,
    pub right_child: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new(
        value: &'a str,
        left_child: Option<Box<Node<'a>>>,
        right_child: Option<Box<Node<'a>>>,
    ) -> Self {
        Self {
            text: value,
            left_child,
            right_child,
        }
    }

    fn pack(node: Self) -> Option<Box<Node<'a>>> {
        Some(Box::new(node))
    }
}

fn print_tree_paths_rec(node: &Option<Box<Node>>, path_text: String) {
    if let Some(node) = node {
        let path_text = format!("{}{path_text}", node.text);

        if node.left_child.is_none() && node.right_child.is_none() {
            println!("{path_text}");
            return;
        }

        print_tree_paths_rec(&node.left_child, path_text.clone());
        print_tree_paths_rec(&node.right_child, path_text.clone());
    }
}

pub fn print_tree_paths(root: Node) {
    let root = &Some(Box::new(root));
    print_tree_paths_rec(root, "".to_owned());
}

pub fn get_example_tree<'a>() -> Node<'a> {
    let left_branch = Node::new("google", None, None);

    let right_branch = Node::new("studia", None, None);
    let right_child = Node::new(".pw", Node::pack(right_branch), None);
    let right_child = Node::new(".edu", None, Node::pack(right_child));

    let root = Node::new(".pl", Node::pack(left_branch), Node::pack(right_child));
    let root = Node::new(".", Node::pack(root), None);

    root
}
