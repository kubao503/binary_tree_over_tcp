mod errors;

use binary_tree::*;
use errors::*;

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

    pub fn add_node<'a>(&mut self, node_data: NodeData) -> Result<(), TreeCreatorError> {
        let NodeData(left_idx, right_idx, text) = node_data;
        let mut new_node = Node::new(text);

        if let Ok(left_idx) = usize::try_from(left_idx) {
            let node = self.take_node(left_idx)?;
            new_node.left_child = node.to_child();
        }
        if let Ok(right_idx) = usize::try_from(right_idx) {
            let node = self.take_node(right_idx)?;
            new_node.right_child = node.to_child();
        }

        self.nodes.push(Some(new_node));
        Ok(())
    }

    fn take_node(&mut self, index: usize) -> Result<Node, TreeCreatorError> {
        let node = self
            .nodes
            .get_mut(index)
            .ok_or(TreeCreatorError::InvalidNodeIndex(index));
        node.and_then(|node| {
            node.take()
                .ok_or(TreeCreatorError::MultipleNodeReferences(index))
        })
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
mod test_tree_creator;
