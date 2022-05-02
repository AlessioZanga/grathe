use super::Mixed;
use crate::{
    traits::{Directed, Undirected},
    types::{directions, Mark},
    E, V,
};

pub trait PartiallyMixed: Mixed {
    /// Constructs from an undirected graph.
    fn from_undirected<G>(g: G) -> Self
    where
        G: Undirected<Vertex = Self::Vertex>,
    {
        Self::new_with_mark([], E!(g).map(|(x, y)| (x.clone(), y.clone(), Mark::CircCirc)))
    }

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
