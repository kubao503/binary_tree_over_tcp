mod binary_tree;
use binary_tree::*;

fn main() {
    let root = get_example_tree();
    print_tree_paths(root.clone());
}
