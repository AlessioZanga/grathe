use crate::errors::Error;
use crate::traits::{Base, Connectivity, Convert};
use crate::types::VertexIterator;
use std::collections::{BTreeSet, VecDeque};

/// Directed graph trait.
pub trait Directed: Base + Connectivity + Convert {
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

macro_rules! impl_directed_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> PartialEq for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn eq(&self, other: &Self) -> bool {
                self.data.eq(&other.data)
            }
        }

        impl<T> Eq for $graph<T> where T: $crate::types::VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
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
                    for w in self.children_iter(z) {
                        // If the vertex has never seen before.
                        if !visited.contains(w) {
                            // Check if vertex is target.
                            if w == y {
                                // Return has directed path.
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

            fn is_connected(&self) -> bool {
                todo!()
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
                writeln!(dot, "digraph {{").ok();
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
                for (x, y) in self.edges_iter() {
                    writeln!(dot, "\t{:?} -> {:?};", x, y).ok();
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
                    fn size(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn add_vertex<U>(&mut self, x: &U) -> Result<Self::Vertex, $crate::errors::Error<Self::Vertex>> where U: Eq + Clone + Into<Self::Vertex>;
                    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>>;
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, $crate::errors::Error<Self::Vertex>>;
                    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>>;
                    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>>;
                }
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T>>
        where
            T: $crate::types::VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::DepthFirstSearch<'a, $graph<T>>
        where
            T: $crate::types::VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::children_iter)
            }
        }
    };
}

pub(crate) use impl_directed_trait;
