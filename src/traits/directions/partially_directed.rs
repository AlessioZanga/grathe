use super::{Directed, Undirected};
use crate::types::Marker;

pub trait PartiallyDirected: Undirected + Directed {
    fn new_with_markers<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex, Marker)>;
    fn has_markers(&self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;
    fn get_markers(&self, x: &Self::Vertex, y: &Self::Vertex) -> Option<Marker>;
    fn set_markers(&mut self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;
    fn unset_markers(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Option<Marker>;
}
