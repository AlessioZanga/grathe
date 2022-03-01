use crate::types::Error;
use crate::types::VertexTrait;
use std::fmt::Debug;

/// The graph attribute trait.
pub trait WithAttributes<T>: Default + Debug
where
    T: VertexTrait,
{
    /// Graph attributes type.
    type GraphAttributes;

    /// Vertex attributes type.
    type VertexAttributes;

    /// Edge attributes type.
    type EdgeAttributes;

    /// New constructor.
    fn new_with_attributes<I, J>(x: Self::GraphAttributes, y: I, z: J) -> Self
    where
        I: IntoIterator<Item = (T, Self::VertexAttributes)>,
        J: IntoIterator<Item = ((T, T), Self::EdgeAttributes)>;

    /// Checks graph attributes.
    ///
    /// Checks whether a graph has attributes.
    ///
    fn has_graph_attrs(&self) -> bool;

    /// Gets graph attributes.
    ///
    /// Returns a reference to graph attributes.
    ///
    fn get_graph_attrs(&self) -> Result<&Self::GraphAttributes, Error<T>>;

    /// Gets mutable graph attributes.
    ///
    /// Returns a mutable reference to graph attributes.
    ///
    fn get_mut_graph_attrs(&mut self) -> Result<&mut Self::GraphAttributes, Error<T>>;

    /// Sets graph attributes.
    ///
    /// Inserts the graph attributes into the graph, overwriting previous assignment.
    ///
    fn set_graph_attrs(&mut self, x: Self::GraphAttributes);

    /// Un-sets graph attributes and returns their value.
    ///
    /// Removes the graph attributes from the graph.
    ///
    fn unset_graph_attrs(&mut self) -> Result<Self::GraphAttributes, Error<T>>;

    /// Checks vertex attributes.
    ///
    /// Checks whether a vertex has attributes.
    ///
    fn has_vertex_attrs(&self, x: &T) -> bool;

    /// Gets vertex attributes.
    ///
    /// Returns a reference to vertex attributes.
    ///
    fn get_vertex_attrs(&self, x: &T) -> Result<&Self::VertexAttributes, Error<T>>;

    /// Gets mutable vertex attributes.
    ///
    /// Returns a mutable reference to vertex attributes.
    ///
    fn get_mut_vertex_attrs(&mut self, x: &T) -> Result<&mut Self::VertexAttributes, Error<T>>;

    /// Sets vertex attributes.
    ///
    /// Inserts the vertex attributes into the graph, overwriting previous assignment.
    ///
    fn set_vertex_attrs(&mut self, x: &T, y: Self::VertexAttributes);

    /// Un-sets vertex attributes and returns their value.
    ///
    /// Removes the vertex attributes from the graph.
    ///
    fn unset_vertex_attrs(&mut self, x: &T) -> Result<Self::VertexAttributes, Error<T>>;

    /// Checks edge attributes.
    ///
    /// Checks whether an edge has attributes.
    ///
    fn has_edge_attrs(&self, x: &T, y: &T) -> bool;

    /// Gets edge attributes.
    ///
    /// Returns a reference to edge attributes.
    ///
    fn get_edge_attrs(&self, x: &T, y: &T) -> Result<&Self::EdgeAttributes, Error<T>>;

    /// Gets mutable edge attributes.
    ///
    /// Returns a mutable reference to edge attributes.
    ///
    fn get_mut_edge_attrs(&mut self, x: &T, y: &T) -> Result<&mut Self::EdgeAttributes, Error<T>>;

    /// Sets edge attributes.
    ///
    /// Inserts the edge attributes into the graph, overwriting previous assignment.
    ///
    fn set_edge_attrs(&mut self, x: &T, y: &T, z: Self::EdgeAttributes);

    /// Un-sets edge attributes and returns their value.
    ///
    /// Removes the edge attributes from the graph.
    ///
    fn unset_edge_attrs(&mut self, x: &T, y: &T) -> Result<Self::EdgeAttributes, Error<T>>;
}

