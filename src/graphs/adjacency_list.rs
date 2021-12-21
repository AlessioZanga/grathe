use crate::directions::{DirectedTrait, DirectionalTrait, UndirectedTrait};
use crate::errors::Error;
use crate::graphs::GraphTrait;
use crate::storages::AdjacencyList;
use crate::storages::StorageTrait;
use crate::types::*;
use crate::{impl_digraph_trait, impl_graph_trait, impl_ungraph_trait};
use delegate::delegate;
use std::any::Any;
use std::cmp::Ordering;

/// Undirected graph based on adjacency list storage.
#[derive(Default, Debug)]
pub struct UndirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    data: AdjacencyList<T>,
    vattrs: Attributes<T>,
    eattrs: Attributes<(T, T)>,
}

// Storage delegation and graph trait implementation.
impl_ungraph_trait!(UndirectedAdjacencyListGraph, AdjacencyList);

impl<T> UndirectedTrait for UndirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    fn neighbors_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>> {
        self.adjacents_iter(x)
    }

    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        self.add_edge(x, y)
    }
}

/// Directed graph based on adjacency list storage.
#[derive(Default, Debug)]
pub struct DirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    data: AdjacencyList<T>,
    vattrs: Attributes<T>,
    eattrs: Attributes<(T, T)>,
}

// Storage delegation and graph trait implementation.
impl_digraph_trait!(DirectedAdjacencyListGraph, AdjacencyList);

impl<T> DirectedTrait for DirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    fn parents_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>> {
        match self.data.contains_key(x) {
            false => Err(Error::VertexNotDefined(x.clone())),
            true => Ok(Box::new(self.data.iter().filter_map(|(y, z)| match z.contains(x) {
                false => None,
                true => Some(y),
            }))),
        }
    }

    fn children_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>> {
        match self.data.get(x) {
            None => Err(Error::VertexNotDefined(x.clone())),
            Some(x) => Ok(Box::new(x.iter())),
        }
    }

    fn add_directed_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        self.add_edge(x, y)
    }
}
