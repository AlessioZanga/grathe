use crate::errors::Error;
use crate::traits::*;
use crate::types::*;
use crate::Adj;
use std::any::Any;

/// The base graph trait.
pub trait Base: Capacity + Convert + Operators + Storage {
    /// Reference to vertex attributes.
    ///
    /// Returns the reference to the vertex attributes map.
    ///
    fn as_vertex_attrs(&self) -> &Attributes<Self::Vertex>;

    /// Checks vertex attribute.
    ///
    /// Checks whether a vertex has an attribute with given key.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    fn has_vertex_attr(&self, x: &Self::Vertex, k: &str) -> Result<bool, Error<Self::Vertex>>;

    /// Gets vertex attribute with given key.
    ///
    /// Returns a reference to an attribute with given key.
    ///
    /// # Errors
    ///
    /// The vertex identifier (or its attribute key) does not exist in the graph.
    ///
    fn get_vertex_attr<'a>(&'a self, x: &'a Self::Vertex, k: &str) -> Result<&'a dyn Any, Error<Self::Vertex>>;

    /// Sets vertex attribute with given key and value.
    ///
    /// Inserts the vertex attribute into the graph, overwriting any previous assignment.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    fn set_vertex_attr<V: 'static>(&mut self, x: &Self::Vertex, k: &str, v: V) -> Result<(), Error<Self::Vertex>>;

    /// Un-sets vertex attribute with given key and returns its value.
    ///
    /// Removes the vertex attribute from the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier (or its attribute key) does not exist in the graph.
    ///
    fn unset_vertex_attr(&mut self, x: &Self::Vertex, k: &str) -> Result<Box<dyn Any>, Error<Self::Vertex>>;

    /// Reference to edge attributes.
    ///
    /// Returns the reference to the edge attributes map.
    ///
    fn as_edge_attrs(&self) -> &Attributes<(Self::Vertex, Self::Vertex)>;

    /// Checks edge attribute.
    ///
    /// Checks whether a edge has an attribute with given key.
    ///
    /// # Errors
    ///
    /// The edge identifier does not exist in the graph.
    ///
    fn has_edge_attr(&self, x: &Self::Vertex, y: &Self::Vertex, k: &str) -> Result<bool, Error<Self::Vertex>>;

