mod binary_tree;
use binary_tree::*;

fn main() {
    let mut root = Node::new(".");
    root.left_child = Some(&Node::new(".pl"));

    print_tree_paths(&root);
}
