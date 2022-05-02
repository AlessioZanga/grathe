use super::Mixed;
use crate::{
    traits::{Directed, Undirected},
    types::{directions, Marker},
    E, V,
};

pub trait PartiallyMixed: Mixed {
    /// Constructs from an undirected graph.
    fn from_undirected<G>(g: G) -> Self
    where
        G: Undirected<Vertex = Self::Vertex>,
    {
        Self::new_with_marker([], E!(g).map(|(x, y)| (x.clone(), y.clone(), Marker::CircCirc)))
    }

    /// Constructs from another mixed graph.
    fn from_mixed<G>(other: G) -> Self
    where
        G: Mixed<Vertex = Self::Vertex, Direction = directions::Mixed>,
    {
        Self::new_with_marker(
            V!(other).cloned(),
            other
                .edges_with_marker_iter()
                .map(|(x, y, m)| (x.clone(), y.clone(), *m)),
        )
    }
}
