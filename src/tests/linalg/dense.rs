#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::linalg::dense as linalg;
    use crate::traits::{From, Storage};
    use ndarray::{Array, arr1, arr2};
    use approx::*;

    #[test]
    fn average_adjacency_modularity_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        assert_abs_diff_eq!(
            linalg::adjacency_matrix(&g),
            arr2(&[
                [0., 1., 0., 0., 1., 0.],
                [1., 0., 1., 0., 1., 0.],
                [0., 1., 0., 1., 0., 0.],
                [0., 0., 1., 0., 1., 1.],
                [1., 1., 0., 1., 0., 0.],
                [0., 0., 0., 1., 0., 0.],
            ])
        );

        assert_abs_diff_eq!(
            linalg::average_adjacency_matrix(&g),
            arr2(&[
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [6., 9., 6., 9., 9., 3.],
                [2., 3., 2., 3., 3., 1.],
            ]) / linalg::degree_vector(&g).sum()
        );

        assert_abs_diff_eq!(
            linalg::modularity_matrix(&g),
            linalg::adjacency_matrix(&g) - arr2(&[
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [4., 6., 4., 6., 6., 2.],
                [6., 9., 6., 9., 9., 3.],
                [6., 9., 6., 9., 9., 3.],
                [2., 3., 2., 3., 3., 1.],
            ]) / linalg::degree_vector(&g).sum()
        );
    }

    #[test]
    fn degree_vector_matrix_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        assert_abs_diff_eq!(linalg::degree_vector(&g), arr1(&[2., 3., 2., 3., 3., 1.]));
        assert_abs_diff_eq!(linalg::degree_matrix(&g), Array::from_diag(&arr1(&[2., 3., 2., 3., 3., 1.])));
        assert_abs_diff_eq!(linalg::degree_matrix(&g).diag(), linalg::degree_vector(&g));
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

        assert_abs_diff_eq!(
            linalg::laplacian_matrix(&g),
            arr2(&[
                [ 2., -1.,  0.,  0., -1.,  0.],
                [-1.,  3., -1.,  0., -1.,  0.],
                [ 0., -1.,  2., -1.,  0.,  0.],
                [ 0.,  0., -1.,  3., -1., -1.],
                [-1., -1.,  0., -1.,  3.,  0.],
                [ 0.,  0.,  0., -1.,  0.,  1.],
            ])
        );
    }

    #[test]
    fn normalized_adjacency_laplacian_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (2, 3)]);

        assert_abs_diff_eq!(
            linalg::adjacency_matrix(&g),
            arr2(&[
                [0., 1., 0.],
                [1., 0., 1.],
                [0., 1., 0.]
            ])
        );

        assert_abs_diff_eq!(
            linalg::degree_matrix(&g),
            arr2(&[
                [1., 0., 0.],
                [0., 2., 0.],
                [0., 0., 1.]
            ])
        );

        assert_abs_diff_eq!(
            linalg::normalized_adjacency_matrix(&g),
            arr2(&[
                [                0., f32::sqrt(1. / 2.),                 0.],
                [f32::sqrt(1. / 2.),                 0., f32::sqrt(1. / 2.)],
                [                0., f32::sqrt(1. / 2.),                 0.],
            ])
        );

        assert_abs_diff_eq!(
            linalg::normalized_laplacian_matrix(&g),
            arr2(&[
                [                 1., -f32::sqrt(1. / 2.),                  0.],
                [-f32::sqrt(1. / 2.),                  1., -f32::sqrt(1. / 2.)],
                [                 0., -f32::sqrt(1. / 2.),                  1.],
            ])
        );
    }

    #[test]
    fn deformed_laplacian_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        assert_abs_diff_eq!(
            linalg::deformed_laplacian_matrix(&g, None),
            arr2(&[
                [ 3.46938755, -1.57142857,  0.        ,  0.        , -1.57142857,  0.        ],
                [-1.57142857,  4.46938776, -1.57142857,  0.        , -1.57142857,  0.        ],
                [ 0.        , -1.57142857,  3.46938755, -1.57142857,  0.        ,  0.        ],
                [ 0.        ,  0.        , -1.57142857,  4.46938776, -1.57142857, -1.57142857],
                [-1.57142857, -1.57142857,  0.        , -1.57142857,  4.46938776,  0.        ],
                [ 0.        ,  0.        ,  0.        , -1.57142857,  0.        ,  2.46938755]
            ])
        );

        assert_abs_diff_eq!(
            linalg::deformed_laplacian_matrix(&g, Some(1.)),
            linalg::laplacian_matrix(&g)
        );

        assert_abs_diff_eq!(
            linalg::deformed_laplacian_matrix(&g, Some(1.)),
            arr2(&[
                [ 2., -1.,  0.,  0., -1.,  0.],
                [-1.,  3., -1.,  0., -1.,  0.],
                [ 0., -1.,  2., -1.,  0.,  0.],
                [ 0.,  0., -1.,  3., -1., -1.],
                [-1., -1.,  0., -1.,  3.,  0.],
                [ 0.,  0.,  0., -1.,  0.,  1.],
            ])
        );

        assert_abs_diff_eq!(
            linalg::deformed_laplacian_matrix(&g, Some(1.5)),
            arr2(&[
                [ 3.25, -1.5 ,  0.  ,  0.  , -1.5 ,  0.  ],
                [-1.5 ,  4.25, -1.5 ,  0.  , -1.5 ,  0.  ],
                [ 0.  , -1.5 ,  3.25, -1.5 ,  0.  ,  0.  ],
                [ 0.  ,  0.  , -1.5 ,  4.25, -1.5 , -1.5 ],
                [-1.5 , -1.5 ,  0.  , -1.5 ,  4.25,  0.  ],
                [ 0.  ,  0.  ,  0.  , -1.5 ,  0.  ,  2.25]
            ])
        );

        assert_abs_diff_eq!(
            linalg::deformed_laplacian_matrix(&g, Some(2.)),
            arr2(&[
                [ 5., -2.,  0.,  0., -2.,  0.],
                [-2.,  6., -2.,  0., -2.,  0.],
                [ 0., -2.,  5., -2.,  0.,  0.],
                [ 0.,  0., -2.,  6., -2., -2.],
                [-2., -2.,  0., -2.,  6.,  0.],
                [ 0.,  0.,  0., -2.,  0.,  4.]
            ])
        );
    }

    #[test]
    fn fiedler<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        let (true_fiedler_val, true_fiedler_vec) = (
            0.7215863905035553,
            arr1(&[0.41486979, 0.30944167, 0.0692328, -0.22093352, 0.22093352, -0.79354426])
        );
        let (pred_fiedler_val, pred_fiedler_vec) = linalg::fiedler(&g, 1e-8);
        assert_abs_diff_eq!(pred_fiedler_val, true_fiedler_val);
        assert_abs_diff_eq!(pred_fiedler_vec, true_fiedler_vec);
    }

    #[instantiate_tests(<UndirectedAdjacencyListGraph<i32>>)]
    mod adjacency_list_graph {}
}
