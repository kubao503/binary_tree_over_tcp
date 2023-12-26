#[derive(PartialEq, Debug)]
pub struct Node {
    pub text: String,
    pub left_child: NodeChild,
    pub right_child: NodeChild,
}

pub type NodeChild = Option<Box<Node>>;
pub struct NodeData(pub i32, pub i32, pub String);

impl Node {
    pub fn new(value: String) -> Self {
        Self {
            text: value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn new_child(value: String) -> NodeChild {
        Some(Box::new(Self::new(value)))
    }

    pub fn to_child(self) -> NodeChild {
        Some(Box::new(self))
    }

    pub fn unwrap_left_child(&mut self) -> &mut Self {
        self.left_child.as_mut().expect("Child is None")
    }

    pub fn unwrap_right_child(&mut self) -> &mut Self {
        self.right_child.as_mut().expect("Child is None")
    }

    pub fn print_tree_paths(&self, logger: &mut impl Logger) {
        self.print_tree_paths_node(logger, "")
    }

    fn print_tree_paths_node(&self, logger: &mut impl Logger, path_text: &str) {
        let path_text = format!("{}{path_text}", self.text);

        if self.left_child.is_none() && self.right_child.is_none() {
            logger.print(&path_text);
            return;
        }

        print_tree_paths_child(logger, &self.left_child, &path_text);
        print_tree_paths_child(logger, &self.right_child, &path_text);
    }
}

fn print_tree_paths_child(logger: &mut impl Logger, node: &NodeChild, path_text: &str) {
    if let Some(node) = node {
        node.print_tree_paths_node(logger, path_text)
    }
}

pub fn get_example_tree() -> Node {
    let mut root = Node::new(".".to_owned());
    root.left_child = Node::new_child(".pl".to_owned());
    root.unwrap_left_child().left_child = Node::new_child(".google".to_owned());
    root.unwrap_left_child().right_child = Node::new_child(".edu".to_owned());
    root.unwrap_left_child().unwrap_right_child().left_child = Node::new_child(".pw".to_owned());

    root
}

pub trait Logger {
    fn print(&mut self, string: &str);
}

pub struct PrintLogger;

impl Logger for PrintLogger {
    fn print(&mut self, string: &str) {
        println!("{string}");
    }
}

#[cfg(test)]
mod test_binary_tree;
