mod binary_tree;

// use binary_tree::*;
use gethostname::gethostname;
use std::io;
use std::{io::Read, net::*};

fn main() {
    // let root = get_example_tree();
    // root.print_tree_paths();
    // println!("{}", root.text);

    let host_name = format!("{}:8000", gethostname().to_str().unwrap());
    println!("{}", host_name);

    let listener = TcpListener::bind(host_name).expect("Failed to bind");
    if let Ok((stream, _)) = listener.accept() {
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut node_count = [0; 4];
    stream.read(&mut node_count)?;
    println!("{}", u32::from_be_bytes(node_count));
    Ok(())
}
