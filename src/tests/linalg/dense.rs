#[cfg(test)]
#[generic_tests::define]
mod tests {
    use crate::graphs::UndirectedAdjacencyListGraph;
    use crate::linalg::dense as linalg;
    use crate::traits::{From, Storage};
    use ndarray::{arr1, arr2};
    use ndarray_linalg::assert::close_l2;

    #[test]
    fn degree_vector_matrix_adjacency_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (1, 5), (2, 3), (2, 5), (3, 4), (4, 5), (4, 6)]);

        close_l2(
            &linalg::degree_vector(&g),
            &arr1(&[2.0, 3.0, 2.0, 3.0, 3.0, 1.0]),
            f32::EPSILON,
        );

        close_l2(
            &linalg::degree_matrix(&g),
            &ndarray::Array2::from_diag(&arr1(&[2.0, 3.0, 2.0, 3.0, 3.0, 1.0])),
            f32::EPSILON,
        );

        close_l2(
            &linalg::adjacency_matrix(&g),
            &arr2(&[
                [0.0, 1.0, 0.0, 0.0, 1.0, 0.0],
                [1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0, 1.0, 1.0],
                [1.0, 1.0, 0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            ]),
            f32::EPSILON,
        );
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

        close_l2(
            &linalg::laplacian_matrix(&g),
            &arr2(&[
                [2.0, -1.0, 0.0, 0.0, -1.0, 0.0],
                [-1.0, 3.0, -1.0, 0.0, -1.0, 0.0],
                [0.0, -1.0, 2.0, -1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 3.0, -1.0, -1.0],
                [-1.0, -1.0, 0.0, -1.0, 3.0, 0.0],
                [0.0, 0.0, 0.0, -1.0, 0.0, 1.0],
            ]),
            f32::EPSILON,
        );
    }

    #[test]
    fn normalized_laplacian_matrix<T>()
    where
        T: Storage + From<Vertex = i32>,
    {
        let g = T::from_edges([(1, 2), (2, 3)]);

        close_l2(
            &linalg::adjacency_matrix(&g),
            &arr2(&[[0.0, 1.0, 0.0], [1.0, 0.0, 1.0], [0.0, 1.0, 0.0]]),
            f32::EPSILON,
        );

        close_l2(
            &linalg::degree_matrix(&g),
            &arr2(&[[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 1.0]]),
            f32::EPSILON,
        );

        close_l2(
            &linalg::normalized_laplacian_matrix(&g),
            &arr2(&[
                [1.0, -f32::sqrt(1.0 / 2.0), 0.0],
                [-f32::sqrt(1.0 / 2.0), 1.0, -f32::sqrt(1.0 / 2.0)],
                [0.0, -f32::sqrt(1.0 / 2.0), 1.0],
            ]),
            f32::EPSILON,
        );
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
