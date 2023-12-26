#[derive(Debug, PartialEq)]
pub enum TreeCreationError {
    InvalidNodeIndex(usize),
    MultipleNodeReferences(usize),
    NotComplete { expected: usize, actual: usize },
    ChildNodeWithoutParent(usize),
}

pub type Result<T> = std::result::Result<T, TreeCreationError>;