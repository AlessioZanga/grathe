use super::attributes::AttributesMap;
use super::storages::AdjacencyListStorage;
use crate::traits::{Directed, Operators, Storage, Undirected, WithAttributes};
use crate::types::Error;
use crate::types::{VertexIterator, Vertex};

/// Undirected graph based on adjacency list storage.
#[derive(Default, Debug)]
pub struct UndirectedAdjacencyListGraph<T, U = AttributesMap<T, (), (), ()>>
where
    T: Vertex,
    U: WithAttributes<T>,
{
    data: AdjacencyListStorage<T>,
    attributes: U,
}

// Storage delegation and graph trait implementation.
crate::traits::direction::impl_undirected!(UndirectedAdjacencyListGraph, AdjacencyListStorage);

impl<T, U> Undirected for UndirectedAdjacencyListGraph<T, U>
where
    T: Vertex,
    U: WithAttributes<T>,
{
    fn neighbors_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        self.adjacents_iter(x)
    }

    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        self.add_edge(x, y)
    }
}

/// Directed graph based on adjacency list storage.
#[derive(Default, Debug)]
pub struct DirectedAdjacencyListGraph<T, U = AttributesMap<T, (), (), ()>>
where
    T: Vertex,
    U: WithAttributes<T>,
{
    data: AdjacencyListStorage<T>,
    attributes: U,
}

// Storage delegation and graph trait implementation.
crate::traits::direction::impl_directed!(DirectedAdjacencyListGraph, AdjacencyListStorage);

impl<T, U> Directed for DirectedAdjacencyListGraph<T, U>
where
    T: Vertex,
    U: WithAttributes<T>,
{
    fn parents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        assert!(self.has_vertex(x));
        Box::new(self.data.storage().iter().filter_map(|(y, z)| match z.contains(x) {
            false => None,
            true => Some(y),
        }))
    }

    fn children_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        Box::new(self.data.storage()[x].iter())
    }

    fn add_directed_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        self.add_edge(x, y)
    }
}
