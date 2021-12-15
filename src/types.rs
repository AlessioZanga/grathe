use bimap::BiHashMap;
use na::{Dynamic, OMatrix};
use nasparse::CsrMatrix;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

/// The base vertex trait.
pub trait VertexTrait: Eq + Ord + Clone + Default + Debug + Hash + FromStr {}

// Blanket implementation of vertex trait.
impl<T> VertexTrait for T where T: Eq + Ord + Clone + Default + Debug + Hash + FromStr {}

// TODO: Where clauses are not enforced in type aliases,
// once done we should add "where T: VertexTrait" down here.

/// Vertex iterator trait (a.k.a Iterator<Item = Vertex> + Debug)
pub trait VertexIterator<T>: Iterator<Item = T> + Debug {}

// Blanket implementation of vertex iterator trait.
impl<T, U> VertexIterator<U> for T where T: Iterator<Item = U> + Debug {}

/// Edge iterator trait (a.k.a Iterator<Item = Edge> + Debug)
pub trait EdgeIterator<T>: Iterator<Item = T> + Debug {}

// Blanket implementation of edge iterator trait.
impl<T, U> EdgeIterator<U> for T where T: Iterator<Item = U> + Debug {}

/// Iterator over edges with exact size_hint
#[derive(Debug)]
pub struct SizedIter<I>
where
    I: Iterator,
{
    iter: I,
    size: usize,
}

// Implement base constructor.
impl<I> SizedIter<I>
where
    I: Iterator,
{
    /// Constructor with given iterator and initial exact size.
    pub fn new(iter: I, size: usize) -> Self {
        Self { iter, size }
    }
}

// Implement Iterator for SizedIter.
impl<I> Iterator for SizedIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    // Forward call to inner iterator.
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.size -= 1;
        }
        self.iter.next()
    }

    // Forward call to predefined size.
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

/// Label bidirectional map type.
pub type LabelMap<T> = BiHashMap<T, String>;

/// Edge list type.
pub type EdgeList<T> = BTreeSet<(T, T)>;

/// Adjacency list type.
pub type AdjacencyList<T> = BTreeMap<T, BTreeSet<T>>;

/// Dense adjacency matrix type.
pub type DenseAdjacencyMatrix = OMatrix<i8, Dynamic, Dynamic>;

/// Sparse adjacency matrix type.
pub type SparseAdjacencyMatrix = CsrMatrix<i8>;
