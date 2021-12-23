use crate::errors::Error;
use crate::graphs::GraphTrait;
use crate::types::*;
use std::collections::{BTreeSet, VecDeque};

/// Directed graph trait.
pub trait DirectedTrait: GraphTrait {
    /// Ancestors iterator.
    ///
    /// Iterates over the vertex set $An(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn ancestors_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>> {
        // Initialize ancestors.
        let mut ancestors = BTreeSet::new();
        // Initialize visiting queue.
        let mut queue = VecDeque::from([x]);
        // Loop until the visiting queue is empty.
        while let Some(x) = queue.pop_front() {
            // There is at least one vertex that has not been visited yet, therefore ...
            queue.extend(
                // ... insert any parent of this vertex ...
                self.parents_iter(x)?
                    // ... that has not already been added to the ancestors set.
                    .filter(|&x| ancestors.insert(x)),
            )
        }
        // Return ancestors set iterator.
        Ok(Box::new(ancestors.into_iter()))
    }

    /// Parents iterator.
    ///
    /// Iterates over the vertex set $Pa(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn parents_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>>;

    /// Children iterator.
    ///
    /// Iterates over the vertex set $Ch(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn children_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>>;

    /// Descendants iterator.
    ///
    /// Iterates over the vertex set $De(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn descendants_iter<'a>(
        &'a self,
        x: &'a Self::Vertex,
    ) -> Result<Box<dyn VertexIterator<'a, Self::Vertex> + 'a>, Error<Self::Vertex>> {
        // Initialize descendants.
        let mut descendants = BTreeSet::new();
        // Initialize visiting queue.
        let mut queue = VecDeque::from([x]);
        // Loop until the visiting queue is empty.
        while let Some(x) = queue.pop_front() {
            // There is at least one vertex that has not been visited yet, therefore ...
            queue.extend(
                // ... insert any child of this vertex ...
                self.children_iter(x)?
                    // ... that has not already been added to the descendants set.
                    .filter(|&x| descendants.insert(x)),
            )
        }
        // Return descendants set iterator.
        Ok(Box::new(descendants.into_iter()))
    }

    /// Adds directed edge to the graph.
    ///
    /// Add new directed edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers does not exist in the graph,
    /// or the directed edge identifier already exists in the graph.
    ///
    fn add_directed_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;
}

/// Ancestors iterator.
///
/// Return the vertex iterator representing $An(G, X)$.
///
#[macro_export]
macro_rules! An {
    ($g:expr, $x:expr) => {
        $g.ancestors_iter($x)
    };
}

/// Parents iterator.
///
/// Return the vertex iterator representing $Pa(G, X)$.
///
#[macro_export]
macro_rules! Pa {
    ($g:expr, $x:expr) => {
        $g.parents_iter($x)
    };
}

/// Children iterator.
///
/// Return the vertex iterator representing $Ch(G, X)$.
///
#[macro_export]
macro_rules! Ch {
    ($g:expr, $x:expr) => {
        $g.children_iter($x)
    };
}

/// Descendants iterator.
///
/// Return the vertex iterator representing $De(G, X)$.
///
#[macro_export]
macro_rules! De {
    ($g:expr, $x:expr) => {
        $g.descendants_iter($x)
    };
}