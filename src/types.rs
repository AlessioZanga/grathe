use ndarray::Array2;
use sprs::TriMat;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

/// The base vertex trait.
pub trait VertexTrait: Eq + Ord + Clone + Default + Debug + Hash + FromStr {}

// Blanket implementation of vertex trait.
impl<T> VertexTrait for T where T: Eq + Ord + Clone + Default + Debug + Hash + FromStr {}

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

/// Dense adjacency matrix type.
pub type DenseAdjacencyMatrix = Array2<bool>;

/// Sparse adjacency matrix type.
pub type SparseAdjacencyMatrix = TriMat<bool>;
