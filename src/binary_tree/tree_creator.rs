use super::*;

pub struct TreeCreator {
    nodes: Vec<Option<Node>>,
    node_count: usize,
}

impl TreeCreator {
    pub fn new(node_count: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(node_count),
            node_count,
        }
    }

    pub fn add_node<'a>(&mut self, node_data: NodeData) {
        let NodeData(left_idx, right_idx, text) = node_data;
        let mut node = Node::new(text);

        if let Ok(left_idx) = usize::try_from(left_idx) {
            let child = self.become_parent_of(left_idx);
            node.left_child = child.to_child();
        }
        if let Ok(right_idx) = usize::try_from(right_idx) {
            let child = self.become_parent_of(right_idx);
            node.right_child = child.to_child();
        }

        self.nodes.push(Some(node));
    }

    fn become_parent_of(&mut self, child_index: usize) -> Node {
        std::mem::replace(&mut self.nodes[child_index], None)
            .expect("Two references to the same node")
    }

    fn validate_tree(&mut self) {
        if self.nodes.len() != self.node_count {
            panic!("Not all nodes are present");
        }
        let all_without_last = self.nodes.split_last().unwrap().1;
        if all_without_last.iter().any(|x| x.is_some()) {
            panic!("Not all non-root nodes have parent");
        }
    }

    pub fn get_tree(mut self) -> Node {
        self.validate_tree();
        self.nodes.pop().unwrap().unwrap()
    }
}

#[cfg(test)]
mod tests;
