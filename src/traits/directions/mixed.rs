use super::PartiallyDirected;
use crate::{
    types::{directions, VertexIterator},
    V,
};

pub trait Mixed: PartiallyDirected {
    fn spouses_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    fn add_bidirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Constructs from another mixed graph.
    fn from_mixed<G>(other: G) -> Self
    where
        G: Mixed<Vertex = Self::Vertex, Direction = directions::Mixed>,
    {
        Self::new_with_mark(
            V!(other).cloned(),
            other.edges_with_mark_iter().map(|(x, y, m)| (x.clone(), y.clone(), *m)),
        )
    }
}
