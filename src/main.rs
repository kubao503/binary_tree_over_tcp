mod binary_tree;

use binary_tree::{tree_creator::TreeCreator, NodeData, PrintLogger};
use gethostname::gethostname;
use std::{io::Read, net::*};

const INT_SIZE: usize = 4;

fn main() {
    let host_name = format!("{}:8000", gethostname().to_str().unwrap());
    println!("Will listen on {host_name}");

    let listener = TcpListener::bind(host_name).expect("Failed to bind");
    if let Ok((stream, address)) = listener.accept() {
        println!("Connection from {address}");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let node_count = read_node_count(&mut stream);
    let mut tree_creator = TreeCreator::new(node_count.try_into().unwrap());

    for _ in 0..node_count {
        let node_data = read_node_data(&mut stream);
        tree_creator.add_node(node_data);
    }

    let root = tree_creator.get_tree();
    root.print_tree_paths(&mut PrintLogger);
}

fn read_node_count(stream: &mut TcpStream) -> u32 {
    let mut node_count = [0; INT_SIZE];
    stream.read(&mut node_count).unwrap();
    let node_count = u32::from_be_bytes(node_count);

    node_count
}

fn read_node_data(stream: &mut TcpStream) -> NodeData {
    let mut text_len = [0; INT_SIZE];
    stream.read(&mut text_len).unwrap();
    let text_len = u32::from_be_bytes(text_len);

    let mut left_idx = [0; INT_SIZE];
    stream.read(&mut left_idx).unwrap();
    let left_idx = i32::from_be_bytes(left_idx);

    let mut right_idx = [0; INT_SIZE];
    stream.read(&mut right_idx).unwrap();
    let right_idx = i32::from_be_bytes(right_idx);

    let mut text = vec![0u8; text_len.try_into().unwrap()];
    stream.read(&mut text).unwrap();
    let text = String::from_utf8(text).unwrap();

    NodeData(left_idx, right_idx, text)
}
