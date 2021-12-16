use thiserror::Error;

/// Error enumerator.
#[derive(Error, Debug)]
pub enum Error<T> {
    /// Vertex not defined error type.
    #[error("vertex identifier `{0:?}` not defined")]
    VertexNotDefined(T),
    /// Vertex already defined error type.
    #[error("vertex identifier `{0:?}` already defined")]
    VertexAlreadyDefined(T),
    /// Edge not defined error type.
    #[error("edge identifier `({0:?}, {1:?})` not defined")]
    EdgeNotDefined(T, T),
    /// Edge already defined error type.
    #[error("edge identifier `({0:?}, {1:?})` already defined")]
    EdgeAlreadyDefined(T, T),
    /// Vertex attribute not defined error type.
    #[error("vertex `{0:?}` has no attribute `{1:?}`")]
    VertexAttributeNotDefined(T, String),
    /// Edge attribute not defined error type.
    #[error("edge `({0:?}, {1:?})` has no attribute `{2:?}`")]
    EdgeAttributeNotDefined(T, T, String),
    /// Parsing error type.
    #[error("failed to parse graph")]
    ParseFailed(String),
}
