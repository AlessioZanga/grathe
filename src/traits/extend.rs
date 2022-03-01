use super::Storage;
use crate::types::Error;

/// The graph extend trait.
pub trait Extend: Storage {
    /// Extends graph with given vertices.
    ///
    /// Extends graph with given sequence of vertex identifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Extend graph with vertices.
    /// g.extend_vertices([0, 3, 1, 2])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 0);
    ///
    /// // Extending with existing vertices yields an error.
    /// assert_true!(g.extend_vertices([0]).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = Graphl::null();
    ///
    /// // Extend graph with vertices.
    /// g.extend_vertices(["0", "3", "1", "2"])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 0);
    ///
    /// // Extending with existing vertices yields an error.
    /// assert_true!(g.extend_vertices(["0"]).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn extend_vertices<I, V>(&mut self, iter: I) -> Result<(), Error<Self::Vertex>>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>,
    {
        // Get vertex iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Reserve additional capacity.
        self.reserve(lower);
        // Add vertex to the graph.
        for x in iter {
            self.add_vertex(x)?;
        }

        Ok(())
    }

    /// Extends graph with given edges.
    ///
    /// Extends graph with given sequence of edges identifiers.
    /// Non-existing vertices will be added as well.
    ///
    /// # Examples
    ///
    /// ```
    /// use all_asserts::*;
    /// use grathe::prelude::*;
    ///
    /// # fn main() -> Result<(), anyhow::Error> {
    /// // Build a null graph.
    /// let mut g = Graph::null();
    ///
    /// // Extend graph with edges.
    /// g.extend_edges([(0, 3), (1, 2)])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 2);
    ///
    /// // Extending with existing edges yields an error.
    /// assert_true!(g.extend_edges([(0, 3)]).is_err());
    ///
    /// // Build a null graph.
    /// let mut g = Graphl::null();
    ///
    /// // Extend graph with edges.
    /// g.extend_edges([("0", "3"), ("1", "2")])?;
    /// assert_eq!(g.order(), 4);
    /// assert_eq!(g.size(), 2);
    ///
    /// // Extending with existing edges yields an error.
    /// assert_true!(g.extend_edges([("0", "3")]).is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn extend_edges<I, V>(&mut self, iter: I) -> Result<(), Error<Self::Vertex>>
    where
        I: IntoIterator<Item = (V, V)>,
        V: Into<Self::Vertex>,
    {
        // Get edge iterator.
        let iter = iter.into_iter();
        // Get lower bound size hint.
        let (lower, _) = iter.size_hint();
        // Reserve additional capacity.
        self.reserve(lower);
        // Add edge to the graph.
        for (x, y) in iter {
            // Try to add vertex, ignore error if vertex already defined.
            let x = match self.add_vertex(x) {
                Err(Error::VertexAlreadyDefined(x)) | Ok(x) => x,
                Err(_) => unreachable!(),
            };
            // Try to add vertex, ignore error if vertex already defined.
            let y = match self.add_vertex(y) {
                Err(Error::VertexAlreadyDefined(y)) | Ok(y) => y,
                Err(_) => unreachable!(),
            };
            // Add vertex given new vertices.
            self.add_edge(&x, &y)?;
        }

        Ok(())
    }
}

macro_rules! impl_extend {
    ($graph:ident) => {
        impl<T, U> $crate::traits::Extend for $graph<T, U>
        where
            T: $crate::types::Vertex,
            U: $crate::traits::WithAttributes<T>,
        {
        }
    };
}

pub(crate) use impl_extend;
