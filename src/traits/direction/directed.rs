use crate::traits::{Capacity, Connectivity, Convert, Extend, Operators, Storage};
use crate::types::Error;
use crate::types::VertexIterator;
use std::collections::{BTreeSet, VecDeque};

/// Directed graph trait.
pub trait Directed: Capacity + Connectivity + Convert + Extend + Operators + Storage {
    /// Ancestors iterator.
    ///
    /// Iterates over the vertex set $An(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn ancestors_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        // Initialize ancestors.
        let mut ancestors = BTreeSet::new();
        // Initialize visiting queue.
        let mut queue = VecDeque::from([x]);
        // Loop until the visiting queue is empty.
        while let Some(x) = queue.pop_front() {
            // There is at least one vertex that has not been visited yet, therefore ...
            queue.extend(
                // ... insert any parent of this vertex ...
                self.parents_iter(x)
                    // ... that has not already been added to the ancestors set.
                    .filter(|&x| ancestors.insert(x)),
            )
        }
        // Return ancestors set iterator.
        Box::new(ancestors.into_iter())
    }

    /// Parents iterator.
    ///
    /// Iterates over the vertex set $Pa(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn parents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    /// Children iterator.
    ///
    /// Iterates over the vertex set $Ch(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn children_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    /// Descendants iterator.
    ///
    /// Iterates over the vertex set $De(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn descendants_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a> {
        // Initialize descendants.
        let mut descendants = BTreeSet::new();
        // Initialize visiting queue.
        let mut queue = VecDeque::from([x]);
        // Loop until the visiting queue is empty.
        while let Some(x) = queue.pop_front() {
            // There is at least one vertex that has not been visited yet, therefore ...
            queue.extend(
                // ... insert any child of this vertex ...
                self.children_iter(x)
                    // ... that has not already been added to the descendants set.
                    .filter(|&x| descendants.insert(x)),
            )
        }
        // Return descendants set iterator.
        Box::new(descendants.into_iter())
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

    /// In-degree of a given vertex.
    ///
    /// Computes the in-degree of a given vertex, i.e. $|Pa(G, X)|$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn in_degree_of(&self, x: &Self::Vertex) -> usize {
        return self.parents_iter(x).count();
    }

    /// Out-degree of a given vertex.
    ///
    /// Computes the out-degree of a given vertex, i.e. $|Ch(G, X)|$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn out_degree_of(&self, x: &Self::Vertex) -> usize {
        return self.children_iter(x).count();
    }
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

