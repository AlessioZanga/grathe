use std::fmt::Debug;
use std::path::Path;

pub trait FromDOT: Sized {
    type Error: Debug;

    fn from_dot(value: &String) -> Result<Self, Self::Error>;

    /// Read DOT file.
    ///
    /// Read DOT file into graph.
    ///
    fn read_dot(path: &Path) -> Result<Self, Self::Error> {
        // Read DOT file
        let string = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file \"{:?}\"", &path));
        // Parse DOT string.
        Self::from_dot(&string)
    }
}

pub trait IntoDOT: Sized {
    type Error: Debug;

    fn into_dot(&self) -> String;

    /// Write DOT file.
    ///
    /// Write graph to DOT file.
    ///
    fn write_dot(&self, path: &Path) -> Result<(), Self::Error> {
        // Export graph into DOT string.
        let dot = self.into_dot();
        // Write string to file.
        std::fs::write(path, dot).unwrap_or_else(|_| panic!("Failed to write file \"{:?}\"", &path));

        Ok(())
    }
}
