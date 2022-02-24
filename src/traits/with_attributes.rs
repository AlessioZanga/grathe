use super::Storage;
use crate::errors::Error;

/// The graph attribute trait.
pub trait WithAttributes: Storage {
    /// Graph attributes type.
    type GraphAttributes;

    /// Vertex attributes type.
    type VertexAttributes;

    /// Edge attributes type.
    type EdgeAttributes;

    /// Checks vertex attributes.
    ///
    /// Checks whether a vertex has attributes.
    ///
    fn has_vertex_attrs(&self, x: &Self::Vertex) -> bool;

    /// Gets vertex attributes.
    ///
    /// Returns a reference to vertex attributes.
    ///
    fn get_vertex_attrs<'a>(&'a self, x: &'a Self::Vertex) -> Result<&'a Self::VertexAttributes, Error<Self::Vertex>>;

    /// Sets vertex attributes.
    ///
    /// Inserts the vertex attributes into the graph, overwriting previous assignment.
    ///
    fn set_vertex_attrs(&mut self, x: &Self::Vertex, y: Self::VertexAttributes);

    /// Un-sets vertex attributes and returns their value.
    ///
    /// Removes the vertex attributes from the graph.
    ///
    fn unset_vertex_attrs(&mut self, x: &Self::Vertex) -> Result<Self::VertexAttributes, Error<Self::Vertex>>;

    /// Checks edge attributes.
    ///
    /// Checks whether an edge has attributes.
    ///
    fn has_edge_attrs(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool;

    /// Gets edge attributes.
    ///
    /// Returns a reference to edge attributes.
    ///
    fn get_edge_attrs<'a>(
        &'a self,
        x: &'a Self::Vertex,
        y: &'a Self::Vertex,
    ) -> Result<&'a Self::EdgeAttributes, Error<Self::Vertex>>;

    /// Sets edge attributes.
    ///
    /// Inserts the edge attributes into the graph, overwriting previous assignment.
    ///
    fn set_edge_attrs(&mut self, x: &Self::Vertex, y: &Self::Vertex, z: Self::EdgeAttributes);

    /// Un-sets edge attributes and returns their value.
    ///
    /// Removes the edge attributes from the graph.
    ///
    fn unset_edge_attrs(
        &mut self,
        x: &Self::Vertex,
        y: &Self::Vertex,
    ) -> Result<Self::EdgeAttributes, Error<Self::Vertex>>;
}

macro_rules! impl_with_attributes {
    ($graph:ident) => {
        impl<T, X, Y, Z> $crate::traits::WithAttributes for $graph<T, X, Y, Z>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            type GraphAttributes = X;
            type VertexAttributes = Y;
            type EdgeAttributes = Z;

            fn has_vertex_attrs(&self, x: &Self::Vertex) -> bool {
                todo!()
            }

            fn get_vertex_attrs<'a>(
                &'a self,
                x: &'a Self::Vertex,
            ) -> Result<&'a Self::VertexAttributes, Error<Self::Vertex>> {
                todo!()
            }

            fn set_vertex_attrs(&mut self, x: &Self::Vertex, y: Self::VertexAttributes) {
                todo!()
            }

            fn unset_vertex_attrs(&mut self, x: &Self::Vertex) -> Result<Self::VertexAttributes, Error<Self::Vertex>> {
                todo!()
            }

            fn has_edge_attrs(&self, x: &Self::Vertex, y: &Self::Vertex) -> bool {
                todo!()
            }

            fn get_edge_attrs<'a>(
                &'a self,
                x: &'a Self::Vertex,
                y: &'a Self::Vertex,
            ) -> Result<&'a Self::EdgeAttributes, Error<Self::Vertex>> {
                todo!()
            }

            fn set_edge_attrs(&mut self, x: &Self::Vertex, y: &Self::Vertex, z: Self::EdgeAttributes) {
                todo!()
            }

            fn unset_edge_attrs(
                &mut self,
                x: &Self::Vertex,
                y: &Self::Vertex,
            ) -> Result<Self::EdgeAttributes, Error<Self::Vertex>> {
                todo!()
            }
        }
    };
}

pub(crate) use impl_with_attributes;
