mod simple_paths;
pub use simple_paths::*;

mod traversal;
pub use traversal::*;

macro_rules! impl_algorithms_directed {
    ($graph:ident) => {
        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<(&'a $graph<V, A>, &'a V, &'a V)> for $crate::algorithms::AllSimplePaths<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            fn from((g, x, y): (&'a $graph<V, A>, &'a V, &'a V)) -> Self {
                Self::new(g, x, y, $graph::<V, A>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<&'a $graph<V, A>> for $crate::algorithms::AllSimpleCycles<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            fn from(g: &'a $graph<V, A>) -> Self {
                Self::new(g, $graph::<V, A>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<&'a $graph<V, A>> for $crate::algorithms::BreadthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<V, A>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<V, A>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<(&'a $graph<V, A>, &'a V)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<V, A>, &'a V)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<V, A>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<&'a $graph<V, A>> for $crate::algorithms::DepthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<V, A>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<V, A>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<(&'a $graph<V, A>, &'a V)> for $crate::algorithms::DepthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<V, A>, &'a V)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<V, A>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }
    };
}

macro_rules! impl_algorithms_undirected {
    ($graph:ident) => {
        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<(&'a $graph<V, A>, &'a V, &'a V)> for $crate::algorithms::AllSimplePaths<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            fn from((g, x, y): (&'a $graph<V, A>, &'a V, &'a V)) -> Self {
                Self::new(g, x, y, $graph::<V, A>::neighbors_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<&'a $graph<V, A>> for $crate::algorithms::AllSimpleCycles<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            fn from(g: &'a $graph<V, A>) -> Self {
                todo!()
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<&'a $graph<V, A>> for $crate::algorithms::BreadthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<V, A>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<V, A>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<(&'a $graph<V, A>, &'a V)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<V, A>, &'a V)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<V, A>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<&'a $graph<V, A>> for $crate::algorithms::DepthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<V, A>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<V, A>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, V, A> From<(&'a $graph<V, A>, &'a V)> for $crate::algorithms::DepthFirstSearch<'a, $graph<V, A>>
        where
            V: $crate::types::Vertex,
            A: $crate::traits::WithAttributes<V>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<V, A>, &'a V)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<V, A>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }
    };
}

pub(crate) use impl_algorithms_directed;
pub(crate) use impl_algorithms_undirected;
