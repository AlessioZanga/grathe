mod dot;
pub use dot::*;

use crate::traits::Storage;
use std::path::Path;

/// The parser IO trait.
pub trait IO: TryFrom<String> + TryInto<String> {
    /// Map parsed data into graph.
    ///
    fn map<G, F>(self, f: F) -> Vec<G>
    where
        G: Storage,
        F: FnMut(String) -> G::Vertex;

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
