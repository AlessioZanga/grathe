use crate::directions::DirectionalTrait;
use crate::errors::Error;
use crate::io::*;
use crate::storages::StorageTrait;
use crate::types::*;
use crate::Adj;
use std::any::Any;
use std::path::Path;

/// The base graph trait.
pub trait GraphTrait: DirectionalTrait + StorageTrait {
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
    ///     g.degree_of(&1)? == 3 &&
    ///     g.degree_of(&1)? == Adj!(g, &1)?.count()
    /// );
    ///
    /// // Getting the degree of a non-existing vertex yields an error.
    /// assert_true!(g.degree_of(&4).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn degree_of(&self, x: &Self::Vertex) -> Result<usize, Error<Self::Vertex>> {
        Ok(Adj!(self, x)?.count())
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
    /// assert_false!(g.is_isolated_vertex(&0)?);
    ///
    /// // Checking a non-existing vertex yields an error.
    /// assert_true!(g.is_isolated_vertex(&4).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn is_isolated_vertex(&self, x: &Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
        Ok(self.degree_of(x)? == 0)
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
    /// assert_true!(g.is_pendant_vertex(&0)?);
    ///
    /// // Checking a non-existing vertex yields an error.
    /// assert_true!(g.is_pendant_vertex(&4).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn is_pendant_vertex(&self, x: &Self::Vertex) -> Result<bool, Error<Self::Vertex>> {
        Ok(self.degree_of(x)? == 1)
    }

    /// From DOT string constructor.
    ///
    /// Construct a graph from a given DOT string.
    /// If more than one graph is parsed, the last one will be returned.
    ///
    /// # Errors
    ///
    /// The DOT string is invalid.
    ///
    /// # Panics
    ///
    /// Panics if the DOT string does not contain at least one graph.
    ///
    fn from_dot(string: &str) -> Result<Self, Error<Self::Vertex>> {
        let mut g = from_dot::<Self>(string).map_err(|x| Error::ParseFailed(x.to_string()))?;
        Ok(g.pop().unwrap())
    }

    /// To DOT string.
    ///
    /// Return a DOT string representation of the graph.
    ///
    fn to_dot(&self) -> Result<String, Error<Self::Vertex>> {
        to_dot::<Self>(self).map_err(|x| Error::ParseFailed(x.to_string()))
    }

    /// Read DOT file constructor.
    ///
    /// Construct a graph from a given DOT file.
    /// If more than one graph is parsed, the last one will be returned.
    ///
    /// # Errors
    ///
    /// The DOT string is invalid.
    ///
    /// # Panics
    ///
    /// Panics if the DOT string does not contain at least one graph.
    ///
    fn read_dot(path: &Path) -> Result<Self, Error<Self::Vertex>> {
        let mut g = read_dot::<Self>(path).map_err(|x| Error::ParseFailed(x.to_string()))?;
        Ok(g.pop().unwrap())
    }

    /// Write DOT file.
    ///
    /// Write the graph to a given DOT file.
    ///
    fn write_dot(&self, path: &Path) -> Result<(), Error<Self::Vertex>> {
        write_dot::<Self>(path, self).map_err(|x| Error::ParseFailed(x.to_string()))
    }
}
