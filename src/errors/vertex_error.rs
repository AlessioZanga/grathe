use std::error::Error;
use std::fmt::{Display, Formatter};

/// Invalid vertex error.
#[derive(Debug)]
pub struct VertexError;

impl Display for VertexError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "invalid vertex")
    }
}

impl Error for VertexError {}
