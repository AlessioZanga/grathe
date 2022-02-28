mod dot;
pub use dot::*;

use std::path::Path;

/// The parser IO trait.
pub trait IO: TryFrom<String> + TryInto<String> {

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
