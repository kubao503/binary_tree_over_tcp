mod binary_tree;

use binary_tree::Node;
use gethostname::gethostname;
use std::{io::Read, net::*};

struct NodeData(i32, i32, String);

const INT_SIZE: usize = 4;

fn main() {
    let host_name = format!("{}:8000", gethostname().to_str().unwrap());
    println!("{}", host_name);

    let listener = TcpListener::bind(host_name).expect("Failed to bind");
    if let Ok((stream, _)) = listener.accept() {
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut nodes = Vec::new();
    let node_count = read_node_count(&mut stream);

    for _ in 0..node_count {
        let node_data = read_node_data(&mut stream);
        add_node(&mut nodes, node_data);
    }

    let root = nodes.pop().unwrap();
    root.print_tree_paths();
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

    let mut text = vec![0u8; text_len as usize];
    stream.read(&mut text).unwrap();
    let text = String::from_utf8(text).unwrap();

    NodeData(left_idx, right_idx, text)
}

fn add_node<'a>(nodes: &mut Vec<Node>, node_data: NodeData) {
    let NodeData(left_idx, right_idx, text) = node_data;

    let mut node = Node::new(text);
    if left_idx >= 0 {
        node.left_child = nodes[left_idx as usize].clone().to_child()
    }
    if right_idx >= 0 {
        node.right_child = nodes[right_idx as usize].clone().to_child();
    }
    nodes.push(node);
}
