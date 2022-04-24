use super::{Directed, Undirected};
use crate::types::Marker;

pub trait PartiallyDirected: Undirected + Directed {
    fn new_with_marker<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex, Marker)>;
    fn has_marker(&self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;
    fn get_marker(&self, x: &Self::Vertex, y: &Self::Vertex) -> Option<Marker>;
    fn set_marker(&mut self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;
    fn unset_marker(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Option<Marker>;
}
