macro_rules! impl_partial_ord {
    ($graph: ident) => {
        impl<V, A> PartialOrd for $graph<V, A>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Comparable operator.
            ///
            /// Let $G$ and $H$ be two graphs, then $G$ is partially comparable to $H$
            /// if and only if they have the partially comparable vertex set $V$ and
            /// partially comparable edge set $E$:
            ///
            /// $$G \leq H \equiv V(G) \leq V(H) \wedge E(G) \leq E(H)$$
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
            /// let g = Graph::null();
            /// let h = Graph::null();
            ///
            /// assert_le!(g, h);
            /// ```
            ///
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                // Collect vertex sets for comparison.
                let a: std::collections::HashSet<_> = self.vertices_iter().collect();
                let b: std::collections::HashSet<_> = other.vertices_iter().collect();
                // Partial ordering of vertex sets.
                let vertices: Option<std::cmp::Ordering> = crate::utils::partial_cmp_sets!(a, b);
                // If vertices are comparable.
                if let Some(vertices) = vertices {
                    // Collect edge sets for comparison.
                    let a: std::collections::HashSet<_> = self.edges_iter().collect();
                    let b: std::collections::HashSet<_> = other.edges_iter().collect();
                    // Partial ordering of edge sets.
                    let edges: Option<std::cmp::Ordering> = $crate::utils::partial_cmp_sets!(a, b);
                    // If edges are comparable.
                    if let Some(edges) = edges {
                        // If vertices are equal,
                        // then order is determined by edges.
                        if matches!(vertices, std::cmp::Ordering::Equal) {
                            return Some(edges);
                        }
                        // If vertices are different but edges are equal,
                        // then order is determined by vertices.
                        if matches!(edges, std::cmp::Ordering::Equal) {
                            return Some(vertices);
                        }
                        // If orders are coherent, then return the order.
                        if vertices == edges {
                            return Some(vertices);
                        }
                    }
                }
                // Otherwise, self and other are not comparable.
                None
            }
        }
    };
}

pub(crate) use impl_partial_ord;
