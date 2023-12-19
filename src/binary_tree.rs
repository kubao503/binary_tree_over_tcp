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

    pub fn to_child(self) -> NodeChild<'a> {
        Some(Box::new(self))
    }

    pub fn print_tree_paths(&self) {
        self.print_tree_paths_node("".to_owned())
    }

    fn print_tree_paths_node(&self, path_text: String) {
        let path_text = format!("{}{path_text}", self.text);

        if self.left_child.is_none() && self.right_child.is_none() {
            println!("{path_text}");
            return;
        }

        print_tree_paths_child(&self.left_child, path_text.clone());
        print_tree_paths_child(&self.right_child, path_text);
    }
}

fn print_tree_paths_child(node: &NodeChild, path_text: String) {
    if let Some(node) = node {
        node.print_tree_paths_node(path_text)
    }
}

pub fn get_example_tree<'a>() -> Node<'a> {
    let left_branch = Node::new("google", None, None);

    let right_branch = Node::new("studia", None, None);
    let right_child = Node::new(".pw", right_branch.to_child(), None);
    let right_child = Node::new(".edu", None, Node::to_child(right_child));

    let root = Node::new(
        ".pl",
        Node::to_child(left_branch),
        Node::to_child(right_child),
    );
    let root = Node::new(".", Node::to_child(root), None);

    root
}
