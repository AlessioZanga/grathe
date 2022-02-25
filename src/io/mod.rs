mod dot;
pub use dot::*;

use std::path::Path;

pub trait IO: TryFrom<String> + TryInto<String> {
    fn read(path: &Path) -> Result<Self, <Self as TryFrom<String>>::Error>;
    fn write(self);
}
