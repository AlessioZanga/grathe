use crate::directions::UndirectedTrait;
use crate::impl_ungraph_trait;
use crate::storages::AdjacencyList;
use crate::types::*;

/// Undirected graph based on adjacency list storage.
#[derive(Default, Debug)]
pub struct UndirectedAdjacencyListGraph<T>(AdjacencyList<T>)
where
    T: VertexTrait;

// Storage delegation and graph trait implementation.
impl_ungraph_trait!(UndirectedAdjacencyListGraph, AdjacencyList);

impl<T> UndirectedAdjacencyListGraph<T> where T: VertexTrait {}

impl<T> UndirectedTrait for UndirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    #[inline(always)]
    fn neighbors_iter<'a, U>(
        &'a self,
        x: &'a U,
    ) -> Result<Box<dyn VertexIterator<&'a Self::Vertex> + 'a>, Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        self.adjacents_iter(x)
    }

    #[inline(always)]
    fn add_undirected_edge<'a, U>(
        &mut self,
        x: (&'a U, &'a U),
    ) -> Result<(&'a U, &'a U), Error<Self::Vertex>>
    where
        U: Eq + Clone + Into<Self::Vertex>,
    {
        self.add_edge(x)
    }
}
