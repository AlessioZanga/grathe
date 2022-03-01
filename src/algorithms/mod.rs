mod simple_paths;
pub use simple_paths::*;

mod traversal;
pub use traversal::*;

macro_rules! impl_algorithms_directed {
    ($graph:ident) => {
        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<(&'a $graph<T, U>, &'a T, &'a T)> for $crate::algorithms::AllSimplePaths<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn from((g, x, y): (&'a $graph<T, U>, &'a T, &'a T)) -> Self {
                Self::new(g, x, y, $graph::<T, U>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<&'a $graph<T, U>> for $crate::algorithms::AllSimpleCycles<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn from(g: &'a $graph<T, U>) -> Self {
                Self::new(g, $graph::<T, U>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<&'a $graph<T, U>> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, U>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, U>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<(&'a $graph<T, U>, &'a T)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, U>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, U>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<&'a $graph<T, U>> for $crate::algorithms::DepthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, U>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, U>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<(&'a $graph<T, U>, &'a T)> for $crate::algorithms::DepthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, U>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, U>::children_iter,
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
        impl<'a, T, U> From<(&'a $graph<T, U>, &'a T, &'a T)> for $crate::algorithms::AllSimplePaths<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn from((g, x, y): (&'a $graph<T, U>, &'a T, &'a T)) -> Self {
                Self::new(g, x, y, $graph::<T, U>::neighbors_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<&'a $graph<T, U>> for $crate::algorithms::AllSimpleCycles<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            fn from(g: &'a $graph<T, U>) -> Self {
                todo!()
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<&'a $graph<T, U>> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, U>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, U>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<(&'a $graph<T, U>, &'a T)> for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, U>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, U>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<&'a $graph<T, U>> for $crate::algorithms::DepthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, U>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, U>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, U> From<(&'a $graph<T, U>, &'a T)> for $crate::algorithms::DepthFirstSearch<'a, $graph<T, U>>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, U>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, U>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }
    };
}

pub(crate) use impl_algorithms_directed;
pub(crate) use impl_algorithms_undirected;
