use super::{Directed, Undirected};
use crate::types::Marker;

pub trait PartiallyDirected: Undirected + Directed {
    fn new_with_markers<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = ((Self::Vertex, Self::Vertex), (Marker, Marker))>;
    fn has_marker(&mut self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;
    fn get_marker(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Marker;
    fn set_marker(&mut self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;
}
