#[derive(Debug, PartialEq)]
pub enum TreeCreatorError {
    InvalidNodeIndex(usize),
    MultipleNodeReferences(usize),
    NotComplete { expected: usize, actual: usize },
    ChildNodeWithoutParent(usize),
}
