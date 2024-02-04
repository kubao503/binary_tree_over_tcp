mod tree_flattener;

use binary_tree::*;
use std::{io::Write, net::TcpStream};
use tree_flattener::*;

fn main() {
    let root = get_example_tree();
    root.print_tree_paths(&mut PrintLogger);

    let flattener = TreeFlattener::from(&root);
    let buf = flattener.get();
    println!("{buf:?}");

    let addr = String::from("server:8000");
    let mut stream = TcpStream::connect(&addr).expect(&format!("Failed to connect to {addr}"));
    stream.write(&buf).unwrap();
}
