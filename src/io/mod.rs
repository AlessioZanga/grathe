mod dot;
use std::{collections::HashMap, path::Path};

pub use dot::*;

use crate::traits::{Storage, WithAttributes};

/// The parser IO trait.
pub trait IO: TryFrom<String> + TryInto<String> {
    /// Map parsed data into graph.
    ///
    fn map<G, F>(self, f: F) -> Vec<G>
    where
        G: Storage,
        F: Fn(String) -> G::Vertex;

    /// Map parsed data into graph with attributes.
    ///
    fn map_with_attributes<G, F, X, Y, Z>(self, f: F, x: X, y: Y, z: Z) -> Vec<G>
    where
        G: Storage + WithAttributes<G::Vertex>,
        F: Fn(String) -> G::Vertex,
        X: Fn(HashMap<String, String>) -> G::GraphAttributes,
        Y: Fn(HashMap<String, String>) -> G::VertexAttributes,
        Z: Fn(HashMap<String, String>) -> G::EdgeAttributes;

    /// Reads file given its path.
    ///
    /// # Errors
    ///
    /// An I/O operation failed.
    ///
    fn read(path: &Path) -> Result<Self, std::io::Error>;

    /// Writes to file given its path.
    ///
    /// # Errors
    ///
    /// An I/O operation failed.
    ///
    fn write(self, path: &Path) -> Result<(), std::io::Error>;
}
