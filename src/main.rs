mod binary_tree;
use binary_tree::*;

fn main() {
    let root = get_example_tree();
    root.print_tree_paths();
    println!("{}", root.text);
}
