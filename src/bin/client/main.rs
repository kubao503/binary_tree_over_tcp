use binary_tree::*;
use std::{io::Write, net::TcpStream};

fn main() {
    let root = get_example_tree();
    root.print_tree_paths(&mut PrintLogger);

    let buf = convert_tree_to_buf(&root);
    println!("{buf:?}");

    let addr = String::from("server:8000");
    let mut stream = TcpStream::connect(&addr).expect(&format!("Failed to connect to {addr}"));
    stream.write(&buf).unwrap();
}

const NULL_IDX: u32 = 0xFFFFFFFFu32;
const INT_SIZE: usize = 4;

fn convert_tree_to_buf(root: &Node) -> Vec<u8> {
    let mut buf = vec![0; INT_SIZE];
    let mut idx = 0;
    convert_tree_to_buf_node(root, &mut buf, &mut idx);
    buf[..INT_SIZE].copy_from_slice(&idx.to_be_bytes());
    buf
}

fn convert_tree_to_buf_node(node: &Node, buf: &mut Vec<u8>, idx: &mut u32) -> Option<u32> {
    let left_idx = match convert_tree_to_buf_child(&node.left_child, buf, idx) {
        Some(idx) => idx,
        None => NULL_IDX,
    };
    let right_idx = match convert_tree_to_buf_child(&node.right_child, buf, idx) {
        Some(idx) => idx,
        None => NULL_IDX,
    };

    let text = node.text.as_bytes();
    let text_len: u32 = text.len().try_into().expect("Text too long");

    buf.extend_from_slice(&text_len.to_be_bytes());
    buf.extend_from_slice(&left_idx.to_be_bytes());
    buf.extend_from_slice(&right_idx.to_be_bytes());
    buf.extend_from_slice(text);

    let current_idx: u32 = *idx;
    *idx += 1;

    Some(current_idx)
}

fn convert_tree_to_buf_child(child: &NodeChild, buf: &mut Vec<u8>, idx: &mut u32) -> Option<u32> {
    match child {
        Some(node) => convert_tree_to_buf_node(node, buf, idx),
        None => None,
    }
}

#[cfg(test)]
mod test_tree_flattening;
