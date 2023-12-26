pub mod errors;

use binary_tree::{Node, NodeData};
use errors::{TreeCreationError, TreeCreationResult};

pub struct TreeCreator {
    nodes: Vec<Option<Node>>,
    node_count: usize,
}

impl TreeCreator {
    pub fn new(node_count: usize) -> Self {
        if node_count == 0 {
            panic!("Cannot instantiate zero-sized tree")
        }
        Self {
            nodes: Vec::with_capacity(node_count),
            node_count,
        }
    }

    pub fn add_node<'a>(&mut self, node_data: NodeData) -> TreeCreationResult<()> {
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

    fn take_node(&mut self, index: usize) -> TreeCreationResult<Node> {
        let node = self
            .nodes
            .get_mut(index)
            .ok_or(TreeCreationError::InvalidNodeIndex(index));
        node.and_then(|node| {
            node.take()
                .ok_or(TreeCreationError::MultipleNodeReferences(index))
        })
    }

    fn validate_tree(&mut self) -> TreeCreationResult<()> {
        if self.nodes.len() != self.node_count {
            return Err(TreeCreationError::NotComplete {
                actual: self.nodes.len(),
                expected: self.node_count,
            });
        }
        let (_, all_without_last) = self.nodes.split_last().expect("Tree empty");

        if let Some((index, _)) = all_without_last
            .iter()
            .enumerate()
            .find(|(_, node)| node.is_some())
        {
            return Err(TreeCreationError::ChildNodeWithoutParent(index));
        }
        Ok(())
    }

    pub fn get_tree(mut self) -> TreeCreationResult<Node> {
        self.validate_tree()?;
        Ok(self.nodes.pop().expect("Tree empty").expect("Root moved"))
    }
}

#[cfg(test)]
mod test_tree_creator;
