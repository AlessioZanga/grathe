use crate::types::{Result, EID, VID};

/// The base graph trait.
pub trait Graph {
    fn order(&self) -> usize;

    fn size(&self) -> usize;

    fn has_vertex(&self, v: &VID) -> bool;

    fn add_vertex(&mut self, v: &VID) -> Result<()>;

    fn del_vertex(&mut self, v: &VID) -> Result<()>;

    fn has_edge(&self, e: &EID) -> Result<bool>;

    fn add_edge(&mut self, e: &EID) -> Result<()>;

    fn del_edge(&mut self, e: &EID) -> Result<()>;
}
