use crate::errors::Error;
use crate::traits::Storage;
use crate::types::Attributes;
use std::any::Any;

pub trait WithAttributes: Storage {
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
}

macro_rules! impl_with_attributes_trait {
    ($graph:ident) => {
        impl<T> $crate::traits::WithAttributes for $graph<T>
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

pub(crate) use impl_with_attributes_trait;
