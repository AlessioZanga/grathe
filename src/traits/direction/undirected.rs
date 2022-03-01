use crate::traits::{Capacity, Connectivity, Convert, Extend, Operators, Storage};
use crate::types::Error;
use crate::types::VertexIterator;

/// Undirected graph trait.
pub trait Undirected: Capacity + Connectivity + Convert + Extend + Operators + Storage {
    /// Neighbor iterator.
    ///
    /// Iterates over the vertex set $Ne(G, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    fn neighbors_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;

    /// Adds undirected edge to the graph.
    ///
    /// Add new undirected edge identifier into the graph.
    ///
    /// # Errors
    ///
    /// At least one of the vertex identifiers does not exist in the graph,
    /// or the undirected edge identifier already exists in the graph.
    ///
    fn add_undirected_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;
}

/// Neighborhood iterator.
///
/// Return the vertex iterator representing $Ne(G, X)$.
///
#[macro_export]
macro_rules! Ne {
    ($g:expr, $x:expr) => {
        $g.neighbors_iter($x)
    };
}

macro_rules! impl_undirected {
    ($graph:ident, $storage:ident) => {
        impl<T, U> PartialEq for $graph<T, U>
        where
            T: $crate::types::VertexTrait,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Equality operator.
            ///
            /// Let $G$ and $H$ be two graphs, then $G$ is equal to $H$ if and only if
            /// they have the same vertex set $V$ and the same edge set $E$:
            ///
            /// $$G = H \iff V(G) = V(H) \wedge E(G) = E(H)$$
            ///
            /// # Complexity
            ///
            /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
            ///
            /// # Examples
            ///
            /// ```
            /// use grathe::prelude::*;
            ///
            /// let g = Graph::new();
            /// let h = Graph::new();
            ///
            /// assert_eq!(g, h);
            /// ```
            ///
            fn eq(&self, other: &Self) -> bool {
                self.data.eq(&other.data)
            }
        }

        impl<T, U> Eq for $graph<T, U>
        where
            T: $crate::types::VertexTrait,
            U: $crate::traits::WithAttributes<T>,
        {}

        impl<T, U> PartialOrd for $graph<T, U>
        where
            T: $crate::types::VertexTrait,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Comparable operator.
            ///
            /// Let $G$ and $H$ be two graphs, then $G$ is partially comparable to $H$
            /// if and only if they have the partially comparable vertex set $V$ and
            /// partially comparable edge set $E$:
            ///
            /// $$G \leq H \iff V(G) \leq V(H) \wedge E(G) \leq E(H)$$
            ///
            /// # Complexity
            ///
            /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
            ///
            /// # Examples
            ///
            /// ```
            /// use all_asserts::*;
            /// use grathe::prelude::*;
            ///
            /// let g = Graph::new();
            /// let h = Graph::new();
            ///
            /// assert_le!(g, h);
            /// ```
            ///
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        $crate::traits::impl_capacity!($graph);

        impl<T, U> $crate::traits::Connectivity for $graph<T, U>
        where
            T: $crate::types::VertexTrait,
            U: $crate::traits::WithAttributes<T>,
        {
            fn has_path(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
                // Import `tuple_windows`.
                use itertools::Itertools;
                // Check edge case with a single self-edge.
                if self.has_edge(x, y).unwrap() {
                    return true;
                }
                // Check if search object reaches a vertex that is adjacent to the source.
                $crate::algorithms::BFS::from((self, x))
                    .tuple_windows()
                    .any(|(z, x)| z != y && self.has_edge(x, y).unwrap())
            }

            fn is_connected(&self) -> bool {
                // Check if search object reaches any vertex in the graph.
                $crate::algorithms::BFS::from(self).count() == self.order()
            }

            fn is_acyclic(&self) -> bool {
                // TODO: Find DFS edge such that Back(_, _).
                todo!()
            }
        }

        $crate::traits::impl_extend!($graph);
        $crate::traits::impl_from!($graph);
        $crate::traits::impl_operators!($graph);
        $crate::traits::impl_with_attributes!($graph);

        impl<T, U> $crate::traits::Storage for $graph<T, U>
        where
            T: $crate::types::VertexTrait,
            U: $crate::traits::WithAttributes<T>,
        {
            type Vertex = T;

            type Storage = $storage<T>;

            fn new() -> Self {
                Default::default()
            }

            fn storage(&self) -> &Self::Storage {
                &self.data
            }

            delegate::delegate! {
                to self.data {
                    fn clear(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn $crate::types::EdgeIterator<'a, Self::Vertex> + 'a>;
                    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn order(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, $crate::types::Error<Self::Vertex>>;
                }
            }

            fn size(&self) -> usize {
                // De-symmetrize edge set for correct size computation
                self.edges_iter().filter(|(x, y)| x <= y).count()
            }

            fn add_vertex<V>(&mut self, x: V) -> Result<Self::Vertex, $crate::types::Error<Self::Vertex>>
            where
                V: Into<Self::Vertex>
            {
                // Add vertex into the graph.
                let x = self.data.add_vertex(x)?;
                // Return successfully.
                Ok(x)
            }

            fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), $crate::types::Error<Self::Vertex>> {
                // Delete vertex from the graph.
                self.data.del_vertex(x)?;
                // Delete associated attribute map.
                self.attributes.unset_vertex_attrs(x).ok();
                // Return successfully.
                Ok(())
            }

            fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::types::Error<Self::Vertex>> {
                // Add edge (y, x)
                self.data.add_edge(y, x)?;
                // Add edge (x, y)
                if x != y {
                    self.data.add_edge(x, y)?;
                }
                // Return successfully.
                Ok(())
            }

            fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::types::Error<Self::Vertex>> {
                // Del edge (y, x)
                self.data.del_edge(y, x)?;
                // Del edge (x, y)
                if x != y {
                    self.data.del_edge(x, y)?;
                }
                // Delete associated attribute map.
                self.attributes.unset_edge_attrs(x, y).ok();
                // Return successfully.
                Ok(())
            }
        }

        $crate::algorithms::impl_algorithms_undirected!($graph);
    };
}

pub(crate) use impl_undirected;
