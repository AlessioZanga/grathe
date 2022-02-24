#[cfg(test)]
#[generic_tests::define]
mod integer {
    use crate::errors::Error;
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::traits::Base;
    use crate::{E, V};
    use all_asserts::*;

    #[test]
    fn degree_of_and_isolated_pendant<T>() -> Result<(), Error<i32>>
    where
        T: Base<Vertex = i32>,
    {
        let mut g = T::new();

        // Test for isolated vertex
        let i = g.add_vertex(&0)?;
        assert_eq!(g.degree_of(&i), 0);
        assert_true!(g.is_isolated_vertex(&i));

        // Test for pendant vertex
        let j = g.add_vertex(&1)?;
        g.add_edge(&i, &j)?;
        assert_eq!(g.degree_of(&i), 1);
        assert_true!(g.is_pendant_vertex(&i));

        Ok(())
    }

    #[test]
    fn subgraph<T>() -> Result<(), Error<i32>>
    where
        T: Base<Vertex = i32>,
    {
        let g = T::from_edges(&[(0, 1), (0, 2), (1, 2), (2, 3), (3, 3)]);

        // Build subgraph over 0, 2 and 3.
        let h = g.subgraph(&[0, 2, 3]);

        // Check if it is a subgraph.
        assert_le!(h, g);

        // Check if only selected vertices are preserved.
        assert_true!(V!(h).eq(&[0, 2, 3]));

        // Check if only associated edges are preserved.
        assert_true!(E!(h).eq([(&0, &2), (&2, &3), (&3, &3)]));

        Ok(())
    }

    #[test]
    fn is_subgraph<T>() -> Result<(), Error<i32>>
    where
        T: Base<Vertex = i32>,
    {
        let g = T::from_edges(&[(0, 1)]);
        let h = T::from_edges(&[(0, 1), (0, 2)]);

        assert_le!(g, h);
        assert_eq!((g <= h), g.is_subgraph(&h));

        Ok(())
    }

    #[test]
    fn is_supergraph<T>() -> Result<(), Error<i32>>
    where
        T: Base<Vertex = i32>,
    {
        let g = T::from_edges(&[(0, 1), (0, 2)]);
        let h = T::from_edges(&[(0, 1)]);

        assert_ge!(g, h);
        assert_eq!((g >= h), g.is_supergraph(&h));

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32, (), (), ()>>)]
    mod adjacency_list_graph {}
}
