use bimap::BiHashMap;
use na::{Dynamic, OMatrix};
use nasparse::CsrMatrix;
use num_traits::{FromPrimitive, PrimInt};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

/// The base vertex trait.
pub trait VertexTrait: PrimInt + FromPrimitive + FromStr + Debug + Hash {}

// Blanket implementation of the vertex trait.
impl<T> VertexTrait for T where T: PrimInt + FromPrimitive + FromStr + Debug + Hash {}

// TODO: Where clauses are not enforced in type aliases,
// once done we should add "where T: VertexTrait" down here.

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
