mod simple_paths;
pub use simple_paths::*;

mod traversal;
pub use traversal::*;

macro_rules! impl_algorithms_directed {
    ($graph:ident) => {
        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<(&'a $graph<T, X, Y, Z>, &'a T, &'a T)>
            for $crate::algorithms::AllSimplePaths<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            fn from((g, x, y): (&'a $graph<T, X, Y, Z>, &'a T, &'a T)) -> Self {
                Self::new(g, x, y, $graph::<T, X, Y, Z>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<&'a $graph<T, X, Y, Z>>
            for $crate::algorithms::AllSimpleCycles<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            fn from(g: &'a $graph<T, X, Y, Z>) -> Self {
                Self::new(g, $graph::<T, X, Y, Z>::children_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<&'a $graph<T, X, Y, Z>>
            for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, X, Y, Z>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, X, Y, Z>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<(&'a $graph<T, X, Y, Z>, &'a T)>
            for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, X, Y, Z>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, X, Y, Z>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<&'a $graph<T, X, Y, Z>>
            for $crate::algorithms::DepthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, X, Y, Z>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, X, Y, Z>::children_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<(&'a $graph<T, X, Y, Z>, &'a T)>
            for $crate::algorithms::DepthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, X, Y, Z>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, X, Y, Z>::children_iter,
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
        impl<'a, T, X, Y, Z> From<(&'a $graph<T, X, Y, Z>, &'a T, &'a T)>
            for $crate::algorithms::AllSimplePaths<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            fn from((g, x, y): (&'a $graph<T, X, Y, Z>, &'a T, &'a T)) -> Self {
                Self::new(g, x, y, $graph::<T, X, Y, Z>::neighbors_iter)
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<&'a $graph<T, X, Y, Z>>
            for $crate::algorithms::AllSimpleCycles<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            fn from(g: &'a $graph<T, X, Y, Z>) -> Self {
                todo!()
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<&'a $graph<T, X, Y, Z>>
            for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, X, Y, Z>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, X, Y, Z>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<(&'a $graph<T, X, Y, Z>, &'a T)>
            for $crate::algorithms::BreadthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, X, Y, Z>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, X, Y, Z>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<&'a $graph<T, X, Y, Z>>
            for $crate::algorithms::DepthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, without a source vertex.
            ///
            /// The first vertex of the vertex set is chosen as source vertex.
            ///
            fn from(g: &'a $graph<T, X, Y, Z>) -> Self {
                Self::new(
                    g,
                    None,
                    $graph::<T, X, Y, Z>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }

        // TODO: Once `min_specialization` will be stabilized,
        // replace this with blanket `From` implementation.
        impl<'a, T, X, Y, Z> From<(&'a $graph<T, X, Y, Z>, &'a T)>
            for $crate::algorithms::DepthFirstSearch<'a, $graph<T, X, Y, Z>>
        where
            T: $crate::types::VertexTrait,
            X: Default + std::fmt::Debug,
            Y: Default + std::fmt::Debug,
            Z: Default + std::fmt::Debug,
        {
            /// Builds a search object from a given graph, with a source vertex.
            ///
            /// # Panics
            ///
            /// Panics if the source vertex is not in the graph.
            ///
            fn from((g, x): (&'a $graph<T, X, Y, Z>, &'a T)) -> Self {
                Self::new(
                    g,
                    Some(x),
                    $graph::<T, X, Y, Z>::neighbors_iter,
                    $crate::algorithms::Traversal::Tree,
                )
            }
        }
    };
}

pub(crate) use impl_algorithms_directed;
pub(crate) use impl_algorithms_undirected;
