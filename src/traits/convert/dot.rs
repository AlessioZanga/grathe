use crate::errors::Error;
use crate::traits::Storage;
use std::fmt::Debug;
use std::path::Path;

pub trait FromDOT: Storage {
    fn from_dot(value: &String) -> Result<Self, Error<Self::Vertex>>;

    /// Read DOT file.
    ///
    /// Read DOT file into graph.
    ///
    fn read_dot(path: &Path) -> Result<Self, Error<Self::Vertex>> {
        // Read DOT file
        let string = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file \"{:?}\"", &path));
        // Parse DOT string.
        Self::from_dot(&string)
    }
}

pub trait IntoDOT: Storage {
    fn into_dot(&self) -> String;

    /// Write DOT file.
    ///
    /// Write graph to DOT file.
    ///
    fn write_dot(&self, path: &Path) {
        // Export graph into DOT string.
        let dot = self.into_dot();
        // Write string to file.
        std::fs::write(path, dot).unwrap_or_else(|_| panic!("Failed to write file \"{:?}\"", &path));
    }
}
