use super::Mixed;
use crate::{
    traits::Undirected,
    types::{directions, Mark as M},
    E, V,
};

pub trait PartiallyMixed: Mixed {
    /// Constructs from an undirected graph.
    fn from_undirected<G>(g: G) -> Self
    where
        G: Undirected<Vertex = Self::Vertex>,
    {
        Self::new_with_mark([], E!(g).map(|(x, y)| (*x, *y, M::CircCirc)))
    }

    /// Constructs from another mixed graph.
    fn from_mixed<G>(other: G) -> Self
    where
        G: Mixed<Vertex = Self::Vertex, Direction = directions::Mixed>,
    {
        Self::new_with_mark(
            V!(other).cloned(),
            other.edges_with_mark_iter().map(|(x, y, m)| (*x, *y, *m)),
        )
    }
}
