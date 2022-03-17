use crate::traits::WithAttributes;
use crate::types::{Error, Vertex};
use std::collections::HashMap;
use std::fmt::Debug;

/// A simple hashmap-based attribute manager.
#[derive(Debug)]
pub struct AttributesMap<V, X, Y, Z>
where
    V: Vertex,
    X: Debug,
    Y: Debug,
    Z: Debug,
{
    gattrs: Option<X>,
    vattrs: HashMap<V, Y>,
    eattrs: HashMap<(V, V), Z>,
}

impl<V, X, Y, Z> Default for AttributesMap<V, X, Y, Z>
where
    V: Vertex,
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

impl<V, X, Y, Z> WithAttributes<V> for AttributesMap<V, X, Y, Z>
where
    V: Vertex,
    X: Debug,
    Y: Debug,
    Z: Debug,
{
    type GraphAttributes = X;
    type VertexAttributes = Y;
    type EdgeAttributes = Z;

    fn new_with_attributes<I, J>(x: Self::GraphAttributes, y: I, z: J) -> Self
    where
        I: IntoIterator<Item = (V, Self::VertexAttributes)>,
        J: IntoIterator<Item = ((V, V), Self::EdgeAttributes)>,
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

    fn get_graph_attrs(&self) -> Result<&Self::GraphAttributes, Error<V>> {
        self.gattrs.as_ref().ok_or(Error::GraphAttributesNotDefined())
    }

    fn get_mut_graph_attrs(&mut self) -> Result<&mut Self::GraphAttributes, Error<V>> {
        self.gattrs.as_mut().ok_or(Error::GraphAttributesNotDefined())
    }

    fn set_graph_attrs(&mut self, x: Self::GraphAttributes) {
        self.gattrs = Some(x);
    }

    fn unset_graph_attrs(&mut self) -> Result<Self::GraphAttributes, Error<V>> {
        self.gattrs.take().ok_or(Error::GraphAttributesNotDefined())
    }

    fn has_vertex_attrs(&self, x: &V) -> bool {
        self.vattrs.contains_key(x)
    }

    fn get_vertex_attrs(&self, x: &V) -> Result<&Self::VertexAttributes, Error<V>> {
        self.vattrs
            .get(x)
            .ok_or_else(|| Error::VertexAttributesNotDefined(x.clone()))
    }

    fn get_mut_vertex_attrs(&mut self, x: &V) -> Result<&mut Self::VertexAttributes, Error<V>> {
        self.vattrs
            .get_mut(x)
            .ok_or_else(|| Error::VertexAttributesNotDefined(x.clone()))
    }

    fn set_vertex_attrs(&mut self, x: &V, y: Self::VertexAttributes) {
        self.vattrs.insert(x.clone(), y);
    }

    fn unset_vertex_attrs(&mut self, x: &V) -> Result<Self::VertexAttributes, Error<V>> {
        self.vattrs
            .remove(x)
            .ok_or_else(|| Error::VertexAttributesNotDefined(x.clone()))
    }

    fn has_edge_attrs(&self, x: &V, y: &V) -> bool {
        self.eattrs.contains_key(&(x.clone(), y.clone()))
    }

    fn get_edge_attrs(&self, x: &V, y: &V) -> Result<&Self::EdgeAttributes, Error<V>> {
        self.eattrs
            .get(&(x.clone(), y.clone()))
            .ok_or_else(|| Error::EdgeAttributesNotDefined(x.clone(), y.clone()))
    }

    fn get_mut_edge_attrs(&mut self, x: &V, y: &V) -> Result<&mut Self::EdgeAttributes, Error<V>> {
        self.eattrs
            .get_mut(&(x.clone(), y.clone()))
            .ok_or_else(|| Error::EdgeAttributesNotDefined(x.clone(), y.clone()))
    }

    fn set_edge_attrs(&mut self, x: &V, y: &V, z: Self::EdgeAttributes) {
        self.eattrs.insert((x.clone(), y.clone()), z);
    }

    fn unset_edge_attrs(&mut self, x: &V, y: &V) -> Result<Self::EdgeAttributes, Error<V>> {
        self.eattrs
            .remove(&(x.clone(), y.clone()))
            .ok_or_else(|| Error::EdgeAttributesNotDefined(x.clone(), y.clone()))
    }
}
