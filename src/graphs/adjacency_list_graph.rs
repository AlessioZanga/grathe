use crate::errors::VertexError;
use crate::graphs::{GraphTrait, VertexTrait};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};

/// Adjacency list type.
type AdjacencyList<T> = BTreeMap<T, BTreeSet<T>>;

/// Graph structure based on adjacency list storage.
pub struct AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    data: AdjacencyList<T>,
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

impl<T> Debug for AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
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

impl<T> From<usize> for AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    /// From order constructor.
    ///
    /// Construct a graph of a given order.
    ///
    /// # Complexity
    ///
    /// $O(|V|)$ - Linear in the order of the graph.
    ///
    /// # Examples
    ///
    /// ```
    ///     use grathe::graphs::{GraphTrait, AdjacencyListGraph};
    ///
    ///     let order: usize = 3;
    ///     let g = AdjacencyListGraph::<u32>::from(order);
    ///
    ///     assert_eq!(g.order(), order);
    /// ```
    ///
    fn from(other: usize) -> Self {
        // Initialize new graph.
        let mut graph = Self::new();
        // Iterate over range inserting adjacency lists.
        for i in 0..other {
            graph.data.insert(
                // TODO: Refactor when Step is stable.
                T::from_usize(i).unwrap(),
                BTreeSet::new(),
            );
        }
        graph
    }
}

impl<T> GraphTrait for AdjacencyListGraph<T>
where
    T: VertexTrait,
{
    type Vertex = T;
    type Edge = (T, T); // TODO: Remove once associated type defaults are stable.
    type Storage = AdjacencyList<T>;

    fn new() -> Self {
        AdjacencyListGraph {
            data: Self::Storage::new(),
        }
    }

    fn vertices_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Vertex> + 'a> {
        Box::new(self.data.iter().map(|(x, _)| *x))
    }

    fn edges_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Edge> + 'a> {
        Box::new(
            self.data
                .iter()
                .flat_map(|(x, ys)| std::iter::repeat(*x).zip(ys.iter().copied())),
        )
    }

    fn data(&self) -> &Self::Storage {
        &self.data
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

    fn try_add_vertex(&mut self, x: &Self::Vertex) -> Result<(), VertexError> {
        // TODO: Update using try_insert once stable.
        if self.has_vertex(x) {
            return Err(VertexError);
        }
        match self.data.insert(*x, BTreeSet::new()) {
            Some(_) => Err(VertexError),
            None => Ok(()),
        }
    }

    fn try_del_vertex(&mut self, x: &Self::Vertex) -> Result<(), VertexError> {
        // Remove vertex from map.
        match self.data.remove(x) {
            // If no vertex found return error.
            None => Err(VertexError),
            // Otherwise return successful.
            Some(_) => Ok(()),
        }
    }

    fn try_has_edge(&self, e: &Self::Edge) -> Result<bool, VertexError> {
        // Get vertex adjacency list.
        match self.data.get(&e.0) {
            // If no vertex found return error.
            None => Err(VertexError),
            // Otherwise check second vertex.
            Some(adj) => match self.data.contains_key(&e.1) {
                // If no vertex found return error.
                false => Err(VertexError),
                // Otherwise check if it is in the adjacency list.
                true => Ok(adj.contains(&e.1)),
            },
        }
    }

    fn try_add_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError> {
        // Check if second vertex exists. NOTE: Check second vertex before first
        // in order to avoid contemporaneous immutable and mutable refs to data.
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(VertexError),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(VertexError),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.insert(e.1) {
                    // If no vertex inserted return error.
                    // FIXME: Change to EdgeError.
                    false => Err(VertexError),
                    // Otherwise return successful.
                    true => Ok(()),
                },
            },
        }
    }

    fn try_del_edge(&mut self, e: &Self::Edge) -> Result<(), VertexError> {
        // Check if second vertex exists.
        match self.data.contains_key(&e.1) {
            // If no vertex found return error.
            false => Err(VertexError),
            // Otherwise get mutable vertex adjacency list.
            true => match self.data.get_mut(&e.0) {
                // If no vertex exists return error.
                None => Err(VertexError),
                // Otherwise try to insert vertex into adjacency list.
                Some(adj) => match adj.remove(&e.1) {
                    // If no vertex inserted return error.
                    // FIXME: Change to EdgeError.
                    false => Err(VertexError),
                    // Otherwise return successful.
                    true => Ok(()),
                },
            },
        }
    }
}
