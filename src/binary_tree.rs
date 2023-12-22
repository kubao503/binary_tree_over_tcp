pub mod tree_creator;

pub struct Node {
    text: String,
    left_child: NodeChild,
    right_child: NodeChild,
}

type NodeChild = Option<Box<Node>>;
pub struct NodeData(pub i32, pub i32, pub String);

impl Node {
    pub fn new(value: String) -> Self {
        Self {
            text: value,
            left_child: None,
            right_child: None,
        }
    }

    fn new_child(value: String) -> NodeChild {
        Some(Box::new(Self::new(value)))
    }

    pub fn to_child(self) -> NodeChild {
        Some(Box::new(self))
    }

    fn left(&mut self) -> &mut Self {
        self.left_child.as_mut().expect("Child is None")
    }

    fn right(&mut self) -> &mut Self {
        self.right_child.as_mut().expect("Child is None")
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

#[allow(dead_code)]
pub fn get_example_tree() -> Node {
    let mut root = Node::new(".".to_owned());
    root.left_child = Node::new_child(".pl".to_owned());
    root.left().left_child = Node::new_child(".google".to_owned());
    root.left().right_child = Node::new_child(".edu".to_owned());
    root.left().right().left_child = Node::new_child(".pw".to_owned());

    root
}

#[cfg(test)]
mod test_binary_tree;
