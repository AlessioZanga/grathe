#[macro_export]
macro_rules! delegate_storage {
    ($graph:ident) => {
        use crate::errors::*;
        use delegate::delegate;
        use std::cmp::Ordering;

        impl<T> PartialEq for $graph<T>
        where
            T: VertexTrait,
        {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<T> Eq for $graph<T> where T: VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: VertexTrait,
        {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<T> Default for $graph<T>
        where
            T: VertexTrait,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<T> StorageTrait for $graph<T>
        where
            T: VertexTrait,
        {
            type Vertex = T;
            fn new() -> Self {
                Self {
                    0: Default::default()
                }
            }
            delegate! {
                to self.0 {
                    fn vertices_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Self::Vertex, Self::Vertex)> + 'a>;
                    fn adjacents_iter<'a>(
                        &'a self,
                        x: &Self::Vertex,
                    ) -> Result<Box<dyn Iterator<Item = Self::Vertex> + 'a>, Error<Self::Vertex>>;
                    fn as_vertices_labels(&self) -> &LabelMap<Self::Vertex>;
                    fn as_mut_vertices_labels(&mut self) -> &mut LabelMap<Self::Vertex>;
                    fn as_edges_labels(&self) -> &LabelMap<(Self::Vertex, Self::Vertex)>;
                    fn as_mut_edges_labels(&mut self) -> &mut LabelMap<(Self::Vertex, Self::Vertex)>;
                    fn order(&self) -> usize;
                    fn size(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn add_vertex(&mut self) -> Result<Self::Vertex, Error<Self::Vertex>>;
                    fn reserve_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>>;
                    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<Self::Vertex, Error<Self::Vertex>>;
                    fn has_edge(&self, e: &(Self::Vertex, Self::Vertex)) -> Result<bool, Error<Self::Vertex>>;
                    fn add_edge(
                        &mut self,
                        e: &(Self::Vertex, Self::Vertex),
                    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>>;
                    fn del_edge(
                        &mut self,
                        e: &(Self::Vertex, Self::Vertex),
                    ) -> Result<(Self::Vertex, Self::Vertex), Error<Self::Vertex>>;
                }
            }
        }
    };
}
