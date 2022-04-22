use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    hash::Hash,
};

use ndarray::prelude::*;
use sprs::TriMat;
use thiserror::Error;

/// The base vertex trait.
pub trait Vertex: Clone + Debug + Default + Eq + Hash + Ord {}

// Blanket implementation of vertex trait.
impl<V> Vertex for V where V: Clone + Debug + Default + Eq + Hash + Ord {}

/// Vertex iterator trait.
#[rustfmt::skip]
pub trait VertexIterator<'a, V: 'a>: Iterator<Item = &'a V> {}

// Blanket implementation of vertex iterator trait.
#[rustfmt::skip]
impl<'a, I, V> VertexIterator<'a, V> for I where I: Iterator<Item = &'a V>, V: 'a {}

/// Edge iterator trait.
#[rustfmt::skip]
pub trait EdgeIterator<'a, V: 'a>: ExactSizeIterator + Iterator<Item = (&'a V, &'a V)> {}

// Blanket implementation of edge iterator trait.
#[rustfmt::skip]
impl<'a, I, V> EdgeIterator<'a, V> for I where I: ExactSizeIterator + Iterator<Item = (&'a V, &'a V)>, V: 'a {}

/// Iterator with exact size.
#[derive(Debug)]
pub struct ExactSizeIter<I>
where
    I: Iterator,
{
    iter: I,
    size: usize,
}

// Implement base constructor for ExactSizeIter.
impl<I> ExactSizeIter<I>
where
    I: Iterator,
{
    /// Constructor with given iterator and initial exact size.
    pub fn new(iter: I, size: usize) -> Self {
        Self { iter, size }
    }
}

// Implement Iterator for ExactSizeIter.
impl<I> Iterator for ExactSizeIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    // Forward call to inner iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.size -= 1;
        }
        self.iter.next()
    }

    // Forward call to predefined size.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

// Implement ExactSizeIter for ExactSizeIterator.
impl<I> ExactSizeIterator for ExactSizeIter<I> where I: Iterator {}

/// Edge list type.
pub type EdgeList<V> = BTreeSet<(V, V)>;

/// Adjacency list type.
pub type AdjacencyList<V> = BTreeMap<V, BTreeSet<V>>;

/// Dense adjacency matrix type.
pub type DenseAdjacencyMatrix = Array2<bool>;

/// Sparse adjacency matrix type.
pub type SparseAdjacencyMatrix = TriMat<bool>;

/// Error enumerator.
#[derive(Debug, Error, PartialEq)]
pub enum Error<V> {
    /// Graph attributes not defined error type.
    #[error("graph has no attribute defined")]
    GraphAttributesNotDefined(),
    /// Vertex attribute not defined error type.
    #[error("vertex `{0:?}` has no attribute defined")]
    VertexAttributesNotDefined(V),
    /// Edge attribute not defined error type.
    #[error("edge `({0:?}, {1:?})` has no attribute defined")]
    EdgeAttributesNotDefined(V, V),
    /// Parsing error type.
    #[error("failed to parse graph")]
    ParseFailed(String),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Marker {
    None = 0,
    Tail = 1,
    Head = 2,
    Cric = 3,
}

impl Default for Marker {
    fn default() -> Self {
        Marker::None
    }
}

pub type DenseMarkerMatrix = Array2<Marker>;

pub type SparseMarkerMatrix = TriMat<Marker>;

/// Directions pseudo-enumerator for generics algorithms.
pub mod directions {
    /// Undirected pseudo-enumerator for generics algorithms.
    pub struct Undirected {}
    /// Directed pseudo-enumerator for generics algorithms.
    pub struct Directed {}
    pub struct PartiallyDirected {}
    pub struct Mixed {}
    pub struct PartiallyMixed {}
}
