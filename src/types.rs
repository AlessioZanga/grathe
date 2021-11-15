use na::{Dynamic, OMatrix};
use nasparse::CsrMatrix;
use num_traits::{FromPrimitive, PrimInt};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;

/// The base vertex trait.
pub trait VertexTrait: PrimInt + FromPrimitive + Debug + Hash {}

// Blanket implementation of the vertex trait.
impl<T> VertexTrait for T where T: PrimInt + FromPrimitive + Debug + Hash {}

// TODO: Where clauses are not enforced in type aliases,
// once done we should add "where T: VertexTrait" down here.

/// Edge list type.
pub type EdgeList<T> = BTreeSet<(T, T)>;

/// Adjacency list type.
pub type AdjacencyList<T> = BTreeMap<T, BTreeSet<T>>;

/// Dense adjacency matrix type.
pub type DenseAdjacencyMatrix = OMatrix<i8, Dynamic, Dynamic>;

/// Sparse adjacency matrix type.
pub type SparseAdjacencyMatrix = CsrMatrix<i8>;
