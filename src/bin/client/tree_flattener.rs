use binary_tree::*;

pub struct TreeFlattener {
    buf: Vec<u8>,
    idx: u32,
}

impl TreeFlattener {
    const NULL_IDX: u32 = 0xFFFFFFFFu32;
    const INT_SIZE: usize = 4;

    pub fn get(self) -> Vec<u8> {
        self.buf
    }

    pub fn from(root: &Node) -> Self {
        let mut flattener = TreeFlattener::new();
        flattener.convert_tree_to_buf_node(root);
        flattener.write_node_count();

        flattener
    }

    fn new() -> Self {
        Self {
            buf: vec![0; Self::INT_SIZE],
            idx: 0,
        }
    }

    fn write_node_count(&mut self) {
        let node_count = self.idx;
        self.buf[..Self::INT_SIZE].copy_from_slice(&node_count.to_be_bytes());
    }

    fn convert_tree_to_buf_node(&mut self, node: &Node) -> Option<u32> {
        let left_idx = match self.convert_tree_to_buf_child(&node.left_child) {
            Some(idx) => idx,
            None => Self::NULL_IDX,
        };
        let right_idx = match self.convert_tree_to_buf_child(&node.right_child) {
            Some(idx) => idx,
            None => Self::NULL_IDX,
        };

        let text = node.text.as_bytes();
        let text_len: u32 = text.len().try_into().expect("Text too long");

        self.buf.extend_from_slice(&text_len.to_be_bytes());
        self.buf.extend_from_slice(&left_idx.to_be_bytes());
        self.buf.extend_from_slice(&right_idx.to_be_bytes());
        self.buf.extend_from_slice(text);

        let current_idx: u32 = self.idx;
        self.idx += 1;

        Some(current_idx)
    }

    fn convert_tree_to_buf_child(&mut self, child: &NodeChild) -> Option<u32> {
        match child {
            Some(node) => self.convert_tree_to_buf_node(node),
            None => None,
        }
    }
}

#[cfg(test)]
mod test_tree_flattener;
