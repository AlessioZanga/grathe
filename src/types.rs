use bimap::BiHashMap;
use na::{Dynamic, OMatrix};
use nasparse::CsrMatrix;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;

/// The base vertex trait.
pub trait VertexTrait: PartialEq + Eq + Ord + Copy + Default + Debug + Hash + TryFrom<usize> { }

// Blanket implementation of vertex trait.
impl<T> VertexTrait for T where T: PartialEq + Eq + Ord + Copy + Default + Debug + Hash + TryFrom<usize> { }

// TODO: Where clauses are not enforced in type aliases,
// once done we should add "where T: VertexTrait" down here.

/// Vertex iterator trait (a.k.a Iterator<Item = Vertex> + Debug)
pub trait VertexIterator<T>: Iterator<Item = T> + Debug where T: VertexTrait {}

// Blanket implementation of vertex iterator trait.
impl<T, U> VertexIterator<U> for T where T: Iterator<Item = U> + Debug, U: VertexTrait {}

/// Edge iterator trait (a.k.a Iterator<Item = Edge> + Debug)
pub trait EdgeIterator<T>: Iterator<Item = (T, T)> + Debug where T: VertexTrait {}

// Blanket implementation of edge iterator trait.
impl<T, U> EdgeIterator<U> for T where T: Iterator<Item = (U, U)> + Debug, U: VertexTrait {}

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
