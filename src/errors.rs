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
    #[error("edge identifier `{0:?}` not defined")]
    EdgeNotDefined((T, T)),
    /// Edge already defined error type.
    #[error("edge identifier `{0:?}` already defined")]
    EdgeAlreadyDefined((T, T)),
    /// Vertex label not defined error type.
    #[error("vertex label `{0:?}` not defined")]
    VertexLabelNotDefined(String),
    /// Vertex label already defined error type.
    #[error("vertex label `{0:?}` already defined")]
    VertexLabelAlreadyDefined(String),
    /// Edge label not defined error type.
    #[error("edge label `{0:?}` not defined")]
    EdgeLabelNotDefined(String),
    /// Edge label already defined error type.
    #[error("edge label `{0:?}` already defined")]
    EdgeLabelAlreadyDefined(String),
    /// Parsing error type.
    #[error("failed to parse graph")]
    ParseFailed(String),
}
