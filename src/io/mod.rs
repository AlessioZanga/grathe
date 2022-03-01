mod dot;
pub use dot::*;

use crate::traits::{Storage, WithAttributes};
use std::collections::HashMap;
use std::path::Path;

/// The parser IO trait.
pub trait IO: TryFrom<String> + TryInto<String> {
    /// Map parsed data into graph.
    ///
    fn map<G, F>(self, f: F) -> Vec<G>
    where
        G: Storage,
        F: FnMut(String) -> G::Vertex;

    /// Map parsed data into graph with attributes.
    ///
    fn map_with_attributes<G, F, X, Y, Z>(self, f: F, x: X, y: Y, z: Z) -> Vec<G>
    where
        G: Storage + WithAttributes<G::Vertex>,
        F: FnMut(String) -> G::Vertex,
        X: FnMut(HashMap<String, String>) -> G::GraphAttributes,
        Y: FnMut(HashMap<String, String>) -> G::VertexAttributes,
        Z: FnMut(HashMap<String, String>) -> G::EdgeAttributes;

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
