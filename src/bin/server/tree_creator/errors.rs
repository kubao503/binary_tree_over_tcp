#[derive(Debug, PartialEq)]
pub enum TreeCreatorError {
    InvalidNodeIndex(usize),
    MultipleNodeReferences(usize),
}
