#[derive(Clone)]
pub struct Node<'a> {
    pub text: &'a str,
    pub left_child: NodeChild<'a>,
    pub right_child: NodeChild<'a>,
}

type NodeChild<'a> = Option<Box<Node<'a>>>;

impl<'a> Node<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            text: value,
            left_child: None,
            right_child: None,
        }
    }

    fn new_child(value: &'a str) -> NodeChild<'a> {
        Some(Box::new(Self::new(value)))
    }

    fn left(&mut self) -> &mut Self {
        self.left_child.as_mut().unwrap()
    }

    fn right(&mut self) -> &mut Self {
        self.right_child.as_mut().unwrap()
    }

    pub fn print_tree_paths(&self) {
        self.print_tree_paths_node("")
    }

    fn print_tree_paths_node(&self, path_text: &str) {
        let path_text = format!("{}{path_text}", self.text);

        if self.left_child.is_none() && self.right_child.is_none() {
            println!("{path_text}");
            return;
        }

        print_tree_paths_child(&self.left_child, &path_text);
        print_tree_paths_child(&self.right_child, &path_text);
    }
}

fn print_tree_paths_child(node: &NodeChild, path_text: &str) {
    if let Some(node) = node {
        node.print_tree_paths_node(path_text)
    }
}

pub fn get_example_tree<'a>() -> Node<'a> {
    let mut root = Node::new(".");
    root.left_child = Node::new_child(".pl");
    root.left().left_child = Node::new_child(".google");
    root.left().right_child = Node::new_child(".edu");
    root.left().right().left_child = Node::new_child(".pw");

    root
}
