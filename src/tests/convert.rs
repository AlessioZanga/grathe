#[cfg(test)]
#[generic_tests::define]
mod undirected {
    use crate::graphs::storages::UndirectedAdjacencyList;
    use crate::traits::Convert;
    use ndarray::{arr2, Array2};
    use sprs::TriMat;

    #[test]
    fn edge_list<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::edge_list() -> EdgeList`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], vec![]),
            // ... one vertex and zero edges,
            (vec![0], vec![], vec![]),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], vec![(0, 0)]),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], vec![]),
            // ... multiple vertices and one edge,
            (vec![0, 1, 2, 3], vec![(0, 1)], vec![(0, 1), (1, 0)]),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                vec![(0, 1), (1, 0), (1, 2), (2, 1), (2, 3), (3, 2)],
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                vec![(1, 58), (1, 71), (3, 58), (3, 75), (58, 1), (58, 3), (71, 1), (75, 3)],
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let k = k.into_iter();
            assert_eq!(g.edge_list(), FromIterator::from_iter(k));
        }
    }

    #[test]
    fn adjacency_list<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::adjacency_list() -> AdjacencyList`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], vec![]),
            // ... one vertex and zero edges,
            (vec![0], vec![], vec![(0, vec![])]),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], vec![(0, vec![0])]),
            // ... multiple vertices and zero edges,
            (
                vec![0, 1, 2, 3],
                vec![],
                vec![(0, vec![]), (1, vec![]), (2, vec![]), (3, vec![])],
            ),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                vec![(0, vec![1]), (1, vec![0]), (2, vec![]), (3, vec![])],
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                vec![(0, vec![1]), (1, vec![0, 2]), (2, vec![1, 3]), (3, vec![2])],
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                vec![
                    (1, vec![58, 71]),
                    (3, vec![58, 75]),
                    (58, vec![1, 3]),
                    (71, vec![1]),
                    (75, vec![3]),
                ],
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let k = k.into_iter().map(|(x, y)| (x, FromIterator::from_iter(y.into_iter())));
            assert_eq!(g.adjacency_list(), FromIterator::from_iter(k));
        }
    }

    #[test]
    fn dense_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::dense_adjacency_matrix() -> Array2<bool>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], Array2::from_elem((0, 0), false)),
            // ... one vertex and zero edges,
            (vec![0], vec![], arr2(&[[false]])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], arr2(&[[true]])),
            // ... multiple vertices and zero edges,
            (
                vec![0, 1, 2, 3],
                vec![],
                arr2(&[
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ]),
            ),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                arr2(&[
                    [false, true, false, false],
                    [true, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ]),
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                arr2(&[
                    [false, true, false, false],
                    [true, false, true, false],
                    [false, true, false, true],
                    [false, false, true, false],
                ]),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                arr2(&[
                    [false, false, true, true, false],
                    [false, false, true, false, true],
                    [true, true, false, false, false],
                    [true, false, false, false, false],
                    [false, true, false, false, false],
                ]),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            assert_eq!(g.dense_adjacency_matrix(), k);
        }
    }

    #[test]
    fn sparse_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::sparse_adjacency_matrix() -> TriMat<bool>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], ((1, 1), vec![], vec![], vec![])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], ((1, 1), vec![0], vec![0], vec![true])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], ((4, 4), vec![], vec![], vec![])),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                ((4, 4), vec![0, 1], vec![1, 0], vec![true, true]),
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                (
                    (4, 4),
                    vec![0, 1, 1, 2, 2, 3],
                    vec![1, 0, 2, 1, 3, 2],
                    vec![true, true, true, true, true, true],
                ),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                (
                    (5, 5),
                    vec![0, 2, 0, 3, 1, 2, 1, 4],
                    vec![2, 0, 3, 0, 2, 1, 4, 1],
                    vec![true, true, true, true, true, true, true, true],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let (s, x, y, z) = k;
            assert_eq!(g.sparse_adjacency_matrix(), TriMat::from_triplets(s, x, y, z));
        }
    }

    #[test]
    fn dense_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::dense_incidence_matrix() -> Array2<i8>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], Array2::from_elem((0, 0), 0)),
            // ... one vertex and zero edges,
            (vec![0], vec![], Array2::from_elem((1, 0), 0)),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], arr2(&[[2]])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], Array2::from_elem((4, 0), 0)),
            // ... multiple vertices and one edge,
            (vec![0, 1, 2, 3], vec![(0, 1)], arr2(&[[1], [1], [0], [0]])),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                arr2(&[[1, 0, 0], [1, 1, 0], [0, 1, 1], [0, 0, 1]]),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                arr2(&[[1, 1, 0, 0], [0, 0, 1, 1], [1, 0, 1, 0], [0, 1, 0, 0], [0, 0, 0, 1]]),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            assert_eq!(g.dense_incidence_matrix(), k);
        }
    }

    #[test]
    fn sparse_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::sparse_incidence_matrix() -> TriMat<i8>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], ((1, 0), vec![], vec![], vec![])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], ((1, 1), vec![0], vec![0], vec![2])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], ((4, 0), vec![], vec![], vec![])),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                ((4, 1), vec![0, 1], vec![0, 0], vec![1, 1]),
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                (
                    (4, 3),
                    vec![0, 1, 1, 2, 2, 3],
                    vec![0, 0, 1, 1, 2, 2],
                    vec![1, 1, 1, 1, 1, 1],
                ),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                (
                    (5, 4),
                    vec![0, 2, 0, 3, 1, 2, 1, 4],
                    vec![0, 0, 1, 1, 2, 2, 3, 3],
                    vec![1, 1, 1, 1, 1, 1, 1, 1],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let (s, x, y, z) = k;
            assert_eq!(g.sparse_incidence_matrix(), TriMat::from_triplets(s, x, y, z));
        }
    }

    #[instantiate_tests(<UndirectedAdjacencyList<i32>>)]
    mod undirected_adjacency_list {}
}

