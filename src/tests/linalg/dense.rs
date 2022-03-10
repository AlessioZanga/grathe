#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::linalg::dense as linalg;
    use crate::traits::{From, Storage};
    use ndarray::{arr1, arr2};

    #[test]
    fn average_adjacency_modularity_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        assert!(linalg::adjacency_matrix(&g).abs_diff_eq(
            &arr2(&[
                [0., 1., 0., 0., 1., 0.],
                [1., 0., 1., 0., 1., 0.],
                [0., 1., 0., 1., 0., 0.],
                [0., 0., 1., 0., 1., 1.],
                [1., 1., 0., 1., 0., 0.],
                [0., 0., 0., 1., 0., 0.],
            ]),
            f32::EPSILON,
        ));

        assert!(linalg::average_adjacency_matrix(&g).abs_diff_eq(
            &(arr2(&[
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [6., 9., 6., 9., 9., 3.],
                [2., 3., 2., 3., 3., 1.],
            ]) / linalg::degree_vector(&g).sum()),
            f32::EPSILON,
        ));

        assert!(linalg::modularity_matrix(&g).abs_diff_eq(
            &(linalg::adjacency_matrix(&g) - arr2(&[
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [6., 9., 6., 9., 9., 3.],
                [2., 3., 2., 3., 3., 1.],
            ]) / linalg::degree_vector(&g).sum()),
            f32::EPSILON,
        ));
    }

    #[test]
    fn degree_vector_matrix_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        assert!(linalg::degree_vector(&g).abs_diff_eq(&arr1(&[2., 3., 2., 3., 3., 1.]), f32::EPSILON));

        assert!(linalg::degree_matrix(&g).abs_diff_eq(
            &ndarray::Array2::from_diag(&arr1(&[2., 3., 2., 3., 3., 1.])),
            f32::EPSILON,
        ));

        assert!(linalg::degree_matrix(&g)
            .diag()
            .abs_diff_eq(&linalg::degree_vector(&g), f32::EPSILON));
    }

    #[test]
    fn incidence_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
    }

    #[test]
    fn laplacian_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        assert!(linalg::laplacian_matrix(&g).abs_diff_eq(
            &arr2(&[
                [ 2., -1.,  0.,  0., -1.,  0.],
                [-1.,  3., -1.,  0., -1.,  0.],
                [ 0., -1.,  2., -1.,  0.,  0.],
                [ 0.,  0., -1.,  3., -1., -1.],
                [-1., -1.,  0., -1.,  3.,  0.],
                [ 0.,  0.,  0., -1.,  0.,  1.],
            ]),
            f32::EPSILON,
        ));
    }

    #[test]
    fn normalized_adjacency_laplacian_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (2, 3)]);

        assert!(linalg::adjacency_matrix(&g).abs_diff_eq(
            &arr2(&[[0., 1., 0.], [1., 0., 1.], [0., 1., 0.]]),
            f32::EPSILON,
        ));

        assert!(linalg::degree_matrix(&g).abs_diff_eq(
            &arr2(&[[1., 0., 0.], [0., 2., 0.], [0., 0., 1.]]),
            f32::EPSILON,
        ));

        assert!(linalg::normalized_adjacency_matrix(&g).abs_diff_eq(
            &arr2(&[
                [                0., f32::sqrt(1. / 2.),                 0.],
                [f32::sqrt(1. / 2.),                 0., f32::sqrt(1. / 2.)],
                [                0., f32::sqrt(1. / 2.),                 0.],
            ]),
            f32::EPSILON,
        ));

        assert!(linalg::normalized_laplacian_matrix(&g).abs_diff_eq(
            &arr2(&[
                [                 1., -f32::sqrt(1. / 2.),                  0.],
                [-f32::sqrt(1. / 2.),                  1., -f32::sqrt(1. / 2.)],
                [                 0., -f32::sqrt(1. / 2.),                  1.],
            ]),
            f32::EPSILON,
        ));
    }

    #[test]
    fn deformed_laplacian_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32>>)]
    mod adjacency_list_graph {}
}
