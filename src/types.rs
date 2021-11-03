use std::error::Error;
use std::fmt::{Display, Formatter};

pub type VID = usize;

pub type EID = (VID, VID);

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct VertexError;

impl Display for VertexError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "invalid vertex")
    }
}

impl Error for VertexError {}
