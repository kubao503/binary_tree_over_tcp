use binary_tree::*;

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

    pub fn add_node<'a>(&mut self, node_data: NodeData) -> Result<(), String> {
        let NodeData(left_idx, right_idx, text) = node_data;
        let mut node = Node::new(text);

        if let Ok(left_idx) = usize::try_from(left_idx) {
            self.can_node_be_taken(left_idx)?;

            let child = self.take_node(left_idx);
            node.left_child = child.to_child();
        }
        if let Ok(right_idx) = usize::try_from(right_idx) {
            self.can_node_be_taken(right_idx)?;

            let child = self.take_node(right_idx);
            node.right_child = child.to_child();
        }

        self.nodes.push(Some(node));
        Ok(())
    }

    fn can_node_be_taken(&self, index: usize) -> Result<&Node, String> {
        match self.nodes.get(index) {
            Some(node) => {
                match node {
                    None => Err(String::from("Two references to the same node")),
                    Some(node) => Ok(node),
                }
            },
            None => Err(format!("Invalid node index: {index}")),
        }
    }

    fn take_node(&mut self, node_index: usize) -> Node {
        let child = self.nodes.get_mut(node_index).expect("Invalid node index");
        child.take().expect("Two references to the same node")
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
