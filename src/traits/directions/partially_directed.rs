use super::{Directed, Undirected};
use crate::{
    prelude::DenseMarkMatrix,
    types::{directions, Mark as M},
    E, V,
};

pub trait PartiallyDirected: Undirected + Directed {
    /// Constructs from an undirected graph.
    fn from_undirected<G>(g: G) -> Self
    where
        G: Undirected<Vertex = Self::Vertex>,
    {
        Self::new_with_mark([], E!(g).map(|(x, y)| (*x, *y, M::TailTail)))
    }

    /// Constructs from a directed graph.
    fn from_directed<G>(g: G) -> Self
    where
        G: Directed<Vertex = Self::Vertex>,
    {
        Self::new_with_mark([], E!(g).map(|(x, y)| (*x, *y, M::TailHead)))
    }

    /// Constructs from another partially-directed graph.
    fn from_partially_directed<G>(other: G) -> Self
    where
        G: PartiallyDirected<Vertex = Self::Vertex, Direction = directions::PartiallyDirected>,
    {
        Self::new_with_mark(
            V!(other).cloned(),
            other.edges_with_mark_iter().map(|(x, y, m)| (*x, *y, *m)),
        )
    }

    fn from_dense_mark_matrix(data: DenseMarkMatrix, variables: Vec<Self::Vertex>) -> Self;

    fn new_with_mark<I, J>(v_iter: I, e_iter: J) -> Self
    where
        I: IntoIterator<Item = Self::Vertex>,
        J: IntoIterator<Item = (Self::Vertex, Self::Vertex, M)>;

    fn edges_with_mark_iter<'a>(&'a self)
        -> Box<dyn Iterator<Item = (&'a Self::Vertex, &'a Self::Vertex, &'a M)> + 'a>;

    fn has_mark(&self, x: &Self::Vertex, y: &Self::Vertex, m: M) -> bool;

    fn get_mark(&self, x: &Self::Vertex, y: &Self::Vertex) -> Option<M>;

    fn set_mark(&mut self, x: &Self::Vertex, y: &Self::Vertex, m: M) -> bool;
}
