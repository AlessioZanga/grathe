use crate::errors::Error;
use crate::storages::AdjacencyListStorage;
use crate::traits::{Directed, Operators, Storage, Undirected};
use crate::types::{VertexIterator, VertexTrait};
use std::collections::HashMap;

/// Undirected graph based on adjacency list storage.
#[derive(Default, Debug)]
pub struct UndirectedAdjacencyListGraph<T, X, Y, Z>
where
    T: VertexTrait,
    X: Default + std::fmt::Debug,
    Y: Default + std::fmt::Debug,
    Z: Default + std::fmt::Debug,
{
    data: AdjacencyListStorage<T>,
    gattrs: Option<X>,
    vattrs: HashMap<T, Y>,
    eattrs: HashMap<(T, T), Z>,
}

// Storage delegation and graph trait implementation.
crate::traits::direction::impl_undirected!(UndirectedAdjacencyListGraph, AdjacencyListStorage);

impl<T, X, Y, Z> Undirected for UndirectedAdjacencyListGraph<T, X, Y, Z>
where
    T: VertexTrait,
    X: Default + std::fmt::Debug,
    Y: Default + std::fmt::Debug,
    Z: Default + std::fmt::Debug,
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
pub struct DirectedAdjacencyListGraph<T, X, Y, Z>
where
    T: VertexTrait,
    X: Default + std::fmt::Debug,
    Y: Default + std::fmt::Debug,
    Z: Default + std::fmt::Debug,
{
    data: AdjacencyListStorage<T>,
    gattrs: Option<X>,
    vattrs: HashMap<T, Y>,
    eattrs: HashMap<(T, T), Z>,
}

// Storage delegation and graph trait implementation.
crate::traits::direction::impl_directed!(DirectedAdjacencyListGraph, AdjacencyListStorage);

impl<T, X, Y, Z> Directed for DirectedAdjacencyListGraph<T, X, Y, Z>
where
    T: VertexTrait,
    X: Default + std::fmt::Debug,
    Y: Default + std::fmt::Debug,
    Z: Default + std::fmt::Debug,
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
