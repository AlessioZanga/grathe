use crate::traits::WithAttributes;
use crate::types::Error;
use crate::types::Vertex;
use std::collections::HashMap;
use std::fmt::Debug;

/// A simple hashmap-based attribute manager.
#[derive(Debug)]
pub struct AttributesMap<T, X, Y, Z>
where
    T: Vertex,
    X: Debug,
    Y: Debug,
    Z: Debug,
{
    gattrs: Option<X>,
    vattrs: HashMap<T, Y>,
    eattrs: HashMap<(T, T), Z>,
}

impl<T, X, Y, Z> Default for AttributesMap<T, X, Y, Z>
where
    T: Vertex,
    X: Debug,
    Y: Debug,
    Z: Debug,
{
    fn default() -> Self {
        Self {
            gattrs: None,
            vattrs: HashMap::new(),
            eattrs: HashMap::new(),
        }
    }
}

impl<T, X, Y, Z> WithAttributes<T> for AttributesMap<T, X, Y, Z>
where
    T: Vertex,
    X: Debug,
    Y: Debug,
    Z: Debug,
{
    type GraphAttributes = X;
    type VertexAttributes = Y;
    type EdgeAttributes = Z;

    fn new_with_attributes<I, J>(x: Self::GraphAttributes, y: I, z: J) -> Self
    where
        I: IntoIterator<Item = (T, Self::VertexAttributes)>,
        J: IntoIterator<Item = ((T, T), Self::EdgeAttributes)>,
    {
        Self {
            gattrs: Some(x),
            vattrs: y.into_iter().collect(),
            eattrs: z.into_iter().collect(),
        }
    }

    fn has_graph_attrs(&self) -> bool {
        self.gattrs.is_some()
    }

    fn get_graph_attrs(&self) -> Result<&Self::GraphAttributes, Error<T>> {
        self.gattrs.as_ref().ok_or(Error::GraphAttributesNotDefined())
    }

    fn get_mut_graph_attrs(&mut self) -> Result<&mut Self::GraphAttributes, Error<T>> {
        self.gattrs.as_mut().ok_or(Error::GraphAttributesNotDefined())
    }

    fn set_graph_attrs(&mut self, x: Self::GraphAttributes) {
        self.gattrs = Some(x);
    }

    fn unset_graph_attrs(&mut self) -> Result<Self::GraphAttributes, Error<T>> {
        self.gattrs.take().ok_or(Error::GraphAttributesNotDefined())
    }

    fn has_vertex_attrs(&self, x: &T) -> bool {
        self.vattrs.contains_key(x)
    }

    fn get_vertex_attrs(&self, x: &T) -> Result<&Self::VertexAttributes, Error<T>> {
        self.vattrs.get(x).ok_or_else(|| Error::VertexAttributesNotDefined(x.clone()))
    }

    fn get_mut_vertex_attrs(&mut self, x: &T) -> Result<&mut Self::VertexAttributes, Error<T>> {
        self.vattrs
            .get_mut(x)
            .ok_or_else(|| Error::VertexAttributesNotDefined(x.clone()))
    }

    fn set_vertex_attrs(&mut self, x: &T, y: Self::VertexAttributes) {
        self.vattrs.insert(x.clone(), y);
    }

    fn unset_vertex_attrs(&mut self, x: &T) -> Result<Self::VertexAttributes, Error<T>> {
        self.vattrs
            .remove(x)
            .ok_or_else(|| Error::VertexAttributesNotDefined(x.clone()))
    }

    fn has_edge_attrs(&self, x: &T, y: &T) -> bool {
        self.eattrs.contains_key(&(x.clone(), y.clone()))
    }

    fn get_edge_attrs(&self, x: &T, y: &T) -> Result<&Self::EdgeAttributes, Error<T>> {
        self.eattrs
            .get(&(x.clone(), y.clone()))
            .ok_or_else(|| Error::EdgeAttributesNotDefined(x.clone(), y.clone()))
    }

    fn get_mut_edge_attrs(&mut self, x: &T, y: &T) -> Result<&mut Self::EdgeAttributes, Error<T>> {
        self.eattrs
            .get_mut(&(x.clone(), y.clone()))
            .ok_or_else(|| Error::EdgeAttributesNotDefined(x.clone(), y.clone()))
    }

    fn set_edge_attrs(&mut self, x: &T, y: &T, z: Self::EdgeAttributes) {
        self.eattrs.insert((x.clone(), y.clone()), z);
    }

    fn unset_edge_attrs(&mut self, x: &T, y: &T) -> Result<Self::EdgeAttributes, Error<T>> {
        self.eattrs
            .remove(&(x.clone(), y.clone()))
            .ok_or_else(|| Error::EdgeAttributesNotDefined(x.clone(), y.clone()))
    }
}
