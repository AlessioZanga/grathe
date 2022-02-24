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

            delegate::delegate! {
                to self.attributes {
                    fn has_graph_attrs(&self) -> bool;
                    fn get_graph_attrs(&self) -> Result<&Self::GraphAttributes, Error<T>>;
                    fn get_mut_graph_attrs(&mut self) -> Result<&mut Self::GraphAttributes, Error<T>>;
                    fn set_graph_attrs(&mut self, x: Self::GraphAttributes);
                    fn unset_graph_attrs(&mut self) -> Result<Self::GraphAttributes, Error<T>>;
                    fn has_vertex_attrs(&self, x: &T) -> bool;
                    fn get_vertex_attrs(&self, x: &T) -> Result<&Self::VertexAttributes, Error<T>>;
                    fn get_mut_vertex_attrs(&mut self, x: &T) -> Result<&mut Self::VertexAttributes, Error<T>>;
                    fn set_vertex_attrs(&mut self, x: &T, y: Self::VertexAttributes);
                    fn unset_vertex_attrs(&mut self, x: &T) -> Result<Self::VertexAttributes, Error<T>>;
                    fn has_edge_attrs(&self, x: &T, y: &T) -> bool;
                    fn get_edge_attrs(&self, x: &T, y: &T) -> Result<&Self::EdgeAttributes, Error<T>>;
                    fn get_mut_edge_attrs(&mut self, x: &T, y: &T) -> Result<&mut Self::EdgeAttributes, Error<T>>;
                    fn set_edge_attrs(&mut self, x: &T, y: &T, z: Self::EdgeAttributes);
                    fn unset_edge_attrs(&mut self, x: &T, y: &T) -> Result<Self::EdgeAttributes, Error<T>>;
                }
            }
        }
    };
}

pub(crate) use impl_with_attributes;
