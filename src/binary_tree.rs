pub struct Node<'a> {
    pub left_child: Option<&'a Node<'a>>,
    pub right_child: Option<&'a Node<'a>>,
    pub value: &'a str,
}

impl<'a> Node<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            left_child: None,
            right_child: None,
        }
    }
}

fn print_tree_paths_rec(node: Option<&Node>, path_text: &str) {
    if let None = node {
        return;
    }
}

pub fn print_tree_paths(root: &Node) {
    print_tree_paths_rec(Some(root), "");
}
