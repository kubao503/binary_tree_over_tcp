#[derive(Debug, PartialEq)]
pub enum TreeCreationError {
    InvalidNodeIndex(usize),
    MultipleNodeReferences(usize),
    NotComplete { expected: usize, actual: usize },
    ChildNodeWithoutParent(usize),
}

pub type TreeCreationResult<T> = Result<T, TreeCreationError>;
