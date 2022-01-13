#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::errors::Error;
    use crate::graphs::DirectedAdjacencyListGraph;
    use crate::traits::{Convert, Storage};
    use crate::types::{AdjacencyList, DenseAdjacencyMatrix, EdgeList, SparseAdjacencyMatrix};
    use itertools::Itertools;

    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    #[test]
    fn into_edge_list<T>() -> Result<(), Error<i32>>
    where
        T: Storage<Vertex = i32> + Convert,
    {
        let g = T::from_edges(&E);
        assert_eq!(g.into_edge_list(), EdgeList::from(E));

        Ok(())
    }

    #[test]
    fn into_adjacency_list<T>() -> Result<(), Error<i32>>
    where
        T: Storage<Vertex = i32> + Convert,
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
        T: Storage<Vertex = i32> + Convert,
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
        T: Storage<Vertex = i32> + Convert,
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
    mod directed_adjacency_list_graph {}
}

#[cfg(test)]
#[generic_tests::define]
mod dot {
    use crate::errors::Error;
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::traits::{Base, Convert};
    use std::path::Path;
    use tempfile::NamedTempFile;

    // Set DOT string
    const DOT: &str = "graph {\n\t\"A\";\n\t\"B\";\n\t\"A\" -- \"B\";\n}\n";

    #[test]
    fn from_dot<T>() -> Result<(), Error<String>>
    where
        T: Base<Vertex = String> + Convert,
    {
        // Read DOT file to string
        let dot = std::fs::read_to_string("src/tests/data/14.dot").unwrap();
        // Load graph using io function
        let g = crate::io::from_dot::<T>(&dot).unwrap().pop().unwrap();
        // Test from DOT string constructor
        assert_eq!(T::from_dot(&dot), Ok(g));

        Ok(())
    }

    #[test]
    fn into_dot<T>() -> Result<(), Error<String>>
    where
        T: Base<Vertex = String> + Convert,
    {
        // Init graph
        let mut g = T::new();
        let i = g.add_vertex(&"A")?;
        let j = g.add_vertex(&"B")?;
        g.add_edge(&i, &j)?;
        // Test
        assert_eq!(g.into_dot(), DOT);

        Ok(())
    }

    #[test]
    fn read_dot<T>() -> Result<(), Error<String>>
    where
        T: Base<Vertex = String> + Convert,
    {
        // Read DOT file to string
        let dot = Path::new("src/tests/data/14.dot");
        // Load graph using io function
        let g = T::read_dot(dot).unwrap();
        // Test from DOT string constructor
        assert_eq!(T::read_dot(dot), Ok(g));

        Ok(())
    }

    #[test]
    fn write_dot<T>() -> Result<(), Error<String>>
    where
        T: Base<Vertex = String> + Convert,
    {
        // Init graph
        let mut g = T::new();
        let i = g.add_vertex(&"A")?;
        let j = g.add_vertex(&"B")?;
        g.add_edge(&i, &j)?;
        // Get temporary file path
        let path = NamedTempFile::new().unwrap().into_temp_path();
        // Write to DOT file
        g.write_dot(&path);
        // Test
        assert_eq!(T::read_dot(&path), Ok(g));

        Ok(())
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<String>>)]
    mod undirected_adjacency_list_graph {}
}
