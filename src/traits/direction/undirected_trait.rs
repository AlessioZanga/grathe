use crate::errors::Error;
use crate::traits::{Base, Connectivity, Convert};
use crate::types::VertexIterator;

/// Undirected graph trait.
pub trait Undirected: Base + Connectivity + Convert {
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

macro_rules! impl_undirected_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> PartialEq for $graph<T>
        where
            T: $crate::types::VertexTrait,
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

        impl<T> Eq for $graph<T> where T: $crate::types::VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: $crate::types::VertexTrait,
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

        $crate::traits::impl_capacity_trait!($graph);

        impl<T> $crate::traits::Connectivity for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn has_path(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
                // Sanitize input.
                assert!(self.has_vertex(x) && self.has_vertex(y));
                // Initialize the to-be-visited queue with the source vertex.
                let mut queue = std::collections::VecDeque::from([x]);
                // Initialize the visited set.
                let mut visited = std::collections::HashSet::from([x]);
                // If there are still vertices to be visited.
                while let Some(z) = queue.pop_front() {
                    // Iterate over the reachable vertices of the popped vertex.
                    for w in self.neighbors_iter(z) {
                        // If the vertex was never seen before.
                        if !visited.contains(w) {
                            // Check if vertex is target.
                            if w == y {
                                // Return has undirected path.
                                return true;
                            }
                            // Set as visited.
                            visited.insert(w);
                            // Push it into the to-be-visited queue.
                            queue.push_back(w);
                        }
                    }
                }

                false
            }

            fn is_acyclic(&self) -> bool {
                todo!()
            }
        }

        impl<T> $crate::traits::convert::FromDOT for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn from_dot(value: &str) -> Result<Self, Error<Self::Vertex>> {
                Ok($crate::io::from_dot::<Self>(value).map_err(|e| Error::ParseFailed(format!("{}", e)))?.pop().unwrap())
            }
        }

        impl<T> $crate::traits::convert::IntoDOT for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn into_dot(&self) -> String {
                use std::fmt::Write;
                use itertools::Itertools;
                // Initialize empty string.
                let mut dot = String::new();
                // Get reference to vertices attributes.
                let vattrs = self.as_vertex_attrs();
                // Open DOT string by escaping "{".
                writeln!(dot, "graph {{").ok();
                // Write vertex with its attributes.
                for x in self.vertices_iter() {
                    // Write the vertex identifier.
                    write!(dot, "\t{:?}", x).ok();
                    // Get vertex attributes.
                    let attrs = &vattrs[x];
                    // Write its attributes, if any.
                    if !attrs.is_empty() {
                        // Format key-value pairs.
                        let attrs = attrs
                            .iter()
                            .map(|(k, v)| {
                                // Try to downcast the Any type to a printable one.
                                if let Some(v) = v.downcast_ref::<String>() {
                                    format!("{}={:?}", k, v)
                                } else if let Some(v) = v.downcast_ref::<&str>() {
                                    format!("{}={:?}", k, v)
                                } else {
                                    format!("{}=\"{:?}\"", k, v)
                                }
                            })
                            .join(", ");
                        // Write key-value pair.
                        write!(dot, " [{}]", attrs).ok();
                    }
                    // End vertex statement.
                    writeln!(dot, ";").ok();
                }
                // Write edges with label
                for (x, y) in self.edges_iter().filter(|(x, y)| x <= y) {
                    writeln!(dot, "\t{:?} -- {:?};", x, y).ok();
                    // FIXME: writeln!(dot, "\t{:?} {} {:?} [label=\"{}\"];", x, edge_type, y, z).ok();
                }
                // Close DOT string by escaping "}"
                writeln!(dot, "}}").ok();
                // Return resulting DOT string
                dot
            }
        }

        $crate::traits::impl_operators_trait!($graph);
        $crate::traits::impl_with_attributes_trait!($graph);

        impl<T> $crate::traits::Storage for $graph<T>
        where
            T: $crate::types::VertexTrait,
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
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, $crate::errors::Error<Self::Vertex>>;
                }
            }

            fn size(&self) -> usize {
                // De-symmetrize edge set for correct size computation
                self.edges_iter().filter(|(x, y)| x <= y).count()
            }

            fn add_vertex<U>(&mut self, x: &U) -> Result<Self::Vertex, $crate::errors::Error<Self::Vertex>> where U: Eq + Clone + Into<Self::Vertex> {
                // Add vertex into the graph.
                let x = self.data.add_vertex(x)?;
                // Add associated attribute map.
                self.vattrs.insert(x.clone(), Default::default());
                // Return successfully.
                Ok(x)
            }

            fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>> {
                // Delete vertex from the graph.
                self.data.del_vertex(x)?;
                // Delete associated attribute map.
                self.vattrs.remove(x);
                // Return successfully.
                Ok(())
            }

            fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>> {
                // Add edge (y, x)
                self.data.add_edge(y, x)?;
                // Add edge (x, y)
                if x != y {
                    self.data.add_edge(x, y)?;
                }
                // Add associated attribute map.
                self.eattrs.insert((x.clone(), y.clone()), Default::default());
                // Return successfully.
                Ok(())
            }

            fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>> {
                // Del edge (y, x)
                self.data.del_edge(y, x)?;
                // Del edge (x, y)
                if x != y {
                    self.data.del_edge(x, y)?;
                }
                // Delete associated attribute map.
                self.eattrs.remove(&(x.clone(), y.clone()));
                // Return successfully.
                Ok(())
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T, &'a T)> for $crate::algorithms::AllSimplePaths<'a, $graph<T>>
        where
            T: $crate::types::VertexTrait,
        {
            fn from((g, x, y): (&'a $graph<T>, &'a T, &'a T)) -> Self {
                Self::new(g, x, y, $graph::<T>::neighbors_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T>>
        where
            T: $crate::types::VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::neighbors_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::DepthFirstSearch<'a, $graph<T>>
        where
            T: $crate::types::VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::neighbors_iter)
            }
        }
    };
}

pub(crate) use impl_undirected_trait;
