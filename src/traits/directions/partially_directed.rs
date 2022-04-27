use super::{Directed, Undirected};
use crate::{
    prelude::DenseMarkerMatrix,
    types::{directions, Marker},
    E, V,
};

pub trait PartiallyDirected: Undirected + Directed {
    /// Constructs from an undirected graph.
    fn from_undirected<G>(g: G) -> Self
    where
        G: Undirected<Vertex = Self::Vertex>,
    {
        Self::new_with_marker([], E!(g).map(|(x, y)| (x.clone(), y.clone(), Marker::TailTail)))
    }

    /// Constructs from a directed graph.
    fn from_directed<G>(g: G) -> Self
    where
        G: Directed<Vertex = Self::Vertex>,
    {
        Self::new_with_marker([], E!(g).map(|(x, y)| (x.clone(), y.clone(), Marker::TailHead)))
    }

    /// Constructs from another partially-directed graph.
    fn from_partially_directed<G>(other: G) -> Self
    where
        G: PartiallyDirected<Vertex = Self::Vertex, Direction = directions::PartiallyDirected>,
    {
        Self::new_with_marker(
            V!(other).cloned(),
            other
                .edges_with_marker_iter()
                .map(|(x, y, m)| (x.clone(), y.clone(), m.clone())),
        )
    }

    fn from_dense_marker_matrix(data: DenseMarkerMatrix, variables: Vec<Self::Vertex>) -> Self;

    fn new_with_marker<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex, Marker)>;

    fn edges_with_marker_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (&'a Self::Vertex, &'a Self::Vertex, &'a Marker)> + 'a>;

    fn has_marker(&self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;

    fn get_marker(&self, x: &Self::Vertex, y: &Self::Vertex) -> Option<Marker>;

    fn set_marker(&mut self, x: &Self::Vertex, y: &Self::Vertex, m: Marker) -> bool;
}
