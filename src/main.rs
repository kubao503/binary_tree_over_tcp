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

#[derive(Debug)]
struct NodeData(i32, i32, String);

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut node_count = [0; 4];
    stream.read(&mut node_count)?;
    let node_count = u32::from_be_bytes(node_count);
    println!("{node_count}");

    for _ in 0..node_count {
        let node_data = read_node_data(&mut stream);
        println!("{:?}", node_data);
    }

    Ok(())
}

fn read_node_data(stream: &mut TcpStream) -> NodeData {
    let mut text_len = [0; 4];
    stream.read(&mut text_len).unwrap();
    let text_len = u32::from_be_bytes(text_len);

    let mut left_idx = [0; 4];
    stream.read(&mut left_idx).unwrap();
    let left_idx = i32::from_be_bytes(left_idx);

    let mut right_idx = [0; 4];
    stream.read(&mut right_idx).unwrap();
    let right_idx = i32::from_be_bytes(right_idx);

    let mut text = vec![0u8; text_len as usize];
    stream.read(&mut text).unwrap();
    let text = String::from_utf8(text).unwrap();

    NodeData(left_idx, right_idx, text)
}
