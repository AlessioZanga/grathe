/// Storage delegation and graph trait implementation for undirected graphs.
#[macro_export]
macro_rules! impl_ungraph_trait {
    ($graph:ident, $storage:ident) => {
        use crate::errors::Error;
        use crate::storages::StorageTrait;
        use crate::graphs::GraphTrait;
        use delegate::delegate;
        use std::cmp::Ordering;

        impl<T> PartialEq for $graph<T>
        where
            T: VertexTrait,
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
            /// use grathe::graphs::Graph;
            /// use grathe::storages::StorageTrait;
            ///
            /// let g = Graph::new();
            /// let h = Graph::new();
            ///
            /// assert_eq!(g, h);
            /// ```
            ///
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<T> Eq for $graph<T> where T: VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: VertexTrait,
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
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<T> StorageTrait for $graph<T>
        where
            T: VertexTrait,
        {
            type Vertex = T;

            type Storage = $storage<T>;

            #[inline(always)]
            fn new() -> Self {
                Self {
                    0: Default::default()
                }
            }

            #[inline(always)]
            fn with_capacity(capacity: usize) -> Self {
                Self {
                    0: Self::Storage::with_capacity(capacity)
                }
            }

            delegate! {
                to self.0 {
                    fn clear(&mut self);
                    fn capacity(&self) -> usize;
                    fn reserve(&mut self, additional: usize);
                    fn shrink_to(&mut self, min_capacity: usize);
                    fn shrink_to_fit(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<&'a Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<(&'a Self::Vertex, &'a Self::Vertex)> + 'a>;
                    fn adjacents_iter<'a>(
                        &'a self,
                        x: &'a Self::Vertex,
                    ) -> Result<Box<dyn VertexIterator<&'a Self::Vertex> + 'a>, Error<Self::Vertex>>;
                    fn order(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn add_vertex<'a>(&mut self, x: &'a Self::Vertex) -> Result<&'a Self::Vertex, Error<Self::Vertex>>;
                    fn del_vertex<'a>(&mut self, x: &'a Self::Vertex) -> Result<&'a Self::Vertex, Error<Self::Vertex>>;
                    fn has_edge<'a>(&self, x: (&'a Self::Vertex, &'a Self::Vertex)) -> Result<bool, Error<Self::Vertex>>;
                }
            }

            #[inline(always)]
            fn size(&self) -> usize {
                // De-symmetrize edge set for correct size computation
                self.edges_iter().filter(|(x, y)| x <= y).count()
            }

            #[inline(always)]
            fn add_edge<'a>(&mut self, (x, y): (&'a Self::Vertex, &'a Self::Vertex)) -> Result<(&'a Self::Vertex, &'a Self::Vertex), Error<Self::Vertex>> {
                // Add edge (y, x)
                self.0.add_edge((y, x))?;
                // Add edge (x, y)
                match x == y {
                    false => self.0.add_edge((x, y)),
                    true => Ok((x, y))
                }
            }

            #[inline(always)]
            fn del_edge<'a>(&mut self, (x, y): (&'a Self::Vertex, &'a Self::Vertex)) -> Result<(&'a Self::Vertex, &'a Self::Vertex), Error<Self::Vertex>> {
                // Del edge (y, x)
                self.0.del_edge((y, x))?;
                // Del edge (x, y)
                match x == y {
                    false => self.0.del_edge((x, y)),
                    true => Ok((x, y))
                }
            }
        }

        impl<T> GraphTrait for $graph<T> where T: VertexTrait {}
    };
}

/// Storage delegation and graph trait implementation for directed/partially-directed graphs.
#[macro_export]
macro_rules! impl_digraph_trait {
    ($graph:ident, $storage:ident) => {
        use crate::errors::Error;
        use crate::storages::StorageTrait;
        use crate::graphs::GraphTrait;
        use delegate::delegate;
        use std::cmp::Ordering;

        impl<T> PartialEq for $graph<T>
        where
            T: VertexTrait,
        {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<T> Eq for $graph<T> where T: VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: VertexTrait,
        {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<T> StorageTrait for $graph<T>
        where
            T: VertexTrait,
        {
            type Vertex = T;

            type Storage = $storage<T>;

            #[inline(always)]
            fn new() -> Self {
                Self {
                    0: Default::default()
                }
            }

            #[inline(always)]
            fn with_capacity(capacity: usize) -> Self {
                Self {
                    0: Self::Storage::with_capacity(capacity)
                }
            }

            delegate! {
                to self.0 {
                    fn clear(&mut self);
                    fn capacity(&self) -> usize;
                    fn reserve(&mut self, additional: usize);
                    fn shrink_to(&mut self, min_capacity: usize);
                    fn shrink_to_fit(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<&'a Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<(&'a Self::Vertex, &'a Self::Vertex)> + 'a>;
                    fn adjacents_iter<'a>(
                        &'a self,
                        x: &'a Self::Vertex,
                    ) -> Result<Box<dyn VertexIterator<&'a Self::Vertex> + 'a>, Error<Self::Vertex>>;
                    fn order(&self) -> usize;
                    fn size(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn add_vertex<'a>(&mut self, x: &'a Self::Vertex) -> Result<&'a Self::Vertex, Error<Self::Vertex>>;
                    fn del_vertex<'a>(&mut self, x: &'a Self::Vertex) -> Result<&'a Self::Vertex, Error<Self::Vertex>>;
                    fn has_edge<'a>(&self, x: (&'a Self::Vertex, &'a Self::Vertex)) -> Result<bool, Error<Self::Vertex>>;
                    fn add_edge<'a>(&mut self, (x, y): (&'a Self::Vertex, &'a Self::Vertex)) -> Result<(&'a Self::Vertex, &'a Self::Vertex), Error<Self::Vertex>>;
                    fn del_edge<'a>(&mut self, (x, y): (&'a Self::Vertex, &'a Self::Vertex)) -> Result<(&'a Self::Vertex, &'a Self::Vertex), Error<Self::Vertex>>;
                }
            }
        }

        impl<T> GraphTrait for $graph<T> where T: VertexTrait {}
    };
}
