/// Storage delegation and graph trait implementation for graphs.
#[macro_export]
macro_rules! impl_graph_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> std::ops::BitAnd for &$graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            type Output = $graph<T>;

            /// Intersection of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the intersection graph $G \cap H$ is defined as:
            ///
            /// $$ G \cap H \implies V(G) \cap V(H) \wedge E(G) \cap E(H) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn bitand(self, rhs: Self) -> Self::Output {
                todo!()
            }
        }
        impl<T> std::ops::BitOr for &$graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            type Output = $graph<T>;

            /// Union of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the union graph $G \cup H$ is defined as:
            ///
            /// $$ G \cup H \implies V(G) \cup V(H) \wedge E(G) \cup E(H) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn bitor(self, rhs: Self) -> Self::Output {
                todo!()
            }
        }
        impl<T> std::ops::BitXor for &$graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            type Output = $graph<T>;

            /// Symmetric difference of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the symmetric difference graph $G \thinspace \Delta \thinspace H$ is defined as:
            ///
            /// $$ G \thinspace \Delta \thinspace H \implies V(G) \thinspace \Delta \thinspace V(H) \wedge E(G) \thinspace \Delta \thinspace E(H) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn bitxor(self, rhs: Self) -> Self::Output {
                todo!()
            }
        }
        impl<T> std::ops::Sub for &$graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            type Output = $graph<T>;

            /// Difference of two graphs.
            ///
            /// Let $G$ and $H$ be two graphs, then the difference graph $G - H$ is defined as:
            ///
            /// $$ G - H \implies V(G) - V(H) \wedge E(G) - E(H) $$
            ///
            /// Ignores additional attributes (for now).
            ///
            fn sub(self, rhs: Self) -> Self::Output {
                todo!()
            }
        }

        impl<T> $crate::graphs::GraphTrait for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn union(&self, other: &Self) -> Self {
                self | other
            }

            fn intersection(&self, other: &Self) -> Self {
                self & other
            }

            fn symmetric_difference(&self, other: &Self) -> Self {
                self ^ other
            }

            fn difference(&self, other: &Self) -> Self {
                self - other
            }

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

/// Storage delegation and graph trait implementation for undirected graphs.
#[macro_export]
macro_rules! impl_ungraph_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> PartialEq for $graph<T>
        where
            T: $crate::types::VertexTrait,
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

        impl<T> Eq for $graph<T> where T: $crate::types::VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: $crate::types::VertexTrait,
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
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        impl<T> $crate::storages::StorageTrait for $graph<T>
        where
            T: $crate::types::VertexTrait,
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

            delegate::delegate! {
                to self.data {
                    fn clear(&mut self);
                    fn capacity(&self) -> usize;
                    fn reserve(&mut self, additional: usize);
                    fn shrink_to(&mut self, min_capacity: usize);
                    fn shrink_to_fit(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn $crate::types::EdgeIterator<'a, Self::Vertex> + 'a>;
                    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn order(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, $crate::errors::Error<Self::Vertex>>;
                }
            }

            fn size(&self) -> usize {
                // De-symmetrize edge set for correct size computation
                self.edges_iter().filter(|(x, y)| x <= y).count()
            }

            fn add_vertex<U>(&mut self, x: &U) -> Result<Self::Vertex, $crate::errors::Error<Self::Vertex>> where U: Eq + Clone + Into<Self::Vertex> {
                // Add vertex into the graph.
                let x = self.data.add_vertex(x)?;
                // Add associated attribute map.
                self.vattrs.insert(x.clone(), Default::default());
                // Return successfully.
                Ok(x)
            }

            fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>> {
                // Delete vertex from the graph.
                self.data.del_vertex(x)?;
                // Delete associated attribute map.
                self.vattrs.remove(x);
                // Return successfully.
                Ok(())
            }

            fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>> {
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

            fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>> {
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

        impl<T> $crate::directions::DirectionalTrait for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn is_directed(&self) -> bool {
                false
            }

            fn is_partially_directed(&self) -> bool {
                false
            }
        }

        impl_graph_trait!($graph, $storage);
    };
}

/// Storage delegation and graph trait implementation for directed graphs.
#[macro_export]
macro_rules! impl_digraph_trait {
    ($graph:ident, $storage:ident) => {
        impl<T> PartialEq for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn eq(&self, other: &Self) -> bool {
                self.data.eq(&other.data)
            }
        }

        impl<T> Eq for $graph<T> where T: $crate::types::VertexTrait {}

        impl<T> PartialOrd for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }

        impl<T> $crate::storages::StorageTrait for $graph<T>
        where
            T: $crate::types::VertexTrait,
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

            delegate::delegate! {
                to self.data {
                    fn clear(&mut self);
                    fn capacity(&self) -> usize;
                    fn reserve(&mut self, additional: usize);
                    fn shrink_to(&mut self, min_capacity: usize);
                    fn shrink_to_fit(&mut self);
                    fn vertices_iter<'a>(&'a self) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn edges_iter<'a>(&'a self) -> Box<dyn $crate::types::EdgeIterator<'a, Self::Vertex> + 'a>;
                    fn adjacents_iter<'a>(&'a self, x: &'a Self::Vertex) -> Box<dyn $crate::types::VertexIterator<'a, Self::Vertex> + 'a>;
                    fn order(&self) -> usize;
                    fn size(&self) -> usize;
                    fn has_vertex(&self, x: &Self::Vertex) -> bool;
                    fn add_vertex<U>(&mut self, x: &U) -> Result<Self::Vertex, $crate::errors::Error<Self::Vertex>> where U: Eq + Clone + Into<Self::Vertex>;
                    fn del_vertex(&mut self, x: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>>;
                    fn has_edge(&self, x: &Self::Vertex, y: &Self::Vertex) -> Result<bool, $crate::errors::Error<Self::Vertex>>;
                    fn add_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>>;
                    fn del_edge(&mut self, x: &Self::Vertex, y: &Self::Vertex) -> Result<(), $crate::errors::Error<Self::Vertex>>;
                }
            }
        }

        impl<T> $crate::directions::DirectionalTrait for $graph<T>
        where
            T: $crate::types::VertexTrait,
        {
            fn is_directed(&self) -> bool {
                true
            }

            fn is_partially_directed(&self) -> bool {
                false
            }
        }

        impl_graph_trait!($graph, $storage);
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
