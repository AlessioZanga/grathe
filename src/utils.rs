/// Storage delegation and graph trait implementation for graphs.
#[macro_export]
macro_rules! impl_graph_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> GraphTrait for $graph<T>
        where
            T: VertexTrait,
        {
            fn as_vertex_attrs(&self) -> &Attributes<Self::Vertex> {
                &self.vattrs
            }

            fn has_vertex_attr(&self, x: &Self::Vertex, k: &str) -> Result<bool, Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get(x) {
                    // If vertex is not defined return error.
                    None => Err(Error::VertexNotDefined(x.clone())),
                    // Otherwise, check if vertex has any attribute or an attribute with given key.
                    Some(attrs) => Ok(attrs.contains_key(k)),
                }
            }

            fn get_vertex_attr<'a>(&'a self, x: &'a Self::Vertex, k: &str) -> Result<&'a dyn Any, Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get(x) {
                    // If vertex is not defined return error.
                    None => Err(Error::VertexNotDefined(x.clone())),
                    // Otherwise, check if vertex has an attribute with given key.
                    Some(attrs) => match attrs.get(k) {
                        // If attribute key is not defined return error.
                        None => Err(Error::VertexAttributeNotDefined(x.clone(), k.to_string())),
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
            ) -> Result<(), Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get_mut(x) {
                    // Vertex is not defined, return error.
                    None => Err(Error::VertexNotDefined(x.clone())),
                    // Otherwise, check if vertex has any attribute or an attribute with given key.
                    Some(attrs) => {
                        // Insert attribute in map.
                        attrs.insert(k.to_string(), Box::new(v));
                        // Return successfully.
                        Ok(())
                    }
                }
            }

            fn unset_vertex_attr(&mut self, x: &Self::Vertex, k: &str) -> Result<Box<dyn Any>, Error<Self::Vertex>> {
                // Check if vertex is valid.
                match self.vattrs.get_mut(x) {
                    // If vertex is not defined return error.
                    None => Err(Error::VertexNotDefined(x.clone())),
                    // Otherwise, try to remove the attribute with given key.
                    Some(attrs) => match attrs.remove(k) {
                        // There is no such attribute, return error.
                        None => Err(Error::VertexAttributeNotDefined(x.clone(), k.to_string())),
                        // Return successfully.
                        Some(v) => Ok(v),
                    },
                }
            }

            fn as_edge_attrs(&self) -> &Attributes<(Self::Vertex, Self::Vertex)> {
                &self.eattrs
            }

            fn has_edge_attr(&self, x: &Self::Vertex, y: &Self::Vertex, k: &str) -> Result<bool, Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.eattrs.get(&(x.clone(), y.clone())) {
                    // If edge is not defined return error.
                    None => Err(Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise, check if edge has any attribute or an attribute with given key.
                    Some(attrs) => Ok(attrs.contains_key(k)),
                }
            }

            fn get_edge_attr<'a>(
                &'a self,
                x: &Self::Vertex,
                y: &Self::Vertex,
                k: &str,
            ) -> Result<&'a dyn Any, Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.eattrs.get(&(x.clone(), y.clone())) {
                    // If edge is not defined return error.
                    None => Err(Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise, check if edge has an attribute with given key.
                    Some(vattrs) => match vattrs.get(k) {
                        // If attribute key is not defined return error.
                        None => Err(Error::EdgeAttributeNotDefined(
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
            ) -> Result<(), Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.eattrs.get_mut(&(x.clone(), y.clone())) {
                    // Edge is not defined, return error.
                    None => Err(Error::EdgeNotDefined(x.clone(), y.clone())),
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
            ) -> Result<Box<dyn Any>, Error<Self::Vertex>> {
                // Check if edge is valid.
                match self.vattrs.get_mut(x) {
                    // If edge is not defined return error.
                    None => Err(Error::EdgeNotDefined(x.clone(), y.clone())),
                    // Otherwise, try to remove the attribute with given key.
                    Some(attrs) => match attrs.remove(k) {
                        // There is no such attribute, return error.
                        None => Err(Error::EdgeAttributeNotDefined(
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

/// Storage delegation and graph trait implementation for undirected graphs.
#[macro_export]
macro_rules! impl_ungraph_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> PartialEq for $graph<T>
        where
            T: VertexTrait,
        {
            /// Equality operator.
            ///
            /// Let $G$ and $H$ be two graphs, then $G$ is equal to $H$ if and only if
            /// they have the same vertex set $V$ and the same edge set $E$:
            ///
            /// $$G = H \iff V(G) = V(H) \wedge E(G) = E(H)$$
            ///
            /// # Complexity
            ///
            /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
            ///
            /// # Examples
            ///
            /// ```
            /// use grathe::graphs::Graph;
            /// use grathe::storages::StorageTrait;
            ///
            /// let g = Graph::new();
            /// let h = Graph::new();
            ///
            /// assert_eq!(g, h);
            /// ```
            ///
            fn eq(&self, other: &Self) -> bool {
                self.data.eq(&other.data)
            }
        }

        impl<T> Eq for $graph<T> where T: VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: VertexTrait,
        {
            /// Comparable operator.
            ///
            /// Let $G$ and $H$ be two graphs, then $G$ is partially comparable to $H$
            /// if and only if they have the partially comparable vertex set $V$ and
            /// partially comparable edge set $E$:
            ///
            /// $$G \leq H \iff V(G) \leq V(H) \wedge E(G) \leq E(H)$$
            ///
            /// # Complexity
            ///
            /// $O(|V| + |E|)$ - Linear in the order and size of the graph.
            ///
            /// # Examples
            ///
            /// ```
            /// use all_asserts::*;
            /// use grathe::prelude::*;
            ///
            /// let g = Graph::new();
            /// let h = Graph::new();
            ///
            /// assert_le!(g, h);
            /// ```
            ///
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        impl<T> StorageTrait for $graph<T>
        where
            T: VertexTrait,
        {
            type Vertex = T;

            type Storage = $storage<T>;

            fn new() -> Self {
                Default::default()
            }

            fn storage(&self) -> &Self::Storage {
                &self.data
            }

            fn with_capacity(capacity: usize) -> Self {
                Self {
                    data: Self::Storage::with_capacity(capacity),
                    vattrs: Default::default(),
                    eattrs: Default::default(),
                }
            }

            delegate! {
                to self.data {
                    fn clear(&mut self);
                    fn capacity(&self) -> usize;
                    fn reserve(&mut self, additional: usize);
                    fn shrink_to(&mut self, min_capacity: usize);
                    fn shrink_to_fit(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a>;
                    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;
                    fn order(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, Error<Self::Vertex>>;
                }
            }

            fn size(&self) -> usize {
                // De-symmetrize edge set for correct size computation
                self.edges_iter().filter(|(x, y)| x <= y).count()
            }

            fn add_vertex<U>(&mut self, x: &U) -> Result<Self::Vertex, Error<Self::Vertex>> where U: Eq + Clone + Into<Self::Vertex> {
                // Add vertex into the graph.
                let x = self.data.add_vertex(x)?;
                // Add associated attribute map.
                self.vattrs.insert(x.clone(), Default::default());
                // Return successfully.
                Ok(x)
            }

            fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
                // Delete vertex from the graph.
                self.data.del_vertex(x)?;
                // Delete associated attribute map.
                self.vattrs.remove(x);
                // Return successfully.
                Ok(())
            }

            fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
                // Add edge (y, x)
                self.data.add_edge(y, x)?;
                // Add edge (x, y)
                if x != y {
                    self.data.add_edge(x, y)?;
                }
                // Add associated attribute map.
                self.eattrs.insert((x.clone(), y.clone()), Default::default());
                // Return successfully.
                Ok(())
            }

            fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>> {
                // Del edge (y, x)
                self.data.del_edge(y, x)?;
                // Del edge (x, y)
                if x != y {
                    self.data.del_edge(x, y)?;
                }
                // Delete associated attribute map.
                self.eattrs.remove(&(x.clone(), y.clone()));
                // Return successfully.
                Ok(())
            }
        }

        impl<T> DirectionalTrait for $graph<T>
        where
            T: VertexTrait,
        {
            fn is_directed(&self) -> bool {
                false
            }

            fn is_partially_directed(&self) -> bool {
                false
            }
        }

        impl_graph_trait!($graph, $storage);

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T>>
        where
            T: VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::neighbors_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::DepthFirstSearch<'a, $graph<T>>
        where
            T: VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::neighbors_iter)
            }
        }
    };
}

/// Storage delegation and graph trait implementation for directed graphs.
#[macro_export]
macro_rules! impl_digraph_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> PartialEq for $graph<T>
        where
            T: VertexTrait,
        {
            fn eq(&self, other: &Self) -> bool {
                self.data.eq(&other.data)
            }
        }

        impl<T> Eq for $graph<T> where T: VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: VertexTrait,
        {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        impl<T> StorageTrait for $graph<T>
        where
            T: VertexTrait,
        {
            type Vertex = T;

            type Storage = $storage<T>;

            fn new() -> Self {
                Default::default()
            }

            fn storage(&self) -> &Self::Storage {
                &self.data
            }

            fn with_capacity(capacity: usize) -> Self {
                Self {
                    data: Self::Storage::with_capacity(capacity),
                    vattrs: Default::default(),
                    eattrs: Default::default(),
                }
            }

            delegate! {
                to self.data {
                    fn clear(&mut self);
                    fn capacity(&self) -> usize;
                    fn reserve(&mut self, additional: usize);
                    fn shrink_to(&mut self, min_capacity: usize);
                    fn shrink_to_fit(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn EdgeIterator<'a, Self::Vertex> + 'a>;
                    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn VertexIterator<'a, Self::Vertex> + 'a>;
                    fn order(&self) -> usize;
                    fn size(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn add_vertex<U>(&mut self, x: &U) -> Result<Self::Vertex, Error<Self::Vertex>> where U: Eq + Clone + Into<Self::Vertex>;
                    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, Error<Self::Vertex>>;
                    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;
                    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), Error<Self::Vertex>>;
                }
            }
        }

        impl<T> DirectionalTrait for $graph<T>
        where
            T: VertexTrait,
        {
            fn is_directed(&self) -> bool {
                true
            }

            fn is_partially_directed(&self) -> bool {
                false
            }
        }

        impl_graph_trait!($graph, $storage);

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T>>
        where
            T: VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T> From<(&'a $graph<T>, &'a T)> for $crate::algorithms::DepthFirstSearch<'a, $graph<T>>
        where
            T: VertexTrait,
        {
            fn from((g, x): (&'a $graph<T>, &'a T)) -> Self {
                Self::new(g, x, $graph::<T>::children_iter)
            }
        }
    };
}

/// Partial compares sets.
#[macro_export]
macro_rules! partial_cmp_sets {
    ($a:ident, $b:ident) => {
        if $a.len() == $b.len() && $a.eq(&$b) {
            Some(Ordering::Equal)
        } else if $a.len() < $b.len() && $a.is_subset(&$b) {
            Some(Ordering::Less)
        } else if $a.len() > $b.len() && $a.is_superset(&$b) {
            Some(Ordering::Greater)
        } else {
            None
        }
    };
}