macro_rules! impl_with_attributes {
    ($graph:ident) => {
        impl<T, U> $crate::traits::WithAttributes<T> for $graph<T, U>
        where
            T: $crate::types::VertexTrait,
            U: $crate::traits::WithAttributes<T>,
        {
            type GraphAttributes = U::GraphAttributes;
            type VertexAttributes = U::VertexAttributes;
            type EdgeAttributes = U::EdgeAttributes;

            fn new_with_attributes<I, J>(x: Self::GraphAttributes, y: I, z: J) -> Self
            where
                I: IntoIterator<Item = (T, Self::VertexAttributes)>,
                J: IntoIterator<Item = ((T, T), Self::EdgeAttributes)>,
            {
                Self {
                    // FIXME: Change from Default to actual construction of storage (V, E).
                    data: Default::default(),
                    attributes: U::new_with_attributes(x, y, z),
                }
            }

            fn has_vertex_attrs(&self, x: &T) -> bool {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self.attributes.has_vertex_attrs(x)
            }

            fn get_vertex_attrs(&self, x: &T) -> Result<&Self::VertexAttributes, Error<T>> {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self.attributes.get_vertex_attrs(x)
            }

            fn get_mut_vertex_attrs(&mut self, x: &T) -> Result<&mut Self::VertexAttributes, Error<T>> {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self.attributes.get_mut_vertex_attrs(x)
            }

            fn set_vertex_attrs(&mut self, x: &T, y: Self::VertexAttributes) {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self.attributes.set_vertex_attrs(x, y)
            }

            fn unset_vertex_attrs(&mut self, x: &T) -> Result<Self::VertexAttributes, Error<T>> {
                // Sanitize inputs.
                assert!(self.has_vertex(x));
                // Delegate method.
                self.attributes.unset_vertex_attrs(x)
            }

            fn has_edge_attrs(&self, x: &T, y: &T) -> bool {
                // Sanitize inputs.
                assert!(self.has_edge(x, y).unwrap());
                // Delegate method.
                self.attributes.has_edge_attrs(x, y)
            }

            fn get_edge_attrs(&self, x: &T, y: &T) -> Result<&Self::EdgeAttributes, Error<T>> {
                // Sanitize inputs.
                assert!(self.has_edge(x, y).unwrap());
                // Delegate method.
                self.attributes.get_edge_attrs(x, y)
            }

            fn get_mut_edge_attrs(&mut self, x: &T, y: &T) -> Result<&mut Self::EdgeAttributes, Error<T>> {
                // Sanitize inputs.
                assert!(self.has_edge(x, y).unwrap());
                // Delegate method.
                self.attributes.get_mut_edge_attrs(x, y)
            }

            fn set_edge_attrs(&mut self, x: &T, y: &T, z: Self::EdgeAttributes) {
                // Sanitize inputs.
                assert!(self.has_edge(x, y).unwrap());
                // Delegate method.
                self.attributes.set_edge_attrs(x, y, z)
            }

            fn unset_edge_attrs(&mut self, x: &T, y: &T) -> Result<Self::EdgeAttributes, Error<T>> {
                // Sanitize inputs.
                assert!(self.has_edge(x, y).unwrap());
                // Delegate method.
                self.attributes.unset_edge_attrs(x, y)
            }

            delegate::delegate! {
                to self.attributes {
                    fn has_graph_attrs(&self) -> bool;
                    fn get_graph_attrs(&self) -> Result<&Self::GraphAttributes, Error<T>>;
                    fn get_mut_graph_attrs(&mut self) -> Result<&mut Self::GraphAttributes, Error<T>>;
                    fn set_graph_attrs(&mut self, x: Self::GraphAttributes);
                    fn unset_graph_attrs(&mut self) -> Result<Self::GraphAttributes, Error<T>>;
                }
            }
        }
    };
}

pub(crate) use impl_with_attributes;
