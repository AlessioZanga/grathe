use crate::directions::DirectionalTrait;
use crate::errors::*;
use crate::io::*;
use crate::storages::StorageTrait;
use std::path::Path;

/// The base graph trait.
pub trait GraphTrait: DirectionalTrait + StorageTrait {
    /// From DOT string constructor.
    ///
    /// Construct a graph from a given DOT string.
    /// If more than one graph is parsed, the last one will be returned.
    ///
    /// # Errors
    ///
    /// The DOT string is invalid.
    ///
    /// # Panics
    ///
    /// Panics if the DOT string does not contain at least one graph.
    ///
    fn from_dot(string: &str) -> Result<Self, Error<Self::Vertex>> {
        let mut g = from_dot::<Self>(string).map_err(|x| Error::ParseFailed(x.to_string()))?;
        Ok(g.pop().unwrap())
    }

    /// To DOT string.
    ///
    /// Return a DOT string representation of the graph.
    ///
    fn to_dot(&self) -> Result<String, Error<Self::Vertex>> {
        to_dot::<Self>(&self).map_err(|x| Error::ParseFailed(x.to_string()))
    }

    /// Read DOT file constructor.
    ///
    /// Construct a graph from a given DOT file.
    /// If more than one graph is parsed, the last one will be returned.
    ///
    /// # Errors
    ///
    /// The DOT string is invalid.
    ///
    /// # Panics
    ///
    /// Panics if the DOT string does not contain at least one graph.
    ///
    fn read_dot(path: &Path) -> Result<Self, Error<Self::Vertex>> {
        let mut g = read_dot::<Self>(path).map_err(|x| Error::ParseFailed(x.to_string()))?;
        Ok(g.pop().unwrap())
    }

    /// Write DOT file.
    ///
    /// Write the graph to a given DOT file.
    ///
    fn write_dot(&self, path: &Path) -> Result<(), Error<Self::Vertex>> {
        write_dot::<Self>(path, &self).map_err(|x| Error::ParseFailed(x.to_string()))
    }
}
