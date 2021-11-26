use crate::errors::*;
use crate::graphs::GraphTrait;
use crate::types::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Graph structure based on adjacency list storage.
#[derive(Debug)]
pub struct AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    data: AdjacencyList<T>,
    v_labels: LabelMap<T>,
    e_labels: LabelMap<(T, T)>,
}

impl<T> PartialEq for AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    /// Equality operator.
    ///
    /// Let $G$ and $H$ be two graphs, then $G$ is equal to $H$ if and only if
    /// they have the same vertex set $V$ and the same edge set $E$:
    /// $$G = H \iff V(G) = V(H) \wedge E(G) = E(H)$$
    ///
    /// # Complexity
    ///
    /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
    ///
    /// # Examples
    ///
    /// ```
    ///     use grathe::graphs::{GraphTrait, AdjacencyListGraph};
    ///
    ///     let g = AdjacencyListGraph::<u32>::new();
    ///     let h = AdjacencyListGraph::<u32>::new();
    ///
    ///     assert_eq!(g, h);
    /// ```
    ///
    fn eq(&self, other: &Self) -> bool {
        // Compare maps.
        self.data == other.data
    }
}

impl<T> Eq for AdjacencyListGraph<T> where T: VertexTrait {}

impl<T> PartialOrd for AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    /// Equality operator.
    ///
    /// Let $G$ and $H$ be two graphs, then $G$ is partially comparable to $H$
    /// if and only if they have the partially comparable vertex set $V$ and
    /// partially comparable edge set $E$:
    /// $$G \leq H \iff V(G) \leq V(H) \wedge E(G) \leq E(H)$$
    ///
    /// # Complexity
    ///
    /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
    ///
    /// # Examples
    ///
    /// ```
    ///     use all_asserts::*;
    ///     use grathe::graphs::{GraphTrait, AdjacencyListGraph};
    ///
    ///     let g = AdjacencyListGraph::<u32>::new();
    ///     let h = AdjacencyListGraph::<u32>::new();
    ///
    ///     assert_le!(g, h);
    /// ```
    ///
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare maps.
        self.data.partial_cmp(&other.data)
    }
}

impl<T> Default for AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> GraphTrait for AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    type Vertex = T;
    type Storage = AdjacencyList<T>;

    fn new() -> Self {
        AdjacencyListGraph {
            data: Self::Storage::new(),
            v_labels: LabelMap::new(),
            e_labels: LabelMap::new(),
        }
    }

    fn vertices_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Vertex> + 'a> {
        Box::new(self.data.iter().map(|x| x.0).copied())
    }

    fn edges_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Self::Vertex, Self::Vertex)> + 'a> {
        Box::new(
            self.data
                .iter()
                .flat_map(|(x, ys)| std::iter::repeat(x).copied().zip(ys.iter().copied())),
        )
    }

    fn adjacents_iter<'a>(
        &'a self,
        x: &Self::Vertex,
    ) -> Box<dyn Iterator<Item = Self::Vertex> + 'a> {
        Box::new(self.data.get(x).unwrap().iter().copied())
    }

    fn as_data(&self) -> &Self::Storage {
        &self.data
    }

    fn as_vertices_labels(&self) -> &LabelMap<Self::Vertex> {
        &self.v_labels
    }

    fn as_mut_vertices_labels(&mut self) -> &mut LabelMap<Self::Vertex> {
        &mut self.v_labels
    }

    fn as_edges_labels(&self) -> &LabelMap<(Self::Vertex, Self::Vertex)> {
        &self.e_labels
    }

    fn as_mut_edges_labels(&mut self) -> &mut LabelMap<(Self::Vertex, Self::Vertex)> {
        &mut self.e_labels
    }

    fn order(&self) -> usize {
        // Get map size.
        self.data.len()
    }

    fn size(&self) -> usize {
        self.data
            .iter() // Iterate over the adjacency lists.
            .map(|(_, adj)| adj.len())
            .sum::<usize>() // Accumulate their sizes.
    }

    fn has_vertex(&self, x: &Self::Vertex) -> bool {
        // Check if map contains key.
        self.data.contains_key(x)
    }

    fn add_vertex(&mut self) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // Increase last identifier or get default.
        let x = self
            .data
            .iter()
            .rev()
            .next()
            .map(|x| *x.0 + Self::Vertex::one())
            .unwrap_or(Self::Vertex::zero());
        // Reserve new identifier
        self.reserve_vertex(&x)
    }

    fn reserve_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // TODO: Update using insert once stable.
        if self.has_vertex(x) {
            return Err(Error::VertexAlreadyDefined(*x));
        }
        self.data.insert(*x, BTreeSet::new());
        Ok(*x)
    }

    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>> {
        // Remove vertex from map.
        match self.data.remove(x) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(*x)),
            // Otherwise return successful.
            Some(_) => Ok(*x),
        }
    }

    fn has_edge(&self, e: &(Self::Vertex, Self::Vertex)) -> Result<bool, Error<Self::Vertex>> {
        // Get vertex adjacency list.
        match self.data.get(&e.0) {
            // If no vertex found return error.
            None => Err(Error::VertexNotDefined(e.0)),
            // Otherwise check second vertex.
            Some(adj) => match self.data.contains_key(&e.1) {
                // If no vertex found return error.
                false => Err(Error::VertexNotDefined(e.1)),
                // Otherwise check if it is in the adjacency list.
                true => Ok(adj.contains(&e.1)),
            },
        }
    }

    fn add_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        // Check if second vertex exists. NOTE: Check second vertex before first
        // in order to avoid contemporaneous immutable and mutable refs to data.
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(e.1)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(e.0)),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.insert(e.1) {
                    // If edge already defined return error.
                    false => Err(Error::EdgeAlreadyDefined(*e)),
                    // Otherwise return successful.
                    true => Ok(*e),
                },
            },
        }
    }

    fn del_edge(
        &mut self,
        e: &(Self::Vertex, Self::Vertex),
    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>> {
        // Check if second vertex exists.
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(Error::VertexNotDefined(e.1)),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(Error::VertexNotDefined(e.0)),
                // Otherwise try to remove vertex from adjacency list.
                Some(adj) => match adj.remove(&e.1) {
                    // If no edge defined return error.
                    false => Err(Error::EdgeNotDefined(*e)),
                    // Otherwise return successful.
                    true => Ok(*e),
                },
            },
        }
    }
}
