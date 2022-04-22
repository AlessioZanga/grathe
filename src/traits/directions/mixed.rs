use super::PartiallyDirected;
use crate::types::VertexIterator;

pub trait Mixed: PartiallyDirected {
    fn spouses_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;
    fn add_bidirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;
}
