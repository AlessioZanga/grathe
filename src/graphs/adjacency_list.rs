use crate::directions::UndirectedTrait;
use crate::impl_ungraph_trait;
use crate::storages::AdjacencyList;
use crate::types::*;

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

impl<T> UndirectedAdjacencyListGraph<T> where T: VertexTrait {}

impl<T> UndirectedTrait for UndirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    fn neighbors_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>> {
        self.adjacents_iter(x)
    }

    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
        self.add_edge(x, y)
    }
}
