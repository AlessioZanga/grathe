mod dot;
pub use dot::*;

use std::path::Path;

pub trait IO: TryFrom<String> + TryInto<String> {
    fn read(path: &Path) -> Result<Self, std::io::Error>;
    fn write(self, path: &Path) -> Result<(), std::io::Error>;
}
