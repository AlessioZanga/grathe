use std::fmt::Debug;

use crate::types::{Error, Vertex};

/// The graph attribute trait.
pub trait WithAttributes<V>: Clone + Default + Debug
where
    V: Vertex,
{
    /// Graph attributes type.
    type GraphAttributes;

    /// Vertex attributes type.
    type VertexAttributes;

    /// Edge attributes type.
    type EdgeAttributes;

    /// New constructor.
    fn new_with_attributes<I, J>(x: Option<Self::GraphAttributes>, y: I, z: J) -> Self
    where
        I: IntoIterator<Item = (V, Self::VertexAttributes)>,
        J: IntoIterator<Item = ((V, V), Self::EdgeAttributes)>;

    /// Checks graph attributes.
    ///
    /// Checks whether a graph has attributes.
    ///
    fn has_graph_attrs(&self) -> bool;

    /// Gets graph attributes.
    ///
    /// Returns a reference to graph attributes.
    ///
    fn get_graph_attrs(&self) -> Result<&Self::GraphAttributes, Error<V>>;

    /// Gets mutable graph attributes.
    ///
    /// Returns a mutable reference to graph attributes.
    ///
    fn get_mut_graph_attrs(&mut self) -> Result<&mut Self::GraphAttributes, Error<V>>;

    /// Sets graph attributes.
    ///
    /// Inserts the graph attributes into the graph, overwriting previous assignment.
    ///
    fn set_graph_attrs(&mut self, x: Self::GraphAttributes);

    /// Un-sets graph attributes and returns their value.
    ///
    /// Removes the graph attributes from the graph.
    ///
    fn unset_graph_attrs(&mut self) -> Result<Self::GraphAttributes, Error<V>>;

    /// Checks vertex attributes.
    ///
    /// Checks whether a vertex has attributes.
    ///
    fn has_vertex_attrs(&self, x: &V) -> bool;

    /// Gets vertex attributes.
    ///
    /// Returns a reference to vertex attributes.
    ///
    fn get_vertex_attrs(&self, x: &V) -> Result<&Self::VertexAttributes, Error<V>>;

    /// Gets mutable vertex attributes.
    ///
    /// Returns a mutable reference to vertex attributes.
    ///
    fn get_mut_vertex_attrs(&mut self, x: &V) -> Result<&mut Self::VertexAttributes, Error<V>>;

    /// Sets vertex attributes.
    ///
    /// Inserts the vertex attributes into the graph, overwriting previous assignment.
    ///
    fn set_vertex_attrs(&mut self, x: &V, y: Self::VertexAttributes);

    /// Un-sets vertex attributes and returns their value.
    ///
    /// Removes the vertex attributes from the graph.
    ///
    fn unset_vertex_attrs(&mut self, x: &V) -> Result<Self::VertexAttributes, Error<V>>;

    /// Checks edge attributes.
    ///
    /// Checks whether an edge has attributes.
    ///
    fn has_edge_attrs(&self, x: &V, y: &V) -> bool;

    /// Gets edge attributes.
    ///
    /// Returns a reference to edge attributes.
    ///
    fn get_edge_attrs(&self, x: &V, y: &V) -> Result<&Self::EdgeAttributes, Error<V>>;

    /// Gets mutable edge attributes.
    ///
    /// Returns a mutable reference to edge attributes.
    ///
    fn get_mut_edge_attrs(&mut self, x: &V, y: &V) -> Result<&mut Self::EdgeAttributes, Error<V>>;

    /// Sets edge attributes.
    ///
    /// Inserts the edge attributes into the graph, overwriting previous assignment.
    ///
    fn set_edge_attrs(&mut self, x: &V, y: &V, z: Self::EdgeAttributes);

    /// Un-sets edge attributes and returns their value.
    ///
    /// Removes the edge attributes from the graph.
    ///
    fn unset_edge_attrs(&mut self, x: &V, y: &V) -> Result<Self::EdgeAttributes, Error<V>>;
}

macro_rules! impl_with_attributes {
    ($graph: ident) => {
        impl<V, A> $crate::traits::WithAttributes<V> for $graph<V, A>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            type GraphAttributes = A::GraphAttributes;
            type VertexAttributes = A::VertexAttributes;
            type EdgeAttributes = A::EdgeAttributes;

            fn new_with_attributes<I, J>(x: Option<Self::GraphAttributes>, y: I, z: J) -> Self
            where
                I: IntoIterator<Item = (V, Self::VertexAttributes)>,
                J: IntoIterator<Item = ((V, V), Self::EdgeAttributes)>,
            {
                let y: Vec<_> = y.into_iter().collect();
                let z: Vec<_> = z.into_iter().collect();

                let mut out = Self::new(y.iter().map(|(y, _)| y.clone()), z.iter().map(|(z, _)| z.clone()));

                out._attributes = A::new_with_attributes(x, y, z);

                out
            }

            fn has_vertex_attrs(&self, x: &V) -> bool {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self._attributes.has_vertex_attrs(x)
            }

            fn get_vertex_attrs(&self, x: &V) -> Result<&Self::VertexAttributes, Error<V>> {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self._attributes.get_vertex_attrs(x)
            }

            fn get_mut_vertex_attrs(&mut self, x: &V) -> Result<&mut Self::VertexAttributes, Error<V>> {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self._attributes.get_mut_vertex_attrs(x)
            }

            fn set_vertex_attrs(&mut self, x: &V, y: Self::VertexAttributes) {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self._attributes.set_vertex_attrs(x, y)
            }

            fn unset_vertex_attrs(&mut self, x: &V) -> Result<Self::VertexAttributes, Error<V>> {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self._attributes.unset_vertex_attrs(x)
            }

            fn has_edge_attrs(&self, x: &V, y: &V) -> bool {
                // Sanitize inputs.
                assert!(self.has_edge(x, y));
                // Delegate method.
                self._attributes.has_edge_attrs(x, y)
            }

            fn get_edge_attrs(&self, x: &V, y: &V) -> Result<&Self::EdgeAttributes, Error<V>> {
                // Sanitize inputs.
                assert!(self.has_edge(x, y));
                // Delegate method.
                self._attributes.get_edge_attrs(x, y)
            }

            fn get_mut_edge_attrs(&mut self, x: &V, y: &V) -> Result<&mut Self::EdgeAttributes, Error<V>> {
                // Sanitize inputs.
                assert!(self.has_edge(x, y));
                // Delegate method.
                self._attributes.get_mut_edge_attrs(x, y)
            }

            fn set_edge_attrs(&mut self, x: &V, y: &V, z: Self::EdgeAttributes) {
                // Sanitize inputs.
                assert!(self.has_edge(x, y));
                // Delegate method.
                self._attributes.set_edge_attrs(x, y, z)
            }

            fn unset_edge_attrs(&mut self, x: &V, y: &V) -> Result<Self::EdgeAttributes, Error<V>> {
                // Sanitize inputs.
                assert!(self.has_edge(x, y));
                // Delegate method.
                self._attributes.unset_edge_attrs(x, y)
            }

            delegate::delegate! {
                to self._attributes {
                    fn has_graph_attrs(&self) -> bool;
                    fn get_graph_attrs(&self) -> Result<&Self::GraphAttributes, Error<V>>;
                    fn get_mut_graph_attrs(&mut self) -> Result<&mut Self::GraphAttributes, Error<V>>;
                    fn set_graph_attrs(&mut self, x: Self::GraphAttributes);
                    fn unset_graph_attrs(&mut self) -> Result<Self::GraphAttributes, Error<V>>;
                }
            }
        }
    };
}

pub(crate) use impl_with_attributes;
