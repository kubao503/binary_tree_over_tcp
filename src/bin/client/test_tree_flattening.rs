use super::*;

fn get_tree() -> Node {
    let mut root = Node::new("<root>".to_owned());
    root.left_child = Node::new_child("<left>".to_owned());

    root
}

fn get_flattened_tree() -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&2u32.to_be_bytes());
    buf.extend_from_slice(&6u32.to_be_bytes());
    buf.extend_from_slice(&0xFFFFFFFFu32.to_be_bytes());
    buf.extend_from_slice(&0xFFFFFFFFu32.to_be_bytes());
    buf.extend_from_slice("<left>".as_bytes());
    buf.extend_from_slice(&6u32.to_be_bytes());
    buf.extend_from_slice(&0u32.to_be_bytes());
    buf.extend_from_slice(&0xFFFFFFFFu32.to_be_bytes());
    buf.extend_from_slice("<root>".as_bytes());

    buf
}

#[test]
fn test_convert_tree_to_buf() {
    let root = get_tree();
    let buf = convert_tree_to_buf(&root);

    assert_eq!(buf.len(), 40);
    assert_eq!(buf, get_flattened_tree());
}
