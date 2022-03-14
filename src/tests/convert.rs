#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::traits::{Convert, From};
    use crate::types::Error;
    use crate::types::{AdjacencyList, EdgeList};
    use itertools::Itertools;
    use ndarray::Array2;
    use sprs::TriMat;

    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    #[test]
    fn edge_list<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32> + From,
    {
        let g = T::from_edges(E);
        assert_eq!(g.edge_list(), EdgeList::from(E));

        Ok(())
    }

    #[test]
    fn adjacency_list<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32> + From,
    {
        let g = T::from_edges(E);
        let mut a = AdjacencyList::new();
        for (x, y) in E {
            a.entry(x).or_default().insert(y);
        }
        assert_eq!(g.adjacency_list(), a);

        Ok(())
    }

    #[test]
    fn dense_adjacency_matrix<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32> + From,
    {
        let g = T::from_edges(E);
        let mut a = Array2::from_elem((8, 8), false);
        for (x, y) in E {
            a[(x as usize, y as usize)] = true;
        }
        assert_eq!(g.dense_adjacency_matrix(), a);

        Ok(())
    }

    #[test]
    fn sparse_adjacency_matrix<T>() -> Result<(), Error<i32>>
    where
        T: Convert<Vertex = i32> + From,
    {
        let g = T::from_edges(E);
        let (x, y): (Vec<_>, Vec<_>) = E
            .iter()
            .cloned()
            .sorted()
            .map(|(x, y)| (x as usize, y as usize))
            .unzip();
        let v = std::iter::repeat(true).take(g.size()).collect();
        let order = g.order();
        assert_eq!(
            g.sparse_adjacency_matrix(),
            TriMat::from_triplets((order, order), x, y, v)
        );

        Ok(())
    }

    #[instantiate_tests(<DirectedAdjacencyListGraph<i32>>)]
    mod adjacency_list_graph {}
}
