use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;
use thiserror::Error;

/// The base vertex trait.
pub trait Vertex: Eq + Ord + Clone + Default + Debug + Hash {}

// Blanket implementation of vertex trait.
impl<T> Vertex for T where T: Eq + Ord + Clone + Default + Debug + Hash {}

/// Vertex iterator trait.
#[rustfmt::skip]
pub trait VertexIterator<'a, T: 'a>: Iterator<Item = &'a T> + Debug {}

// Blanket implementation of vertex iterator trait.
#[rustfmt::skip]
impl<'a, T, U> VertexIterator<'a, U> for T where T: Iterator<Item = &'a U> + Debug, U: 'a {}

/// Edge iterator trait.
#[rustfmt::skip]
pub trait EdgeIterator<'a, T: 'a>: Iterator<Item = (&'a T, &'a T)> + ExactSizeIterator + Debug {}

// Blanket implementation of edge iterator trait.
#[rustfmt::skip]
impl<'a, T, U> EdgeIterator<'a, U> for T where T: Iterator<Item = (&'a U, &'a U)> + ExactSizeIterator + Debug, U: 'a {}

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
pub type EdgeList<T> = BTreeSet<(T, T)>;

/// Adjacency list type.
pub type AdjacencyList<T> = BTreeMap<T, BTreeSet<T>>;

/// Error enumerator.
#[derive(Error, PartialEq, Debug)]
pub enum Error<T> {
    /// Vertex not defined error type.
    #[error("vertex identifier `{0:?}` not defined")]
    VertexNotDefined(T),
    /// Vertex already defined error type.
    #[error("vertex identifier `{0:?}` already defined")]
    VertexAlreadyDefined(T),
    /// Edge not defined error type.
    #[error("edge identifier `({0:?}, {1:?})` not defined")]
    EdgeNotDefined(T, T),
    /// Edge already defined error type.
    #[error("edge identifier `({0:?}, {1:?})` already defined")]
    EdgeAlreadyDefined(T, T),
    /// Graph attributes not defined error type.
    #[error("graph has no attribute defined")]
    GraphAttributesNotDefined(),
    /// Vertex attribute not defined error type.
    #[error("vertex `{0:?}` has no attribute defined")]
    VertexAttributesNotDefined(T),
    /// Edge attribute not defined error type.
    #[error("edge `({0:?}, {1:?})` has no attribute defined")]
    EdgeAttributesNotDefined(T, T),
    /// Parsing error type.
    #[error("failed to parse graph")]
    ParseFailed(String),
}
