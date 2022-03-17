#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::{Convert, From};
    use itertools::Itertools;
    use ndarray::{arr2, Array2};
    use sprs::TriMat;
    use std::collections::{BTreeMap, BTreeSet};

    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    #[test]
    fn edge_list<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);

        assert_eq!(g.edge_list(), BTreeSet::<(i32, i32)>::from(E));
    }

    #[test]
    fn adjacency_list<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);
        let mut a = BTreeMap::<i32, BTreeSet<i32>>::new();
        for (x, y) in E {
            a.entry(x).or_default().insert(y);
        }

        assert_eq!(g.adjacency_list(), a);
    }

    #[test]
    fn dense_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);
        let mut a = Array2::from_elem((8, 8), false);
        for (x, y) in E {
            a[(x as usize, y as usize)] = true;
        }

        assert_eq!(g.dense_adjacency_matrix(), a);
    }

    #[test]
    fn sparse_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);

        let (x, y): (Vec<_>, Vec<_>) = E
            .iter()
            .cloned()
            .sorted()
            .map(|(x, y)| (x as usize, y as usize))
            .unzip();
        let v = std::iter::repeat(true).take(g.size()).collect();
        let n = g.order();
        assert_eq!(g.sparse_adjacency_matrix(), TriMat::from_triplets((n, n), x, y, v));
    }

    #[test]
    fn dense_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges([(1, 2), (1, 3), (1, 4), (3, 4)]);

        assert_eq!(
            g.dense_incidence_matrix(),
            arr2(&[[1, 1, 1, 0], [-1, 0, 0, 0], [0, -1, 0, 1], [0, 0, -1, -1],])
        );
    }

    #[test]
    fn sparse_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges([(1, 2), (1, 3), (1, 4), (3, 4)]);

        let n = g.order();
        assert_eq!(
            g.sparse_incidence_matrix(),
            TriMat::from_triplets(
                (n, n),
                vec![0, 1, 0, 2, 0, 3, 2, 3],
                vec![0, 0, 1, 1, 2, 2, 3, 3],
                vec![1, -1, 1, -1, 1, -1, 1, -1]
            )
        );
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::{Convert, From};
    use itertools::Itertools;
    use ndarray::{arr2, Array2};
    use sprs::TriMat;
    use std::collections::{BTreeMap, BTreeSet};

    const E: [(i32, i32); 5] = [(4, 3), (0, 1), (2, 3), (5, 6), (7, 2)];

    #[test]
    fn edge_list<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);

        assert_eq!(g.edge_list(), BTreeSet::<(i32, i32)>::from(E));
    }

    #[test]
    fn adjacency_list<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);
        let mut a = BTreeMap::<i32, BTreeSet<i32>>::new();
        for (x, y) in E {
            a.entry(x).or_default().insert(y);
        }

        assert_eq!(g.adjacency_list(), a);
    }

    #[test]
    fn dense_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);
        let mut a = Array2::from_elem((8, 8), false);
        for (x, y) in E {
            a[(x as usize, y as usize)] = true;
        }

        assert_eq!(g.dense_adjacency_matrix(), a);
    }

    #[test]
    fn sparse_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges(E);

        let (x, y): (Vec<_>, Vec<_>) = E
            .iter()
            .cloned()
            .sorted()
            .map(|(x, y)| (x as usize, y as usize))
            .unzip();
        let v = std::iter::repeat(true).take(g.size()).collect();
        let n = g.order();
        assert_eq!(g.sparse_adjacency_matrix(), TriMat::from_triplets((n, n), x, y, v));
    }

    #[test]
    fn dense_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges([(1, 2), (1, 3), (1, 4), (3, 4)]);

        assert_eq!(
            g.dense_incidence_matrix(),
            arr2(&[[1, 1, 1, 0], [-1, 0, 0, 0], [0, -1, 0, 1], [0, 0, -1, -1],])
        );
    }

    #[test]
    fn sparse_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32> + From,
    {
        let g = G::from_edges([(1, 2), (1, 3), (1, 4), (3, 4)]);

        let n = g.order();
        assert_eq!(
            g.sparse_incidence_matrix(),
            TriMat::from_triplets(
                (n, n),
                vec![0, 1, 0, 2, 0, 3, 2, 3],
                vec![0, 0, 1, 1, 2, 2, 3, 3],
                vec![1, -1, 1, -1, 1, -1, 1, -1]
            )
        );
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