macro_rules! impl_directed {
    ($graph:ident, $storage:ident) => {
        impl<T, U> PartialEq for $graph<T, U>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn eq(&self, other: &Self) -> bool {
                self.data.eq(&other.data)
            }
        }

        impl<T, U> Eq for $graph<T, U>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {}

        impl<T, U> PartialOrd for $graph<T, U>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        $crate::traits::impl_capacity!($graph);

        impl<T, U> $crate::traits::Connectivity for $graph<T, U>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn has_path(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
                // Check if search object reaches a vertex that is adjacent to the source.
                $crate::algorithms::BFS::from((self, x)).any(|x| self.has_edge(x, y).unwrap())
            }

            fn is_connected(&self) -> bool {
                // Check if search object reaches any vertex in the graph.
                $crate::algorithms::BFS::from(self).count() == self.order()
            }

            fn is_acyclic(&self) -> bool {
                // If a topological order exists, then the graph is acyclic.
                $crate::algorithms::TopologicalSort::from(self).all(|x| x.is_ok())
            }
        }

        impl<T, U> $crate::traits::Convert for $graph<T, U>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn edge_list(&self) -> $crate::types::EdgeList<Self::Vertex> {
                let mut out = $crate::types::EdgeList::new();
                for (x, y) in self.edges_iter() {
                    out.insert((x.clone(), y.clone()));
                }

                out
            }

            fn adjacency_list(&self) -> $crate::types::AdjacencyList<Self::Vertex> {
                let mut out = $crate::types::AdjacencyList::new();
                for (x, y) in self.edges_iter() {
                    out.entry(x.clone()).or_default().insert(y.clone());
                }

                out
            }

            fn dense_adjacency_matrix(&self) -> ndarray::Array2<bool> {
                let n = self.order();
                let mut idx = std::collections::HashMap::with_capacity(n);
                let mut out = ndarray::Array2::from_elem((n, n), false);
                // Build vid-to-index mapping.
                idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
                // Fill the output matrix.
                for (x, y) in self.edges_iter() {
                    out[(idx[&x], idx[&y])] = true;
                }

                out
            }

            fn sparse_adjacency_matrix(&self) -> sprs::TriMat<bool> {
                let n = self.order();
                let mut idx = std::collections::HashMap::with_capacity(n);
                let mut out = sprs::TriMat::new((n, n));
                // Reserve capacity for sparse matrix.
                out.reserve(self.size());
                // Build vid-to-index mapping.
                idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
                // Fill the output matrix.
                for (x, y) in self.edges_iter() {
                    out.add_triplet(idx[&x], idx[&y], true);
                }

                out
            }

            fn dense_incidence_matrix(&self) -> ndarray::Array2<i8> {
                let (n, m) = (self.order(), self.size());
                let mut idx = std::collections::HashMap::with_capacity(n);
                let mut out = ndarray::Array2::from_elem((n, m), 0);
                // Build vid-to-index mapping.
                idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
                // Fill the output matrix.
                for (i, (x, y)) in self.edges_iter().enumerate() {
                    out[(idx[&x], i)] = 1;
                    out[(idx[&y], i)] = -1;
                }

                out
            }

            fn sparse_incidence_matrix(&self) -> sprs::TriMat<i8> {
                let (n, m) = (self.order(), self.size());
                let mut idx = std::collections::HashMap::with_capacity(n);
                let mut out = sprs::TriMat::new((n, m));
                // Reserve capacity for sparse matrix.
                out.reserve(2 * m);
                // Build vid-to-index mapping.
                idx.extend(self.vertices_iter().enumerate().map(|(i, x)| (x, i)));
                // Fill the output matrix.
                for (i, (x, y)) in self.edges_iter().enumerate() {
                    out.add_triplet(idx[&x], i, 1);
                    out.add_triplet(idx[&y], i, -1);
                }

                out
            }
        }

        $crate::traits::impl_extend!($graph);
        $crate::traits::impl_from!($graph);
        $crate::traits::impl_operators!($graph);
        $crate::traits::impl_with_attributes!($graph);

        impl<T, U> $crate::traits::Storage for $graph<T, U>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            type Vertex = T;

            const DIRECTION: usize = $crate::types::Direction::DIRECTED;

            type Storage = $storage<T, { $crate::types::Direction::DIRECTED }>;

            fn storage(&self) -> &Self::Storage {
                &self.data
            }

            fn new<I, J, V>(v_iter: I, e_iter: J) -> Self
            where
                I: IntoIterator<Item = V>,
                J: IntoIterator<Item = (V, V)>,
                V: Into<Self::Vertex>
            {
                Self {
                    data: Storage::new(v_iter, e_iter),
                    attributes: Default::default(),
                }
            }

            fn null() -> Self {
                Default::default()
            }

            fn empty<I, V>(iter: I) -> Self
            where
                I: IntoIterator<Item = V>,
                V: Into<Self::Vertex>,
            {
                Self {
                    data: Storage::empty(iter),
                    attributes: Default::default(),
                }
            }

            fn complete<I, V>(iter: I) -> Self
            where
                I: IntoIterator<Item = V>,
                V: Into<Self::Vertex>,
            {
                Self {
                    data: Storage::complete(iter),
                    attributes: Default::default(),
                }
            }

            delegate::delegate! {
                to self.data {
                    fn clear(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn $crate::types::EdgeIterator<'a, Self::Vertex> + 'a>;
                    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn order(&self) -> usize;
                    fn size(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn add_vertex<V>(&mut self, x: V) -> Result<Self::Vertex, $crate::types::Error<Self::Vertex>> where V: Into<Self::Vertex>;
                    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), $crate::types::Error<Self::Vertex>>;
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, $crate::types::Error<Self::Vertex>>;
                    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::types::Error<Self::Vertex>>;
                    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::types::Error<Self::Vertex>>;
                }
            }
        }

        $crate::algorithms::impl_algorithms_directed!($graph);
    };
}

pub(crate) use impl_directed;
