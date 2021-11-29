use crate::directions::UndirectedTrait;
use crate::impl_ungraph_trait;
use crate::storages::AdjacencyListStorage;
use crate::types::*;

/// Undirected graph based on adjacency list storage.
#[derive(Debug)]
pub struct UndirectedAdjacencyListGraph<T>(AdjacencyListStorage<T>)
where
    T: VertexTrait;

// Storage delegation and graph trait implementation.
impl_ungraph_trait!(UndirectedAdjacencyListGraph);

impl<T> UndirectedAdjacencyListGraph<T> where T: VertexTrait {}

impl<T> UndirectedTrait for UndirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    #[inline(always)]
    fn neighbors_iter<'a>(&'a self, x: &T) -> Result<Box<dyn Iterator<Item = T> + 'a>, Error<T>> {
        self.adjacents_iter(&x)
    }

    #[inline(always)]
    fn add_undirected_edge(&mut self, e: &(T, T)) -> Result<(T, T), Error<T>> {
        self.add_edge(&e)
    }
}
