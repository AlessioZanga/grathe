use crate::impl_graph_trait;
use crate::directions::UndirectedTrait;
use crate::storages::{AdjacencyListStorage, StorageTrait};
use crate::types::*;

/// Undirected graph based on adjacency list storage.
#[derive(Debug)]
pub struct UndirectedAdjacencyListGraph<T>(AdjacencyListStorage<T>)
where
    T: VertexTrait;

// Storage delegation and graph trait implementation.
impl_graph_trait!(UndirectedAdjacencyListGraph);

impl<T> UndirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    /// Adds undirected edge to the graph.
    ///
    /// Add new undirected edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the undirected edge identifier already exists in the graph.
    ///
    #[inline(always)]
    pub fn add_edge(&mut self, e: &(T, T)) -> Result<(T, T), Error<T>> {
        // Add edge (y, x)
        self.0.add_edge(&(e.1, e.0))?;
        // Add edge (x, y)
        self.0.add_edge(&e)
    }

    /// Deletes undirected edge from the graph.
    ///
    /// Remove given undirected edge identifier from the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers do not exist in the graph,
    /// or the undirected edge identifier does not exists in the graph.
    ///
    #[inline(always)]
    pub fn del_edge(&mut self, e: &(T, T)) -> Result<(T, T), Error<T>> {
        // Del edge (y, x)
        self.0.del_edge(&(e.1, e.0))?;
        // Del edge (x, y)
        self.0.del_edge(&e)
    }
}

impl<T> UndirectedTrait for UndirectedAdjacencyListGraph<T>
where
    T: VertexTrait,
{
    #[inline(always)]
    fn neighbors_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Result<Box<dyn Iterator<Item = Self::Vertex> + 'a>, Error<Self::Vertex>> {
        self.adjacents_iter(&x)
    }

    #[inline(always)]
    fn add_undirected_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        self.add_edge(&e)
    }
}