#[cfg(test)]
#[generic_tests::define]
mod directed {
    use crate::graphs::storages::DirectedAdjacencyList;
    use crate::traits::Convert;
    use ndarray::{arr2, Array2};
    use sprs::TriMat;

    #[test]
    fn edge_list<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::edge_list() -> EdgeList`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], vec![]),
            // ... one vertex and zero edges,
            (vec![0], vec![], vec![]),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], vec![(0, 0)]),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], vec![]),
            // ... multiple vertices and one edge,
            (vec![0, 1, 2, 3], vec![(0, 1)], vec![(0, 1)]),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                vec![(0, 1), (1, 2), (2, 3)],
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                vec![(1, 58), (3, 75), (58, 3), (71, 1)],
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let k = k.into_iter();
            assert_eq!(g.edge_list(), FromIterator::from_iter(k));
        }
    }

    #[test]
    fn adjacency_list<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::adjacency_list() -> AdjacencyList`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], vec![]),
            // ... one vertex and zero edges,
            (vec![0], vec![], vec![(0, vec![])]),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], vec![(0, vec![0])]),
            // ... multiple vertices and zero edges,
            (
                vec![0, 1, 2, 3],
                vec![],
                vec![(0, vec![]), (1, vec![]), (2, vec![]), (3, vec![])],
            ),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                vec![(0, vec![1]), (1, vec![]), (2, vec![]), (3, vec![])],
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                vec![(0, vec![1]), (1, vec![2]), (2, vec![3]), (3, vec![])],
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                vec![(1, vec![58]), (3, vec![75]), (58, vec![3]), (71, vec![1]), (75, vec![])],
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let k = k.into_iter().map(|(x, y)| (x, FromIterator::from_iter(y.into_iter())));
            assert_eq!(g.adjacency_list(), FromIterator::from_iter(k));
        }
    }

    #[test]
    fn dense_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::dense_adjacency_matrix() -> Array2<bool>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], Array2::from_elem((0, 0), false)),
            // ... one vertex and zero edges,
            (vec![0], vec![], arr2(&[[false]])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], arr2(&[[true]])),
            // ... multiple vertices and zero edges,
            (
                vec![0, 1, 2, 3],
                vec![],
                arr2(&[
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ]),
            ),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                arr2(&[
                    [false, true, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ]),
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                arr2(&[
                    [false, true, false, false],
                    [false, false, true, false],
                    [false, false, false, true],
                    [false, false, false, false],
                ]),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                arr2(&[
                    [false, false, true, false, false],
                    [false, false, false, false, true],
                    [false, true, false, false, false],
                    [true, false, false, false, false],
                    [false, false, false, false, false],
                ]),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            assert_eq!(g.dense_adjacency_matrix(), k);
        }
    }

    #[test]
    fn sparse_adjacency_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::sparse_adjacency_matrix() -> TriMat<bool>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], ((1, 1), vec![], vec![], vec![])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], ((1, 1), vec![0], vec![0], vec![true])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], ((4, 4), vec![], vec![], vec![])),
            // ... multiple vertices and one edge,
            (vec![0, 1, 2, 3], vec![(0, 1)], ((4, 4), vec![0], vec![1], vec![true])),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                ((4, 4), vec![0, 1, 2], vec![1, 2, 3], vec![true, true, true]),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                ((5, 5), vec![0, 1, 2, 3], vec![2, 4, 1, 0], vec![true, true, true, true]),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let (s, x, y, z) = k;
            assert_eq!(g.sparse_adjacency_matrix(), TriMat::from_triplets(s, x, y, z));
        }
    }

    #[test]
    fn dense_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::dense_incidence_matrix() -> Array2<i8>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], Array2::from_elem((0, 0), 0)),
            // ... one vertex and zero edges,
            (vec![0], vec![], Array2::from_elem((1, 0), 0)),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], arr2(&[[0]])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], Array2::from_elem((4, 0), 0)),
            // ... multiple vertices and one edge,
            (vec![0, 1, 2, 3], vec![(0, 1)], arr2(&[[-1], [1], [0], [0]])),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                arr2(&[[-1, 0, 0], [1, -1, 0], [0, 1, -1], [0, 0, 1]]),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                arr2(&[[-1, 0, 0, 1], [0, -1, 1, 0], [1, 0, -1, 0], [0, 0, 0, -1], [0, 1, 0, 0]]),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            assert_eq!(g.dense_incidence_matrix(), k);
        }
    }

    #[test]
    fn sparse_incidence_matrix<G>()
    where
        G: Convert<Vertex = i32>,
    {
        // Test `G::sparse_incidence_matrix() -> TriMat<i8>`.

        // Test for ...
        let data = [
            // ... zero vertices and zero edges,
            (vec![], vec![], ((0, 0), vec![], vec![], vec![])),
            // ... one vertex and zero edges,
            (vec![0], vec![], ((1, 0), vec![], vec![], vec![])),
            // ... one vertex and one edge,
            (vec![0], vec![(0, 0)], ((1, 1), vec![0], vec![0], vec![0])),
            // ... multiple vertices and zero edges,
            (vec![0, 1, 2, 3], vec![], ((4, 0), vec![], vec![], vec![])),
            // ... multiple vertices and one edge,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1)],
                ((4, 1), vec![0, 1], vec![0, 0], vec![-1, 1]),
            ),
            // ... multiple vertices and multiple edges,
            (
                vec![0, 1, 2, 3],
                vec![(0, 1), (1, 2), (2, 3)],
                (
                    (4, 3),
                    vec![0, 1, 1, 2, 2, 3],
                    vec![0, 0, 1, 1, 2, 2],
                    vec![-1, 1, -1, 1, -1, 1],
                ),
            ),
            // ... random vertices and edges,
            (
                vec![71, 1, 58, 3, 75],
                vec![(71, 1), (1, 58), (58, 3), (3, 75)],
                (
                    (5, 4),
                    vec![0, 2, 1, 4, 2, 1, 3, 0],
                    vec![0, 0, 1, 1, 2, 2, 3, 3],
                    vec![-1, 1, -1, 1, -1, 1, -1, 1],
                ),
            ),
        ];

        // Test for each scenario.
        for (i, j, k) in data {
            let g = G::new(i, j);
            let (s, x, y, z) = k;
            assert_eq!(g.sparse_incidence_matrix(), TriMat::from_triplets(s, x, y, z));
        }
    }

    #[instantiate_tests(<DirectedAdjacencyList<i32>>)]
    mod directed_adjacency_list {}
}