    /// Gets edge attribute with given key.
    ///
    /// Returns a reference to an attribute with given key.
    ///
    /// # Errors
    ///
    /// The edge identifier (or its attribute key) does not exist in the graph.
    ///
    fn get_edge_attr<'a>(
        &'a self,
        x: &'a Self::Vertex,
        y: &Self::Vertex,
        k: &str,
    ) -> Result<&'a dyn Any, Error<Self::Vertex>>;

    /// Sets edge attribute with given key and value.
    ///
    /// Inserts the edge attribute into the graph, overwriting any previous assignment.
    ///
    /// # Errors
    ///
    /// The edge identifier does not exist in the graph.
    ///
    fn set_edge_attr<V: 'static>(
        &mut self,
        x: &Self::Vertex,
        y: &Self::Vertex,
        k: &str,
        v: V,
    ) -> Result<(), Error<Self::Vertex>>;

    /// Un-sets edge attribute with given key and returns its value.
    ///
    /// Removes the edge attribute from the graph.
    ///
    /// # Errors
    ///
    /// The edge identifier (or its attribute key) does not exist in the graph.
    ///
    fn unset_edge_attr(
        &mut self,
        x: &Self::Vertex,
        y: &Self::Vertex,
        k: &str,
    ) -> Result<Box<dyn Any>, Error<Self::Vertex>>;

    /// Builds subgraph from given vertices.
    ///
    /// Builds a subgraph, preserving edges between given vertices.
    /// Ignores additional attributes (for now).
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifiers do not exist in the graph.
    ///
    fn subgraph<'a, I>(&self, iter: I) -> Self
    where
        Self: 'a,
        I: IntoIterator<Item = &'a Self::Vertex>,
    {
        // Build a subgraph from the given vertices.
        let mut subgraph = Self::from_vertices(iter);
        // Check if is it a proper subgraph of self,
        // i.e. given vertices are contained in self.
        assert!(subgraph.is_subgraph(self));
        // Copy edges into subgraph.
        for (x, y) in self.edges_iter() {
            subgraph.add_edge(x, y).ok();
        }

        subgraph
    }

    /// Is subgraph of another graph.
    ///
    /// Checks if this graph is subgraph of given graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build two graphs.
    /// let g = Graph::new();
    /// let h = Graph::from_order(2);
    ///
    /// // The null graph is always subgraph of an other graph.
    /// assert_true!(g.is_subgraph(&h));
    ///
    /// // Use the associated `<=` operator.
    /// assert_true!(g <= h);
    /// ```
    ///
    fn is_subgraph(&self, other: &Self) -> bool {
        self <= other
    }

    /// Is supergraph of another graph.
    ///
    /// Checks if this graph is supergraph of given graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// // Build two graphs.
    /// let g = Graph::new();
    /// let h = Graph::from_order(2);
    ///
    /// // Any graph is supergraph of the null graph.
    /// assert_true!(h.is_supergraph(&g));
    ///
    /// // Use the associated `>=` operator.
    /// assert_true!(h >= g);
    /// ```
    ///
    fn is_supergraph(&self, other: &Self) -> bool {
        self >= other
    }

    /// Degree of vertex.
    ///
    /// Degree of given vertex identifier $E$ as $|Adj(G, X)|$.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a graph.
    /// let g = Graph::from_edges(&[(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Get the degree of `1`.
    /// assert_true!(
    ///     g.degree_of(&1) == 3 &&
    ///     g.degree_of(&1) == Adj!(g, &1).count()
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn degree_of(&self, x: &Self::Vertex) -> usize {
        Adj!(self, x).count()
    }

    /// Is isolated vertex.
    ///
    /// Checks whether the vertex is not adjacent to any other vertex in the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a graph.
    /// let g = Graph::from_edges(&[(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is isolated (a.k.a not connected).
    /// assert_false!(g.is_isolated_vertex(&0));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn is_isolated_vertex(&self, x: &Self::Vertex) -> bool {
        self.degree_of(x) == 0
    }

    /// Is pendant vertex.
    ///
    /// Checks whether the vertex is adjacent to only one vertex in the graph.
    ///
    /// # Errors
    ///
    /// The vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a graph.
    /// let g = Graph::from_edges(&[(0, 1), (2, 1), (3, 1)]);
    ///
    /// // Check if `0` is pendant (a.k.a is connected to just one vertex).
    /// assert_true!(g.is_pendant_vertex(&0));
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn is_pendant_vertex(&self, x: &Self::Vertex) -> bool {
        self.degree_of(x) == 1
    }
}

