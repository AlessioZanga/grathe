#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::traits::Extend;
    use crate::types::Error;
    use all_asserts::*;

    #[test]
    fn extend_vertices<T>() -> Result<(), Error<i32>>
    where
        T: Extend<Vertex = i32>,
    {
        let mut g = T::null();

        // Extend graph with vertices.
        g.extend_vertices([0, 3, 1, 2])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 0);
        // Extending with existing vertices yields an error.
        assert_true!(g.extend_vertices([0]).is_err());
        Ok(())
    }

    #[test]
    fn extend_edges<T>() -> Result<(), Error<i32>>
    where
        T: Extend<Vertex = i32>,
    {
        let mut g = T::null();

        // Extend graph with edges.
        g.extend_edges([(0, 3), (1, 2)])?;
        assert_eq!(g.order(), 4);
        assert_eq!(g.size(), 2);

        // Extending with existing edges yields an error.
        assert_true!(g.extend_edges([(0, 3)]).is_err());
        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod adjacency_list_graph {}
}
