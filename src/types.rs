use std::error::Error;

pub type VID = u32;

pub type EID = (VID, VID);

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct VertexError;

impl std::fmt::Display for VertexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid vertex")
    }
}

impl Error for VertexError {}