/// Storage delegation and graph trait implementation for graphs.
#[macro_export]
macro_rules! impl_base_trait {
    ($graph:ident, $storage:ident) => {
        // FIXME: Remove after stabilization.
        $crate::impl_convert_trait!($graph);

        impl<T> $crate::traits::Base for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn as_vertex_attrs(&self) -> &$crate::types::Attributes<Self::Vertex> {
                &self.vattrs
            }

            fn has_vertex_attr(&self, x: &Self::Vertex, k: &str) -> Result<bool, $crate::errors::Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get(x) {
                    // If vertex is not defined return error.
                    None => Err($crate::errors::Error::VertexNotDefined(x.clone())),
                    // Otherwise, check if vertex has any attribute or an attribute with given key.
                    Some(attrs) => Ok(attrs.contains_key(k)),
                }
            }

            fn get_vertex_attr<'a>(
                &'a self,
                x: &'a Self::Vertex,
                k: &str,
            ) -> Result<&'a dyn std::any::Any, $crate::errors::Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get(x) {
                    // If vertex is not defined return error.
                    None => Err($crate::errors::Error::VertexNotDefined(x.clone())),
                    // Otherwise, check if vertex has an attribute with given key.
                    Some(attrs) => match attrs.get(k) {
                        // If attribute key is not defined return error.
                        None => Err($crate::errors::Error::VertexAttributeNotDefined(
                            x.clone(),
                            k.to_string(),
                        )),
                        // Otherwise, cast pointer to given type.
                        Some(v) => Ok(v),
                    },
                }
            }

            fn set_vertex_attr<V: 'static>(
                &mut self,
                x: &Self::Vertex,
                k: &str,
                v: V,
            ) -> Result<(), $crate::errors::Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get_mut(x) {
                    // Vertex is not defined, return error.
                    None => Err($crate::errors::Error::VertexNotDefined(x.clone())),
                    // Otherwise, check if vertex has any attribute or an attribute with given key.
                    Some(attrs) => {
                        // Insert attribute in map.
                        attrs.insert(k.to_string(), Box::new(v));
                        // Return successfully.
                        Ok(())
                    }
                }
            }

            fn unset_vertex_attr(
                &mut self,
                x: &Self::Vertex,
                k: &str,
            ) -> Result<Box<dyn std::any::Any>, $crate::errors::Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get_mut(x) {
                    // If vertex is not defined return error.
                    None => Err($crate::errors::Error::VertexNotDefined(x.clone())),
                    // Otherwise, try to remove the attribute with given key.
                    Some(attrs) => match attrs.remove(k) {
                        // There is no such attribute, return error.
                        None => Err($crate::errors::Error::VertexAttributeNotDefined(
                            x.clone(),
                            k.to_string(),
                        )),
                        // Return successfully.
                        Some(v) => Ok(v),
                    },
                }
            }

            fn as_edge_attrs(&self) -> &$crate::types::Attributes<(Self::Vertex, Self::Vertex)> {
                &self.eattrs
            }

            fn has_edge_attr(
                &self,
                x: &Self::Vertex,
                y: &Self::Vertex,
                k: &str,
            ) -> Result<bool, $crate::errors::Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.eattrs.get(&(x.clone(), y.clone())) {
                    // If edge is not defined return error.
                    None => Err($crate::errors::Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise, check if edge has any attribute or an attribute with given key.
                    Some(attrs) => Ok(attrs.contains_key(k)),
                }
            }

            fn get_edge_attr<'a>(
                &'a self,
                x: &Self::Vertex,
                y: &Self::Vertex,
                k: &str,
            ) -> Result<&'a dyn std::any::Any, $crate::errors::Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.eattrs.get(&(x.clone(), y.clone())) {
                    // If edge is not defined return error.
                    None => Err($crate::errors::Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise, check if edge has an attribute with given key.
                    Some(vattrs) => match vattrs.get(k) {
                        // If attribute key is not defined return error.
                        None => Err($crate::errors::Error::EdgeAttributeNotDefined(
                            x.clone(),
                            y.clone(),
                            k.to_string(),
                        )),
                        // Otherwise, cast pointer to given type.
                        Some(v) => Ok(v),
                    },
                }
            }

            fn set_edge_attr<V: 'static>(
                &mut self,
                x: &Self::Vertex,
                y: &Self::Vertex,
                k: &str,
                v: V,
            ) -> Result<(), $crate::errors::Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.eattrs.get_mut(&(x.clone(), y.clone())) {
                    // Edge is not defined, return error.
                    None => Err($crate::errors::Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise, check if edge has any attribute or an attribute with given key.
                    Some(attrs) => {
                        // Insert attribute in map.
                        attrs.insert(k.to_string(), Box::new(v));
                        // Return successfully.
                        Ok(())
                    }
                }
            }

            fn unset_edge_attr(
                &mut self,
                x: &Self::Vertex,
                y: &Self::Vertex,
                k: &str,
            ) -> Result<Box<dyn std::any::Any>, $crate::errors::Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.vattrs.get_mut(x) {
                    // If edge is not defined return error.
                    None => Err($crate::errors::Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise, try to remove the attribute with given key.
                    Some(attrs) => match attrs.remove(k) {
                        // There is no such attribute, return error.
                        None => Err($crate::errors::Error::EdgeAttributeNotDefined(
                            x.clone(),
                            y.clone(),
                            k.to_string(),
                        )),
                        // Return successfully.
                        Some(v) => Ok(v),
                    },
                }
            }
        }
    };
}
