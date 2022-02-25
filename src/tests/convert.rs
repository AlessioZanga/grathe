#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::types::Error;
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::traits::Convert;
    use crate::types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, SparseAdjacencyMatrix};
    use itertools::Itertools;

    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    #[test]
    fn into_edge_list<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        assert_eq!(g.into_edge_list(), EdgeList::from(E));

        Ok(())
    }

    #[test]
    fn into_adjacency_list<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        let mut a = AdjacencyList::new();
        for (x, y) in E {
            a.entry(x).or_default().insert(y);
        }
        assert_eq!(g.into_adjacency_list(), a);

        Ok(())
    }

    #[test]
    fn into_dense_adjacency_matrix<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        let mut a = DenseAdjacencyMatrix::from_elem((8, 8), false);
        for (x, y) in E {
            a[(x as usize, y as usize)] = true;
        }
        assert_eq!(g.into_dense_adjacency_matrix(), a);

        Ok(())
    }

    #[test]
    fn into_sparse_adjacency_matrix<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32>,
    {
        let g = T::from_edges(&E);
        let (x, y): (Vec<_>, Vec<_>) = E
            .iter()
            .cloned()
            .sorted()
            .map(|(x, y)| (x as usize, y as usize))
            .unzip();
        let v = std::iter::repeat(true).take(g.size()).collect();
        assert_eq!(
            g.into_sparse_adjacency_matrix(),
            SparseAdjacencyMatrix::from_triplets((g.order(), g.order()), x, y, v)
        );

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod adjacency_list_graph {}
}
